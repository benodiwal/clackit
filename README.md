# clackit

A rust cli tool to make a normal keyboard sound like a  mechanical keyboard

## Installation
```bash
cargo install clackit
```

## Linux 
You need To install Advanced Linux Sound Architecture [ ALSA ]

Ubuntu / debian
```
sudo apt-get install alsa-tools
```

Fedora
```
sudo dnf install alsa-lib-devel
```

## Usage

```
clackit -s <soundpack_path> -v <volume> (0-100 | optional)
```
## LICENSE
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
