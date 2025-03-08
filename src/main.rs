use std::thread;

use clap::Parser;
use qmk_hid::{cli::Cli, messenger::HidMessenger};

fn main() {
    let config = Cli::parse();

    let mut hid_read = HidMessenger::new(config.clone());
    let mut hid_write = HidMessenger::new(config);

    let read_thread = thread::spawn(move || hid_read.read_device_loop());
    let write_thread = thread::spawn(move || hid_write.read_stdin_loop());

    read_thread.join().unwrap();
    write_thread.join().unwrap();
}
