use crate::{ParseState, write_comments};

impl super::ToTypescript for syn::ItemType {
    fn convert_to_ts(self, state: &mut ParseState, _debug: bool, _uses_typeinterface: bool) {
        state.types_file.push_str("\n");
        let name = self.ident.to_string();
        let ty = crate::typescript::convert_type(&self.ty);
        let comments = crate::utils::get_comments(self.attrs);
        write_comments(&mut state.types_file, &comments, 0);
        state
            .types_file
            .push_str(format!("export type {name} = {ty}", name = name, ty = ty.ts_type).as_str());

        state.types_file.push_str("\n");
    }
}
