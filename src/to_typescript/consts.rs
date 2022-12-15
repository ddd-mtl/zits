use syn::__private::ToTokens;
use syn::{Attribute, Ident};

use crate::utils::*;
use crate::ParseState;

impl super::ToTypescript for syn::ItemConst {

    fn attrs(&self) -> Vec<Attribute> {self.attrs.clone()}
    fn ident(&self) -> Ident {self.ident.clone()}
    fn kind(&self) -> &'static str {"const"}

    fn convert_to_ts(self, state: &mut ParseState, debug: bool, uses_typeinterface: bool) {
        // ignore if we aren't in a type interface
        if uses_typeinterface {
            return;
        }

        // this currently only supports literals
        // e.g. const NAME: [type_ignored] = 0
        // e.g. const NAME: [type_ignored] = "some_string"
        // e.g. const NAME: [type_ignored] = serde_json::json!({ "I am valid": "json with no free variables" })
        // however doesn't enforce that the json! macro contains no variables.
        // if your lucky you might have also zitsed them but otherwise you will get a typescript error.

        let name = self.ident.to_string();
        let body = match self.expr.as_ref() {
            syn::Expr::Lit(literal) => {
                // convert it directly to a string to put in TS.
                Some(literal.to_token_stream().to_string())
            }
            syn::Expr::Macro(mcr) => {
                if mcr
                    .mac
                    .path
                    .segments
                    .iter()
                    .any(|x| x.to_token_stream().to_string() == "json")
                {
                    Some(mcr.mac.tokens.to_string())
                } else {
                    None
                }
            }
            _ => None,
        };
        match body {
            Some(body) => {
                state.type_defs_output.push_str("\n");
                let comments = get_comments(&self.attrs);
                write_comments(&mut state.type_defs_output, &comments, 0);
                state
                    .type_defs_output
                    .push_str(&format!("export const {} = {};", name, body));
                state.type_defs_output.push_str("\n");
            }
            _ => {
                if debug {
                    println!(
                        "#[zits][warn] failed for const {}",
                        self.to_token_stream().to_string()
                    );
                }
            }
        }
    }
}
