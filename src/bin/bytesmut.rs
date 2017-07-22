extern crate bytes;

use bytes::BytesMut;

fn main() {
    let mut a = BytesMut::from(&b"hello world"[..]);

    let b = a[0];

    // b.fuck();
    println!("{:?}", b);
    println!("{:?}", a[0]);
}