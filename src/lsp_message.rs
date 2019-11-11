use json::*;
use std::string::*;

struct InitRequest {
    json_message: JsonValue,
}

impl InitRequest {
    fn new() -> InitRequest {
        InitRequest {
            json_message: object! {
                "id" => 0,
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
                "id" => 2,
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
                "id" => 4,
                "jsonrpc" => 2.0,
                "method" => "workspace/symbol",
                "params" => object!{
                  "query" => symbol_name
                }
            },
        }
    }
}

struct DocSymbolRequest {
  json_message: JsonValue,
}

impl DocSymbolRequest {
    fn new(file_name: String) -> SymbolRequest {
        SymbolRequest {
            json_message: object! {
                "id" => 4,
                "jsonrpc" => 2.0,
                "method" => "textDocument/documentSymbol",
                "params" => object!{
                  "textDocument" => object!{
                    "uri" => file_name
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

pub fn doc_symbol_request(file_uri: String) -> String {
  get_formatted_message_str(&DocSymbolRequest::new(file_uri).json_message)
}
