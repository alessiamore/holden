use crate::cipher::Cipher;

pub fn show(

    cipher:&Cipher

){

    println!("Selected Cipher\n");

    println!("{}",cipher.name);

    println!();

    println!("Invented\n");

    println!("{}",cipher.year);

    println!();

    println!("Category\n");

    println!("{}",cipher.category);

    println!();

    println!("Security\n");

    println!("{}",cipher.security);

    println!();

    println!("Description\n");

    println!("{}",cipher.description);

    println!();

    println!("Successor\n");

    println!("{}",cipher.successor);

}
