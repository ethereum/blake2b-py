#!/usr/bin/env python3

import json
import logging
import os
from typing import (
    Any,
    Dict,
)
import urllib.parse
import urllib.request

logger = logging.getLogger(__name__)

GITHUB_PROJECT = os.getenv('GITHUB_PROJECT')

APPVEYOR_TOKEN = os.getenv('CIRCLE_TOKEN')

CIRCLE_TOKEN = os.getenv('CIRCLE_TOKEN')
CIRCLE_JOB_NAME = os.getenv('CIRCLE_JOB_NAME')

OUTPUT_DIR = os.getenv('OUTPUT_DIR')


def get_url(url: str, *, query: Dict = None, bearer_token: str = None) -> Any:
    if query is not None:
        query_str = urllib.parse.urlencode(query)
        req_url = url + '?' + query_str
    else:
        req_url = url

    req = urllib.request.Request(req_url)
    req.add_header('Accept', 'application/json')
    if bearer_token is not None:
        req.add_unredirected_header('Authorization', f'Bearer {bearer_token}')

    logger.debug(f'Request url: {req.get_full_url()}')
    logger.debug(f'Request headers: {req.header_items()}')

    with urllib.request.urlopen(req) as f:
        return f.read(), req_url


def get_json_from_url(
    url: str,
    *,
    query: Dict = None,
    bearer_token: str = None,
) -> Any:
    res, req_url = get_url(url, query=query, bearer_token=bearer_token)
    res_decoded = res.decode('utf8')

    logger.debug(f'Response:\n{res_decoded}')

    return json.loads(res)


def get_file_from_url(
    url: str,
    *,
    query: Dict = None,
    bearer_token: str = None,
) -> Any:
    res, req_url = get_url(url, query=query, bearer_token=bearer_token)

    logger.debug(
        f'Binary response: {res[:10]} (truncated)',  # noqa: E501
    )

    return res


def get_circle_artifacts(project, job_name, token):
    logger.info(
        f'Requesting recent build nums for Circle CI job "{job_name}"...',
    )

    build_nums_res = get_json_from_url(
        f'https://circleci.com/api/v1.1/project/github/{project}',
        query={
            'filter': 'successful',
            'circle-token': token,
        },
    )
    build_nums = [
        r['build_num'] for r in build_nums_res
        if r['workflows']['job_name'] == job_name
    ]

    logger.info(
        f'...found: {build_nums}',
    )

    build_num = build_nums[0]

    logger.info(
        f'Requesting artifacts for build num {build_num}...',
    )

    artifacts_res = get_json_from_url(
        f'https://circleci.com/api/v1.1/project/github/{project}/{build_num}/artifacts',  # noqa: E501
        query={'circle-token': token},
    )
    artifact_urls = [r['url'] for r in artifacts_res]
    artifact_names = [u.rsplit('/', 1)[1] for u in artifact_urls]

    artifact_files = []
    for url in artifact_urls:
        logger.info(f'  Requesting artifact at url "{url}"...')
        artifact_files.append(get_file_from_url(url))

    logger.info(
        '...done',
    )

    return zip(artifact_names, artifact_files)


def get_appveyor_artifacts(project, token):
    logger.info('Requesting build id for most recent Appveyor job...')

    job_id_res = get_json_from_url(
        f'https://ci.appveyor.com/api/projects/{project}',
        bearer_token=token,
    )
    job_id = job_id_res['build']['jobs'][0]['jobId']

    logger.info(
        f'...found: {job_id}',
    )

    logger.info('Requesting artifacts for job id "{job_id}"...')

    artifacts_res = get_json_from_url(
        f'https://ci.appveyor.com/api/buildjobs/{job_id}/artifacts',
        bearer_token=token,
    )
    artifact_names = [r['fileName'] for r in artifacts_res]

    artifact_files = []
    for name in artifact_names:
        url = f'https://ci.appveyor.com/api/buildjobs/{job_id}/artifacts/{name}'  # noqa: E501

        logger.info(f'  Requesting artifact at url "{url}"...')
        artifact_files.append(get_file_from_url(url, bearer_token=token))

    logger.info('...done')

    return zip(artifact_names, artifact_files)


if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)

    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR)

    circleci_res = get_circle_artifacts(
        GITHUB_PROJECT,
        CIRCLE_JOB_NAME,
        CIRCLE_TOKEN,
    )
    for name, file_content in circleci_res:
        with open(os.path.join(OUTPUT_DIR, name), 'wb') as f:
            f.write(file_content)

    appveyor_res = get_appveyor_artifacts(
        GITHUB_PROJECT,
        APPVEYOR_TOKEN,
    )
    for name, file_content in appveyor_res:
        with open(os.path.join(OUTPUT_DIR, name), 'wb') as f:
            f.write(file_content)
