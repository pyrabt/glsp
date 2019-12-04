extern crate colored;
extern crate json;
#[macro_use]
extern crate clap;

use clap::App;
mod lsp_message;
mod result_handler;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::{thread, time};

fn run_server() -> Result<std::process::Child, io::Error> {
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



// use enum for flags instead?
fn get_flags(matches: &clap::ArgMatches) -> Vec<String> {
    let mut flags: Vec<String> = Vec::new();

    if matches.is_present("all") {
        flags.push("All".to_string())
    }
    if matches.is_present("array") {
        flags.push("Array".to_string())
    }
    if matches.is_present("boolean") {
        flags.push("Boolean".to_string())
    }
    if matches.is_present("class") {
        flags.push("Class".to_string())
    }
    if matches.is_present("constant") {
        flags.push("Constant".to_string())
    }
    if matches.is_present("enum") {
        flags.push("Enum".to_string())
    }
    if matches.is_present("function") {
        flags.push("Function".to_string())
    }
    if matches.is_present("method") {
        flags.push("Method".to_string())
    }
    if matches.is_present("module") {
        flags.push("Module".to_string())
    }
    if matches.is_present("number") {
        flags.push("Number".to_string())
    }
    if matches.is_present("object") {
        flags.push("Object".to_string())
    }
    if matches.is_present("property") {
        flags.push("Property".to_string())
    }
    if matches.is_present("struct") {
        flags.push("Struct".to_string())
    }
    if matches.is_present("variable") {
        flags.push("Variable".to_string())
    }

    return flags;
}

fn get_symbol_req_response(reader: &mut BufReader<std::process::ChildStdout>, id: u32) -> json::JsonValue {
  let mut res: String;
  let check_str = format!("\"id\":{}", id); 
    loop {
        let y = match lsp_message::read_message(reader) {
            Ok(message) => Some(message),
            Err(_err) => None,
        };
        res = y.unwrap();
        if res.contains(&check_str) {
            break;
        }
    }

    return json::parse(&res).unwrap();
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
    let flags = get_flags(&matches);

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

    let _x = match lsp_message::read_message(&mut lock) {
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

    let res_json = get_symbol_req_response(&mut lock, 10);

    result_handler::print_results(&res_json, filename, flags, regex, rls_stdin, &mut lock);
}

#[cfg(test)]
mod main_tests {

    use super::*;

    #[test]
    fn run_server_returns_child_process() {
        assert!(!run_server().is_err());
    }
}
