use std::io::IsTerminal;

pub struct ColorConfig {
    pub enabled: bool,
}

impl ColorConfig {
    pub fn new(force_plain: bool) -> Self {
        if force_plain {
            return Self { enabled: false };
        }

        // Respect NO_COLOR convention
        if std::env::var("NO_COLOR").is_ok() {
            return Self { enabled: false };
        }

        // Respect FORCE_COLOR
        if std::env::var("FORCE_COLOR").is_ok() {
            return Self { enabled: true };
        }

        Self {
            enabled: std::io::stdout().is_terminal(),
        }
    }

    pub fn bold(&self, s: &str) -> String {
        if self.enabled {
            format!("\x1b[1m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }

    pub fn blue_bold(&self, s: &str) -> String {
        if self.enabled {
            format!("\x1b[34m\x1b[1m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn force_plain_disables_color() {
        let config = ColorConfig::new(true);
        assert!(!config.enabled);
    }

    #[test]
    fn bold_plain_returns_plain_text() {
        let config = ColorConfig { enabled: false };
        assert_eq!(config.bold("hello"), "hello");
    }

    #[test]
    fn bold_enabled_returns_ansi() {
        let config = ColorConfig { enabled: true };
        let result = config.bold("hello");
        assert!(result.starts_with("\x1b[1m"));
        assert!(result.ends_with("\x1b[0m"));
        assert!(result.contains("hello"));
    }

    #[test]
    fn blue_bold_plain_returns_plain_text() {
        let config = ColorConfig { enabled: false };
        assert_eq!(config.blue_bold("hello"), "hello");
    }

    #[test]
    fn blue_bold_enabled_returns_ansi() {
        let config = ColorConfig { enabled: true };
        let result = config.blue_bold("hello");
        assert!(result.starts_with("\x1b[34m\x1b[1m"));
        assert!(result.ends_with("\x1b[0m"));
        assert!(result.contains("hello"));
    }
}
