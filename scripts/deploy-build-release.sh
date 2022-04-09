#!/bin/bash
set -e

## Deploy the release build on an Rpi

. ./scripts/-base.sh

cargo build --target="${TARGET_ARCH}" --release
tar -czf "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${LOCAL_RELEASE_BINARY_SOURCE_PATH}" "${CONFIG_FILE}" "${RUN_SIRIUS_HYDRA_FILE}"

# create the scripts target path
sshpass -p "$SSH_PASSWORD" ssh -o StrictHostKeyChecking=no "${TARGET_HOST}" "mkdir --parents ${TARGET_RELEASE_PATH}"

# copy the compressed binary and config archive file into the target path
sshpass -p "$SSH_PASSWORD" scp "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${TARGET_HOST}:${TARGET_RELEASE_PATH}"

# decompress the archive
sshpass -p "$SSH_PASSWORD" ssh -o StrictHostKeyChecking=no "${TARGET_HOST}" "tar --transform 's/.*\///g' -xzf ${TARGET_RELEASE_PATH}/${LOCAL_COMPRESSED_EXECUTABLE} -C ${TARGET_RELEASE_PATH}"

# copy ./scripts into the remote Rpi
sshpass -p "$SSH_PASSWORD" scp -rp "${LOCAL_SCRIPTS_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_RELEASE_PATH}"

sshpass -p "$SSH_PASSWORD" ssh -t "${TARGET_HOST}" "${TARGET_RELEASE_PATH}/scripts/-deploy-build-release.sh"
