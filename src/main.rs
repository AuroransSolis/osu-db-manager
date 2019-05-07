extern crate byteorder;

use byteorder::{WriteBytesExt, ReadBytesExt};

mod databases;
mod deserialize_primitives;
mod interactive;
mod query;
mod serialize_primitives;

fn main() {
    println!("Hello, world!");
}
