#![allow(unused_doc_comments)]

mod to_typescript;
mod typescript;
pub mod utils;
pub(crate) mod casing;
pub(crate) mod holochain_imports;
mod parser;


use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::parser::*;


const MAGIC_FIRST_LINE: &str = "/* This file is generated by zits. Do not edit manually */";


pub struct GenConfig {
    can_debug: bool,
    //can_proxy: bool,
    can_hc_imports: bool,
    uses_typeinterface: bool,
}


///
pub fn generate_typescript_bindings(
    input: Vec<PathBuf>,
    external_imports: Vec<String>,
    output: PathBuf,
    can_debug: bool,
    can_hc_imports: bool,
    can_proxy: bool,
    maybe_default_zome_name: Option<String>,
) {
    let uses_typeinterface = output
       .as_os_str()
       .to_str()
       .map(|x| x.ends_with(".d.ts"))
       .unwrap_or(true);


    let mut state: ParseState = ParseState::new(
        GenConfig {
            can_debug,
            //can_proxy,
            can_hc_imports,
            uses_typeinterface,
        });

    state.set_external_import_header(external_imports);

    let file_name = output.file_stem().unwrap().to_str().unwrap();
    let zome_name: &str = file_name.split(".").collect::<Vec<&str>>()[0];
    let default_zome_name: String = if let Some(dzn) = maybe_default_zome_name {
        dzn
    } else {
        zome_name.to_string()
    };

    if !can_debug {
        state.write_type_defs_header();
        state.write_zome_fn_names_header(&zome_name);
        if can_proxy { state.write_zome_proxy_header(&zome_name, &default_zome_name); }
    }


    /// Parse input files
    for input_path in input {
        if !input_path.exists() {
            if can_debug {
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
                                state.parse_rust_file(path);
                            } else if can_debug {
                                println!("Encountered non-rust file `{:#?}`", path);
                            }
                        } else if can_debug {
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
            state.parse_rust_file(input_path);
        }
    }



    if can_proxy {
        /// ZomeProxy file footer
        state.zome_proxy_output.push_str(&format!("}}\n"));
        /// Append type imports to ZomeProxy
        state.write_type_defs_import(&zome_name);
        /// ZomeFnNames file footer
        state.write_zome_fn_names_footer(&zome_name, &default_zome_name);
        //state.zome_fn_names_output.push_str(&format!("]\n"));
    }

    /** */
    if can_debug {
        println!("\n");
        println!("======================================");
        println!("Debug mode try run output");
        println!("======================================");
        //println!("======================================");
        println!("TYPE DEFS FILE for \"{}\"", zome_name);
        println!("======================================");
        println!("{}", state.type_defs_output);
        println!("======================================");
        if can_proxy {
            println!("ZomeProxy FILE for \"{}\"", zome_name);
            println!("======================================");
            println!("{}", state.zome_proxy_output);
            println!("======================================");
        }
        println!("Function Names for \"{}\"", zome_name);
        println!("======================================");
        println!("{}", state.zome_fn_names_output);
        println!("======================================");
    } else {
        println!("======================================");
        println!("Bindings generated for \"{}\"", zome_name);
        println!("======================================");

        let count_const = state.converted_items["const"].len();
        let count_type = state.converted_items["type"].len();
        let count_struct = state.converted_items["struct"].len();
        let count_enum = state.converted_items["enum"].len();
        let count_fn = state.converted_items["fn"].len();

        println!("Total Items found: {}", count_const + count_type + count_struct + count_enum + count_fn);
        if count_const > 0 {println!("  -  const: {}", count_const)}
        if count_type > 0 {println!("  -   type: {}", count_type)}
        if count_struct > 0 {println!("  - struct: {}", count_struct)}
        if count_enum > 0 {println!("  -   enum: {}", count_enum)}
        if count_fn > 0 {println!("  -     fn: {}", count_fn)}

        // Verify that the output file either doesn't exists or has been generated by zits.
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
                panic!("Aborting: specified output file exists but doesn't seem to be a zits output file: {}", first_line)
            }
        }

        if count_const + count_type + count_struct + count_enum > 0 {
            let mut types_output: PathBuf = output.clone();
            types_output.set_file_name(format!("{}.types.ts", zome_name));
            let mut types_file: File = File::create(&types_output).expect("Unable to write to file");
            match types_file.write_all(state.type_defs_output.as_bytes()) {
                Ok(_) => println!("Successfully generated types: {:#?}", types_output),
                Err(_) => println!("Failed to generate types, an error occurred."),
            }
        } else {
            println!("Types file not generated as no types have been found.");
        }

        if can_proxy {
            /// Proxy file
            let mut proxy_output: PathBuf = output.clone();
            proxy_output.set_file_name(format!("{}.proxy.ts", zome_name));
            //println!("ProxyFile: {:?}", proxy_output);
            let mut proxy_file: File = File::create(&proxy_output).expect("Unable to write to file");
            match proxy_file.write_all(state.zome_proxy_output.as_bytes()) {
                Ok(_) => println!("Successfully generated ZomeProxy: {:#?}", proxy_output),
                Err(_) => println!("Failed to generate ZomeProxy, an error occurred."),
            }
        }

        /// FnNames file
        if count_fn > 0 {
            let mut fn_output: PathBuf = output.clone();
            fn_output.set_file_name(format!("{}.fn.ts", zome_name));
            //println!("ProxyFile: {:?}", proxy_output);
            let mut fn_file: File = File::create(&fn_output).expect("Unable to write to file");
            match fn_file.write_all(state.zome_fn_names_output.as_bytes()) {
                Ok(_) => println!("Successfully generated FnNames: {:#?}", fn_output),
                Err(_) => println!("Failed to generate FnNames, an error occurred."),
            }
        } else {
            println!("FnNames file not generated as no functions have been found");
        }
    }

    if state.unprocessed_files.len() > 0 {
        println!("[zits][info] Could not parse the following files:");
        for unprocessed_file in state.unprocessed_files {
            println!("• {:#?}", unprocessed_file);
        }
    }
    println!("======================================");
}
