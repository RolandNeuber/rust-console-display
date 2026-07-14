use std::io::{self, Read, Write};

use crossterm::terminal::enable_raw_mode;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    stdout.write_all(b"\x1b[18t")?;
    stdout.flush()?;

    let mut buffer = [0u8; 64];
    let _ = stdin.read(&mut buffer)?;

    stdout.write_all(b"\x1b[14t")?;
    stdout.flush()?;

    let mut buffer2 = [0u8; 64];
    let _ = stdin.read(&mut buffer2)?;

    let response = String::from_utf8_lossy(&buffer);
    let response2 = String::from_utf8_lossy(&buffer2);
    
    let resp = dbg!(query_pixel_size(&response).expect(""), query_pixel_size(&response2).expect(""));
    println!("Raw response: {:?} {:?}", resp.1.0 / resp.0.0, resp.1.1 / resp.0.1);

    Ok(())
}

fn query_pixel_size(response: &str) -> Option<(u32, u32)> {
    let start = response.find("\x1b[")?;
    let end = response[start..].find('t')? + start;

    let seq = &response[start + 2..end];

    let mut parts = seq.split(';');

    parts.next()?;
    // if parts.next()? != "6" {
    //     return None;
    // }

    let height = parts.next()?.parse().ok()?;
    let width  = parts.next()?.parse().ok()?;

    Some((height, width))
}