FROM ubuntu:jammy

# install dependencies
RUN set -eux; \
    DEBIAN_FRONTEND="noninteractive"; \
    apt-get update; \
    apt-get upgrade -y; \
    apt-get install -y python3-venv apt-transport-https software-properties-common gnupg wget git zsh; \
    rm -rf /var/lib/apt/lists/*

# install modular SDK
RUN set -eux; \
    KEYRING_LOCATION=/usr/share/keyrings/modular-installer-archive-keyring.gpg; \
    MODULAR_INSTALLER=https://dl.modular.com/public/installer; \
    wget -qO - "$MODULAR_INSTALLER/gpg.0E4925737A3895AD.key" | gpg --dearmor >> "$KEYRING_LOCATION"; \
    wget -qO - "$MODULAR_INSTALLER/config.deb.txt?distro=ubuntu&codename=jammy" > /etc/apt/sources.list.d/modular-installer.list; \
    apt-get update; \
    apt-get install -y modular; \
    rm -rf /var/lib/apt/lists/*

# install LLVM for llvm-symbolizer
RUN set -eux; \
    wget -qO- https://apt.llvm.org/llvm-snapshot.gpg.key | tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc; \
    add-apt-repository "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main"; \
    apt-get update; \
    apt-get install -y llvm; \
    rm -rf /var/lib/apt/lists/*

# create user
ARG USERNAME=dev
ENV MODULAR_HOME="/home/$USERNAME/.modular"
ENV PATH="$MODULAR_HOME/pkg/packages.modular.com_mojo/bin:$PATH"
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$USERNAME"; \
    mkdir -p "/home/$USERNAME/repo"; \
    chown -R "$USERNAME:$USERNAME" "/home/$USERNAME"
USER "$USERNAME"

# install Mojo
RUN set -eux; \
    modular auth mut_9661c1c514ad48da93c679da214385fb; \
    modular install mojo

# entrypoint
CMD ["sleep", "infinity"]
