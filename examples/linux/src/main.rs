use linux_embedded_hal::SpidevDevice;
use max6675_hal::Max6675;

fn main() -> anyhow::Result<()> {
    let spi_path = "/dev/spidev-0.0"; // spi-der pig, spi-der pig

    // does whatever spi-der pig does
    let spi = SpidevDevice::open(spi_path)?;

    // can he swing? from a web? no he can't because he's a pig
    let mut max = Max6675::new(spi)?;

    // look out
    loop {
        // he's spi-derpig
        let temp = max.read_fahrenheit()?;
        println!("temp in f: {temp}")
    }
}
