# Initial Setup

## VS Code


## Setting up Docker


Objective: Setup a Docker Container
Problem(s): Not able to setup Docker Desktop
Solution: Setup apt repository

https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository

1. Clear out all the previous unofficial packages.
    
    `docker.io
    docker-compose
    docker-compose-v2
    docker-doc
    podman-docker`

    `$ for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do sudo apt-get remove $pkg; done`
2. Install official GPG key & Apt Source

    ### Add Docker's official GPG key:
    sudo apt-get update
    sudo apt-get install ca-certificates curl
    sudo install -m 0755 -d /etc/apt/keyrings
    sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
    sudo chmod a+r /etc/apt/keyrings/docker.asc

    ### Add the repository to Apt sources:
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    sudo apt-get update
3. Install Docker Packages
    `sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin`
4. Hello World

    *Start Docker container* 
    `systemctl start docker`

    Run command
    `sudo docker run hello-world`

        [sudo] password for vboxuser: 

        Hello from Docker!
        This message shows that your installation appears to be working correctly.

        To generate this message, Docker took the following steps:
        1. The Docker client contacted the Docker daemon.
        2. The Docker daemon pulled the "hello-world" image from the Docker Hub.
            (amd64)
        3. The Docker daemon created a new container from that image which runs the
            executable that produces the output you are currently reading.
        4. The Docker daemon streamed that output to the Docker client, which sent it
            to your terminal.

        To try something more ambitious, you can run an Ubuntu container with:
        $ docker run -it ubuntu bash

        Share images, automate workflows, and more with a free Docker ID:
        https://hub.docker.com/

        For more examples and ideas, visit:
        https://docs.docker.com/get-started/

# Lab Exercise 1 – Containerize your Rust Environment

Notes: Ensure you have watched and/or attended the Session 1 presentation.
Overview:
In this lab, you will set up the Rust environment on your machine inside of a Docker container.
Lab Instructions:
For this lab, you’ll need your PC and an internet connection. You won’t need your development board quite yet. The goal is to have a containerized development environment that you can use to build Rust applications.
The major steps that you will take to accomplish this includes:

1) Creating a Docker Container

2) Verify your Docker Container with Rust

3) Create a “Hello World!” application

4) Modify “Hello World!” to become familiar with Rust functions

./svd2rustU5/devManager.sh docker_image
Building the Docker image...
ERROR: permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Head "http://%2Fvar%2Frun%2Fdocker.sock/_ping": dial unix /var/run/do
cker.sock: connect: permission denied                                                                                                                                             Docker image built successfully.
