use std::net::{Ipv4Addr, TcpListener};

use super::ip_error::IpError;

pub struct IpFunctions;

impl IpFunctions {
    pub fn get_local_ipv4() -> Result<Ipv4Addr, IpError> {
        match TcpListener::bind("0.0.0.0:0") {
            Ok(listener) => match listener.local_addr() {
                Ok(addr) => match addr {
                    std::net::SocketAddr::V4(addr) => Ok(addr.ip().to_owned()),
                    _ => Err(IpError::InvalidAddress),
                },
                Err(e) => Err(IpError::IoError(e)),
            },
            Err(e) => Err(IpError::IoError(e)),
        }
    }
}
