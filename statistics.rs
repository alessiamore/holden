use crate::cipher::Cipher;

pub fn print(

    list:&[Cipher]

){

    println!();

    println!("--------------------");

    println!();

    println!(

        "Total ciphers: {}",

        list.len()

    );

}
