#!/bin/bash

# this is for executing the sirius-hydra binary on the remote RPi. It doesnt work on the local machine

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit

# dont load the app if the process is already running
pgrep sirius-hydra > /dev/null || {

  # auto restart the app if it dies
  while true
  do
    ./sirius-hydra
    sleep 2
  done
}
