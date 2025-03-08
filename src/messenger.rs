use std::{
    io::{self, BufRead},
    thread,
    time::Duration,
};

use hidapi::{HidApi, HidDevice, HidError, HidResult};

use crate::{cli::Cli, constants::HID_MSG_LEN};

#[derive(Debug)]
pub struct HidMessenger {
    config: Cli,
    poll_interval: Duration,
    buffer: [u8; HID_MSG_LEN],
}

impl HidMessenger {
    pub fn new(config: Cli) -> Self {
        Self {
            poll_interval: Duration::from_millis(config.poll_interval),
            buffer: [0; HID_MSG_LEN],
            config,
        }
    }

    pub fn read_device_loop(&mut self) {
        loop {
            match self.read_device() {
                Err(HidError::HidApiErrorEmpty) => {
                    eprintln!("Error(read): The device was not found, retrying...")
                }
                Err(_) => {
                    eprintln!("Error(read): An error occured, reconnecting...")
                }
                Ok(_) => {}
            }

            thread::sleep(self.poll_interval);
        }
    }

    pub fn read_stdin_loop(&mut self) {
        let mut line = String::new();
        let mut input = io::stdin().lock();

        loop {
            match input
                .read_line(&mut line)
                .map_err(|error| HidError::IoError { error })
                .and_then(|_| self.write_device(line.trim()))
            {
                Err(HidError::HidApiErrorEmpty) => {
                    eprintln!("Error(write): The device was not found, retrying...");
                    thread::sleep(self.poll_interval);
                }
                Err(error) => {
                    eprintln!("Error(write): {error}");
                }
                _ => {}
            }

            line.clear();
        }
    }

    fn find_device(&self) -> HidResult<HidDevice> {
        let api = HidApi::new()?;

        api.device_list()
            .find(|device| {
                device.vendor_id() == self.config.vendor_id
                    && device.product_id() == self.config.product_id
                    && device.usage_page() == self.config.usage_page
                    && device.usage() == self.config.usage_id
            })
            .ok_or(HidError::HidApiErrorEmpty)?
            .open_device(&api)
    }

    fn read_device(&mut self) -> HidResult<()> {
        let device = self.find_device()?;

        loop {
            let bytes_read = device.read(&mut self.buffer)?;
            let payload = String::from_utf8_lossy(&self.buffer[..bytes_read]);

            println!("{payload}");
        }
    }

    fn write_device(&mut self, message: &str) -> HidResult<()> {
        let device = self.find_device()?;
        let bytes = message.as_bytes();
        let length = bytes.len();

        if length > HID_MSG_LEN {
            return Err(HidError::IoError {
                error: io::Error::other(format!("Invalid message length ({length})")),
            });
        }

        self.buffer[..length].copy_from_slice(bytes);
        device.write(&self.buffer)?;
        self.buffer.fill(0);

        Ok(())
    }
}
