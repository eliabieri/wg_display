#!/usr/bin/env python3
import os
import sys
import time
import platform
from urllib import request

WG_DISPLAY_PATH = '/home/pi/wgdisplay'


def get_release_name() -> str:
    machine = platform.machine()
    if 'armv8' in machine:
        return "aarch64-unknown-linux-gnueabihf"
    print(f'Unsupported architecture: {machine}. Only 64-bit capable Raspberry Pi\'s are supported for now.')
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

def install_dependencies() -> None:
    os.system("sudo apt install iptables -y")

def patch_bashrc() -> None:
    commands = [
        "sudo iptables -t nat -A PREROUTING -p tcp --dport 80 -j REDIRECT --to-port 8000\n",
        f"{WG_DISPLAY_PATH}\n"
    ]
    bashrc_file = '/home/pi/.bashrc'
    with open(bashrc_file, 'r+', encoding='utf-8') as f:
        content = f.readlines()
        if any(command in content for command in commands):
            print('Already patched .bashrc')
            return
    with open(bashrc_file, 'a+', encoding='utf-8') as f:
        f.writelines([
            "# WG Display\n",
            *commands
        ])
        print('Patched .bashrc')


def change_hostname() -> None:
    print("Changing hostname to wgdisplay")
    os.system("sudo sh -c 'echo wgdisplay > /etc/hostname'")

def enable_console_autologin() -> None:
    print("Enabling autologin")
    content = "[Service]\nExecStart=\nExecStart=-/sbin/agetty --autologin pi --noclear %I $TERM"
    shell_command = f'echo "{content}" > /etc/systemd/system/getty@tty1.service.d/autologin.conf'
    command = f"sudo sh -c '{shell_command}'"
    os.system(command)

def reboot() -> None:
    print("Rebooting...")
    time.sleep(5)
    os.system("sudo reboot")

def main() -> None:
    print("Welcome to WG Display installer")
    release_name = get_release_name()
    download_release(release_name)
    make_executable()
    install_dependencies()
    patch_bashrc()
    change_hostname()
    enable_console_autologin()
    reboot()

if __name__ == '__main__':
    main()
