use convert_case::{Case, Casing};
use syn::{Attribute, Ident};
use crate::typescript::convert_type;
use crate::{utils, ParseState};
use crate::casing::get_serde_casing;
use crate::utils::write_comments;


impl super::ToTypescript for syn::ItemStruct {

    fn attrs(&self) -> Vec<Attribute> {self.attrs.clone()}
    fn ident(&self) -> Ident {self.ident.clone()}
    fn kind(&self) -> &'static str {"struct"}

    fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool) {
        state.type_defs_output.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        write_comments(&mut state.type_defs_output, &comments, 0);

        //let casing = utils::get_attribute_arg("serde", "renameAll", &self.attrs);
        let casing = get_serde_casing(&self.attrs);

        state.type_defs_output.push_str(&format!(
            "export interface {interface_name}{generics} {{\n",
            interface_name = self.clone().ident.to_string(),
            generics = utils::extract_struct_generics(self.generics.clone())
        ));
        process_fields(self.fields, state, 2, casing);
        state.type_defs_output.push_str("}");

        state.type_defs_output.push('\n');
    }
}

pub fn process_fields(fields: syn::Fields, state: &mut ParseState, indentation_amount: i8, casing: Option<Case>) {
    //println!("\n process_fields(): {:?}", casing);
    let space = utils::build_indentation(indentation_amount);
    for field in fields {
        let comments = utils::get_comments(field.attrs);
        write_comments(&mut state.type_defs_output, &comments, 2);
        //let field_name = field.ident.unwrap().to_string();
        let field_name = if let Some(casing) = casing {
            field.ident.unwrap().to_string().to_case(casing)
        } else {
            field.ident.unwrap().to_string()
        };
        let field_type = convert_type(&field.ty);
        state.type_defs_output.push_str(&format!(
            "{space}{field_name}{optional_parameter_token}: {field_type}\n",
            space = space,
            field_name = field_name,
            optional_parameter_token = if field_type.is_optional { "?" } else { "" },
            field_type = field_type.ts_type
        ));
    }
}
