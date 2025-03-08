use clap::Parser;

use crate::constants::{USAGE_ID, USAGE_PAGE};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
pub struct Cli {
    /// The vendor ID of the keyboard, see info.json
    #[clap(short, long)]
    pub vendor_id: u16,

    /// The product ID of the keyboard, see info.json
    #[clap(short, long)]
    pub product_id: u16,

    /// The usage page of the Raw HID interface
    #[clap(default_value_t = USAGE_ID)]
    pub usage_id: u16,

    /// The usage ID of the Raw HID interface
    #[clap(default_value_t = USAGE_PAGE)]
    pub usage_page: u16,

    /// Polling interval for the connection in ms
    #[clap(default_value_t = 3000)]
    pub poll_interval: u64,
}
