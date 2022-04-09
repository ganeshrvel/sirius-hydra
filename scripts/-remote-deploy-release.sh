#!/bin/bash
set -e

# cd to the root directory of hydra
SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
cd "$SCRIPTPATH" || exit
cd ../

./scripts/-resets.sh

/home/pi/.cargo/bin/cargo build --release

killall -9 sirius-hydra || true

cp ./target/release/sirius-hydra /home/pi/sirius-hydra

