#![allow(unused_doc_comments)]

mod to_typescript;
mod typescript;
pub mod utils;
pub(crate) mod casing;
pub(crate) mod zome_proxy;


use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::to_typescript::ToTypescript;
use crate::zome_proxy::write_zome_proxy_header;


const MAGIC_FIRST_LINE: &str = "/* This file is generated by zits. Do not be edited manually */";


/// macro to check from an syn::Item most of them have ident attribs
/// that is the one we want to print but not sure!
macro_rules! check_ztsync {
    ($x: ident, $a: ident, in: $y: tt, $z: tt, $debug: ident) => {
        let has_ztsync_attribute = has_ztsync_attribute(&$a.attrs);
        if $debug {
            if has_ztsync_attribute {
                println!("[zits][debug] Encountered {}: {}", $y, $x.ident.to_string());
            } else {
                println!("[zits][debug] Encountered non-ztsync {}: {}", $y, $x.ident.to_string());
            }
        }

        if has_ztsync_attribute {
            $z
        }
    };
}



// fn should_ignore_file(ignore_file: &gitignore::File, entry: &DirEntry) -> bool {
//     let path = entry.path();

//     ignore_file.is_excluded(&path).unwrap_or(false)
// }



///
fn has_ztsync_attribute(attributes: &Vec<syn::Attribute>) -> bool {
    //println!("has_ztsync_attribute(): {:?}", attributes);
    return utils::has_attribute("hdk_entry_helper", attributes)
       || utils::has_attribute("hdk_extern", attributes)
       || utils::has_attribute("unit_enum", attributes)
       || utils::has_attribute("hdk_entry_defs", attributes)
       || utils::has_attribute("serde", attributes)
}



///
pub struct ParseState /*<'a>*/ {
    pub unprocessed_files: Vec<PathBuf>,
    pub types_file: String,
    pub fns_file: String,
    // pub ignore_file_config: Option<gitignore::File<'a>>,
}



pub fn write_comments(target_str: &mut String, comments: &Vec<String>, indentation_amount: i8) {
    let indentation = utils::build_indentation(indentation_amount);
    match comments.len() {
        0 => (),
        1 => target_str.push_str(&format!("{}/** {} */\n", indentation, &comments[0])),
        _ => {
            target_str.push_str(&format!("{}/**\n", indentation));
            for comment in comments {
                target_str.push_str(&format!("{} * {}\n", indentation, &comment))
            }
            target_str.push_str(&format!("{} */\n", indentation))
        }
    }
}


