#!/bin/bash
set -e

. ./env.config

readonly TARGET_ARCH="arm-unknown-linux-musleabi"

readonly TARGET_HOST=$SSH_ADDRESS
readonly REMOTE_SIRIUS_HYDRA_SOURCE_PATH="/home/pi/hydra-src"

readonly TARGET_DEBUG_PATH="/home/pi/sirius-hydra-debug"
readonly TARGET_RELEASE_PATH="/home/pi/sirius-hydra-release"

readonly CONFIG_FILE="config.yaml"
readonly RUN_SIRIUS_HYDRA_FILE="scripts/run-sirius-hydra.sh"
readonly LOCAL_SCRIPTS_SOURCE_PATH="./scripts"
readonly LOCAL_COMPRESSED_EXECUTABLE="sirius-hydra.tar.gz"
readonly LOCAL_COMPRESSED_EXECUTABLE_PATH="target/sirius-hydra.tar.gz"
readonly LOCAL_DEBUG_BINARY_SOURCE_PATH="./target/${TARGET_ARCH}/debug/sirius-hydra"
readonly LOCAL_RELEASE_BINARY_SOURCE_PATH="./target/${TARGET_ARCH}/release/sirius-hydra"



