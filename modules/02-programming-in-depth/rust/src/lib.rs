pub mod app;
pub mod config;

pub fn normalize_user_input(s: &str) -> String {
    let mut out = String::new();
    let mut prev_ws = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            prev_ws = true;
            continue;
        } else {
            if prev_ws && !out.is_empty() {
                out.push(' ');
            }
            prev_ws = false;
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::normalize_user_input;

    #[test]
    fn trims_and_collapses_spaces() {
        assert_eq!(normalize_user_input("  Hello   world\n"), "Hello world");
    }

    #[test]
    fn empty_and_spaces() {
        assert_eq!(normalize_user_input(""), "");
        assert_eq!(normalize_user_input("   \t\n"), "");
    }

    #[test]
    fn tabs_and_newlines() {
        assert_eq!(normalize_user_input("Foo\tBar\nBaz"), "Foo Bar Baz");
    }
}
