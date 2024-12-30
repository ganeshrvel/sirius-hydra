#!/bin/bash
set -e

echo "Starting debug deployment script for Raspberry Pi..."

echo "Sourcing base script..."
. ./scripts/-base.sh

echo "Building debug target for ${TARGET_ARCH}..."
cargo build --target="${TARGET_ARCH}"

echo "Creating compressed archive with executable and config files..."
tar -czf "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${LOCAL_DEBUG_BINARY_SOURCE_PATH}" "${CONFIG_FILE}"

echo "Creating target debug directory on remote host..."
ssh -i "${SSH_PRIVATE_KEY}" -o StrictHostKeyChecking=no "${TARGET_HOST}" "mkdir --parents ${TARGET_DEBUG_PATH}"

echo "Copying compressed archive to remote host..."
scp -i "${SSH_PRIVATE_KEY}" "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${TARGET_HOST}:${TARGET_DEBUG_PATH}"

echo "Decompressing archive on remote host..."
ssh -i "${SSH_PRIVATE_KEY}" -o StrictHostKeyChecking=no "${TARGET_HOST}" "tar --transform 's/.*\///g' -xzf ${TARGET_DEBUG_PATH}/${LOCAL_COMPRESSED_EXECUTABLE} -C ${TARGET_DEBUG_PATH}"

echo "Copying scripts directory to remote host..."
scp -i "${SSH_PRIVATE_KEY}" -rp "${LOCAL_SCRIPTS_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_DEBUG_PATH}"

echo "Running debug build script on remote host..."
ssh -i "${SSH_PRIVATE_KEY}" -t "${TARGET_HOST}" "${TARGET_DEBUG_PATH}/scripts/-build-run-dev.sh"

echo "Debug deployment complete!"