use lscolors::LsColors;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    const ALWAYS: &'static str = "always";
    const AUTO: &'static str = "auto";
    const NEVER: &'static str = "never";
}

impl From<Color> for LsColors {
    fn from(value: Color) -> Self {
        match value {
            Color::Always => LsColors::from_env().unwrap_or_default(),
            Color::Auto => {
                if atty::is(atty::Stream::Stdout) {
                    LsColors::from_env().unwrap_or_default()
                } else {
                    LsColors::empty()
                }
            }
            Color::Never => LsColors::empty(),
        }
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            Color::ALWAYS => Ok(Color::Always),
            Color::AUTO => Ok(Color::Auto),
            Color::NEVER => Ok(Color::Never),
            _ => Err(format!(
                "color option can only be [always|auto|never], found : {}",
                value
            )),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Auto
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Color::Always => Color::ALWAYS,
            Color::Auto => Color::AUTO,
            Color::Never => Color::NEVER,
        };
        fmt::Display::fmt(s, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde() {
        assert_eq!(Color::AUTO, format!("{}", Color::default()));
        assert_eq!(Color::ALWAYS, format!("{}", Color::Always));
        assert_eq!(Color::AUTO, format!("{}", Color::Auto));
        assert_eq!(Color::NEVER, format!("{}", Color::Never));
        assert_eq!(Color::ALWAYS.parse::<Color>().unwrap(), Color::Always);
        assert_eq!(Color::AUTO.parse::<Color>().unwrap(), Color::Auto);
        assert_eq!(Color::NEVER.parse::<Color>().unwrap(), Color::Never);
        assert_eq!(
            "none".parse::<Color>().unwrap_err(),
            "color option can only be [always|auto|never], found : none"
        );
    }
}
