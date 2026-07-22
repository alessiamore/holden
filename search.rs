use crate::cipher::Cipher;

pub struct Search;

impl Search{

    pub fn by_name(

        list:&[Cipher],

        name:&str

    )->Option<Cipher>{

        list.iter()

            .find(|c|

                c.name.eq_ignore_ascii_case(name)

            )

            .cloned()

    }

}
