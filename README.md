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

## Additional features - unimplemented!()
- [ ] implement the finding of the corresponding Touchpad event in /proc/bus/input/devices
- [ ] Implement for different Asus Zenbook models with/without % symbol
- [ ] systemd module

## Thanks!
Thanks to Mohamed Badaoui for his [Python implementation](https://github.com/mohamed-badaoui/asus-touchpad-numpad-driver) and inspiration.
