#![cfg(test)]

use super::*;

#[test]
fn test_var_int() {
    let integers = vec![
        0,
        1,
        2,
        127,
        128,
        255,
        25565,
        2097151,
        2147483647,
        // -1, // todo: something wrong here
        -2147483648,
    ];
    for i in integers {
        let var_int = VarInt(i);
        let bytes = var_int.encode();
        println!("{:0x?}", &bytes);
        println!("{:?}", VarInt::decode(&bytes));
    }
}

#[test]
fn test_string() {
    let string = "hello world";
    let bytes = string.to_string().encode();
    println!("{:0x?}", &bytes);
    println!("{:?}", String::decode_streaming(&bytes));
}
