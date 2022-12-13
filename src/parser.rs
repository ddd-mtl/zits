use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::GenConfig;
use crate::to_typescript::ToTypescript;
use crate::utils::has_zits_attribute;


///
pub struct ParseState {
   pub unprocessed_files: Vec<PathBuf>,
   pub type_defs_output: String,
   pub zome_proxy_output: String,
   pub config: GenConfig,
}


impl ParseState {

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
}