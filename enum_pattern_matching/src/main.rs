fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Null values
    let set_number = Some(5);
    let set_string = Some("toto");
    let absent_number: Option<i32> = None;
}
