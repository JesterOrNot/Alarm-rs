FROM gitpod/workspace-full-vnc

USER gitpod

RUN sudo apt-get -q update \
    && sudo apt-get install -yq \
        libgtk-3-dev \
        libnss3-dev \
    && sudo rm -rf /var/lib/apt/lists/*

USER root