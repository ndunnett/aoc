FROM rustlang/rust:nightly
ARG USERNAME
ARG REPO

ENV AOC_USERNAME="$USERNAME" AOC_REPO="$REPO"

# install mold and clang for faster linking, zsh, sudo and git for convenience
ARG DEBIAN_FRONTEND=noninteractive
RUN set -eux; apt update; \
    apt install -y mold clang zsh sudo git; \
    apt clean; rm -rf /var/lib/apt/lists/*

# create non-root sudo user
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$AOC_USERNAME"; \
    echo "$USERNAME ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/$AOC_USERNAME"; \
    chmod 0440 "/etc/sudoers.d/$AOC_USERNAME"

# add cargo config for multithreaded frontend and use mold linker
COPY fast_cargo_config.toml "/home/$AOC_USERNAME/.cargo/config.toml"

# create workspace dir and fix ownership
RUN set -eux; \
    mkdir -p "$AOC_REPO"; \
    chown -R "$AOC_USERNAME:$AOC_USERNAME" "$AOC_REPO" "/home/$AOC_USERNAME"
WORKDIR "$AOC_REPO"

# create shellscript to conveniently call AOC runner
RUN set -x; \
    echo "#!/bin/sh" > /usr/local/bin/aoc; \
    echo "cd $AOC_REPO/rust && cargo run -- \$@" >> /usr/local/bin/aoc; \
    chmod +x /usr/local/bin/aoc

# change user
USER "$AOC_USERNAME"

# add local bin to path
ENV PATH="/home/$AOC_USERNAME/.local/bin:$PATH"

# install clippy and rustfmt
RUN set -eux; rustup component add clippy rustfmt

# replace entrypoint
CMD sleep infinity