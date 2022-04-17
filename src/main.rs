use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

trait IsLocalhost {
    fn is_localhost(&self) -> bool;
}

impl IsLocalhost for Ipv4Addr {
    fn is_localhost(&self) -> bool {
        Ipv4Addr::new(127, 0, 0, 1).eq(self) || Ipv4Addr::new(0, 0, 0, 0).eq(self)
    }
}

impl IsLocalhost for Ipv6Addr {
    fn is_localhost(&self) -> bool {
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).eq(self)
    }
}
impl IsLocalhost for IpAddr {
    fn is_localhost(&self) -> bool {
        match self {
            IpAddr::V4(a) => a.is_localhost(),
            IpAddr::V6(a) => a.is_localhost(),
        }
    }
}

trait Print {
    fn print(&self);
}

impl<T: std::fmt::Debug> Print for T {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let ip_address = Ipv4Addr::new(0, 0, 0, 0);
    ip_address.print();
    ip_address.is_localhost().print();
}
