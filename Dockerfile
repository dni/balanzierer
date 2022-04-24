FROM ubuntu:20.04 as builder
RUN apt-get update -qq && \
    DEBIAN_FRONTEND=noninteractive apt-get install -qq -y --no-install-recommends \
        autoconf \
        automake \
        build-essential \
        ca-certificates \
        curl \
        dirmngr \
        gettext \
        git \
        gnupg \
        libpq-dev \
        libtool \
        libffi-dev \
        python3 \
        python3-dev \
        python3-mako \
        python3-pip \
        python3-venv \
        python3-setuptools \
        wget \
        libsqlite3-dev

WORKDIR /opt/lightningd
COPY ./lightning /tmp/lightning
RUN git clone --recursive /tmp/lightning . && \
    git checkout $(git --work-tree=/tmp/lightning --git-dir=/tmp/lightning/.git rev-parse HEAD)
ARG DEVELOPER=0
ENV PYTHON_VERSION=3
RUN curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/install-poetry.py | python3 - \
    && pip3 install -U pip \
    && pip3 install -U wheel \
    && /root/.local/bin/poetry config virtualenvs.create false \
    && /root/.local/bin/poetry install

RUN ./configure --prefix=/tmp/lightning_install && make -j3 DEVELOPER=${DEVELOPER} && make install

FROM ubuntu:20.04 as final

RUN apt-get update && apt-get install -y --no-install-recommends socat inotify-tools python3 python3-pip libpq5\
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /tmp/lightning_install/ /usr/local/
