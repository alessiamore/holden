use crate::cipher::Cipher;
use crate::sample_data;

pub struct Repository;

impl Repository{

    pub fn load()->Vec<Cipher>{

        sample_data::load()

    }

}
