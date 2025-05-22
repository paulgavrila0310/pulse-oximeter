use embedded_hal_async::i2c::I2c;

// I2C address for MAX30100
const MAX30100_ADDRESS: u8 = 0x57;

// Register addresses
const REG_MODE_CONFIG: u8 = 0x06;
const REG_SPO2_CONFIG: u8 = 0x07;
const REG_LED_CONFIG: u8 = 0x09;
const REG_FIFO_DATA: u8 = 0x05;

pub struct Max30100<I2C> {
    i2c: I2C,
}

impl<I2C, E> Max30100<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub async fn init(&mut self) -> Result<(), E> {
        // Reset
        self.write_register(REG_MODE_CONFIG, 0x40).await?;
        // Wait after reset (caller should delay if needed)

        // SpO2 mode
        self.write_register(REG_MODE_CONFIG, 0x03).await?;
        // Sample rate, LED pulse width
        self.write_register(REG_SPO2_CONFIG, 0x27).await?;
        // LED current (IR + RED)
        self.write_register(REG_LED_CONFIG, 0x24).await?;
        Ok(())
    }

    pub async fn read_fifo(&mut self) -> Result<(u16, u16), E> {
        let mut data = [0u8; 4];
        self.i2c.write_read(MAX30100_ADDRESS, &[REG_FIFO_DATA], &mut data).await?;

        let ir = ((data[0] as u16) << 8) | (data[1] as u16);
        let red = ((data[2] as u16) << 8) | (data[3] as u16);
        Ok((ir, red))
    }

    async fn write_register(&mut self, reg: u8, value: u8) -> Result<(), E> {
        self.i2c.write(MAX30100_ADDRESS, &[reg, value]).await
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
