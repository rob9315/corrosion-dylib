ARG ROS_DISTRO=noetic
FROM osrf/ros:$ROS_DISTRO-desktop-full

# dependencies and tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    sudo git vim nano curl tree wget build-essential dirmngr gnupg2 clang apt-utils \
    && rm -rf /var/lib/apt/lists/*

## cmake ppa ##
RUN curl -fsSL https://apt.kitware.com/keys/kitware-archive-latest.asc \
    | gpg --dearmor > /usr/share/keyrings/kitware-archive-keyring.gpg
RUN echo "deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ $(lsb_release -cs) main" \
    > /etc/apt/sources.list.d/kitware.list

# upgrade cmake for corrosion
RUN apt-get update && apt-get upgrade -y cmake && rm -rf /var/lib/apt/lists/*

# catkin_tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3-catkin-tools \
    && rm -rf /var/lib/apt/lists/*

## add entrypoint ##
RUN echo "#!/bin/bash\nsource /opt/ros/${ROS_DISTRO}/setup.bash" > /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

## add user ##
ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
USER $USERNAME
SHELL ["bash", "-c"]
RUN mkdir -p ~/.local/share/bash-completion/completions

## rust ##
RUN curl -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y \
    && source ~/.cargo/env \
    && rustup completions bash > ~/.local/share/bash-completion/completions/rustup \
    && rustup completions bash cargo > ~/.local/share/bash-completion/completions/cargo

## adjust bashrc ##
RUN echo 'source /entrypoint.sh' >> ~/.bashrc
