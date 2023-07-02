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

# install rust if not installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# open the project directory:
cd /path/to/project
rustup target add arm-unknown-linux-musleabi
```

**Installation**
- Download the OS image burner: https://downloads.raspberrypi.org/imager/imager_latest.dmg
- Download the latest "Raspberry Pi OS with desktop" distro via torrents:
    - Choose 32 bits or 64 bits as required https://www.raspberrypi.com/software/operating-systems/
- In the Rpi Imager app choose settings gear icon and set the ssh keys and wifi password
- After burning the OS using the imager app, plug in the sdcard into the rpi
- Wait for the bootup


**Prerequisites**
The app uses Chromium to play the web radio due to better HLS support. However, Chromium requires the following keys to function properly: `GOOGLE_API_KEY`, `GOOGLE_DEFAULT_CLIENT_ID`, and `GOOGLE_DEFAULT_CLIENT_SECRET`.

To generate these keys, please visit https://www.chromium.org/developers/how-tos/api-keys/ and follow the provided instructions. Once you have generated the keys, make sure to update the corresponding values inside the config.yaml file.


**Web Radio hosting**
To enable web radio playback, the files within the ./web-radio directory need to be deployed. You can choose to deploy these files to services such as Cloudflare Pages, GitHub Pages, Vercel, or similar platforms. Remember to update the URL of the web radio website inside the config.yaml file accordingly.

**SSH Config**
- Rename `env.sample.config` to `env.config`
- Edit values inside `env.config`
- Rename `config.sample.yaml` to `config.yaml`
- Edit values inside `config.yaml`

```shell
# Run
sudo raspi-config
```
- Choose System Options > Boot / Auto Login > Console Autologin
- Choose Interface Options > Serial Port > `Login shell access over serial` -> Yes

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

- Increase the volume
- 
```shell
# Run
alsamixer

# keep pressing the Navigation Up arrow on the keyboard to increase the volume
```

```shell
# Run
sudo apt-get install -y alsa-base pulseaudio pslist chromium-browser
sudo reboot
```

- Install the latest version of rust compiler

**Build**
- Run `./scripts/build-run-dev.sh` to build the code on the local machine and execute the binary on the remote RPi
- Run `./scripts/deploy-build-release.sh` to build release the binary on the local machine and to deploy it on the remote RPi

**Other scripts**
To enable remote development on your Raspberry Pi, you can utilize remote debugging tools like IntelliJ IDEA. Follow the steps below to sync the files to your Raspberry Pi and code remotely.

1. Use the following files to connect to your Raspberry Pi and sync the local files to it:
  - Set up a deployment configuration in IntelliJ IDEA by going to `Tools -> Deployment -> Configuration`.
  - Configure the deployment settings to connect to the Raspberry Pi remote machine.
  - Sync the files from your local machine to the Raspberry Pi using the deployment configuration.

2. Run the appropriate scripts for remote development on the Raspberry Pi:
  - For debugging with a debug build, run `./scripts/remote-build-run-dev.sh`.
  - For remote development with a release build, run `./scripts/remote-build-run-release.sh`.
  - To deploy the build for remote development, run `./scripts/remote-deploy-release.sh`.

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

```shell
(cd ~/ && source ~/.bashrc)
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
sirius Hydra - Raspberry Pi IOT | Internet Radio [rust] is released under [MIT License](https://github.com/ganeshrvel/sirius-hydra/blob/master/LICENSE "MIT License").

Copyright © 2018-Present Ganesh Rathinavel
