#!/usr/bin/env python3
import os
import sys
import platform
from urllib import request

WG_DISPLAY_PATH = '/home/pi/wgdisplay'

def get_release_name() -> str:
    machine = platform.machine()
    if 'armv6' in machine:
        return "wg-display-arm-unknown-linux-gnueabihf"
    if 'armv7' in machine or 'armv8' in machine:
        return "wg-display-armv7-unknown-linux-gnueabihf"
    print(f'Unsupported architecture: {machine}')
    sys.exit(1)

def download_release(release_name: str) -> None:
    url = f"https://github.com/eliabieri/wg_display/releases/latest/download/{release_name}"
    try:
        print("Downloading release...")
        request.urlretrieve(url, WG_DISPLAY_PATH)
        print(f'Downloaded release: {release_name}')
    except Exception as e:
        print(f'Failed to download release: {e}')
        sys.exit(1)

def make_executable() -> None:
    os.system(f"sudo chmod +x {WG_DISPLAY_PATH}")

def patch_bashrc() -> None:
    COMMANDS = [
        f"sudo setcap CAP_NET_BIND_SERVICE=+eip {WG_DISPLAY_PATH}\n",
        f"{WG_DISPLAY_PATH}\n"
    ]
    BASHRC_FILE = '/home/pi/.bashrc'
    with open(BASHRC_FILE, 'r+') as f:
        content = f.readlines()
        if any(command in content for command in COMMANDS):
            print('Already patched .bashrc')
            return
    with open(BASHRC_FILE, 'a+') as f:
            f.writelines([
                "# WG Display\n",
                *COMMANDS
            ])
            print('Patched .bashrc')


def change_hostname() -> None:
    print("Changing hostname to wgdisplay")
    os.system("sudo sh -c 'echo wgdisplay > /etc/hostname'")

def reboot() -> None:
    print("Rebooting...")
    os.system("sudo reboot")

def main() -> None:
    print("Welcome to WG Display installer")
    release_name = get_release_name()
    download_release(release_name)
    make_executable()
    patch_bashrc()
    change_hostname()
    reboot()

if __name__ == '__main__':
    main()