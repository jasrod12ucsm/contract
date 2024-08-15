
use std::
    net::IpAddr
;

use if_addrs::get_if_addrs;


pub struct IpFunctions;

impl IpFunctions {
    pub fn get_local_ipv4() -> Result<String, Box<dyn std::error::Error>> {
        let interfaces = get_if_addrs().unwrap();
        for iface in interfaces {
            if iface.name.starts_with("w") {
                if let IpAddr::V4(ipv4) = iface.addr.ip() {
                    if ipv4 != std::net::Ipv4Addr::LOCALHOST {
                        return Ok(ipv4.to_string());
                    }
                }
            }
        }
        Err("not ipv4 is sended".into())
    }
}
