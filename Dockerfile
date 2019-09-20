from circleci/rust:latest

# Setup rust
RUN rustup install nightly \
    && rustup default nightly

USER root

# Install python
ARG PYTHON_VERSION
RUN [ -n $PYTHON_VERSION ] \
    && apt-get install \
        python$PYTHON_VERSION \
        python$PYTHON_VERSION-dev

USER circleci

CMD ["/bin/sh"]
