use rand::Rng;

/// Configuration for password generation
#[derive(Debug, Clone)]
struct PasswordConfig {
    length: usize,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    symbols: bool,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self {
            length: 15,
            lowercase: true,
            uppercase: true,
            numbers: true,
            symbols: true,
        }
    }
}

impl PasswordConfig {
    /// Check if a character matches the enabled character sets
    fn is_valid_char(&self, c: char) -> bool {
        (self.lowercase && c.is_ascii_lowercase())
            || (self.uppercase && c.is_ascii_uppercase())
            || (self.numbers && c.is_ascii_digit())
            || (self.symbols && matches!(c, '!'..='/' | ':'..='@' | '['..='_' | '{'..='~'))
    }

    /// Generate a password based on this configuration
    fn generate(&self) -> String {
        let mut rng = rand::rng();

        std::iter::from_fn(|| {
            let random_number = rng.random_range(33..=126);
            char::from_u32(random_number)
        })
        .filter(|&c| self.is_valid_char(c))
        .take(self.length)
        .collect()
    }
}

fn main() {
    let config = PasswordConfig::default();
    let password = config.generate();
    println!("{password}");
}
