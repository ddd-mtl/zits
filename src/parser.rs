use std::collections::{BTreeMap};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use convert_case::{Case, Casing};
use syn::Visibility;
use crate::{GenConfig, MAGIC_FIRST_LINE};
use crate::holochain_imports::{HOD_CORE_TYPES_IMPORTS, HOLOCHAIN_CLIENT_IMPORTS};
use crate::to_typescript::ToTypescript;
use crate::utils::has_zits_attribute;


///
pub struct ParseState {
   pub config: GenConfig,
   pub unprocessed_files: Vec<PathBuf>,
   pub type_defs_output: String,
   pub zome_proxy_output: String,
   pub zome_fn_names_output: String,
   /// item_kind -> item_ident[]
   pub converted_items: BTreeMap<&'static str, Vec<String>>,
}


impl ParseState {

   ///
   pub fn new(config: GenConfig) -> Self {
      let mut converted_items = BTreeMap::new();
      converted_items.insert("const", Vec::<String>::new());
      converted_items.insert("fn", Vec::<String>::new());
      converted_items.insert("struct", Vec::<String>::new());
      converted_items.insert("enum", Vec::<String>::new());
      converted_items.insert("type", Vec::<String>::new());

      Self {
         config,
         unprocessed_files: Vec::<PathBuf>::new(),
         type_defs_output: String::new(),
         zome_proxy_output: String::new(),
         zome_fn_names_output: String::new(),
         converted_items,
      }
   }

   ///
   fn parse_item<T: ToTypescript>(&mut self, item: T, is_pub_const: bool) {
      /// Const only needs the "pub" token
      if !is_pub_const {
         /// Must have a zits attributes
         let has_zits_attribute = has_zits_attribute(&item.attrs());
         if !has_zits_attribute {
            if self.config.can_debug {
               println!("[zits][debug] Encountered non-zits {} \"{}\"", item.kind(), item.ident().to_string());
            }
            return;
         }
      }
      /// Store item
      if self.config.can_debug {
         println!("[zits][debug] Encountered {} \"{}\"", item.kind(), item.ident().to_string());
      }
      // TODO: Fix ugly copy
      let mut new_vec = self.converted_items[item.kind()].clone();
      new_vec.push(item.ident().to_string());
      self.converted_items.insert(item.kind(), new_vec);
      /// Parse item
      item.convert_to_ts(self, self.config.can_debug, self.config.uses_typeinterface);
   }


   ///
   pub fn parse_rust_file(&mut self, input_path: PathBuf) {
      if self.config.can_debug {
         dbg!(self.config.uses_typeinterface);

         println!(
            "Parsing rust file: {:?}",
            input_path.clone().into_os_string().into_string().unwrap()
         );
      }

      let file = File::open(&input_path);
      let Ok(mut file) = file else {
         self.unprocessed_files.push(input_path);
         return;
      };

      let mut src = String::new();
      if file.read_to_string(&mut src).is_err() {
         self.unprocessed_files.push(input_path);
         return;
      }

      let syntax = syn::parse_file(&src);
      let Ok(syntax) = syntax else {
         self.unprocessed_files.push(input_path);
         return;
      };

      for item in syntax.items {
         match item {
            syn::Item::Const(item) => {
               let is_pub = if let Visibility::Public(_) = item.vis {true} else {false};
               self.parse_item(item, is_pub)
            },
            syn::Item::Struct(item) => self.parse_item(item, false),
            syn::Item::Enum(item) => self.parse_item(item, false),
            syn::Item::Type(item) => self.parse_item(item, false),
            syn::Item::Fn(item) => self.parse_item(item, false),
            _ => {}
         }
      }
   }


   ///
   pub fn write_type_defs_header(&mut self) {
      self.type_defs_output.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
      if self.config.can_hc_imports {
         self.type_defs_output.push_str(HOLOCHAIN_CLIENT_IMPORTS);
         self.type_defs_output.push_str(HOD_CORE_TYPES_IMPORTS);
      }
   }


   pub fn write_type_defs_import(&mut self, zome_name: &str) {
      let mut all_types = String::new();
      for (kind, types) in self.converted_items.iter() {
         if *kind == "fn" {
            continue;
         }
         for new_type in types.iter() {
            all_types.push_str(&new_type);
            all_types.push_str(", ");
         }
      }
      if all_types.is_empty() {
         return;
      }
      self.zome_proxy_output.insert_str(
         MAGIC_FIRST_LINE.len() + 1,
         &format!("\nimport {{{}}} from './{}.types';", all_types, zome_name));
   }


   ///
   pub fn write_zome_proxy_header(&mut self, zome_name: &str, default_zome_name: &str) {
      self.zome_proxy_output.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
      if self.config.can_hc_imports {
         self.zome_proxy_output.push_str(HOLOCHAIN_CLIENT_IMPORTS);
         self.zome_proxy_output.push_str(HOD_CORE_TYPES_IMPORTS);
      }

      self.zome_proxy_output.push_str(&format!("
import {{ZomeProxy}} from '@ddd-qc/lit-happ';
import {{{camel_name}FunctionNames}} from './{zome_name}.fn';

/**
 *
 */
export class {pascal_name}Proxy extends ZomeProxy {{
  static readonly DEFAULT_ZOME_NAME = \"{default_name}\"
  static readonly FN_NAMES = {camel_name}FunctionNames
 ", pascal_name = zome_name.to_case(Case::Pascal)
 , zome_name = zome_name

                                               ,default_name = default_zome_name 
                                               ,camel_name = zome_name.to_case(Case::Camel)
      ));
   }



   ///
   pub fn write_zome_fn_names_header(&mut self, zome_name: &str) {
      self.zome_fn_names_output.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
      self.zome_fn_names_output.push_str(&format!("
import {{ZomeName, FunctionName}} from '@holochain/client';


/** Array of all zome function names in \"{zome_name}\" */
export const {zome_name}FunctionNames: FunctionName[] = [
\t\"entry_defs\", \n\t\"get_zome_info\", \n\t\"get_dna_info\","
         , zome_name = zome_name.to_case(Case::Camel)
      ));
   }


      ///
      pub fn write_zome_fn_names_footer(&mut self, zome_name: &str, default_zome_name: &str) {
         self.zome_fn_names_output.push_str(&format!("];


/** Generate tuple array of function names with given zomeName */
export function generate{pascal_name}ZomeFunctionsArray(zomeName: ZomeName): [ZomeName, FunctionName][] {{
   const fns: [ZomeName, FunctionName][] = [];
   for (const fn of {zome_name}FunctionNames) {{
      fns.push([zomeName, fn]);
   }}
   return fns;
}}


/** Tuple array of all zome function names with default zome name \"{default_zome_name}\" */
export const {zome_name}ZomeFunctions: [ZomeName, FunctionName][] = generate{pascal_name}ZomeFunctionsArray(\"{default_zome_name}\");
"
, pascal_name = zome_name.to_case(Case::Pascal)
, zome_name = zome_name.to_case(Case::Camel)
, default_zome_name = default_zome_name
      ));
   }
}