use display_info::DisplayInfo;

pub fn get_display_info() -> String {
    let displays = DisplayInfo::all().unwrap_or_else(|_| vec![]);

    if let Some(main) = displays.iter().find(|d| d.is_primary) {
        let p_width = (main.width as f32 * main.scale_factor) as u32;
        let p_height = (main.height as f32 * main.scale_factor) as u32;

        let diag_mm = ((main.width_mm as f32).powi(2) + (main.height_mm as f32).powi(2)).sqrt();
        let inches = (diag_mm / 25.4).round() as u32;

        let name = if main.name.is_empty() {
            "Color LCD"
        } else {
            &main.name
        };

        let tag = if main.is_primary {
            "[Built-in]"
        } else {
            "[External]"
        };

        format!(
            "({}): {}x{} @ {}x in {}\", {} Hz {}",
            name, p_width, p_height, main.scale_factor as u32, inches, main.frequency as u32, tag
        )
    } else {
        "unknown".to_string()
    }
}
