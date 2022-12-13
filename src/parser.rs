use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use convert_case::{Case, Casing};
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
   pub new_types: Vec<String>,
}


impl ParseState {


   pub fn new(config: GenConfig) -> Self {
      Self {
         config,
         unprocessed_files: Vec::<PathBuf>::new(),
         type_defs_output: String::new(),
         zome_proxy_output: String::new(),
         new_types: Vec::new(),
      }
   }

   ///
   fn parse_item<T: ToTypescript>(&mut self, item: T) {
      let has_zits_attribute = has_zits_attribute(&item.attrs());
      if !has_zits_attribute {
         if self.config.can_debug {
            println!("[zits][debug] Encountered non-zits {} \"{}\"", item.kind(), item.ident().to_string());
         }
         return;
      }
      if self.config.can_debug {
         println!("[zits][debug] Encountered {} \"{}\"", item.kind(), item.ident().to_string());
      }
      self.new_types.push(item.ident().to_string());
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
            syn::Item::Const(item) => self.parse_item(item),
            syn::Item::Struct(item) => self.parse_item(item),
            syn::Item::Enum(item) => self.parse_item(item),
            syn::Item::Type(item) => self.parse_item(item),
            syn::Item::Fn(item) => self.parse_item(item),
            _ => {}
         }
      }
   }

   pub fn write_type_defs_header(&mut self) {
      self.type_defs_output.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
      if self.config.can_hc_imports {
         self.type_defs_output.push_str(HOLOCHAIN_CLIENT_IMPORTS);
         self.type_defs_output.push_str(HOD_CORE_TYPES_IMPORTS);
      }
   }


   pub fn write_type_defs_import(&mut self, types_path: &PathBuf) {
      let mut types = String::new();
      for new_type in self.new_types.iter() {
         types.push_str(&new_type);
         types.push_str(", ");
      }
      self.zome_proxy_output.insert_str(
         MAGIC_FIRST_LINE.len() + 1,
         &format!("\nimport {{{}}} from './{}';", types, types_path.file_name().unwrap().to_str().unwrap()));
   }


   ///
   pub fn write_zome_proxy_header(&mut self, zome_name: &str) {
      self.zome_proxy_output.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
      if self.config.can_hc_imports {
         self.zome_proxy_output.push_str(HOLOCHAIN_CLIENT_IMPORTS);
         self.zome_proxy_output.push_str(HOD_CORE_TYPES_IMPORTS);
      }

      self.zome_proxy_output.push_str(&format!("
import {{ZomeProxy}} from '@ddd-qc/lit-happ';

/**
 *
 */
export class {zome_name}Proxy extends ZomeProxy {{
  static readonly DEFAULT_ZOME_NAME = \"z{zome_name}\"
 "
                                   , zome_name = zome_name.to_case(Case::Pascal)
      ));
   }

}