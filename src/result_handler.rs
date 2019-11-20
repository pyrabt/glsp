use colored::*;
use json::JsonValue;

struct LspResult {
    name: String,
    line_num: u32,
    location: String,
    kind: String,
}

impl LspResult {
    fn new(name: String, kind_num: u32, location: String, line: u32) -> LspResult {
        LspResult {
            name: name,
            location: location,
            line_num: line,
            kind: get_symbol_type(kind_num),
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

fn print_heading() {
    println!(
        "| {0: <15} | {1: <10} | {2: <10} | {3: <10}",
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

fn read_result(json: &JsonValue, index: u64) -> LspResult {
    let name = json["result"][index as usize]["name"].to_string();
    let ret_type = &json["result"][index as usize]["kind"];
    let type_int: u32 = ret_type.dump().parse::<u32>().unwrap();
    let location: String = json["result"][index as usize]["location"]["uri"].to_string();
    let line_num_str =
        json["result"][index as usize]["location"]["range"]["start"]["line"].to_string();
    let line_num = line_num_str.parse::<u32>().unwrap();
    return LspResult::new(name, type_int, location, line_num);
}

pub fn print_results(json: &JsonValue, filename: String, flags: Vec<String>, regex: &str) {
    let max_index = get_response_array_length(json);
    print_heading();

    //let results = Vec::<LSP_Result>::new();

    for i in 0..max_index {
        let query_res = read_result(json, i);

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
                    "| {0: <15} | {1: <10} | {2: <10} | {3: <10}",
                    query_res.name, query_res.kind, query_res.line_num, query_res.location
                );
            }
        }
    }
}

// ------------------- UNIT TESTS --------------------

#[cfg(test)]
mod main_tests {

    use super::*;

    #[test]
    fn get_symbol_type_returns_unknown() {
        assert_eq!("Unknown", get_symbol_type(99));
    }
}
