# Numpad-touchpad Linux driver for Asus Zenbook
> On linux there is no numericpad support for my laptop "out of the box", so I will try to implement it.


## Screenshot (photo tbe)
<details>
  <summary>LED Numpad integrated into the touchpad:</summary>
  
  ![](https://github.com/khadievedem/asus-zenbook-numpad-driver-rs/blob/numericpad-img/numberpad.jpg)
  
</details>

## Features
- [x] Switching LED on touchpad
- [x] 5 levels of brightness
- [x] So far only for model UM433D
- [x] Use log instead of println :)
- [x] Make numpad work (UM433D only)
- [x] Support for a custom application starts on the upper left button press

## Additional features - unimplemented!()
- [ ] implement the finding of the corresponding Touchpad event in /proc/bus/input/devices
- [ ] Implement for different Asus Zenbook models with/without % symbol
- [ ] systemd module

## How to run (temporary instruction for tough guys)
### Prerequisites
#### Install `rustup`:
> Read ["Installation"] from ['The Rust Programming Language' book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
['The Rust Programming Language' book]: https://doc.rust-lang.org/book/index.html

#### Install [ libevdev i2c-tools git ]
##### I hope you're using archlinux :-)
```sh
$ sudo pacman -S libevdev i2c-tools git
```

### Load i2c-dev module
```sh
$ sudo modprobe i2c-dev
```

### Clone
```sh
$ git clone https://github.com/khadievedem/asus-zenbook-numpad-driver-rs
$ cd ./asus-zenbook-numpad-driver-rs
```

### Run
#### without Debug info
```sh
$ sudo cargo run *arg
```
#### with Debug info
```sh
$ sudo RUST_LOG=debug cargo run *arg
```
> Add *arg - the path to your application binary that will run when you press on upper-left button


## Thanks!
Thanks to Mohamed Badaoui for his [Python implementation](https://github.com/mohamed-badaoui/asus-touchpad-numpad-driver) and inspiration.
