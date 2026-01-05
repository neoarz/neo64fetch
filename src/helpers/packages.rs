use std::process::Command;

pub fn get_brew_info() -> String {
    let brew_count = count_lines("--formula");
    let cask_count = count_lines("--cask");
    format!("{} (brew), {} (brew-cask)", brew_count, cask_count)
}

fn count_lines(arg: &str) -> usize {
    let output = Command::new("brew").arg("list").arg(arg).output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.lines().filter(|l| !l.is_empty()).count()
        }
        Err(_) => 0,
    }
}
