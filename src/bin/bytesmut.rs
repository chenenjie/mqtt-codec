extern crate bytes;

use bytes::BytesMut;

struct Enjie{
    df: i32,
    ff: i32,
}
fn main() {
    let mut a = BytesMut::from(&b"hello world"[..]);

    let b = a[0];

    let enjie = Enjie{
        df: 32,
        ff: 32,
    };

    // println!("{:?}", enjie.df);
    //let slice = a[2..4];
    println!("defined on line:{} {}", file!(), line!());

    //println!("{:?}", slice);
    // b.fuck();
    //println!("{:?}", b);
    //println!("{:?}", a[0]);
}
