# Sirius Hydra - Raspberry Pi IOT | Internet Radio [rust]

- Author: [Ganesh Rathinavel](https://www.linkedin.com/in/ganeshrvel "Ganesh Rathinavel")
- License: [MIT](https://github.com/ganeshrvel/sirius-hydra/blob/master/LICENSE "MIT")
- Repo URL: [https://github.com/ganeshrvel/sirius-hydra](https://github.com/ganeshrvel/sirius-hydra/ "https://github.com/ganeshrvel/sirius-hydra")
- Contacts: ganeshrvel@outlook.com

### Setup

**macOS setup**

Ref: https://pixelspark.nl/2020/cross-compiling-rust-programs-for-a-raspberry-pi-from-macos

```shell
brew install arm-linux-gnueabihf-binutils sshpass
rustup target add arm-unknown-linux-musleabi
```

**SSH Config**
- Rename `env.sample.config` to `env.config`
- Edit values inside `env.config`

**Installation**
- Download the OS image burner: https://downloads.raspberrypi.org/imager/imager_latest.dmg
- Download the latest "Raspberry Pi OS with desktop" distro via torrents:
    - Choose 32 bits or 64 bits as required https://www.raspberrypi.com/software/operating-systems/
- In the Rpi Imager app choose settings gear icon and set the ssh keys and wifi password
- After burning the OS using the imager app, plug in the sdcard into the rpi
- Wait for the bootup

```shell
# Run
sudo raspi-config
```
- Choose Boot / Auto Login options > Command Line Auto Login

```shell
# Run
sudo reboot
```

**Fix source**

```shell
# Run
sudo nano /etc/apt/sources.list
```

- Add this line to the file:

```
deb https://raspbian.mirror.uk.sargasso.net/raspbian/ buster main contrib non-free rpi
deb-src https://raspbian.mirror.uk.sargasso.net/raspbian/ bullseye  main contrib non-free rpi
```

- Comment out the existing line
- Save and exit

```shell
# Run
sudo apt update
sudo apt upgrade
```

**Set up audio driver**

- Plug in the USB sound card to the usb 2.0 port

```shell
# Run
sudo raspi-config
```
- Choose System Options > Audio > USB 0

```shell
# Run
sudo apt-get install -y vlc libvlc-dev alsa-base pulseaudio pslist
sudo reboot
```

- Install the latest version of rust compiler

**Build**
- Run `./scripts/build-run-dev.sh` to build the code on the local machine and execute the binary on the remote RPi
- Run `./scripts/deploy-build-release.sh` to build release the binary on the local machine and to deploy it on the remote RPi

**Other scripts**
- Run `./scripts/remote-build-run-dev.sh` for Remote developement on the RPi - debug build and run
- Run `./scripts/remote-build-run-release.sh` for Remote developement on the RPi - release build and run
- Run `./scripts/remote-deploy-release.sh` for Remote developement on the RPi - deploying the build

**Paths**
- Deployment path: `/home/pi/sirius-hydra-release`
- Release binary execution path: `./sirius-hydra-release/run-sirius-hydra.sh`

**Auto start program**

```shell
# Run
nano ~/.bashrc
```

- Add this line to the end of the `~/.bashrc` file

```shell
./sirius-hydra-release/run-sirius-hydra.sh

# save it using ctrl+x -> y
```

### Warranty
Read the [license](https://github.com/ganeshrvel/sirius-hydra/blob/master/LICENSE "MIT License") carefully. The license makes it clear that the project is offered "as-is", without warranty, and disclaiming liability for damages resulting from using this project.

### Contacts
Please feel free to contact me at ganeshrvel@outlook.com or [LinkedIn](https://www.linkedin.com/in/ganeshrvel)

### Support
Help me keep my works FREE and open for all.
- Donate Via PayPal: [paypal.me/ganeshrvel](https://paypal.me/ganeshrvel "https://paypal.me/ganeshrvel")
- Buy Me A Coffee (UPI, PayPal, Credit/Debit Cards, Internet Banking): [buymeacoffee.com/ganeshrvel](https://buymeacoffee.com/ganeshrvel "https://buymeacoffee.com/ganeshrvel")

### License
irius Hydra - Raspberry Pi IOT | Internet Radio [rust] is released under [MIT License](https://github.com/ganeshrvel/sirius-hydra/blob/master/LICENSE "MIT License").

Copyright © 2018-Present Ganesh Rathinavel
