use std::process::Command;
use std::error::Error;
use urlencoding::encode;
use urlencoding::decode;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>>
{
    const interval: Duration = Duration::from_secs(1);
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

    let res = reqwest::blocking::get(api)?.text()?;
    let body: serde_json::Value = serde_json::from_str(&res)?;
    
    let mut ptitle = String::new();
    let mut lyric = String::new();

    loop
    {
        let out = Command::new("playerctl")
            .arg("metadata")
            .arg("--format")
            .arg("{{artist}}\n{{album}}\n{{title}}")
            .output()
            .expect("PCTLFuck");
        let data = String::from_utf8_lossy(&out.stdout);
        let mut lines = data.lines();

        let artist = encode(lines.next().unwrap_or(""));
        let album = encode(lines.next().unwrap_or(""));
        let title = encode(lines.next().unwrap_or(""));

        if ptitle != title
        {
            let api = format!("https://lrclib.net/api/get?track_name={}&artist_name={}&album_name={}", title, artist, album);

            let res = reqwest::blocking::get(api)?.text()?;
            let body: serde_json::Value = serde_json::from_str(&res)?;
            
            lyric = body["syncedLyrics"]
                .as_str()
                .unwrap_or("LFuck")
                .to_string();
            ptitle = title.into_owned();
        } else
        {
            let pos = Command::new("playerctl")
                .arg("position")
                .output()
                .expect("PosFuck");

            let pos = String::from_utf8_lossy(&pos.stdout);

            let pos: f32 = pos.trim().parse().unwrap();

            let mut current = "";

            for line in lyric.lines()
            {
                let (time, text) = line.split_once(']').unwrap();

                let time = &time[1..];

                let (min, sec) = time.split_once(':').unwrap();

                let min: f32 = min.parse().unwrap();
                let sec: f32 = sec.parse().unwrap();

                let total = min * 60.0 + sec;

                if pos >= total
                {
                    current = text;
                }
            }

            println!("{}", current);
        }
        thread::sleep(interval);
    }
}
