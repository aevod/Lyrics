use std::process::Command;
use std::thread;
use std::time;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>
{
    let out = Command::new("playerctl")
        .arg("metadata")
        .arg("--format")
        .arg("{{artist}}\n{{album}}\n{{title}}")
        .output()
        .expect("Fuck you");
    let data = String::from_utf8_lossy(&out.stdout);
    let mut lines = data.lines();

    let artist = lines.next().unwrap_or("").to_string();
    let album = lines.next().unwrap_or("").to_string();
    let title = lines.next().unwrap_or("").to_string();
    let api = format!("https://lrclib.net/api/get?track_name={}&artist_name={}&album_name={}", title, artist, album);

    let res = reqwest::blocking::get(api)?.text()?;
    
    Ok(())
}
