use convert_case::{Case, Casing};
use syn::{FnArg, Pat, ReturnType};
use crate::typescript::convert_type;
use crate::{utils, ParseState, write_comments};

///
impl super::ToTypescript for syn::ItemFn {

   fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool) {
      state.fns_file.push('\n');

      let comments = utils::get_comments(self.clone().attrs);
      write_comments(&mut state.fns_file, &comments, 0);

      let fn_name = self.sig.ident.to_string();

      let ReturnType::Type(_arrow, out_type) = self.sig.output else {
         eprintln!("Falied to retrieve function return type");
         return;
      };
      let out_name = convert_type(&out_type).ts_type;

      // Getting first argument
      let first_arg = self.sig.inputs.first().unwrap();
      //println!("first_arg = {:?}", first_arg);
      let FnArg::Typed(patty) = first_arg else {
         eprintln!("Falied to retrieve function first arg type");
         return;
      };
      let Pat::Ident(pat_ident) = *patty.clone().pat else {
         eprintln!("Falied to retrieve function first arg name");
         return;
      };
      let arg_name = pat_ident.ident.to_string();
      //let arg_name = "arg_name";
      let arg_type = convert_type(&patty.ty).ts_type;

      state.fns_file.push_str(&format!(
         "  async {fn_name}{generics}({arg_name}: {arg_type}): {out_name} {{\n"
         , fn_name = fn_name.to_case(Case::Camel)
         , generics = utils::extract_struct_generics(self.sig.generics.clone())
         , arg_name = arg_name.to_case(Case::Camel)
         , arg_type = arg_type
         , out_name = out_name
      ));

      state.fns_file.push_str(&format!(
             "  \treturn this.call('{fn_name}', {arg_name});\n"
             , fn_name = fn_name
             , arg_name = arg_name.to_case(Case::Camel)
      ));

      state.fns_file.push_str("  }\n");
   }
}