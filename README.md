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

### Build the wheels and source distribution

#### Building wheels for macOS

Get a mac or provison a mac VM somehow :/.  Sorry, no more specific
instructions than that for the moment.  Then, in your mac environment:
```bash
pip install -r requirements-dev.txt
make build-local
```
You can also use the included Travis CI config to build the wheels in CI.
You'll need to modify the config to upload build artifacts to an S3 account
since Travis doesn't have any built-in artifact storage.

#### Building wheels for Linux

Ensure you have docker installed, then use the `build-manylinux` make target:
```bash
make build-manylinux
```
If you get errors about not being able to include files in the source
distribution, make sure there are no virtual environments in your project
directory that are being detected by the build routines in the docker
container.  Then, try again.  By default, the source distribution is built
during this step by the linux build routines.

You can also use the included Travis CI config to build the wheels in CI.
You'll need to modify the config to upload build artifacts to an S3 account
since Travis doesn't have any built-in artifact storage.

#### Building wheels for Windows

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
