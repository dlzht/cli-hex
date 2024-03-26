extern crate core;

use std::io::{Read, Write};
use anyhow::{anyhow, Error};
use clap::Parser;

fn main() {
  let param = ApplicationParam::parse();
  match run_main(param) {
    Ok(_) => {
      std::process::exit(0);
    },
    Err(err) => {
      eprintln!("Error: {}", err);
      std::process::exit(1);
    }
  }
}

fn run_main(param: ApplicationParam) -> Result<(), Error> {
  return match (&param.text, &param.file, param.decode) {
    (Some(text), None, false) => {
      encode_text(text, param.lower)
    },
    (None, Some(file), false) => {
      encode_file(file, param.lower)
    },
    (Some(text), None, true) => {
      decode_text(text)
    },
    (None, Some(file), true) => {
      decode_file(file)
    },
    _ => {
      Ok(())
    }
  };
}

const BYTE_BUFFER_SIZE: usize = 4096;
const HEX_BUFFER_SIZE: usize = BYTE_BUFFER_SIZE * 2;

fn encode_text(text: &str, lower: bool) -> Result<(), Error> {
  let capacity = if text.len() <= BYTE_BUFFER_SIZE { text.len() * 2 } else { HEX_BUFFER_SIZE };
  let mut res = String::with_capacity(capacity);
  if text.len() < 2048 {
    hex_encode(text.as_bytes(), &mut res, lower);
    print!("{}", res);
  } else {
    for chunk in text.as_bytes().chunks(BYTE_BUFFER_SIZE) {
      res.truncate(0);
      hex_encode(chunk, &mut res, lower);
      print!("{}", res);
    }
  }
  return Ok(());
}

fn encode_file(file: &str, lower: bool) -> Result<(), Error> {
  let mut file = std::fs::File::open(file)?;
  let size = file.metadata().map(|m| m.len() as usize)?;
  let capacity = if size <= BYTE_BUFFER_SIZE { size * 2 } else { HEX_BUFFER_SIZE };
  let mut res = String::with_capacity(capacity);
  if size <= BYTE_BUFFER_SIZE {
    let mut buffer = Vec::with_capacity(size);
    file.read_to_end(&mut buffer)?;
    hex_encode(buffer.as_slice(), &mut res, lower);
    print!("{}", res);
  } else {
    let mut buffer = [0; BYTE_BUFFER_SIZE];
    let mut remain = size;
    while remain > 0 {
      res.truncate(0);
      let count = file.read(&mut buffer)?;
      remain = remain - count;
      hex_encode(&buffer[0..count], &mut res, lower);
      print!("{}", res);
    }
  }
  return Ok(());
}

fn decode_text(text: &str) -> Result<(), Error> {
  let capacity = if text.len() <= HEX_BUFFER_SIZE { text.len() / 2 } else { HEX_BUFFER_SIZE };
  let mut res = Vec::with_capacity(capacity);
  let mut writer = std::io::stdout();
  if text.len() <= HEX_BUFFER_SIZE {
    hex_decode(text.as_bytes(), &mut res)?;
    writer.write(res.as_slice())?;
  } else {
    for chunk in text.as_bytes().chunks(HEX_BUFFER_SIZE) {
      res.truncate(0);
      hex_decode(chunk, &mut res)?;
      writer.write(res.as_slice())?;
    }
  }
  writer.flush()?;
  return Ok(());
}

fn decode_file(file: &str) -> Result<(), Error> {
  let mut file = std::fs::File::open(file)?;
  let size = file.metadata().map(|m| m.len() as usize)?;
  let capacity = if size <= HEX_BUFFER_SIZE { size / 2 } else { BYTE_BUFFER_SIZE };
  let mut res = Vec::with_capacity(capacity);
  let mut writer = std::io::stdout();
  if size <= HEX_BUFFER_SIZE {
    let mut text = Vec::with_capacity(BYTE_BUFFER_SIZE);
    file.read_to_end(&mut text)?;
    hex_decode(&text, &mut res)?;
    writer.write(res.as_slice())?;
  } else {
    let mut remain = size;
    let mut buffer = [0; HEX_BUFFER_SIZE];
    while remain >= HEX_BUFFER_SIZE {
      res.truncate(0);
      file.read_exact(&mut buffer)?;
      hex_decode(&buffer, &mut res)?;
      writer.write(res.as_slice())?;
      remain = remain - HEX_BUFFER_SIZE;
    }
    if remain > 0 {
      res.truncate(0);
      let mut buffer = Vec::with_capacity(remain);
      file.read_to_end(&mut buffer)?;
      hex_decode(&buffer, &mut res)?;
      writer.write(res.as_slice())?;
    }
  }
  writer.flush()?;
  return Ok(());
}

const LOWER: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
const UPPER: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

fn hex_encode(bytes: &[u8], res: &mut String, lower: bool) {
  let alphabet = if lower { &LOWER } else { &UPPER };
  for byte in bytes {
    res.push(alphabet[(byte >> 4) as usize]);
    res.push(alphabet[(byte & 15) as usize]);
  }
}

fn hex_decode(bytes: &[u8], res: &mut Vec<u8>) -> Result<(), Error> {
  for chunk in bytes.chunks(2) {
    let byte = (hex_value(chunk[0])? << 4) | (hex_value(chunk[1])?);
    res.push(byte);
  }
  return Ok(());
}

fn hex_value(byte: u8) -> Result<u8, Error> {
  return match byte {
    48..=57 => {
      Ok(byte - 48)
    },
    65..=70 => {
      Ok(byte - 55)
    }
    97..=102 => {
      Ok(byte - 87)
    }
    _ => {
      Err(anyhow!("Illegal char: '{}' ({})", byte as char, byte))
    }
  }
}

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct ApplicationParam {

  #[arg(short, long, help = "read in text mode (default)")]
  text: Option<String>,

  #[arg(short, long, help = "read in file mode")]
  file: Option<String>,

  #[arg(short, long, default_value = "false", help = "decode text or file")]
  decode: bool,

  #[arg(short, long, default_value = "false", help = "output lower alphabet")]
  lower: bool,

}
