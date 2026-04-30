use std::process::Command;

fn main()
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

    println!("{}\n{}\n{}", artist, album, title);
}
