use rand::Rng;

fn is_in_character_range(
    c: char,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    symbols: bool,
) -> bool {
    (lowercase && c.is_ascii_lowercase())
        || (uppercase && c.is_ascii_uppercase())
        || (numbers && c.is_ascii_digit())
        || (symbols && matches!(c, '!'..='/' | ':'..='@' | '['..='_' | '{'..='~'))
}

fn main() {
    let password_length: u64 = 100;
    let lowercase = true;
    let uppercase = true;
    let numbers = true;
    let symbols = true;

    let mut password = String::new();
    let mut i = 0;
    while i < password_length {
        let random_number = rand::rng().random_range(33..=126);
        let c = char::from_u32(random_number).unwrap();
        if !is_in_character_range(c, lowercase, uppercase, numbers, symbols) {
            continue;
        }
        password.push(c);
        i += 1;
    }
    println!("{password}");
}
