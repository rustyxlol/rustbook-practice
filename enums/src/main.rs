enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let version4 = IpAddrKind::V4;
    let version6 = IpAddrKind::V6;

    let some_number = Some(4);
    let some_string = Some("A string");

    let no_number: Option<i32> = None;

    let x: i8 = 20;
    let y: Option<i8> = Some(4);

    // let sum = x + y // this will throw an error 
}

fn route(ip_kind: IpAddrKind) {
    // Can be called with both v4 and v6, not restraints  for types
}


