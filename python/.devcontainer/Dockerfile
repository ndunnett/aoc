FROM ubuntu:noble
ARG USERNAME=dev
ARG REPO=/ws

ENV AOC_USERNAME="$USERNAME" AOC_REPO="$REPO"

# install zsh, sudo and git for convenience
ARG DEBIAN_FRONTEND=noninteractive
RUN set -eux; apt update; apt full-upgrade -y; \
    apt install -y zsh sudo git wget; \
    apt autoremove -y; apt clean; rm -rf /var/lib/apt/lists/*

# delete default user on new ubuntu images
RUN set -eux; \
    grep ubuntu /etc/passwd && \
    touch /var/mail/ubuntu && \
    chown ubuntu /var/mail/ubuntu && \
    userdel -r ubuntu

# create non-root sudo user
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$AOC_USERNAME"; \
    echo "$USERNAME ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/$AOC_USERNAME"; \
    chmod 0440 "/etc/sudoers.d/$AOC_USERNAME"

# create workspace dir and fix ownership
RUN set -eux; \
    mkdir -p "$AOC_REPO"; \
    chown -R "$AOC_USERNAME:$AOC_USERNAME" "$AOC_REPO" "/home/$AOC_USERNAME"
WORKDIR "$AOC_REPO"

# create shellscript to conveniently call AOC script runner
RUN set -x; \
    echo "#!/bin/sh" > /usr/local/bin/aoc; \
    echo "$AOC_REPO/python/.venv/bin/python $AOC_REPO/python/run.py \$@" >> /usr/local/bin/aoc; \
    chmod +x /usr/local/bin/aoc

# change user
USER "$AOC_USERNAME"

# install uv
COPY --from=ghcr.io/astral-sh/uv:0.5 /uv /uvx /bin/
ENV UV_LINK_MODE=copy

# create virtual environment
COPY --chown="$AOC_USERNAME:$AOC_USERNAME" requirements.txt "$AOC_REPO/requirements.txt"
RUN set -eux; \
    uv venv .venv; \
    uv pip install -r requirements.txt

# replace entrypoint
CMD sleep infinity