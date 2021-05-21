FROM rust:latest

RUN sudo apt-get update \
 && sudo apt-get install -y \
    libudev-dev zlib1g-dev alsa libasound2-dev \
 && sudo rm -rf /var/lib/apt/lists/*
