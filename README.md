![](https://git.xorycode.dev/xory/pasternakd/-/raw/main/demo.mp4)

# pasternakctl
A lightweight dedicated activator for the PASTERNAK Network-Activated High-speed Killswitch

## Installation Instructions

### Arch Linux
```
sudo pacman -S rust cargo
git clone https://git.xorycode.dev/xory/pasternakctl
cd pasternakctl
cargo build --release
sudo cp ./target/release/pasternakctl /bin/
```

### Debian Linux & Permutations
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://git.xorycode.dev/xory/pasternakctl
cd pasternakctl
cargo build --release
sudo cp ./target/release/pasternakctl /bin/
```

### Configuration file - MANDATORY

Then, create a configuration file similar at ~/.config/pasternakctl/hosts containing the IP addresses of every computer on your network, with a format similar to this:
```
192.168.1.2
192.168.1.3
192.168.1.4
127.0.0.1
```
**WARNING: 127.0.0.1 / 0.0.0.0 / localhost MUST be placed last otherwise the program WILL NOT work properly!**