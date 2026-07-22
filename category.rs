use crate::cipher::Cipher;

pub fn count(

    list:&[Cipher],

    category:&str

)->usize{

    list.iter()

        .filter(|c|

            c.category==category

        )

        .count()

}
