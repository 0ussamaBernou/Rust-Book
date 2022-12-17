enum IpAddrKind {
    V4,
    V6,
}
enum _Result1<T, E> {
    Ok(T),
    Err(E),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    // enum's struct
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // enum without struct
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _loopback = IpAddr::V6(String::from("::1"));
    let _home = IpAddr::V4(127, 0, 0, 1);

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    value_in_cents(Coin::Dime);
}

fn route(_ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
