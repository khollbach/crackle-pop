![crackle-pop running on a micro:bit](video/crackle-pop.gif)

## How?

I'm reading https://docs.rust-embedded.org/discovery/microbit/ and there's a guide there on setting up the hardware. This was made by copying the [project structure](https://github.com/rust-embedded/discovery/tree/master/microbit/src/05-led-roulette) from [Chapter 5: "It blinks"](https://docs.rust-embedded.org/discovery/microbit/05-led-roulette/it-blinks.html), and changing the code to display digits.

If you happen to have a microbit, and you follow the setup steps in that book (install Rust, install cargo-embed, and maybe a couple other things?) you should be able to run this on the device with:
```
$ cargo embed --target thumbv7em-none-eabihf
```
