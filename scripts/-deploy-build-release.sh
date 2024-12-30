#!/bin/bash
set -e

# cd to the root directory of hydra
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit
cd ../

./scripts/-resets.sh

pkill -9 sirius-hydra 2>/dev/null || true



