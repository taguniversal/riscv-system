FROM ubuntu:22.04

ARG USER_ID
ARG GROUP_ID

RUN apt-get update && apt-get install -y \
    build-essential cmake git python3 python3-pip \
    gcc-riscv64-unknown-elf ninja-build curl sudo

RUN groupadd -g ${GROUP_ID} developer && \
    useradd -m -u ${USER_ID} -g developer developer && \
    usermod -aG sudo developer

USER developer
WORKDIR /home/developer

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/developer/.cargo/bin:${PATH}"
RUN rustup default nightly
RUN rustup target add --toolchain nightly riscv64gc-unknown-none-elf

WORKDIR /work