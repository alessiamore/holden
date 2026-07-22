use crate::cipher::Cipher;

pub fn print(

    list:&[Cipher]

){

    let mut items=list.to_vec();

    items.sort_by_key(|c|c.year);

    println!();

    println!("Timeline\n");

    for item in items{

        println!(

            "{} - {}",

            item.year,

            item.name

        );

    }

}
