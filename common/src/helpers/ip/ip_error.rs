use std::io;

#[derive(Debug)]
pub enum IpError {
    IoError(io::Error),
    InvalidAddress,
}