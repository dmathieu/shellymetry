use clap::Clap;

#[derive(Debug, Clap)]
pub struct Flags {
    #[clap(short, long, default_value = "shellymetry.json")]
    pub config: String,
}

pub fn build() -> Flags {
    Flags::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_build() {
        let flags = build();
        assert_eq!("shellymetry.json", flags.config);
    }
}
