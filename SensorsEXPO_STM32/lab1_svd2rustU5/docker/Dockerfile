# Use the latest version of Ubuntu as the base image
FROM ubuntu:22.04

# Set the maintainer label
LABEL maintainer="dvillalva21@gmail.com"

# Set environment variables to non-interactive to prevent prompts
ENV DEBIAN_FRONTEND=non-interactive

# Update package lists, install basic tools, toolchains, stlink-tools, and clean up in one RUN command
RUN apt-get update -y && \
    apt-get install -y --no-install-recommends \
    autoconf \
    automake \
    curl \
    build-essential \
    git \
    libtool \
    make \
    pkg-config \
    ca-certificates \
    software-properties-common \
    xz-utils \
    libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
# TODO: Add the code to install Rust using rustup


# Add Rust to the PATH environment variable
ENV PATH=/root/.cargo/bin:$PATH

# cargo-binutils is a collection of Cargo subcommands that make
# it easy to use the LLVM tools that are shipped with the Rust toolchain.
# These tools include the LLVM versions of objdump, nm and size and are
# used for inspecting binaries.
# referece: https://docs.rust-embedded.org/book/intro/tooling.html
RUN cargo install cargo-binutils
RUN rustup component add llvm-tools

# cargo-generate is a developer tool to help you get up and running quickly
# with a new Rust project by leveraging a pre-existing git repository as a template.
RUN cargo install cargo-generate

# svd2rust is a tool to generate Rust register maps (structs) from SVD files.
# You can use this tool to generate a Peripheral Access Crate (PAC) for your microcontroller.
RUN cargo install svd2rust

# form is a tool to format Rust code according to a style guide.
# It is a wrapper around rustfmt and can be used to format code in a consistent manner.
RUN cargo install form

###################################
# Install cross compilation support for ARM Cortex-M. Uncomment the needed architectures.
###################################

#Cortex-M0, M0+, and M1 (ARMv6-M architecture):
#RUN rustup target add thumbv6m-one-eabi

#Cortex-M3 (ARMv7-M architecture):
#RUN rustup target add thumbv7m-none-eabi

#Cortex-M4 and M7 without FPU (ARMv7E-M architecture):
#RUN rustup target add thumbv7em-none-eabi

#Cortex-M4F and M7F with FPU (ARMv7E-M architecture):
#RUN rustup target add thumbv7em-none-eabihf

#Cortex-M23 (ARMv8-M Baseline architecture):
#RUN rustup target add thumbv8m.base-none-eabi

#Cortex-M33 (ARMv8-M Mainline architecture):
RUN rustup target add thumbv8m.main-none-eabi

#Cortex-M33F with FPU (ARMv8-M Mainline architecture):
#RUN rustup target add thumbv8m.main-none-eabihf

#Cortex-M55 (ARMv8.1-M Mainline architecture):
#RUN rustup target add thumbv8.1m.main-none-eabi

#Cortex-M55F with FPU (ARMv8.1-M Mainline architecture):
#Cortex-M55 with Helium (ARMv8.1-M Mainline architecture):
#Cortex-M55 with Helium and DSP (ARMv8.1-M Mainline architecture):
#RUN rustup target add thumbv8.1m.main-none-eabihf

# Setup a tool directory in /home/dev and download the ARM toolchain
WORKDIR /home/dev

# Install GNU Arm toolchain (Version 13.2.Rel1)
RUN curl -LO https://developer.arm.com/-/media/Files/downloads/gnu/13.2.rel1/binrel/arm-gnu-toolchain-13.2.rel1-x86_64-arm-none-eabi.tar.xz && \
    tar xf arm-gnu-toolchain-13.2.rel1-x86_64-arm-none-eabi.tar.xz && \
    rm arm-gnu-toolchain-13.2.rel1-x86_64-arm-none-eabi.tar.xz

# Add the GNU Arm toolchain to the PATH
ENV PATH="/home/dev/arm-gnu-toolchain-13.2.rel1/bin:${PATH}"

# Create and set the working directory to /home/app
WORKDIR /home/app

# Set the default command to bash
CMD ["/bin/bash"]
