use json::{array, object, JsonValue};
use std::io;
use std::io::BufRead;

const INIT_REQUEST_ID: u32 = 0;
const INIT_NOTIFY_ID: u32 = 1;
const SYMBOL_REQUEST_ID: u32 = 10;
const HOVER_REQUEST_ID: u32 = 20;

struct InitRequest {
    json_message: JsonValue,
}

impl InitRequest {
    fn new() -> InitRequest {
        InitRequest {
            json_message: object! {
                "id" => INIT_REQUEST_ID,
                "jsonrpc" => 2.0,
                "method" => "initialize",
                "params" => object!{
                    "processid" => get_pid(),
                    "rootPath" => get_cur_working_dir(),
                    "rootUri" => get_project_uri(),
                    "capabilities" => object!{
                        "workspace" => object!{
              "applyEdit" => true,
              "workspaceEdit" => object!{
                "documentChanges" => true
              },
              "didChangeConfiguration" => object!{
                "dynamicRegistration" => true
              },
              "didChangeWatchedFiles" => object!{
                "dynamicRegistration" => true
              },
              "symbol" => object!{
                "dynamicRegistration" => true,
                "symbolKind" => object!{
                  "valueSet" => array![
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    9,
                    10,
                    11,
                    12,
                    13,
                    14,
                    15,
                    16,
                    17,
                    18,
                    19,
                    20,
                    21,
                    22,
                    23,
                    24,
                    25,
                    26
                  ]
                }
              },
              "executeCommand" => object!{
                "dynamicRegistration" => true
              },
              "configuration" => true,
              "workspaceFolders" => true
            },
            "textDocument" => object!{
              "publishDiagnostics" => object!{
                "relatedInformation" => true
              },
              "synchronization" => object!{
                "dynamicRegistration" => true,
                "willSave" => true,
                "willSaveWaitUntil" => true,
                "didSave" => true
              },
              "completion" => object!{
                "dynamicRegistration" => true,
                "contextSupport" => true,
                "completionItem" => object!{
                  "snippetSupport" => true,
                  "commitCharactersSupport" => true,
                  "documentationFormat" => array![
                    "markdown",
                    "plaintext"
                  ],
                  "deprecatedSupport" => true
                },
                "completionItemKind" => object!{
                  "valueSet" => array![
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    9,
                    10,
                    11,
                    12,
                    13,
                    14,
                    15,
                    16,
                    17,
                    18,
                    19,
                    20,
                    21,
                    22,
                    23,
                    24,
                    25
                  ]
                }
              },
              "hover" => object!{
                "dynamicRegistration" => true,
                "contentFormat" => array![
                  "markdown",
                  "plaintext"
                ]
              },
              "signatureHelp" => object!{
                "dynamicRegistration" => true,
                "signatureInformation" => object!{
                  "documentationFormat" => array![
                    "markdown",
                    "plaintext"
                  ]
                }
              },
              "definition" => object!{
                "dynamicRegistration" => true
              },
              "references" => object!{
                "dynamicRegistration" => true
              },
              "documentHighlight" => object!{
                "dynamicRegistration" => true
              },
              "documentSymbol" => object!{
                "dynamicRegistration" => true,
                "symbolKind" => object!{
                  "valueSet" => array![
                    1,
                    2,
                    3,
                    4,
                    5,
                    6,
                    7,
                    8,
                    9,
                    10,
                    11,
                    12,
                    13,
                    14,
                    15,
                    16,
                    17,
                    18,
                    19,
                    20,
                    21,
                    22,
                    23,
                    24,
                    25,
                    26
                  ]
                }
              },
              "codeAction" => object!{
                "dynamicRegistration" => true
              },
              "codeLens" => object!{
                "dynamicRegistration" => true
              },
              "formatting" => object!{
                "dynamicRegistration" => true
              },
              "rangeFormatting" => object!{
                "dynamicRegistration" => true
              },
              "onTypeFormatting" => object!{
                "dynamicRegistration" => true
              },
              "rename" => object!{
                "dynamicRegistration" => true
              },
              "documentLink" => object!{
                "dynamicRegistration" => true
              },
              "typeDefinition" => object!{
                "dynamicRegistration" => true
              },
              "implementation" => object!{
                "dynamicRegistration" => true
              },
              "colorProvider" => object!{
                "dynamicRegistration" => true
              },
              "foldingRange" => object!{
                "dynamicRegistration" => false,
                "rangeLimit" => 5000,
                "lineFoldingOnly" => true
              }
            }
                    }
                }
            },
        }
    }
}

struct InitNotify {
    json_message: JsonValue,
}

impl InitNotify {
    fn new() -> InitNotify {
        InitNotify {
            json_message: object! {
                "id" => INIT_NOTIFY_ID,
                "jsonrpc" => 2.0,
                "method" => "initialized",
                "params" => object!{}
            },
        }
    }
}

