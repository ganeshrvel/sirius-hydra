#!/bin/bash
set -e

## Local developement and run the debug build on an Rpi

. ./scripts/-base.sh

cargo build --target="${TARGET_ARCH}"
tar -czf "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${LOCAL_DEBUG_BINARY_SOURCE_PATH}" "${CONFIG_FILE}"

# create the scripts target path
ssh -i "${SSH_PRIVATE_KEY}" -o StrictHostKeyChecking=no "${TARGET_HOST}" "mkdir --parents ${TARGET_DEBUG_PATH}"

# copy the compressed binary and config archive file into the target path
scp -i "${SSH_PRIVATE_KEY}" "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${TARGET_HOST}:${TARGET_DEBUG_PATH}"

# decompress the archive
ssh -i "${SSH_PRIVATE_KEY}" -o StrictHostKeyChecking=no "${TARGET_HOST}" "tar --transform 's/.*\///g' -xzf ${TARGET_DEBUG_PATH}/${LOCAL_COMPRESSED_EXECUTABLE} -C ${TARGET_DEBUG_PATH}"

# copy ./scripts into the remote Rpi
scp -i "${SSH_PRIVATE_KEY}" -rp "${LOCAL_SCRIPTS_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_DEBUG_PATH}"

ssh -i "${SSH_PRIVATE_KEY}" -t "${TARGET_HOST}" "${TARGET_DEBUG_PATH}/scripts/-build-run-dev.sh"
