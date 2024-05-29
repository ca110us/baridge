# Baridge

English | [简体中文](./README-zh_CN.md)

Baridge is a lightweight WOL (Wake-on-LAN) relay server written in Rust. It allows you to wake up devices on your local network remotely.

## Feature
Baridge receives WOL magic packets and forwards them to the target device's MAC address using a broadcast.

## Deployed using docker
```shell
docker run -d --name baridge --network=host verycallous/baridge:latest
```

## Running Baridge from code
```shell
git clone https://github.com/ca110us/baridge.git
cd baridge
cargo build --release
./target/release/baridge
```