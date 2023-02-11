## Running Ubuntu on VirtualBox in macOS for Pyrsia

### Preapring VirtualBox

Download VirtualBox for macOS from https://www.virtualbox.org/wiki/Downloads
Get Ubuntu iso image from https://ubuntu.com/download/alternative-downloads
Prepare VirtualBox with downloaded image.
Base memory: 8GB
CPU: 4 CPU
DISK: 500G
Settings -> Advanced -> Shared Clipboard -> set to Bidirectional**
root permission: On a fresh ubuntu virtual box image vboxuser doesn’t have root permission. Following next steps gives sudo permission to the user. Open terminal and enter su -. Run following as root user.

```
adduser vboxuser sudo
chmod 0440 /etc/sudoers
exit
```

Use reboot to restart the Ubuntu.


Install software / packages to run Pyrsia


sudo apt-get update
sudo apt-get -y install git-all curl
### packages required by Pyrsia
sudo apt-get -y install clang llvm libclang-dev jq protobuf-compiler
### rust installation
curl --pr1oto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default <rust_version>
### Pyrsia project build
Sudo snap install gh
cargo build --all-targets --workspace
