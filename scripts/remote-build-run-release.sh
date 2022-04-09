#!/bin/bash
set -e

## Remote developement and run the release build
## use this file to connect to raspberry pi, sync the local files to raspberrypi, code remotely on the raspberry pi and then run the release build on the remote RPi.
## on intellij idea IDE use tools -> deployment, to sync the files to the RPi remote machine and then run this script to run the release build on the remote RPi.

. ./scripts/-base.sh

sshpass -p "$SSH_PASSWORD" ssh -t "${TARGET_HOST}" "${REMOTE_SIRIUS_HYDRA_SOURCE_PATH}/scripts/-remote-build-run-release.sh"
