# Baridge

[English](./README.md) | 简体中文

Baridge 是一个用 Rust 语言编写轻量级的 WOL (Wake-on-LAN) 唤醒代理服务，它允许您远程唤醒局域网内的设备

## 功能
Baridge 接收 WOL 魔法包，并使用广播的方式将它们转发到目标设备的 MAC 地址

## 使用 Docker 部署
```shell
docker run -d --name baridge --network=host verycallous/baridge:latest
```

## 从源代码运行 Baridge
```shell
git clone https://github.com/ca110us/baridge.git
cd baridge
cargo build --release
./target/release/baridge
```
