#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

# Find the current version from Cargo.toml
VERSION=`grep "^version" ./node/Cargo.toml | egrep -o "([0-9\.]+)"`
GITUSER=webb-tools
IMAGE_NAME=substrate-contracts-node

# Build the image
echo "Building ${GITUSER}/${IMAGE_NAME}:latest docker image, hang on!"
time docker build -f ./docker/contract-node.Dockerfile -t ${GITUSER}/${IMAGE_NAME}:latest .
docker tag ${GITUSER}/${IMAGE_NAME}:latest ${GITUSER}/${IMAGE_NAME}:v${VERSION}

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${IMAGE_NAME}

popd
