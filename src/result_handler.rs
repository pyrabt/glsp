use crate::lsp_message;
use colored::*;
use json::JsonValue;
use std::io::{BufReader, Write};

struct LspResult {
    name: String,
    line_num: u32,
    location: String,
    kind: String,
}

impl LspResult {
    fn new(name: String, kind: String, location: String, line: u32) -> LspResult {
        LspResult {
            name: name,
            location: location,
            line_num: line,
            kind: kind,
        }
    }
}

struct ResultJson {
    name: String,
    kind_int: u32,
    location: String,
    line: u32,
    character: u32,
}

impl ResultJson {
    fn new(name: String, kind: u32, location: String, line: u32, character: u32) -> ResultJson {
        ResultJson {
            name: name,
            location: location,
            line: line,
            kind_int: kind,
            character: character,
        }
    }
}

fn get_symbol_type(kind: u32) -> String {
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

fn get_hover_req_response(
    reader: &mut BufReader<std::process::ChildStdout>,
    id: u32,
) -> json::JsonValue {
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

fn print_heading() {
    println!(
        "| {0: <15} | {1: <20} | {2: <10} | {3: <10}",
        "Name".green().bold(),
        "Type".green().bold(),
        "Line".green().bold(),
        "Location".green().bold()
    );
}

fn get_response_array_length(json: &JsonValue) -> u64 {
    let ret_len = json["result"].len();

    return ret_len as u64;
}

fn get_parsed_result_json(json: &JsonValue, index: u64) -> ResultJson {
    let name = json["result"][index as usize]["name"].to_string();
    let ret_type = &json["result"][index as usize]["kind"];
    let type_int: u32 = ret_type.dump().parse::<u32>().unwrap();
    let location: String = json["result"][index as usize]["location"]["uri"].to_string();
    let line_num_str =
        json["result"][index as usize]["location"]["range"]["start"]["line"].to_string();
    let line_num = line_num_str.parse::<u32>().unwrap();
    let char_num_str =
        json["result"][index as usize]["location"]["range"]["start"]["character"].to_string();
    let char_num = char_num_str.parse::<u32>().unwrap();

    ResultJson::new(name, type_int, location, line_num, char_num)
}

fn read_result(
    json: &JsonValue,
    index: u64,
    rls_stdin: &mut std::process::ChildStdin,
    lock: &mut BufReader<std::process::ChildStdout>,
) -> LspResult {
    let parsed_json = get_parsed_result_json(json, index);
    let data_type: String;
    if get_symbol_type(parsed_json.kind_int) == "Variable" {
        let request = lsp_message::hover(
            &parsed_json.location,
            parsed_json.line,
            parsed_json.character,
        );
        rls_stdin
            .write_all(request.as_bytes())
            .expect("There was an error sending a message to RLS");
        let result = get_hover_req_response(lock, 20);
        data_type = result["result"]["contents"][0]["value"].to_string();
    } else {
        data_type = get_symbol_type(parsed_json.kind_int);
    }

    return LspResult::new(
        parsed_json.name,
        data_type,
        parsed_json.location,
        parsed_json.line,
    );
}

pub fn print_results(
    json: &JsonValue,
    filename: String,
    flags: Vec<String>,
    regex: &str,
    rls_stdin: &mut std::process::ChildStdin,
    lock: &mut BufReader<std::process::ChildStdout>,
) {
    let max_index = get_response_array_length(json);
    println!("{}", json);
    print_heading();

    for i in 0..max_index {
        let location: String = json["result"][i as usize]["location"]["uri"].to_string();
        if location.contains(".rustup") || location.contains(".cargo") {
            continue;
        }

        let query_res = read_result(json, i, rls_stdin, lock);

        let matches_optional_file = filename == "" || query_res.location.contains(&filename);

        let toolchain =
            query_res.location.contains(".rustup") || query_res.location.contains(".cargo");

        if (flags.contains(&query_res.kind)
            || flags.len() == 0
            || flags.contains(&"All".to_string()))
            && !toolchain
        {
            if matches_optional_file && query_res.name.contains(regex) {
                println!(
                    "| {0: <15} | {1: <20} | {2: <10} | {3: <10}",
                    query_res.name, query_res.kind, query_res.line_num, query_res.location
                );
            }
        }
    }
}

// ------------------- UNIT TESTS --------------------

#[cfg(test)]
mod result_handler_tests {

    use super::*;

    #[test]
    fn get_symbol_type_returns_unknown() {
        assert_eq!("Unknown", get_symbol_type(99));
    }
}
