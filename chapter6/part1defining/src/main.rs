fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number: Option<i32> = None;
}

fn route(addr: IpAddr) {
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
