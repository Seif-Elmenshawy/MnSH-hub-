use std::process::Command;

use colored::Colorize;





pub fn mp4_dlp(link: String, resolution: u32) {
    let quality = format!("bestvideo[height <= {}] + bestaudio / best", resolution);
    let args = [
    "-f",
    &quality,
    "--merge-output-format",
    "mp4",
    &link
    ];
    Command::new("yt-dlp").args(args);
}

pub fn mp3_dlp(link:String) -> Result<(), Box<dyn std::error::Error>>{
    let args = [
        "-x",
        "--audio-format",
        "mp3",
        &link
    ];
    let out = Command::new("yt-dlp").args(args).output()?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("LibreOffice failed: {}", stderr).into());
    }
    println!("{}", "Done".green());
    Ok(())
}