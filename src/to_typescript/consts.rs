use syn::__private::ToTokens;

use crate::utils;
use crate::BuildState;

impl super::ToTypescript for syn::ItemConst {
    fn convert_to_ts(self, state: &mut BuildState, debug: bool, uses_typeinterface: bool) {
        // ignore if we aren't in a type interface
        if uses_typeinterface {
            return;
        }

        // this currently only supports literals
        // e.g. const NAME: [type_ignored] = 0
        // e.g. const NAME: [type_ignored] = "some_string"
        // e.g. const NAME: [type_ignored] = serde_json::json!({ "I am valid": "json with no free variables" })
        // however doesn't enforce that the json! macro contains no variables.
        // if your lucky you might have also tsynced them but otherwise you will get a typescript error.

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
                state.types.push_str("\n");
                let comments = utils::get_comments(self.attrs.to_owned());
                state.write_comments(&comments, 0);
                state
                    .types
                    .push_str(&format!("export const {} = {};", name, body));
                state.types.push_str("\n");
            }
            _ => {
                if debug {
                    println!(
                        "#[ztsync] failed for const {}",
                        self.to_token_stream().to_string()
                    );
                }
            }
        }
    }
}
