# blake2b-py

Blake2b hashing in Rust with Python bindings.

## Building/releasing

To build and publish a release, follow these steps:

### Bump the version

First, bump the package version with the included make target:
```bash
make bumpversion bump=patch
```
The above invocation bumps the "patch" version of a semantic version number
("x" in "1.2.x").  Other valid version types are "major" and "minor".  The
version is bumped by modifying source files that contain the version number,
creating a new commit that includes those modifications, then tagging that
commit with the new version.  The new commit and tag are then pushed to the
upstream repository.

### Building & Releasing

Packages are build and distributed via Github Actions as soon as a tag is
pushed to the remote repository which is taken care of by the bumpversion command.

### Developing

You'll need to have [Maturin](https://pyo3.rs/v0.16.4/) installed on your machine.
Create a virtual environment, and then you can do:

```sh
$ pip install maturin
$ maturin develop
```

to install the dependencies. You may need to specify the
`MACOSX_DEPLOYMENT_TARGET` environment variable to your version of MacOS.

#### Run the tests

Running `make test_all` will run all the tests.
