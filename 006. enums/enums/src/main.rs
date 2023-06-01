fn main() {
    struct Ipv4Addr {
        address: String,
        id: i32,
    }

    struct Ipv6Addr {
        address: String,
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    let a = Ipv4Addr {
        address: String::from("aa"),
        id: 1,
    };
    let b = IpAddr::V4(a);

    match b {
        IpAddr::V4(Ipv4Addr { address: a, id: _ }) => println!("{}", a),
        IpAddr::V6(six) => println!("{}", six.address),
    };
}
