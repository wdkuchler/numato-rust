**# numato-rust**

**Simple Console USB Relay Module Controller Utility**

This console utility was created in Rust to control USB Relay Module Controller model USBPOWRL002 from Numato.

You can get more this link: https://numato.com/docs/1-channel-usb-powered-relay-module/

Device: idVendor=2a19, idProduct=0c05

This device is useful to turn ON and OFF motors, ligths, power supply, etc.

In Window run no problems.

**Important notes for linux users:**

You can run this utility on WSL (Windows Subsystem for Linux) but need to install a few things:

1. On Linux terminal:

**sudo apt install linux-tools-virtual hwdata**

**sudo update-alternatives --install /usr/local/bin/usbip usbip `ls /usr/lib/linux-tools/*/usbip | tail -n1` 20**

1. On Windows PowerShell (admin)

**winget install usbipd-win**

**usbipd wsl list**

BUSID VID:PID DEVICE STATE

3-3 0bda:8771 Realtek Bluetooth 5.0 Adapter Not attached

3-6 258a:1006 Dispositivo de Entrada USB Not attached

3-7 138a:0017 Synaptics FP Sensors (WBF) (PID=0017) Not attached

3-12 04f2:b39a Integrated Camera Not attached

4-1 2a19:0c05 Dispositivo Serial USB (COM6) Attached - Ubuntu

4-3 20a0:0001 Dispositivo de Entrada USB, flirc Not attached

4-4 0763:400d M-Track Hub

**usbipd wsl attach --busid 4-1**

On Linux:

**sudo chmod 666 /dev/ttyACM0**

Usage:

cargo run [device] [command]

Where:

device name: Ex.: /dev/ttyACM0 or COM6

command to process: Ex.: on, off or pulse

Ex.:

Linux:

**cargo run /dev/ttACM0 pulse**

Windows:

**cargo run COM6 pulse**