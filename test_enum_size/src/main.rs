use std::mem::size_of_val;

pub enum Cmd {
    broadcast,
    single,
    double,
    broadcast1,
    single1,
    double1,
    broadcast2,
    single2,
    double2,
    broadcast12,
    single12,
    double12,
}

fn main() {
    let a = Cmd::broadcast12;
    let b = Cmd::single;
    let c = Cmd::double2;

    let x = "\u{80}";

    for b in x.bytes() {
        println!("uuu {:X}", b);
    }

    let x = b"\x80";

    for b in x {
        println!("xxx {:X}", b);
    }

    println!(
        "{:?}, {:?},{:?}",
        size_of_val(&a),
        size_of_val(&b),
        size_of_val(&c)
    );
}
