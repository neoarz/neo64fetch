// Kitty Graphics Protocol implementation for terminal image display
//
// Inspired ~~stolen~~ by swiftfetch's implementation:
// https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs
//
// Images are base64-encoded and chunked copying what swiftfetch does
// Compatible terminals: Any terminals which use Kitty protocol, like Ghostty, Kitty, Wezterm

use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use image::{GenericImageView, ImageFormat};
use libc::{STDOUT_FILENO, TIOCGWINSZ, ioctl, winsize};
use std::env;
use std::io::{Cursor, Write};
use std::mem;
use std::path::Path;

// Terminal cell metrics fallback values (pixels per character cell)
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L417-L418
const DEFAULT_CHAR_WIDTH: f32 = 10.0;
const DEFAULT_CHAR_HEIGHT: f32 = 20.0;

// Horizontal spacing between image and text (in terminal columns)
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L419
const DEFAULT_GAP_COLUMNS: usize = 2;

// Maximum bytes per Kitty protocol chunk
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L557
const CHUNK_SIZE: usize = 4096;

// Detects Kitty Graphics Protocol support via environment variables.
//
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L422-L460
//
// Returns false for unsupported terminals
// Right now it just doenst print anything
pub fn terminal_supports_kitty() -> bool {
    if matches!(
        env::var("NEO64FETCH_FORCE_KITTY"),
        Ok(v) if v == "1" || v.eq_ignore_ascii_case("true")
    ) {
        return true;
    }

    if env::var("KITTY_WINDOW_ID").is_ok() {
        return true;
    }
    if env::var("WEZTERM_PANE").is_ok() {
        return true;
    }

    if let Ok(term_program) = env::var("TERM_PROGRAM") {
        let t = term_program.to_lowercase();
        if t.contains("kitty") || t.contains("wezterm") || t.contains("ghostty") {
            return true;
        }
    }

    if let Ok(term) = env::var("TERM")
        && term.to_lowercase().contains("kitty")
    {
        return true;
    }

    false
}

// Queries terminal for character cell dimensions using ioctl(TIOCGWINSZ).
//
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L462-L477
fn terminal_cell_metrics() -> (f32, f32) {
    unsafe {
        let mut ws: winsize = mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut ws) == 0
            && ws.ws_col > 0
            && ws.ws_row > 0
            && ws.ws_xpixel > 0
            && ws.ws_ypixel > 0
        {
            return (
                ws.ws_xpixel as f32 / ws.ws_col as f32,
                ws.ws_ypixel as f32 / ws.ws_row as f32,
            );
        }
    }
    (DEFAULT_CHAR_WIDTH, DEFAULT_CHAR_HEIGHT)
}

// Loads, resizes, and displays an image via Kitty Graphics Protocol.
// Loading Logic:
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L479-L527
//
// Resize logic:
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L530-L545
//
// Transmission logic:
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L547-L578
//
// Returns (column_offset, total_rows) for side-by-side text printing.
// Returns (0, 0) on failure; unsupported terminal or image load error (skill issue)
fn process_and_print_image(img: image::DynamicImage, target_height: u32) -> (usize, usize) {
    let ratio = target_height as f32 / img.height() as f32;
    let w = ((img.width() as f32 * ratio).round().max(1.0)) as u32;
    let image = img.resize_exact(w, target_height, image::imageops::FilterType::Lanczos3);

    let (width, height) = image.dimensions();

    // Kitty protocol requires PNG format, even for JPEG/WebP sources
    let mut png_bytes = Vec::new();
    if image
        .write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)
        .is_err()
    {
        return (0, 0);
    }

    let encoded = BASE64.encode(&png_bytes);
    let mut output = String::new();
    let mut start = 0;
    let mut first = true;

    while start < encoded.len() {
        let end = (start + CHUNK_SIZE).min(encoded.len());
        let chunk = &encoded[start..end];
        let more = if end < encoded.len() { 1 } else { 0 };

        if first {
            output.push_str(&format!(
                "\x1b_Ga=T,f=100,s={},v={},m={};",
                width, height, more
            ));
            first = false;
        } else {
            output.push_str(&format!("\x1b_Gm={};", more));
        }
        output.push_str(chunk);
        output.push_str("\x1b\\");
        start = end;
    }

    // Convert pixel dimensions to terminal columns/rows
    let (char_w, char_h) = terminal_cell_metrics();
    let cols = ((width as f32 / char_w).ceil() as usize).max(1) + DEFAULT_GAP_COLUMNS;
    let rows = ((height as f32 / char_h).ceil() as usize).max(1);
    let padding_top = 0;
    let total_rows = rows + padding_top;

    for _ in 0..total_rows {
        println!();
    }

    // use cursor positioning to place image
    print!("\x1b[{}A", total_rows);

    if padding_top > 0 {
        print!("\x1b[{}B", padding_top);
    }

    print!("\x1b[s");
    print!("{}", output);
    std::io::stdout().flush().ok();


    print!("\x1b[u");

    if padding_top > 0 {
        print!("\x1b[{}A", padding_top);
    }
    std::io::stdout().flush().ok();

    (cols, total_rows)
}

pub fn print_image_and_setup(path: &str, target_height: u32) -> (usize, usize) {
    if !terminal_supports_kitty() {
        return (0, 0);
    }

    match image::open(Path::new(path)) {
        Ok(img) => process_and_print_image(img, target_height),
        Err(_) => (0, 0),
    }
}

pub fn print_image_from_memory(bytes: &[u8], target_height: u32) -> (usize, usize) {
    if !terminal_supports_kitty() {
        return (0, 0);
    }

    match image::load_from_memory(bytes) {
        Ok(img) => process_and_print_image(img, target_height),
        Err(_) => (0, 0),
    }
}

// Prints text at a horizontal offset for side-by-side layout with image.
//
// swiftfetch uses space padding instead of cursor movement:
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L236-L245
pub fn print_with_offset(offset: usize, text: &str) {
    if offset > 0 {
        print!("\r\x1b[{}C{}\n", offset, text);
    } else {
        println!("{}", text);
    }
}

// Fills remaining vertical space if text is shorter than image height.
//
// Called after all info lines are printed to make sure the image is not cut off.
// See: https://github.com/Ly-sec/swiftfetch/blob/main/src/display.rs#L338-L348
pub fn finish_printing(offset: usize, lines_printed: usize, image_rows: usize) {
    if offset > 0 && lines_printed < image_rows {
        for _ in 0..(image_rows - lines_printed) {
            println!();
        }
    }
}

#[macro_export]
macro_rules! offset_println {
    ($offset:expr, $($arg:tt)*) => {
        $crate::output::image::print_with_offset($offset, &format!($($arg)*))
    };
}

pub use offset_println;
