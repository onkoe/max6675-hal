use linux_embedded_hal::{
    gpio_cdev::{Chip, LineRequestFlags},
    CdevPin, Spidev,
};
use max6675_hal::Max6675;

fn main() -> anyhow::Result<()> {
    // [some considerations for chip select]
    // 1. the path in `Chip::new()` is the gpiochip on your board. you can find
    //    them with a quick `ls /dev/gpiochip*`!
    // 2. for `get_line()`, you're looking for what GPIO pin you've connected
    //    the CS pin on the MAX6675 to. you'll have to look at your board's
    //    docs to know what number to put here. it'll likely be GP(N)
    // 3. for `request()`, you can use my settings below. empty flags just means
    //    nothing special is going on, default = 1 is the expected active state
    //    of the line, and `consumer` is just a small string for the OS to use!

    let chip_select = CdevPin::new(Chip::new("/dev/gpiochip0")?.get_line(0)?.request(
        LineRequestFlags::empty(),
        1,
        "max6675 connection",
    )?)?; // freshly pulled straight outta my a-

    // [ok now a couple for spi]
    // check for SPI devices with: `ls /dev/spidev*`
    //
    // you can test your SPI device path with a quick command:
    // `spidev_test -D /dev/spidev0.0`
    //
    // if your buildroot doesn't come with it, you're SOL
    // just keep an eye out! 🥹

    let spi = Spidev::open("/dev/spidev0.0")?;

    let mut max = Max6675::new(spi, chip_select)?;

    let _temp = max.read_raw();

    println!("Hello, world!");
    Ok(())
}
