use crate::cipher::Cipher;

pub fn load()->Vec<Cipher>{

    vec![

        Cipher{

            name:"Caesar Cipher".into(),

            year:50,

            category:"Substitution".into(),

            security:"Broken".into(),

            description:"Alphabet shift encryption.".into(),

            successor:"Vigenère Cipher".into()

        },

        Cipher{

            name:"Vigenère Cipher".into(),

            year:1553,

            category:"Polyalphabetic".into(),

            security:"Historical".into(),

            description:"Uses repeating keyword.".into(),

            successor:"Playfair Cipher".into()

        },

        Cipher{

            name:"Playfair Cipher".into(),

            year:1854,

            category:"Digraph".into(),

            security:"Historical".into(),

            description:"Encrypts letter pairs.".into(),

            successor:"Hill Cipher".into()

        },

        Cipher{

            name:"Hill Cipher".into(),

            year:1929,

            category:"Matrix".into(),

            security:"Historical".into(),

            description:"Linear algebra based.".into(),

            successor:"AES".into()

        },

        Cipher{

            name:"Enigma".into(),

            year:1918,

            category:"Rotor Machine".into(),

            security:"Historical".into(),

            description:"Rotor-based electromechanical machine.".into(),

            successor:"Modern symmetric cryptography".into()

        },

        Cipher{

            name:"One-Time Pad".into(),

            year:1917,

            category:"Stream".into(),

            security:"Perfect".into(),

            description:"Random key equal to message length.".into(),

            successor:"Secure random systems".into()

        }

    ]

}
