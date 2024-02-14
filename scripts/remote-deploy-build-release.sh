#!/bin/bash
set -e

## Remote developement and deloy the release build
## use this file to connect to raspberry pi, sync the local files to raspberrypi, code remotely on the raspberry pi and then deloy the release build on the remote RPi.
## on intellij idea IDE use tools -> deployment, to sync the files to the RPi remote machine and then run this script to deloy the release build on the remote RPi.

. ./scripts/-base.sh

ssh -i "${SSH_PRIVATE_KEY}" -t "${TARGET_HOST}" "${REMOTE_SIRIUS_HYDRA_SOURCE_PATH}/scripts/-remote-deploy-release.sh"
