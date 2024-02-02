# Arduino Mega 2560 R3 Example

**WARNING**: This example is currently out of date. It won't be updated until `avr-hal` supports `embedded-hal@1.0`.

To test this example, please ensure you've first followed the instructions for [setting up `avr-hal`](https://github.com/Rahix/avr-hal#quickstart).

Then, you can run `cargo run --release`! You should see `avrdude` pop up with a friendly greeting and a serial monitor. 😄

**IMPORTANT**: always use `--release`. Otherwise, the program will refuse to compile as [panicking differs](https://doc.rust-lang.org/nomicon/panic-handler.html) on `#![no-std]` platforms.
