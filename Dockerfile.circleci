from buildpack-deps:xenial

# Rust set up code borrowed from official docker-rust repo:
# https://github.com/rust-lang/docker-rust/blob/d55d56e152da6cffe980baee408cd18716df4b44/1.37.0/buster/Dockerfile
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=nightly

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='a46fe67199b7bcbbde2dcbc23ae08db6f29883e260e23899a88b9073effc9076' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='6af5abbbae02e13a9acae29593ec58116ab0e3eb893fa0381991e8b0934caea1' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='51862e576f064d859546cca5f3d32297092a850861e567327422e65b60877a1b' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='91456c3e6b2a3067914b3327f07bc182e2a27c44bff473263ba81174884182be' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.18.3/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Setup deadsnakes ppa for multiple python versions
RUN deadsnakes_list="/etc/apt/sources.list.d/deadsnakes.list"; \
    printf 'deb http://ppa.launchpad.net/deadsnakes/ppa/ubuntu xenial main\n' >> $deadsnakes_list \
    && printf 'deb-src http://ppa.launchpad.net/deadsnakes/ppa/ubuntu xenial main\n' >> $deadsnakes_list \
    && gpg --keyserver keyserver.ubuntu.com --recv-keys 6A755776 \
    && gpg --export 6A755776 | apt-key add - \
    && apt-get update

ARG PYTHON_VERSION

# Install python
RUN apt-get install -y \
        python$PYTHON_VERSION-dev \
        python$PYTHON_VERSION-venv

# Create virtualenv and activate by default
RUN python$PYTHON_VERSION -mvenv /venv
ENV VIRTUAL_ENV=/venv
ENV PATH=/venv/bin:$PATH

RUN pip install --upgrade pip setuptools wheel

CMD ["/bin/bash"]