struct SymbolRequest {
    json_message: JsonValue,
}

impl SymbolRequest {
    fn new(symbol_name: &str) -> SymbolRequest {
        SymbolRequest {
            json_message: object! {
                "id" => SYMBOL_REQUEST_ID,
                "jsonrpc" => 2.0,
                "method" => "workspace/symbol",
                "params" => object!{
                  "query" => symbol_name
                }
            },
        }
    }
}

struct Hover {
    json_message: JsonValue,
}

impl Hover {
    fn new(document: &str, line: u32, character: u32) -> Hover {
        Hover {
            json_message: object! {
                "id" => HOVER_REQUEST_ID,
                "jsonrpc" => 2.0,
                "method" => "textDocument/hover",
                "params" => object!{
                  "textDocument" => object!{
                    "uri" => document
                    },
                  "position" => object!{
                    "line" => line,
                    "character" => character
                  }
                }
            },
        }
    }
}

fn get_pid() -> u32 {
    // get parent pid for transaction
    std::process::id()
}

fn get_cur_working_dir() -> String {
    // get absolute path to current dir
    let cwd_path = std::env::current_dir().expect("Error when getting cwd");
    let p = cwd_path
        .into_os_string()
        .into_string()
        .expect("Error when converting cwd to string");

    p
}

fn get_project_uri() -> String {
    let p = get_cur_working_dir();
    let uri = "file://".to_string() + &p;

    uri
}

fn get_msg_size(message: &json::JsonValue) -> usize {
    message.dump().to_string().len()
}

fn get_formatted_message_str(payload: &json::JsonValue) -> String {
    let size = get_msg_size(&payload);

    // create the header
    let header = format!("Content-Length: {}\r\n\r\n", size);

    // put it all together as a string
    (header + &payload.dump()).to_string()
}

pub fn init_request() -> String {
    get_formatted_message_str(&InitRequest::new().json_message)
}

pub fn init_notification() -> String {
    get_formatted_message_str(&InitNotify::new().json_message)
}

pub fn symbol_request(symbol_name: &str) -> String {
    get_formatted_message_str(&SymbolRequest::new(symbol_name).json_message)
}

pub fn hover(document: &str, line: u32, character: u32) -> String {
    get_formatted_message_str(&Hover::new(document, line, character).json_message)
}

pub fn read_message<R: BufRead>(input: &mut R) -> Result<String, io::Error> {
    // Read in the "Content-Length: xx" part.
    let mut size: Option<usize> = None;
    loop {
        let mut buffer = String::new();
        input.read_line(&mut buffer).expect("Error parsing message");

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
                size = Some(usize::from_str_radix(header_value, 10).map_err(|_e| {
                    io::Error::new(io::ErrorKind::InvalidData, "Couldn't read size")
                })?);
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

#[cfg(test)]
mod lsp_message_tests {

    use super::*;

    #[test]
    fn init_req_id_is_0() {
        assert_eq!(0, INIT_REQUEST_ID);
    }

    #[test]
    fn init_req_has_proper_id() {
        let init_req_json = init_request();
        assert!(init_req_json.contains("\"id\":0"));
    }

    #[test]
    fn init_notification_id_is_1() {
        assert_eq!(1, INIT_NOTIFY_ID);
    }

    #[test]
    fn init_notification_has_proper_id() {
        let init_notify_json = init_notification();
        assert!(init_notify_json.contains("\"id\":1"));
    }

    #[test]
    fn symbol_req_id_is_10() {
        assert_eq!(10, SYMBOL_REQUEST_ID);
    }

    #[test]
    fn symbol_req_has_proper_id() {
        let symbol_req_json = symbol_request("fooBar");
        assert!(symbol_req_json.contains("\"id\":10"));
    }

    #[test]
    fn symbol_req_has_passed_symbol() {
        let symbol_req_json = symbol_request("fooBar");
        assert!(symbol_req_json.contains("\"query\":\"fooBar\""));
    }

    #[test]
    fn hover_req_id_is_20() {
        assert_eq!(20, HOVER_REQUEST_ID);
    }

    #[test]
    fn hover_req_has_proper_id() {
        let hover_json = hover("foobar.rs", 420, 69);
        assert!(hover_json.contains("\"id\":20"));
    }

    #[test]
    fn hover_req_has_passed_document_path() {
        let hover_json = hover("foobar.rs", 420, 69);
        assert!(hover_json.contains("\"uri\":\"foobar.rs\""));
    }

    #[test]
    fn hover_req_has_passed_line_num() {
        let hover_json = hover("fooBar.rs", 420, 69);
        assert!(hover_json.contains("\"line\":420"));
    }

    #[test]
    fn hover_is_has_passed_character_num() {
        let hover_json = hover("fooBar.rs", 420, 69);
        assert!(hover_json.contains("\"character\":69"));
    }
}
