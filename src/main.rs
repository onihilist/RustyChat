
use std::io;
use std::io::{Read, Write};
use std::os::raw::c_int;
use colored::Colorize;
use server::protocols::{protocolData};
use crate::server::client;
use std::env;

mod utils;
mod core;
mod server;

fn main() {
    env::set_var("RUST_BACKTRACE", "0");
    client::handler();
}

fn write_to_server(stream: &mut impl Write, data: protocolData) -> io::Result<()> {
    let (protocol_bytes, sender_bytes, receiver_bytes, data_bytes) = data.to_byte_slices();
    let payload = server::protocols::concatenate_slices(
        b"::",
        protocol_bytes,
        sender_bytes,
        receiver_bytes,
        data_bytes
    );
    stream.write_all(payload)
}