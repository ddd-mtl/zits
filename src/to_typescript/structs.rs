use convert_case::{Case, Casing};
use crate::typescript::convert_type;
use crate::{utils, ParseState, write_comments};
use crate::casing::get_serde_casing;

impl super::ToTypescript for syn::ItemStruct {
    fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool) {
        state.types_file.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        write_comments(&mut state.types_file, &comments, 0);

        //let casing = utils::get_attribute_arg("serde", "renameAll", &self.attrs);
        let casing = get_serde_casing(&self.attrs);

        state.types_file.push_str(&format!(
            "export interface {interface_name}{generics} {{\n",
            interface_name = self.clone().ident.to_string(),
            generics = utils::extract_struct_generics(self.generics.clone())
        ));
        process_fields(self.fields, state, 2, casing);
        state.types_file.push_str("}");

        state.types_file.push('\n');
    }
}

pub fn process_fields(fields: syn::Fields, state: &mut ParseState, indentation_amount: i8, casing: Option<Case>) {
    println!("\n process_fields(): {:?}", casing);
    let space = utils::build_indentation(indentation_amount);
    for field in fields {
        let comments = utils::get_comments(field.attrs);
        write_comments(&mut state.types_file, &comments, 2);
        //let field_name = field.ident.unwrap().to_string();
        let field_name = if let Some(casing) = casing {
            field.ident.unwrap().to_string().to_case(casing)
        } else {
            field.ident.unwrap().to_string()
        };
        let field_type = convert_type(&field.ty);
        state.types_file.push_str(&format!(
            "{space}{field_name}{optional_parameter_token}: {field_type}\n",
            space = space,
            field_name = field_name,
            optional_parameter_token = if field_type.is_optional { "?" } else { "" },
            field_type = field_type.ts_type
        ));
    }
}
