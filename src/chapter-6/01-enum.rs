#[derive(Debug)]
enum IPType {
    IPv4,
    IPv6,
}

enum IPAddr {
    IPv4(String),
    IPv6(String),
}

enum NewIPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

enum StructIPAddr {
    IPv4(IPAddress),
    IPv6(IPAddress),
}

#[derive(Debug)]
struct IPAddress {
    kind: IPType,
    address: String,
}

fn main() {
    // PART 1 枚举定义及其枚举值
    let v4: IPType = IPType::IPv4;
    let v6: IPType = IPType::IPv6;
    let address_v4: IPAddress = route_find(v4);
    let address_v6: IPAddress = route_find(v6);
    // PART 2
    let addr_v4 = IPAddr::IPv4(String::from("127.0.0.1"));
    let addr_v6 = IPAddr::IPv6(String::from("::1"));
    // PART 3
    let new_addr_v4 = NewIPAddr::IPv4(127, 0, 0, 1);
    let new_addr_v6 = NewIPAddr::IPv6(String::from("::1"));
    // PART 3
    let struct_addr_v4 = StructIPAddr::IPv4(IPAddress {
        kind: IPType::IPv4,
        address: String::from("IP_ADDRESS"),
    });
    let struct_addr_v6 = StructIPAddr::IPv6(IPAddress {
        kind: IPType::IPv6,
        address: String::from("IP_ADDRESS"),
    });
}

fn route_find(ip: IPType) -> IPAddress {
    IPAddress {
        kind: ip,
        address: String::from("IP_ADDRESS"),
    }
}
