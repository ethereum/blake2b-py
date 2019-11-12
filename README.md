# blake2b-py

[![Build Status](https://circleci.com/gh/davesque/blake2b-py.svg?style=shield)](https://circleci.com/gh/davesque/blake2b-py)

Blake2b hashing in Rust with Python bindings.

## Building/releasing

### Building wheels for macOS

First, get a mac or provison a mac VM somehow :/.  Sorry, no more specific
instructions than that for the moment.  Then, in your mac environment:
```bash
pip install -r requirements-dev.txt
make build-local
```

You can also use the included Travis CI config to build the wheels in CI.
You'll need to modify the config to upload build artifacts to an S3 account
since Travis doesn't have any built-in artifact storage.

### Building wheels for Linux

Ensure you have docker installed, then use the `build-manylinux` make target:
```bash
make build-manylinux
```

If you get errors about not being able to include files in the source
distribution, make sure there are no virtual environments in your project
directory that are being detected by the build routines in the docker
container.  Then, try again.

You can also use the included Travis CI config to build the wheels in CI.
You'll need to modify the config to upload build artifacts to an S3 account
since Travis doesn't have any built-in artifact storage.

### Building wheels for Windows

As with the mac build process, you'll need to somehow get a Windows environment
running whether locally, inside a VM, or on a CI service.  If you're building
manually, it's the same as mac:
```bash
pip install -r requirements-dev.txt
make build-local
```

You can also use the included Appveyor config to build the wheels in CI.  You
can download any resulting build artifacts through the Appveyor web interface.

### Releasing

Gather your wheels and source distribution into your local `target/wheels`
directory, then use the `publish` make target:
```bash
make publish
```
