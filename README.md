# numato-rust
Simple Console USB Relay Module Controller Utility

This console utility was created in Rust to control USB Relay Module Controller model USBPOWRL002 from Numato.

You can get more this link: https://numato.com/docs/1-channel-usb-powered-relay-module/

Device: idVendor=2a19, idProduct=0c05

In Window run no problems but in PURE-Linux you may configure device permissions via UDEV:

Important notes for linux users:

1. Create(sudo) 70-numato.rules file in the /etc/udev/rules.d/ folder with the following content:

KERNEL=="ttyACM[0-9]*",MODE="0666"

2. If you connect the device (USB) for the first time the program will work perfectly.

3. If you disconnect the device and then immediately connect, you will have to wait a few seconds for this program to work.

Usage:

numato-rust device command

Where:

device name: Ex.: /dev/ttyACM0 or COM1

command to process: Ex.: on, off or pulse

This device is useful to turn ON and OFF motors, ligths, power supply, etc.


Important notes for Linux running into WSL users:

You can run this utility on WSL (Windows Subsystem for Linux) where device name corresponding:

Windows COM1 => Linux /dev/ttyS1

Windows COM2 => Linux /dev/ttyS2

Windows COM3 => Linux /dev/ttyS3, etc.

More this link: https://blogs.msdn.microsoft.com/wsl/2017/04/14/serial-support-on-the-windows-subsystem-for-linux/

compile into terminal: 

'/usr/bin/fpc'  -MObjFPC -Scghi -Cg -O1 -g -gl -l -vewnhibq -Filib/x86_64-linux -Fu. -FUlib/x86_64-linux -FE. -onumato-pas project1.lpr