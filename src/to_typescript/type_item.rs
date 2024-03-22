use syn::{Attribute, Ident};
use crate::{ParseState};
use crate::typescript::convert_type;
use crate::utils::{get_comments, write_comments};


impl super::ToTypescript for syn::ItemType {

    fn attrs(&self) -> Vec<Attribute> {self.attrs.clone()}
    fn ident(&self) -> Ident {self.ident.clone()}
    fn kind(&self) -> &'static str {"type"}

    fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool, _is_blocking: bool) {
        state.type_defs_output.push_str("\n");
        let name = self.ident.to_string();
        let ty = convert_type(&self.ty, false);
        let comments = get_comments(&self.attrs);
        write_comments(&mut state.type_defs_output, &comments, 0);
        state
            .type_defs_output
            .push_str(format!("export type {name} = {ty}", name = name, ty = ty.ts_type).as_str());

        state.type_defs_output.push_str("\n");
    }
}
