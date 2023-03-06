use std::{thread::sleep, time::Duration};

use anyhow::Result;
use rppal::{
    gpio::{Gpio,OutputPin},
    spi::Spi,
};

const BUS_CLOCK_SPEED: u32 = 2_000_000;

const RST_PIN: u8 = 25;
const DC_PIN: u8 = 24;
const CS_PIN: u8 = 8;
const BL_PIN: u8 = 18;
const WIDTH: u8 = 128;
const HEIGHT: u8 = 64;

#[derive(Debug)]
pub enum Protocol {
    SPI,
    I2C,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Display {
    width: u8,
    height: u8,
    dc_pin: OutputPin,
    cs_pin: OutputPin,
    rst_pin: OutputPin,
    bl_pin: OutputPin,
    protocol: Protocol,
    bus: Spi,
}

impl Display {
    pub fn write_command(&mut self, byte: &[u8]) -> Result<()> {
        match self.protocol {
            Protocol::SPI => {
                self.dc_pin.set_low();
                self.spi_write_byte(byte)
            }
            Protocol::I2C => self.i2c_write_byte(byte),
        }
    }

    pub fn write_data(&mut self, byte: &[u8]) -> Result<()> {
        match self.protocol {
            Protocol::SPI => {
                self.dc_pin.set_high();
                self.spi_write_byte(byte)
            }
            Protocol::I2C => self.i2c_write_byte(byte),
        }
    }

    pub fn spi_write_byte(&mut self, byte: &[u8]) -> Result<()> {
        self.bus.write(byte)?;

        Ok(())
    }

    pub fn i2c_write_byte(&self, _byte: &[u8]) -> Result<()> {
        /* TODO: IMPLEMENT I2C */
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.rst_pin.set_high();
        sleep(Duration::from_millis(100));
        self.rst_pin.set_low();
        sleep(Duration::from_millis(100));
        self.rst_pin.set_high();
        sleep(Duration::from_millis(100));

        Ok(())
    }

    pub fn show_image(&mut self, buffer: &[u8]) -> Result<()> {
        for page in 0..8 {
            self.write_command(&[0xB0 + page])?;
            self.write_command(&[0x02])?;
            self.write_command(&[0x10])?;
            sleep(Duration::from_millis(10));
            match self.protocol {
                Protocol::I2C => todo!("implement i2c"),
                Protocol::SPI => {
                    self.dc_pin.set_high();
                    for index in 0..self.width {
                        self.spi_write_byte(&[buffer[(index + (self.width * page)) as usize]])?;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.show_image(&[0xFF; (WIDTH as usize * HEIGHT as usize) / 8usize])?;
        Ok(())
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.reset()?;
        self.write_command(&[0xAE])?; /* TURN OFF */
        self.write_command(&[0x02])?; /* LOW COL ADDR */
        self.write_command(&[0x10])?; /* HOW COL ADDR */
        self.write_command(&[0x40])?; /* SET LINE ADDR */
        self.write_command(&[0x81])?; /* CONTRAST CTLR REGISTER */
        self.write_command(&[0xA0])?; // --Set SEG/Column Mapping
        self.write_command(&[0xC0])?; // Set COM/Row Scan Direction
        self.write_command(&[0xA6])?; // --set normal display
        self.write_command(&[0xA8])?; // --set multiplex ratio(1 to 64)
        self.write_command(&[0x3F])?; // --1/64 duty
        self.write_command(&[0xD3])?; // -set display offset    Shift Mapping RAM Counter (0x00~0x3F)
        self.write_command(&[0x00])?; // -not offset
        self.write_command(&[0xd5])?; // --set display clock divide ratio/oscillator frequency
        self.write_command(&[0x80])?; // --set divide ratio, Set Clock as 100 Frames/Sec
        self.write_command(&[0xD9])?; // --set pre-charge period
        self.write_command(&[0xF1])?; // Set Pre-Charge as 15 Clocks & Discharge as 1 Clock
        self.write_command(&[0xDA])?; // --set com pins hardware configuration
        self.write_command(&[0x12])?; //
        self.write_command(&[0xDB])?; // --set vcomh
        self.write_command(&[0x40])?; // Set VCOM Deselect Level
        self.write_command(&[0x20])?; // -Set Page Addressing Mode (0x00/0x01/0x02)
        self.write_command(&[0x02])?; //
        self.write_command(&[0xA4])?; //  Disable Entire Display On (0xa4/0xa5)
        self.write_command(&[0xA6])?; //  Disable Inverse Display On (0xa6/a7)
        sleep(Duration::from_millis(100));
        self.write_command(&[0xAF])?; // --turn on oled panel
        Ok(())
    }

    pub fn new() -> Result<Self> {
        /* INITIALIZE GPIO */
        let gpio = Gpio::new()?;
        let rst_pin = gpio.get(RST_PIN)?.into_output();
        let mut dc_pin = gpio.get(DC_PIN)?.into_output();
        let mut cs_pin = gpio.get(CS_PIN)?.into_output();
        let mut bl_pin = gpio.get(BL_PIN)?.into_output();

        cs_pin.set_low();
        bl_pin.set_high();
        dc_pin.set_low();

        let bus: Spi = Spi::new(
            rppal::spi::Bus::Spi0,
            rppal::spi::SlaveSelect::Ss0,
            BUS_CLOCK_SPEED,
            rppal::spi::Mode::Mode0,
        )?;

        Ok(Self {
            width: WIDTH,
            height: HEIGHT,
            protocol: Protocol::SPI,
            rst_pin,
            dc_pin,
            cs_pin,
            bl_pin,
            bus,
        })
    }
}
