use std::io;
use std::os::unix::process::CommandExt;
use std::process::Command;

use gpui::SharedString;

pub fn send_notification(name: SharedString, is_active: bool) -> Result<(), io::Error> {
    // hyprctl notify -1 5000 "rgb(d35e04)" "fontsize:35 'home' activated"

    Command::new("hyprctl")
        .args([
            "notify",
            "-1",
            "5000",
            &format!("rgb({})", if is_active { "f4752f" } else { "a6e3b8" }),
            &format!(
                "fontsize:30 '{}' {}",
                name,
                if is_active {
                    "activated"
                } else {
                    "deactivated"
                }
            ),
        ])
        .output()
        .inspect(|res| {
            println!("Activate : {res:?}");
        })
        .inspect_err(|e| {
            eprintln!("Activate : {e}");
        })?;

    Ok(())
}
