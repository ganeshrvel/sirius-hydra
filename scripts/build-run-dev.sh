#!/bin/bash
set -e

## Local developement and run the debug build on an Rpi

. ./scripts/-base.sh

cargo build --target="${TARGET_ARCH}"
tar -czf "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${LOCAL_DEBUG_BINARY_SOURCE_PATH}" "${CONFIG_FILE}"

# create the scripts target path
sshpass -p "$SSH_PASSWORD" ssh -o StrictHostKeyChecking=no "${TARGET_HOST}" "mkdir --parents ${TARGET_DEBUG_PATH}"

# copy the compressed binary and config archive file into the target path
sshpass -p "$SSH_PASSWORD" scp "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${TARGET_HOST}:${TARGET_DEBUG_PATH}"

# decompress the archive
sshpass -p "$SSH_PASSWORD" ssh -o StrictHostKeyChecking=no "${TARGET_HOST}" "tar --transform 's/.*\///g' -xzf ${TARGET_DEBUG_PATH}/${LOCAL_COMPRESSED_EXECUTABLE} -C ${TARGET_DEBUG_PATH}"

# copy ./scripts into the remote Rpi
sshpass -p "$SSH_PASSWORD" scp -rp "${LOCAL_SCRIPTS_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_DEBUG_PATH}"

sshpass -p "$SSH_PASSWORD" ssh -t "${TARGET_HOST}" "${TARGET_DEBUG_PATH}/scripts/-build-run-dev.sh"