fn write_type_defs_header(target_str: &mut String){
    target_str.push_str(&format!("{}\n\n", MAGIC_FIRST_LINE));
    target_str.push_str(&format!("import * from '@holochain-open-dev/core-types';
import * from '@holochain/client';
"));
}



///
fn process_rust_file(
    debug: bool,
    input_path: PathBuf,
    state: &mut ParseState,
    uses_typeinterface: bool,
) {
    if debug {
        dbg!(uses_typeinterface);

        println!(
            "processing rust file: {:?}",
            input_path.clone().into_os_string().into_string().unwrap()
        );
    }

    let file = File::open(&input_path);

    if file.is_err() {
        state.unprocessed_files.push(input_path);
        return;
    }

    let mut file = file.unwrap();

    let mut src = String::new();
    if file.read_to_string(&mut src).is_err() {
        state.unprocessed_files.push(input_path);
        return;
    }

    let syntax = syn::parse_file(&src);

    if syntax.is_err() {
        state.unprocessed_files.push(input_path);
        return;
    }

    let syntax = syntax.unwrap();

    for item in syntax.items {
        match item {
            syn::Item::Const(exported_const) => {
                check_ztsync!(exported_const, exported_const, in: "const", {
                    exported_const.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Struct(exported_struct) => {
                check_ztsync!(exported_struct, exported_struct, in: "struct", {
                    exported_struct.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Enum(exported_enum) => {
                check_ztsync!(exported_enum, exported_enum, in: "enum", {
                    exported_enum.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Type(exported_type) => {
                check_ztsync!(exported_type, exported_type, in: "type", {
                    exported_type.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            syn::Item::Fn(exported_fn) => {
                let sig = exported_fn.sig.clone();
                check_ztsync!(sig, exported_fn, in: "fn", {
                    exported_fn.convert_to_ts(state, debug, uses_typeinterface);
                }, debug);
            }
            _ => {}
        }
    }
}


///
pub fn generate_typescript_defs(input: Vec<PathBuf>, output: PathBuf, debug: bool) {
    let uses_typeinterface = output
        .as_os_str()
        .to_str()
        .map(|x| x.ends_with(".d.ts"))
        .unwrap_or(true);

    let mut state: ParseState = ParseState {
        unprocessed_files: Vec::<PathBuf>::new(),
        types_file: String::new(),
        fns_file: String::new(),
        // ignore_file_config: if args.clone().use_ignore_file.is_some() {
        //     match gitignore::File::new(&args.use_ignore_file.unwrap()) {
        //         Ok(gitignore) => Some(gitignore),
        //         Err(err) => {
        //             if args.debug {
        //                 println!("Error: failed to use ignore file! {:#?}", err);
        //             }
        //             None
        //         }
        //     }
        // } else {
        //     None
        // },
    };


    let file_name = output.file_stem().unwrap().to_str().unwrap();
    let zome_name: &str = file_name.split(".").collect::<Vec<&str>>()[0];

    write_type_defs_header(&mut state.types_file);
    write_zome_proxy_header(&mut state.fns_file, &output, &zome_name);



    /// Parse input files
    for input_path in input {
        if !input_path.exists() {
            if debug {
                println!("Path `{:#?}` does not exist", input_path);
            }

            state.unprocessed_files.push(input_path);
            continue;
        }

        if input_path.is_dir() {
            for entry in WalkDir::new(input_path.clone()).sort_by_file_name() {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.into_path();

                        // skip dir files because they're going to be recursively crawled by WalkDir
                        if !path.is_dir() {
                            // make sure it is a rust file
                            let extension = path.extension();
                            if extension.is_some() && extension.unwrap().eq_ignore_ascii_case("rs")
                            {
                                process_rust_file(debug, path, &mut state, uses_typeinterface);
                            } else if debug {
                                println!("Encountered non-rust file `{:#?}`", path);
                            }
                        } else if debug {
                            println!("Encountered directory `{:#?}`", path);
                        }
                    }
                    Err(_) => {
                        println!(
                            "An error occurred whilst walking directory `{:#?}`...",
                            input_path.clone()
                        );
                        continue;
                    }
                }
            }
        } else {
            process_rust_file(debug, input_path, &mut state, uses_typeinterface);
        }
    }


    /// Proxy file footer and write to disk
    state.fns_file.push_str(&format!("}}\n"));

    /** */
    if debug {
        println!("\n");
        println!("======================================");
        println!("FINAL TYPES FILE:");
        println!("======================================");
        println!("{}", state.types_file);
        println!("======================================");
        println!("FINAL FN FILE:");
        println!("======================================");
        println!("{}", state.fns_file);
        println!("======================================");
        println!("Note: Nothing is written in debug mode");
        println!("======================================");
    } else {
        // Verify that the output file either doesn't exists or has been generated by tsync.
        let original_file_path = Path::new(&output);
        if original_file_path.exists() {
            if !original_file_path.is_file() {
                panic!("Specified output path is a directory but must be a file.")
            }
            let original_file = File::open(original_file_path).expect("Couldn't open output file");
            let mut buffer = BufReader::new(original_file);

            let mut first_line = String::new();

            buffer
                .read_line(&mut first_line)
                .expect("Unable to read line");

            if first_line.trim() != MAGIC_FIRST_LINE {
                panic!("Aborting: specified output file exists but doesn't seem to be a tsync output file: {}", first_line)
            }
        }

        let mut file: File = File::create(&output).expect("Unable to write to file");
        match file.write_all(state.types_file.as_bytes()) {
            Ok(_) => println!("[zits][info] Successfully generated typescript bindings: {:#?}", output),
            Err(_) => println!("Failed to generate types, an error occurred."),
        }

        /// Proxy file
        let mut proxy_output: PathBuf = output.clone();
        proxy_output.set_file_name(format!("{}.proxy.ts", zome_name));
        //println!("ProxyFile: {:?}", proxy_output);
        let mut proxy_file: File = File::create(&proxy_output).expect("Unable to write to file");
        match proxy_file.write_all(state.fns_file.as_bytes()) {
            Ok(_) => println!("[zits][info] Successfully generated ZomeProxy: {:#?}", proxy_output),
            Err(_) => println!("Failed to generate ZomeProxy, an error occurred."),
        }

    }

    if state.unprocessed_files.len() > 0 {
        println!("[zits][info] Could not parse the following files:");
        for unprocessed_file in state.unprocessed_files {
            println!("• {:#?}", unprocessed_file);
        }
    }
}
