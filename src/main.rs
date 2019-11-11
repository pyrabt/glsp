extern crate colored;
extern crate json;
#[macro_use]
extern crate clap;

use clap::App;
mod lsp_message;
use colored::*;
use std::env;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::{exit, Command, Stdio};
use std::{thread, time};

fn run_server() -> Result<std::process::Child, std::io::Error> {
  let instance = match Command::new("rls")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
  {
    Ok(instance) => instance,
    Err(error) => return Err(error),
  };
  Ok(instance)
}

fn get_symbol_type(kind: u64) -> String {
  match kind {
    1 => "File".to_string(),
    2 => "Module".to_string(),
    3 => "Namespace".to_string(),
    4 => "Package".to_string(),
    5 => "Class".to_string(),
    6 => "Method".to_string(),
    7 => "Property".to_string(),
    8 => "Field".to_string(),
    9 => "Constructor".to_string(),
    10 => "Enum".to_string(),
    11 => "Interface".to_string(),
    12 => "Function".to_string(),
    13 => "Variable".to_string(),
    14 => "Constant".to_string(),
    15 => "String".to_string(),
    16 => "Number".to_string(),
    17 => "Boolean".to_string(),
    18 => "Array".to_string(),
    19 => "Object".to_string(),
    20 => "Key".to_string(),
    21 => "Null".to_string(),
    22 => "EnumMember".to_string(),
    23 => "Struct".to_string(),
    24 => "Event".to_string(),
    25 => "Operator".to_string(),
    26 => "TypeParameter".to_string(),
    _ => "Unknown".to_string(),
  }
}

fn read_message<R: BufRead>(input: &mut R) -> Result<String, io::Error> {
  // Read in the "Content-Length: xx" part.
  let mut size: Option<usize> = None;
  loop {
    let mut buffer = String::new();
    input.read_line(&mut buffer)?;

    // End of input.
    if buffer.is_empty() {
      return Err(io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "EOF encountered in the middle of reading LSP headers",
      ));
    }

    // Header section is finished, break from the loop.
    if buffer == "\r\n" {
      break;
    }

    let res: Vec<&str> = buffer.split(' ').collect();

    // Make sure header is valid.
    if res.len() != 2 {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("Header '{}' is malformed", buffer),
      ));
    }
    let header_name = res[0].to_lowercase();
    let header_value = res[1].trim();

    match header_name.as_ref() {
      "content-length:" => {
        size = Some(
          usize::from_str_radix(header_value, 10)
            .map_err(|_e| io::Error::new(io::ErrorKind::InvalidData, "Couldn't read size"))?,
        );
      }
      "content-type:" => {
        if header_value != "utf8" && header_value != "utf-8" {
          return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Content type '{}' is invalid", header_value),
          ));
        }
      }
      // Ignore unknown headers (specification doesn't say what to do in this case).
      _ => (),
    }
  }
  let size = match size {
    Some(size) => size,
    None => {
      return Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Message is missing 'content-length' header",
      ));
    }
  };
  //println!("reading: {:?} bytes", size);

  let mut content = vec![0; size];
  input.read_exact(&mut content)?;

  String::from_utf8(content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn main() {

  // The YAML file is found relative to the current file, similar to how modules are found
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  // get the passed symbol we're looking for
  let regex = matches.value_of("regex").unwrap();

  let mut filename: String = "".to_string();

  if matches.is_present("file") {
      filename = matches.value_of("file").unwrap().to_string();
    }

  // flags
  let mut flags: Vec<String> = Vec::new();
  if matches.is_present("all") { flags.push("All".to_string())}
  if matches.is_present("array") { flags.push("Array".to_string())}
  if matches.is_present("boolean") { flags.push("Boolean".to_string())}
  if matches.is_present("class") { flags.push("Class".to_string())}
  if matches.is_present("constant") { flags.push("Constant".to_string())}
  if matches.is_present("enum") { flags.push("Enum".to_string())}
  if matches.is_present("function") { flags.push("Function".to_string())}
  if matches.is_present("method") { flags.push("Method".to_string())}
  if matches.is_present("module") { flags.push("Module".to_string())}
  if matches.is_present("number") { flags.push("Number".to_string())}
  if matches.is_present("object") { flags.push("Object".to_string())}
  if matches.is_present("property") { flags.push("Property".to_string())}
  if matches.is_present("struct") { flags.push("Struct".to_string())}
  if matches.is_present("variable") { flags.push("Variable".to_string())}


  // start up the server to send/receive
  let mut server_instance = run_server().expect("Unable to start Rust Lang Server");
  let rls_stdin = server_instance.stdin.as_mut().unwrap();
  let mut rls_stdout = server_instance.stdout;

  // get init request string
  let full_msg = lsp_message::init_request();

  rls_stdin
    .write_all(&full_msg.as_bytes())
    .expect("Error writing json dump to stdin");

  let mut lock = BufReader::new(rls_stdout.take().unwrap());

  let _x = match read_message(&mut lock) {
    Ok(message) => Some(message),
    Err(err) => {
      println!("{:?}", err);
      None
    }
  };

  // put it all together as a string
  let full_notify_msg = lsp_message::init_notification();

  let full_req = lsp_message::symbol_request(regex);

  // I guess I need to wait for the server to fully start up after notification
  let ten_millis = time::Duration::from_millis(1400);

  rls_stdin
    .write_all(full_notify_msg.as_bytes())
    .expect("AND I OOP");

  thread::sleep(ten_millis);
  rls_stdin
    .write_all(full_req.as_bytes())
    .expect("sk sk sk sk");

  let mut y = match read_message(&mut lock) {
    Ok(message) => Some(message),
    Err(_err) => None,
  };
  let mut res = y.unwrap();

  loop {
    y = match read_message(&mut lock) {
      Ok(message) => Some(message),
      Err(_err) => None,
    };
    res = y.unwrap();
    if res.contains("result") {
      break;
    }
  }

  let res_json = json::parse(&res).unwrap();

  let ret_len = &res_json["result"].len();
  let max_index = *ret_len as u64;


  println!(
    "| {0: <10} | {1: <10} | {2: <10}",
    "Type".green().bold(),
    "Line".green().bold(),
    "Location".green().bold()
  );

  for i in 0..max_index {
    let ret_type = &res_json["result"][i as usize]["kind"];
    let type_int: u64 = ret_type.dump().parse::<u64>().unwrap();
    let type_name = get_symbol_type(type_int);
    let location: &String = &res_json["result"][i as usize]["location"]["uri"].to_string();
    let line_num = &res_json["result"][i as usize]["location"]["range"]["start"]["line"].to_string();

    let matches_optional_file = filename == "" || location.contains(&filename);

    let toolchain = location.contains(".rustup") || location.contains(".cargo");

    if (flags.contains(&type_name) || flags.len() == 0 || flags.contains(&"All".to_string())) && !toolchain {
      if matches_optional_file {
        println!("| {0: <10} | {1: <10} | {2: <10}", type_name, line_num, location);
      }
    }
  }

}

#[cfg(test)]
mod main_tests {

  use super::*;

  #[test]
  fn get_symbol_type_returns_unknown() {
    assert_eq!("Unknown", get_symbol_type(99));
  }

}