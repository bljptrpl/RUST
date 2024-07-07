#!/bin/bash

# Function to build the Docker image
docker_image() {
    echo "Building the Docker image..."
    docker build -t beningojw/embedded-rust-dev -f docker/Dockerfile .
    echo "Docker image built successfully."
}

# Function to run the Docker container
docker_run() {
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        echo "Running Docker container on Windows..."
        cmd /c "docker run --rm -it --privileged -v "%CD%:/home/app" beningojw/embedded-rust-dev:latest"
    else
        echo "Running Docker container on MacOS/Linux..."
        docker run --rm -it --privileged -v "${PWD}:/home/app" beningojw/embedded-rust-dev:latest bash
    fi
}

# Check the first argument to decide which function to call
case "$1" in
    docker_image)
        docker_image
        ;;
    docker_run)
        docker_run
        ;;
    *)
        echo "Usage: $0 {docker_image|docker_run}"
        exit 1
        ;;
esac
