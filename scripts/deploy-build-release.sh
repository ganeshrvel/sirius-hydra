#!/bin/bash
set -e

echo "Starting deployment script for Raspberry Pi..."

echo "Sourcing base script..."
. ./scripts/-base.sh

echo "Building release target for ${TARGET_ARCH}..."
cargo build --target="${TARGET_ARCH}" --release

echo "Creating compressed archive with executable and config files..."
tar -czf "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${LOCAL_RELEASE_BINARY_SOURCE_PATH}" "${CONFIG_FILE}" "${RUN_SIRIUS_HYDRA_FILE}"

echo "Creating target release directory on remote host..."
ssh -i "${SSH_PRIVATE_KEY}" -o StrictHostKeyChecking=no "${TARGET_HOST}" "mkdir --parents ${TARGET_RELEASE_PATH}"

echo "Copying compressed archive to remote host..."
scp -i "${SSH_PRIVATE_KEY}" "${LOCAL_COMPRESSED_EXECUTABLE_PATH}" "${TARGET_HOST}:${TARGET_RELEASE_PATH}"

echo "Decompressing archive on remote host..."
ssh -i "${SSH_PRIVATE_KEY}" -o StrictHostKeyChecking=no "${TARGET_HOST}" "tar --transform 's/.*\///g' -xzf ${TARGET_RELEASE_PATH}/${LOCAL_COMPRESSED_EXECUTABLE} -C ${TARGET_RELEASE_PATH}"

echo "Copying scripts directory to remote host..."
scp -i "${SSH_PRIVATE_KEY}" -rp "${LOCAL_SCRIPTS_SOURCE_PATH}" "${TARGET_HOST}:${TARGET_RELEASE_PATH}"

echo "Running deployment script on remote host..."
ssh -i "${SSH_PRIVATE_KEY}" -t "${TARGET_HOST}" "${TARGET_RELEASE_PATH}/scripts/-deploy-build-release.sh"

echo "Deployment complete!"
