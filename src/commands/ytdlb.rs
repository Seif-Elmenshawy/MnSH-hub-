use std::process::Command;





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

pub fn mp3_dlp(link:String){
    let args = [
        "-x",
        "--audio-format",
        "mp3",
        &link
    ];
    Command::new("yt-dlp").args(args);
}