FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
RUN sudo apt update && sudo apt upgrade -y && sudo apt install -y libusb-1.0-0-dev libudev-dev
RUN cargo install probe-run