<div align="center">
    <br>
    <img src="docs/images/logo.png" style="height: 80px">
    <br>
    <br>
    <strong>
        🦀 Hackable information display fully built in Rust
    </strong>
    <p>Extensible, open-source and connected to the local community</p>
    <br/>
</div>

[![Cargo test](https://github.com/eliabieri/wg_display/actions/workflows/cargo_test.yml/badge.svg)](https://github.com/eliabieri/wg_display/actions/workflows/cargo_test.yml)

## ⭐️ What WG Display can show you

- 🚂 The next public transport connections between two stations
- 🏊🏻 The current temperature of the Aare River and whether it is a good idea to take a dip
- ⏰ The current date and time
- 🧖🏽‍♀️ The current occupancy in the "Sauna Lorrainebad" in Bern, CH
- 🕺🏾 The next event at the Cafete Club
- .. what ever idea you might have? Simply write your own widget

## ✨ Features

- 🦀 Fully built in Rust
- 🔧 Easily extensible. Write your own widget with ease!
- 🚀 Compiles to single binary
- 🤑 Only needs a Raspberry Pi Zero (or others) and a 15$ display
- ⚙️ Widgets can be configured by the user via a web interface

## 📚 Table of contents

- [⭐️ What WG Display can show you](#️-what-wg-display-can-show-you)
- [✨ Features](#-features)
- [📚 Table of contents](#-table-of-contents)
- [🚀 Getting started](#-getting-started)
- [🛠️ Assembling the hardware](#️-assembling-the-hardware)
- [🔨Building from source](#building-from-source)
  - [Prerequisites](#prerequisites)
  - [Prerequisites for cross-compilation](#prerequisites-for-cross-compilation)
  - [Building the project](#building-the-project)
- [👏 Writing your own widget](#-writing-your-own-widget)
- [📖 Documentation (rustdocs)](#-documentation-rustdocs)
- [🧪 Testing](#-testing)
- [🔮 What comes next](#-what-comes-next)
- [🔒 Safety](#-safety)
- [♻️ Updating the dependencies](#️-updating-the-dependencies)
- [🦾 Developing on target](#-developing-on-target)

![WG Display image front](docs/images/wg_display.jpg)

---

![Configuration dashboard](docs/images/dashboard.jpeg)
The web interface allows the users to configure system aspects like the background color used on the display or various configuration options of the different widgets.

## 🚀 Getting started

1. Change the hostname of the target to wgdisplay  
   `sudo raspi-config` -> `Network Options` -> `Hostname`
2. Download the latest [release](https://github.com/eliabieri/wg_display/releases)
   - Raspberry Pi Zero 1 / Zero W / Zero WH -> `wg-display-arm-unknown-linux-gnueabihf`
   - Raspberry Pi 2 / 3 / 4 / Zero 2 W -> `wg-display-armv7-unknown-linux-gnueabihf`
3. Copy the binary over to the target  
   `scp wg-display-arm-unknown-linux-gnueabihf pi@wgdisplay.local:/home/pi`
4. Enable the binary to be run at reboot  
   `echo "/home/pi/wg-display-arm-unknown-linux-gnueabihf" >> ~/.bashrc`
5. Allow the binary to be bind to port 80  
   `sudo setcap CAP_NET_BIND_SERVICE=+eip /home/pi/wg-display-arm-unknown-linux-gnueabihf`
6. Reboot the target  
   The configuration dashboard should be available at [wgdisplay.local](http://wgdisplay.local)

## 🛠️ Assembling the hardware

WG Display is best deployed on a Raspberry Pi and a cheap display hat.

```text
💡 Even a Raspberry PI Zero is sufficient!  
The application is very ressource efficient  
and generally only utilizes around 3% CPU on a Raspberry PI 3B.
```

Some displays that are tested to be good

- [5" MIPI DIS Display](https://t.ly/fWl3)
  - ✨ Requires no driver
  - ✨ Includes stand
  - 📐 Large enough to display ~ 6 widgets
  - 💲💲💲
- [3.5" HAT Display](https://t.ly/DfWJ)
  - ✨ Includes enclosure for wall mounting
  - ⚠️ Requires a [driver](https://github.com/goodtft/LCD-show/blob/master/MHS35-show)
  - 📐 Large enough to display ~ 3 widgets
  - 💲
- [3.5" HAT HDMI Display](https://t.ly/l2Rd)
  - ✨ Requires no driver
  - 📐 Large enough to display ~ 3 widgets
  - 💲💲
- Any other display you might find
  - WG Display uses the terminal for rendering, so there are no special display requirements

## 🔨Building from source

### Prerequisites

First, install [rustup](https://rustup.rs) then

```bash
# Install WebAssembly target
rustup target add wasm32-unknown-unknown
# Install trunk for building the frontend
cargo install --locked trunk
# Install NPM dependencies
cd frontend && npm install
```

### Prerequisites for cross-compilation

First, install [docker](https://www.docker.com), then

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

### Building the project

```bash
# Native build
make
# Cross compilation (Raspberry PI Zero 1 / Zero W / Zero WH)
make app_arm
# Cross compilation (Raspberry PI 2 / 3 / 4 / Zero 2 W)
make app_armv7
```

Then simply copy over the generated binary to the target and run it.

```text
💡 To run it at boot, simply add the path
to the binary to the end of the ~/.bashrc file.
```

## 👏 Writing your own widget

Want your WG Display to show you

- 🥳 the upcoming events in your favorite nightclub
- 🚮 the trash calendar in your municipality
- 🍺 beers on sale in your local supermarket?  

You've got two options

- [Write your own widget](docs/write_new_widget.md). It's easy using the provided guide and reference implementations
- In case you don't feel capable of writing it yourself, open a [feature request](https://github.com/eliabieri/wg_display/issues/new) and tag it using the `widget request` label

## 📖 Documentation (rustdocs)

The rustdocs can be built using

```bash
make docs
```

This generates three separate documentations, one for each crate

[app](app/target/doc/wg_display/index.html): ```app/target/doc/app/index.html```  
[common](common/target/doc/common/index.html): ```common/target/doc/common/index.html```  
[frontend](frontend/target/doc/frontend/index.html): ```frontend/target/doc/frontend/index.html```

## 🧪 Testing

Widgets should provide unit tests for their functionality where adequate.  
Asynchronous functions can be tested using the [tokio_test::block_on](https://docs.rs/tokio-test/latest/tokio_test/fn.block_on.html) function.

## 🔮 What comes next

- [ ] Allow user to configure WiFi credentials via web interface
- [ ] Starting the binary through systemd
- [ ] Implement an update mechanism
- [ ] Implement authencation for the web interface
- [ ] Dynamically loading widgets (currently, the widgets are part of the app crate)

## 🔒 Safety

This project uses `#[forbid(unsafe_code)]` in all crates to ensure that no `unsafe` Rust is ever added to the project

## ♻️ Updating the dependencies

```bash
# To update the dependencies in all crates, simply run
scripts/update_dependencies.sh
```

## 🦾 Developing on target

When developing, an occasional run on a target may be required.  
You can the following script as a template to copy over the binary to the target and restart it

```bash
#!/bin/sh
set -e

# Note:
# - Set hostname of target to wgdisplay
# - Add public key to authorized_keys on target
# - Enable root ssh login:  
#   https://raspberrypi.stackexchange.com/questions/48056/how-to-login-as-root-remotely

make app_arm
ssh pi@wgdisplay.local "sudo /usr/bin/pkill -9 app || true"
scp /Users/eliabieri/git/wg_display/app/target/arm-unknown-linux-gnueabihf/release/app pi@wgdisplay.local:/home/pi
ssh pi@wgdisplay.local "sudo reboot"
```
