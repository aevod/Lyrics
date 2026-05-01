use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
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
    println!("{}", api);

    let res = reqwest::blocking::get(api)?.text()?;
    let body: serde_json::Value = serde_json::from_str(&res)?;
    
    let lyric = body["syncedLyrics"]
        .as_str()
        .ok_or("Fuck you")?;

    let mut entries = Vec::new();

    for line in lyric.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.contains(']') {
            let parts: Vec<&str> = line.splitn(2, ']').collect();
            let ts = parts[0].trim_start_matches('[');
            let text = parts[1].trim();

            if let Ok(millis) = parse_timestamp(ts) {
                entries.push((millis, text.to_string()));
            }
        }
    }

    let start = Instant::now(); // начало «отсчёта»

    for (millis, text) in entries {
        let elapsed = start.elapsed().as_millis() as u64;

        if millis > elapsed {
            let wait_ms = millis - elapsed;
            thread::sleep(Duration::from_millis(wait_ms));
        }

        println!("{}", text); // выводим строку по таймингу
    }

    Ok(())
}

// парсит "00:01.61" → миллисекунды (u64)
fn parse_timestamp(s: &str) -> Result<u64, ()> {
    if let Some(colon) = s.find(':') {
        let mm = &s[..colon];
        let rest = &s[colon + 1..];

        if let Some(dot) = rest.find('.') {
            let ss = &rest[..dot];
            let ms = &rest[dot + 1..];

            let mm: u64 = mm.parse().map_err(|_| ())?;
            let ss: u64 = ss.parse().map_err(|_| ())?;
            let ms: u64 = ms.parse().map_err(|_| ())?;

            return Ok(mm * 60_000 + ss * 1_000 + ms);
        }
    }
    Err(())
}
