# CAN Examples

CAN is a field bus protocol used for many embedded (real time) applications and is especially used in control units in the auto maker industry. With the introduction of SocketCAN initiated by [Oliver Hartkopp](https://github.com/hartkopp) it has become pretty easy to build CAN applications on Linux. 

Code in this folder shall help you get along using CAN when developing with Rust.

## Build
This will only run on Linux. (However no test yet with WSL2) Socket CAN needs to be installed on your Linux and that can be checked by

    modprobe vcan

After that you can create and turn on a virtual device like so

    ip link add dev vcan0 type vcan
    ip link set up vcan0

It is very recommandable, that you also install can-utils, which on Ubuntu looks like this:

    apt install can-utils

With that you have some nice tools like cansend or candump, which do exactly what their respective names imply.

Rust code itself can be build with the usual command:

    cargo build

## Run
Compiled code can be executed by

    cargo run

If you want to send something to your virtual CAN device you can use a command like this:

    cansend vcan0 101#00FFAA55FFF203FF

