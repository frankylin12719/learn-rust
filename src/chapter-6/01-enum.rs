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

// PART 2 枚举值类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Log(i32, i32, i32),
}

// 等效于
// enum StructMessage {
//     Quit(QuitStruct),
//     Move(MoveStruct),
//     Write(WriteStruct),
//     Log(LogStruct),
// }

// struct QuitStruct;
// struct MoveStruct {
//     x: i32,
//     y: i32,
// }
// struct WriteStruct(String);
// struct LogStruct(i32, i32, i32);

// 枚举中的方法定义 与 结构体数据类型 一样
impl Message {
    fn call(&self) {
        println!("call this");
    }
}

fn main() {
    // PART 1 枚举定义及其枚举值
    let v4: IPType = IPType::IPv4;
    let v6: IPType = IPType::IPv6;
    let address_v4: IPAddress = route_find(v4);
    let address_v6: IPAddress = route_find(v6);

    let addr_v4 = IPAddr::IPv4(String::from("127.0.0.1"));
    let addr_v6 = IPAddr::IPv6(String::from("::1"));

    let new_addr_v4 = NewIPAddr::IPv4(127, 0, 0, 1);
    let new_addr_v6 = NewIPAddr::IPv6(String::from("::1"));

    let struct_addr_v4 = StructIPAddr::IPv4(IPAddress {
        kind: IPType::IPv4,
        address: String::from("IP_ADDRESS"),
    });
    let struct_addr_v6 = StructIPAddr::IPv6(IPAddress {
        kind: IPType::IPv6,
        address: String::from("IP_ADDRESS"),
    });

    // PART 3 枚举的方法
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route_find(ip: IPType) -> IPAddress {
    IPAddress {
        kind: ip,
        address: String::from("IP_ADDRESS"),
    }
}
