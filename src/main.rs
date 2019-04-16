use std::{
    env,
    io::{
        self,
        Read,
        Write,
        ErrorKind,
    },
    process::{
        Command,
        Stdio
    }
    
};



fn main() {
    // Join args with space
    let args: Vec<u8> = env::args()
        .skip(1)
        .map(String::into_bytes)
        .collect::<Vec<Vec<u8>>>()
        .join(&b' ');

    // Reader will be either args or stdin
    let mut reader: Box<dyn Read> = {
        if args.len() > 0 {
            Box::new(&args[..])
        } else {
            Box::new(io::stdin())
        }
    };
        
    let xclip = Command::new("xclip")
        .arg("-selection")
        .arg("c")
        .stdin(Stdio::piped())
        .spawn();

    // Writer will be either xclip's stdin or stdout
    let mut writer: Box<dyn Write> =  {

        let xclip = xclip.ok()
            .and_then(|x| x.stdin);

        match xclip {
            Some(stdin) => Box::new(stdin),
            None => Box::new(io::stdout())
        }
    };

    ri(&mut reader, &mut writer)
        .expect("Something happened.");
}

// std::io::copy clone, except translates characters into Discord's regional indicator blocks.
// Also emboldens characters that aren't regional indicators
// Embiggens spaces.
fn ri<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
    let mut buf = [0u8; 5];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        for &c in &buf[..len] {
                if c == b' ' {
                    writer.write(b"      ")?;
                } else if c.is_ascii_alphabetic() {
                    write!(writer, ":regional_indicator_{}: ", c.to_ascii_lowercase() as char)?;
                } else if c.is_ascii_whitespace() {
                    write!(writer, "{}", c as char)?;
                } else {
                    write!(writer, "**{}** ", c as char)?;
                }
        }
        written += len as u64;
    }
}
