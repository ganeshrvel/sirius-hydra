#!/bin/bash
set -e

## Remote developement
## use this file to connect to raspberry pi, sync the local files to raspberrypi and code remotely on the raspberry pi.
## on intellij idea IDE use tools -> deployment, to sync the files to the RPi remote machine and then run this script

. ./scripts/-base.sh

sshpass -p "$SSH_PASSWORD" ssh -t "${TARGET_HOST}" "${REMOTE_SIRIUS_HYDRA_SOURCE_PATH}/scripts/-remote-build-run-dev.sh"
