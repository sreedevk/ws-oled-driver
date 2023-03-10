use std::{thread::sleep, time::Duration};
pub type Buffer = Vec<u8>;

use anyhow::Result;
use rppal::{
    gpio::{Gpio, OutputPin},
    spi::Spi,
};

const BUS_CLOCK_SPEED: u32 = 8_000_000;
const RST_PIN: u8 = 25;
const DC_PIN: u8 = 24;
const CS_PIN: u8 = 8;
const BL_PIN: u8 = 18;
const WIDTH: u8 = 132;
const HEIGHT: u8 = 64;

/// Protocol used to access the device. Currently only supports SPI
#[derive(Debug)]
pub enum Protocol {
    SPI,
    I2C,
}

#[derive(Debug)]
pub struct Display {
    pub width: u8,
    pub height: u8,
    pub dc_pin: OutputPin,
    pub cs_pin: OutputPin,
    pub rst_pin: OutputPin,
    pub bl_pin: OutputPin,
    pub protocol: Protocol,
    pub bus: Spi,
    pub memory: Buffer,
}

impl Display {
    /// Write a command byte to the device. The byte should be of the type `&[u8]`. The DC pin is
    /// set low to indicate that the byte being written is a command byte.
    pub fn write_command(&mut self, byte: &[u8]) -> Result<()> {
        match self.protocol {
            Protocol::SPI => {
                self.dc_pin.set_low();
                self.spi_write_byte(byte)
            }
            Protocol::I2C => self.i2c_write_byte(byte),
        }
    }

    /// Write a data byte to the device. The byte should be of the type `&[u8]`. The DC pin is set
    /// high to indicate that the byte being written is a data byte
    pub fn write_data(&mut self, byte: &[u8]) -> Result<()> {
        match self.protocol {
            Protocol::SPI => {
                self.dc_pin.set_high();
                self.spi_write_byte(byte)
            }
            Protocol::I2C => self.i2c_write_byte(byte),
        }
    }

    /// Raw SPI write byte function. DO NOT USE. use `write_command` or `write_data` instead.
    pub fn spi_write_byte(&mut self, byte: &[u8]) -> Result<()> {
        self.bus.write(byte)?;

        Ok(())
    }

    /// TODO: Add Support for I2C
    pub fn i2c_write_byte(&self, _byte: &[u8]) -> Result<()> {
        /* TODO: IMPLEMENT I2C */
        Ok(())
    }

    /// Reset the display.
    pub fn reset(&mut self) -> Result<()> {
        self.rst_pin.set_high();
        sleep(Duration::from_millis(100));
        self.rst_pin.set_low();
        sleep(Duration::from_millis(100));
        self.rst_pin.set_high();
        sleep(Duration::from_millis(100));

        Ok(())
    }

    /// The Graphics buffer is stored in the `Display { memory: Vec<u8> }` field. The
    /// `ws_oled_driver::gfx` library functions work on the memory field. It writes the pixes onto the
    /// memory field. The `render` function dumps the `memory` field onto the display.
    pub fn render(&mut self) -> Result<()> {
        for page in 0..8 {
            self.write_command(&[0xB0 + page])?;
            self.write_command(&[0x02])?;
            self.write_command(&[0x10])?;
            sleep(Duration::from_millis(10));
            match self.protocol {
                Protocol::I2C => todo!("implement i2c"),
                Protocol::SPI => {
                    self.dc_pin.set_high();
                    for index in 0..=(self.width as usize) {
                        let byte = self.memory[index + self.width as usize * page as usize];
                        self.spi_write_byte(&[byte])?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Initializes the display device.
    pub fn initialize(&mut self) -> Result<()> {
        self.reset()?;
        self.write_command(&[0xAE])?;
        self.write_command(&[0x02])?;
        self.write_command(&[0x10])?;
        self.write_command(&[0x40])?;
        self.write_command(&[0x81])?;
        self.write_command(&[0xA0])?;
        self.write_command(&[0xC0])?;
        self.write_command(&[0xA6])?;
        self.write_command(&[0xA8])?;
        self.write_command(&[0x3F])?;
        self.write_command(&[0xD3])?;
        self.write_command(&[0x00])?;
        self.write_command(&[0xd5])?;
        self.write_command(&[0x80])?;
        self.write_command(&[0xD9])?;
        self.write_command(&[0xF1])?;
        self.write_command(&[0xDA])?;
        self.write_command(&[0x12])?;
        self.write_command(&[0xDB])?;
        self.write_command(&[0x40])?;
        self.write_command(&[0x20])?;
        self.write_command(&[0x02])?;
        self.write_command(&[0xA4])?;
        self.write_command(&[0xA6])?;
        sleep(Duration::from_millis(100));
        self.write_command(&[0xAF])?; // --turn on oled panel
        Ok(())
    }

    /// Initializes SPI protocol for the device.
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
            memory: vec![0; WIDTH as usize * HEIGHT as usize],
        })
    }
}
