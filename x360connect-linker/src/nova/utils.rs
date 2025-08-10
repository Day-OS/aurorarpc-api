use std::net::IpAddr;

pub fn get_url(ip: IpAddr, port: u16) -> String{
    format!("http://{}:{}", ip, port)
}