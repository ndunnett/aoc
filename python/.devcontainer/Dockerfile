FROM ndunnett/python:latest

# install dependencies
RUN set -eux; \
    DEBIAN_FRONTEND="noninteractive"; \
    apt-get update; \
    apt-get install -y wget git zsh; \
    rm -rf /var/lib/apt/lists/*
ARG USERNAME=dev

# create shellscript to conveniently call AOC script runner
RUN set -x; \
    echo "#!/bin/sh" > /usr/local/bin/aoc; \
    echo "python /home/$USERNAME/repo/python/run.py \$@" >> /usr/local/bin/aoc; \
    chmod +x /usr/local/bin/aoc

# create user
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$USERNAME"; \
    mkdir -p "/home/$USERNAME/repo"; \
    chown -R "$USERNAME:$USERNAME" "/home/$USERNAME"
USER "$USERNAME"

# pip install requirements
COPY --chown="$USERNAME:$USERNAME" requirements.txt /opt/requirements.txt
RUN set -eux; \
    pip install -r /opt/requirements.txt

# entrypoint
CMD ["sleep", "infinity"]
