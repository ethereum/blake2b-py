from circleci/rust:latest

# Setup rust
RUN rustup install nightly \
        && rustup default nightly

# Setup pyenv
RUN curl https://pyenv.run | bash
RUN printf '\nexport PATH="/home/circleci/.pyenv/bin:$PATH"\n' >> /home/circleci/.bashrc \
    && printf 'eval "$(pyenv init -)"\n' >> /home/circleci/.bashrc \
    && printf 'eval "$(pyenv virtualenv-init -)"\n' >> /home/circleci/.bashrc

CMD ["/bin/sh"]
