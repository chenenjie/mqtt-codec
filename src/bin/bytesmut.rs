extern crate bytes;

use bytes::BytesMut;

fn main() {
    let mut a = BytesMut::from(&b"hello world"[..]);

    let b = a[0];
    //let slice = a[2..4];

    //println!("{:?}", slice);
    // b.fuck();
    //println!("{:?}", b);
    //println!("{:?}", a[0]);
}
