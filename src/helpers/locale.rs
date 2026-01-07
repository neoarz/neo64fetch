use std::env;

pub fn get_locale_info() -> String {
    if let Ok(locale) = env::var("LC_ALL") {
        if !locale.is_empty() {
            return format!("Locale: {}", locale);
        }
    }

    if let Ok(locale) = env::var("LANG") {
        if !locale.is_empty() {
            return format!("Locale: {}", locale);
        }
    }

    "<unknown>".to_string()
}

