use std::env;

pub fn get_locale_info() -> String {
    if let Ok(locale) = env::var("LC_ALL")
        && !locale.is_empty()
    {
        return locale;
    }

    if let Ok(locale) = env::var("LANG")
        && !locale.is_empty()
    {
        return locale;
    }

    "<unknown>".to_string()
}
