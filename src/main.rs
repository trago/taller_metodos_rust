enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// enum IpAddr {
//     V4(String),
//     V6(String),
// }
//
// fn main(){
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//     let loopback = IpAddr::V6(String::from("::1"));
// }

// # Option enum
// enum Option<T> {
//     None,
//     Some(T),
// }
// fn main(){
//     let some_number = Some(5);
//     let some_string = Some("a string");
//
//     let absent_number: Option<i32> = None;
//
//     match some_number {
//         Some(v) => println!("The value is {}",v),
//         None => println!("We don't have a valid value")
//     }
// }
