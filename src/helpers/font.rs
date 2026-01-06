// https://github.com/fastfetch-cli/fastfetch/blob/dev/src/detection/font/font_apple.m

use objc2_app_kit::NSFont;
use objc2_foundation::MainThreadMarker;

pub fn get_font_info() -> String {
    let _mtm = MainThreadMarker::new().expect("Must be on the main thread to query fonts");

    let sys_font = NSFont::systemFontOfSize(12.0);
    let sys_name = sys_font
        .familyName()
        .expect("System font must have a family name")
        .to_string();

    let user_font = NSFont::userFontOfSize(12.0);
    let user_name = match user_font {
        Some(font) => font
            .familyName()
            .map(|name| name.to_string())
            .unwrap_or_else(|| "Helvetica".to_string()),
        None => "Helvetica".to_string(),
    };

    format!("{} [System], {} [User]", sys_name, user_name)
}
