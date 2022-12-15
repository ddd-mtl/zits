use convert_case::{Case, Casing};
use syn::{Attribute, Ident};
use crate::typescript::convert_type;
use crate::{utils, ParseState};
use crate::casing::get_serde_casing;
use crate::utils::{write_comments};


impl super::ToTypescript for syn::ItemStruct {

    fn attrs(&self) -> Vec<Attribute> {self.attrs.clone()}
    fn ident(&self) -> Ident {self.ident.clone()}
    fn kind(&self) -> &'static str {"struct"}

    fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool) {
        state.type_defs_output.push('\n');
        /// write comments
        let comments = utils::get_comments(&self.attrs);
        write_comments(&mut state.type_defs_output, &comments, 0);
        /// Name
        let casing = get_serde_casing(&self.attrs);
        let /*mut*/ name = self.clone().ident.to_string();
        // Dont rename because other types might refer to it
        // if has_attribute("hdk_entry_helper", &self.attrs) {
        //     name.push_str("Entry");
        // }


        /// Handle new-type
        if let syn::Fields::Unnamed(unnameds) = self.fields {
            if unnameds.unnamed.is_empty() {
                eprintln!("Empty new type. Skipping");
                return;
            }
            if unnameds.unnamed.len() > 1 {
                eprintln!("Tuple new type not handled. Skipping");
                return;
            }
            println!("\n process_new_type(): {:?}", unnameds);
            /// Write new-type
            let field_type = convert_type(&unnameds.unnamed[0].ty, false);
            let mut field_type_str = field_type.ts_type;
            if field_type.is_optional {
                field_type_str += " | null"
            }
            state.type_defs_output.push_str(&format!("export type {} = {};\n", name, field_type_str));
            return;
        }


        /// Write normal struct
        state.type_defs_output.push_str(&format!(
            "export interface {}{} {{\n",
            name,
            utils::extract_struct_generics(self.generics.clone())
        ));
        process_fields(self.fields, state, 2, casing);
        /// Write normal struct end
        state.type_defs_output.push_str("}\n");
    }
}


///
pub fn process_fields(fields: syn::Fields, state: &mut ParseState, indentation_amount: i8, casing: Option<Case>) {
    //println!("\n process_fields(): {:?}", fields);
    let space = utils::build_indentation(indentation_amount);
    for field in fields {
        /// Write comments
        let comments = utils::get_comments(&field.attrs);
        write_comments(&mut state.type_defs_output, &comments, 2);
        /// Get field name
        let mut field_name = field.ident.clone()
           .expect(&format!("Field should have ident: {:?}", field))
           .to_string();
        if let Some(casing) = casing {
            field_name = field_name.to_case(casing)
        };
        /// Convert field type
        let field_type = convert_type(&field.ty, false);
        /// Write field
        state.type_defs_output.push_str(&format!(
            "{space}{field_name}{optional_parameter_token}: {field_type}\n",
            space = space,
            field_name = field_name,
            optional_parameter_token = if field_type.is_optional { "?" } else { "" },
            field_type = field_type.ts_type
        ));
    }
}
