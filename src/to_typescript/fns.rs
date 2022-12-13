use convert_case::{Case, Casing};
use syn::{Attribute, FnArg, Ident, Pat, ReturnType};
use crate::typescript::convert_type;
use crate::{utils, ParseState};


const HOLOCHAIN_CALLBACKS: [&str; 10] = [
   "init", "entry_defs", "genesis_self_check",
   "post_commit", "recv_remote_signal",
   "validate",  "validate_create_link", "validate_delete_link",
   "migrate_agent_open", "migrate_agent_close",
];




///
impl super::ToTypescript for syn::ItemFn {

   fn attrs(&self) -> Vec<Attribute> {self.attrs.clone()}
   fn ident(&self) -> Ident {self.sig.ident.clone()}
   fn kind(&self) -> &'static str {"fn"}

   fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool) {
      state.zome_proxy_output.push('\n');

      //let comments = utils::get_comments(self.clone().attrs);
      //write_comments(&mut state.fns_file, &comments, 0);

      let fn_name = self.sig.ident.to_string();


      /// Skip Holochain callbacks
      if HOLOCHAIN_CALLBACKS.contains(&fn_name.as_str()) {
         println!("[zits][info] Skipping callback '{}()'", fn_name);
         return;
      }

      let ReturnType::Type(_arrow, out_type) = self.sig.output else {
         eprintln!("Failed to determine return type for function '{}()'", fn_name);
         return;
      };
      let out_name = convert_type(&out_type).ts_type;

      // Getting first argument
      let first_arg = self.sig.inputs.first().unwrap();
      //println!("first_arg = {:?}", first_arg);
      let FnArg::Typed(patty) = first_arg else {
         eprintln!("Failed to determine first argument type for function '{}()'", fn_name);
         return;
      };
      //println!("\n\npatty.{} = {:?}", fn_name, patty);
      let arg_name = match *patty.clone().pat {
         Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
         Pat::Struct(_) => "input".to_string(),
         _ => "null".to_string()
      };

      //let arg_name = "arg_name";
      let arg_type = convert_type(&patty.ty).ts_type;

      let arg = if let Pat::Wild(_) = *patty.pat {
         "".to_string()
      } else {
         format!("{}: {}", arg_name.to_case(Case::Camel), arg_type)
      };

      state.zome_proxy_output.push_str(&format!(
         "  async {fn_name}{generics}({arg}): {out_name} {{\n"
         , fn_name = fn_name.to_case(Case::Camel)
         , generics = utils::extract_struct_generics(self.sig.generics.clone())
         , arg = arg
         , out_name = out_name
      ));

      state.zome_proxy_output.push_str(&format!(
             "  \treturn this.call('{fn_name}', {arg_name});\n"
             , fn_name = fn_name
             , arg_name = arg_name.to_case(Case::Camel)
      ));

      state.zome_proxy_output.push_str("  }\n");
   }
}