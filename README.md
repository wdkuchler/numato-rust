**# numato-rust**

**Simple Console USB Relay Module Controller Utility**

This console utility was created in Rust to control USB Relay Module Controller model USBPOWRL002 from Numato.

You can get more this link: https://numato.com/docs/1-channel-usb-powered-relay-module/

Device: idVendor=2a19, idProduct=0c05

This device is useful to turn ON and OFF motors, ligths, power supply, etc.

In Window run no problems. I was only able to run this program on a Linux machine

**Important notes for linux users:**

Unfortunately I couldn't run on Windows + WSL2. WSL does not work with any hardware.
I was only able to run this program on a Linux machine.

1. On (PURE) Linux, Don't forget to set permission for the device:

**sudo chmod 666 /dev/ttyACM0**

1. Usage:

cargo run [device] [command]

Where:

device name: Ex.: /dev/COM6 or /dev/ttyACM0

command to process: Ex.: on, off or pulse

Ex.:

Linux:

**cargo run /dev/ttACM0 pulse**

Windows:

**cargo run COM6 pulse**