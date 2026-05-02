use std::process::Command;
use std::error::Error;
use urlencoding::encode;

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

    let artist = encode(lines.next().unwrap_or(""));
    let album = encode(lines.next().unwrap_or(""));
    let title = encode(lines.next().unwrap_or(""));
    let api = format!("https://lrclib.net/api/get?track_name={}&artist_name={}&album_name={}", title, artist, album);
    println!("{}", api);

    let res = reqwest::blocking::get(api)?.text()?;
    let body: serde_json::Value = serde_json::from_str(&res)?;
    
    let lyric = body["syncedLyrics"]
        .as_str()
        .ok_or("Fuck you")?;

    println!("{}", lyric);
    Ok(())
}
