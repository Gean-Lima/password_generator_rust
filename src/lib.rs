use std::io::{self, Write};
use rand::{thread_rng, Rng};

static ABC: &str = "abcdefghijklmnopqrstuvxwyz";
static NUMBERS: &str = "0123456789";
static CHAR_SPECIALS: &str = "\\{}[]~^?/;:|";

#[derive(Debug)]
pub struct PasswordOptions {
    pub length: u8,
    pub uppercase: bool,
    pub numbers: bool,
    pub specials: bool
}

impl PasswordOptions {
    pub fn get_user_inputs() -> PasswordOptions {
        let length: u8 = loop {
            print!("Tamanho da senha: ");
            io::stdout().flush().unwrap();

            let mut length_str = String::new();
            io::stdin().read_line(&mut length_str).unwrap();

            break match length_str.trim().parse() {
                Ok(value) => value,
                Err(_) => continue
            };
        };

        let uppercase = Self::format_input_boolean("Conter letras maiusculas? [S/n]: ");
        
        let numbers = Self::format_input_boolean("Conter números? [S/n]: ");
        
        let specials = Self::format_input_boolean("Conter caracteres especiais? [S/n]: ");

        PasswordOptions {
            length,
            uppercase,
            numbers,
            specials
        }
    }

    fn format_input_boolean(message: &str) -> bool {
        let input: bool = loop {
            print!("{message}");
            io::stdout().flush().unwrap();

            let mut input_str = String::new();
            io::stdin().read_line(&mut input_str).unwrap();

            break match input_str.trim().to_lowercase().as_str() {
                "s" => true,
                "n" => false,
                _ => continue
            };
        };

        input 
    }
}

pub fn generate_by_length_digits(options: PasswordOptions) -> String {
    let mut password = String::new();

    let mut password_chars = String::from(ABC);

    if options.uppercase {
        password_chars.push_str(ABC.to_uppercase().as_str());
    }

    if options.numbers {
        password_chars.push_str(NUMBERS.to_uppercase().as_str());
    }

    if options.specials {
        password_chars.push_str(CHAR_SPECIALS.to_uppercase().as_str());
    }

    for _index in 0..options.length {
        password.push(
            password_chars.chars()
                .skip(thread_rng().gen_range(0..password_chars.len()))
                .next()
                .unwrap(),
        );
    }

    password
}
