from circleci/rust:latest

USER root

RUN apt-get install -y python3-dev python-dev

RUN rustup install nightly \
        && rustup default nightly

USER circleci

CMD ["/bin/sh"]
