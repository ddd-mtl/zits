use crate::{utils, ParseState};
use convert_case::{Case, Casing};
use syn::__private::ToTokens;
use syn::{Attribute, Fields, Ident};
//use syn::Type::Path;
use crate::casing::get_serde_casing;
use crate::typescript::convert_type;
use crate::utils::write_comments;


/// Conversion of Rust Enum to Typescript using external tagging as per https://serde.rs/enum-representations.html
/// however conversion will adhere to the `serde` `tag` such that enums are internally tagged
/// (while the other forms such as adjacent tagging aren't supported).
/// `renameAll` attributes for the name of the tag will also be adhered to.
impl super::ToTypescript for syn::ItemEnum {

    fn attrs(&self) -> Vec<Attribute> {self.attrs.clone()}
    fn ident(&self) -> Ident {self.ident.clone()}
    fn kind(&self) -> &'static str {"enum"}


    ///
    fn convert_to_ts(self, state: &mut ParseState, debug: bool, uses_typeinterface: bool, _is_blocking: bool) {
        if debug {
            println!("[zits][debug] Converting enum \"{}\" as:", self.ident.to_string());
        }
        /** Tuple structs not allowed as that could mess things up if we do ignore this struct */
        let have_one_unnamed = self.variants.iter().any(|x| {
            if let Fields::Unnamed(_) = x.fields { return true; }
            return false;
        });

        // let have_all_unnamed = !self.variants.iter().any(|x| {
        //     if let Fields::Unnamed(_) = x.fields { return false; }
        //     return true;
        // });
        // if have_one_unnamed && !have_all_unnamed {
        //     println!("[zits][warn] Failed for mixte enum {}", self.ident.to_string());
        //     return;
        // }


        state.type_defs_output.push('\n');

        let comments = utils::get_comments(&self.attrs);
        write_comments(&mut state.type_defs_output, &comments, 0);

        let casing = get_serde_casing(&self.attrs);

        let have_all_single = !self.variants.iter().any(|x| x.fields.len() > 0);

        if have_all_single {
            if utils::has_attribute_arg("derive", "Serialize_repr", &self.attrs) {
                make_numeric_enum(self, state, casing, uses_typeinterface, debug)
            } else {
                make_unit_enum(self.clone(), state, casing, debug);
                make_unnamed_string_enum(self.clone(), state, debug);
            }
            return;
        }


        if /* have_all_unnamed */ have_one_unnamed {
            make_unnamed_string_enum(self.clone(), state/*, casing*/, debug);
            //make_unnamed_enum(self.clone(), state, casing, debug);
            if let Some(tag_name) = utils::get_attribute_arg("serde", "tag", &self.attrs) {
                let content_name = utils::get_attribute_arg("serde", "content", &self.attrs)
                   .unwrap_or("content".to_string());
                make_tagged_unnamed_enum(&tag_name, &content_name, self, state, casing, debug);
            } else {
                make_unnamed_enum(self, state, casing, debug);
            }
            return;
        }

        if let Some(tag_name) = utils::get_attribute_arg("serde", "tag", &self.attrs) {
            make_variant_enum(tag_name, self, state, casing, debug)
        } else {
            make_externally_tagged_variant_enum(self, state, casing, debug)
        }
    }
}



/// This convert an all unit enums to a union of const strings in Typescript.
/// It will ignore any discriminants.  
// fn make_unit_enum(exported_enum: syn::ItemEnum, state: &mut ParseState, casing: Option<Case>) {
//     println!("[zits][debug]  - unit enum");
//
//     state.type_defs_output.push_str(&format!(
//         "export type {interface_name} =\n{space}",
//         interface_name = exported_enum.ident.to_string(),
//         space = utils::build_indentation(1)
//     ));
//
//     for variant in exported_enum.variants {
//         let field_name = if let Some(casing) = casing {
//             variant.ident.to_string().to_case(casing)
//         } else {
//             variant.ident.to_string()
//         };
//         state.type_defs_output.push_str(&format!(" | \"{}\"", field_name));
//     }
//
//     state.type_defs_output.push_str(";\n");
// }

fn make_unit_enum(exported_enum: syn::ItemEnum, state: &mut ParseState, casing: Option<Case>, debug: bool) {
    if debug {
        println!("[zits][debug]  - unit enum");
    }
    state.type_defs_output.push_str(&format!(
        "export type {interface_name} =\n{space}",
        interface_name = exported_enum.ident.to_string(),
        space = utils::build_indentation(1)
    ));

    for variant in exported_enum.variants {
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        state.type_defs_output.push_str(&format!(" | {{{}: null}}", field_name));
    }

    state.type_defs_output.push_str(";\n");

}


/// Numeric enums. These will be converted using enum syntax
/// ```ignore
/// enum Foo {
///     Bar,            // 0
///     Baz = 123,      // 123
///     Quux,           // 124
/// }
/// enum Animal {
///     Dog,
///     Cat,
/// }
/// ``` to the following
/// ```ignore
/// enum Foo {
///    Bar = 0,          
///    Baz = 123,     
///    Quux = 124,           
/// }
/// enum Animal {
///    Dog = 0,
///    Cat = 1,
/// }
/// ```
///
fn make_numeric_enum(
    exported_enum: syn::ItemEnum,
    state: &mut ParseState,
    casing: Option<Case>,
    uses_typeinterface: bool,
    debug: bool,
) {
    if debug {
        println!("[zits][debug]  - numeric enum");
    }
    let declare = if uses_typeinterface { "declare " } else { "" };
    state.type_defs_output.push_str(&format!(
        "{declare}enum {interface_name} {{",
        interface_name = exported_enum.ident.to_string()
    ));

    let mut num = 0;

    for variant in exported_enum.variants {
        state.type_defs_output.push('\n');
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        if let Some((_, disc)) = variant.discriminant {
            if let Ok(new_disc) = disc.to_token_stream().to_string().parse::<i32>() {
                num = new_disc;
            }
        }
        state
            .type_defs_output
            .push_str(&format!("  {} = {},", field_name, num));
        num += 1;
    }

    state.type_defs_output.push_str("\n}\n");
}

/// Conversion of Rust Enum to Typescript using internal tagging as per https://serde.rs/enum-representations.html
/// meaning tuple structs will not be support e.g.
/// ```ignore
/// #[derive(Serialize, Deserialize)]
/// #[serde(tag = "type")]
/// enum Message {
///     Request { id: String, method: String, params: Params },
///     Response { id: String, result: Value },
/// }
/// ``` goes to `type Message = {"type": "REQUEST", "id": "...", "method": "...", "params": {...}} | {"type": "RESPONSE", "id": string, "result": "Value"}`
/// However there is an edge case: purely literal enums. These will be converted using enum syntax
/// ```ignore
/// enum Foo {
///     Bar,            // 0
///     Baz = 123,      // 123
///     Quux,           // 124
/// }
/// enum Animal {
///     Dog,
///     Cat,
/// }
/// ``` to the following
/// ```ignore
/// enum Foo {
///    Bar = 0,          
///    Baz = 123,     
///    Quux = 124,           
/// }
/// enum Animal {
///    Dog = 0,
///    Cat = 1,
/// }
/// ```
fn make_variant_enum(
    tag_name: String,
    exported_enum: syn::ItemEnum,
    state: &mut ParseState,
    casing: Option<Case>,
    debug: bool,
) {
    if debug {
        println!("[zits][debug]  - variant enum");
    }

    ///////
    let enum_name = exported_enum.ident.to_string();
    /// write each enum variant as type
    let mut variant_types = Vec::new();
    for variant in exported_enum.variants.clone() {
        let variant_name = if let Some(case) = casing {
            variant.ident.to_string().to_case(case)
        } else {
            variant.ident.to_string()
        };
        let variant_type_name = format!("{}Variant{}", enum_name, variant.ident.to_string().to_case(Case::Pascal));

        variant_types.push(variant_type_name.clone());

        let comments = utils::get_comments(&variant.attrs);
        write_comments(&mut state.type_defs_output, &comments, 2);
        state.type_defs_output.push_str(&format!("export type {} = {{\n  {}: \"{}\"\n",
                               variant_type_name, tag_name, variant_name));
        super::structs::process_fields(variant.fields, state, 2, casing);
        state.type_defs_output.push_str("}\n");
    }
    ///////

    state.type_defs_output.push_str(&format!(
        "export type {interface_name}{generics} =",
        interface_name = exported_enum.ident.to_string(),
        generics = utils::extract_struct_generics(exported_enum.generics.clone())
    ));

    for variant_type_name in variant_types {
        state.type_defs_output.push('\n');
        state.type_defs_output.push_str(&format!(
            "  | {}",
            variant_type_name
        ));
        // super::structs::process_fields(variant.fields, state, 6, casing);
        // state.type_defs_output.push_str("    }");
    }
    state.type_defs_output.push_str(";\n");
}


/// This follows serde's default approach of external tagging
fn make_externally_tagged_variant_enum(
    exported_enum: syn::ItemEnum,
    state: &mut ParseState,
    casing: Option<Case>,
    debug: bool,
) {
    if debug {
        println!("[zits][debug]  - externally tagged variant enum");
    }
    state.type_defs_output.push_str(&format!(
        "export type {interface_name}{generics} =",
        interface_name = exported_enum.ident.to_string(),
        generics = utils::extract_struct_generics(exported_enum.generics.clone())
    ));

    for variant in exported_enum.variants {
        state.type_defs_output.push('\n');
        let comments = utils::get_comments(&variant.attrs);
        write_comments(&mut state.type_defs_output, &comments, 2);
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        // add discriminant
        state.type_defs_output.push_str(&format!(
            "  | {{\n{}\"{}\": {{",
            utils::build_indentation(6),
            field_name,
        ));
        let prepend;
        if variant.fields.len() == 0 {
            prepend = "".into();
        } else {
            prepend = utils::build_indentation(6);
            state.type_defs_output.push('\n');
            super::structs::process_fields(variant.fields, state, 8, casing);
        }
        state
            .type_defs_output
            .push_str(&format!("{}}}\n{}}}", prepend, utils::build_indentation(4)));
    }
    state.type_defs_output.push_str(";\n");
}


///
fn make_unnamed_string_enum(exported_enum: syn::ItemEnum, state: &mut ParseState, debug: bool) {
    if debug {
        println!("[zits][debug]  - unnamed string enum");
    }
    state.type_defs_output.push_str(&format!("export enum {}Type {{\n", exported_enum.ident.to_string()));

    for variant in exported_enum.variants {
        let field_name = variant.ident.to_string().to_case(Case::Pascal);
        state.type_defs_output.push_str(&format!("\t{field_name} = '{field_name}',\n", field_name = field_name));
        //state.type_defs_output.push_str(&format!("\t{field_name} = {{{field_name}: null}},\n", field_name = field_name));
    }

    state.type_defs_output.push_str("}\n");
}


///
fn make_tagged_unnamed_enum(
    tag_name: &str,
    content_name: &str,
    exported_enum: syn::ItemEnum,
    state: &mut ParseState,
    casing: Option<Case>,
    debug: bool,
) {
    let enum_name = exported_enum.ident.to_string();
    if debug {
        println!("[zits][debug]  - tagged unnamed enum");
    }
    let mut succeeded = true;
    /// write each enum variant as type
    let mut variant_types = Vec::new();
    for variant in exported_enum.variants {
        let variant_name = if let Some(case) = casing {
            variant.ident.to_string().to_case(case)
        } else {
            variant.ident.to_string()
        };
        let variant_type = get_segment_ident(variant.fields, &enum_name);
        if variant_type.is_err() {
            succeeded = false;
            break;
        }
        variant_types.push(format!("{{{tag_name}: {{{variant_name}: null}}, {content_name}: {variant_type}}}\n",
            tag_name = tag_name,
            content_name = content_name,
            variant_name = variant_name.to_case(Case::Pascal),
            variant_type = variant_type.unwrap(),
        ));
    }
    ///
    if !succeeded {
        state.type_defs_output.push_str(&format!("export type {} = unknown\n", enum_name));
        return;
    }
    /// write enum as type
    state.type_defs_output.push_str(&format!("export type {} = \n", enum_name));
    for variant_type in variant_types.into_iter() {
        state.type_defs_output.push_str(&format!(" | {}", variant_type));
    }
    state.type_defs_output.push_str("\n");
}



///
fn make_unnamed_enum(exported_enum: syn::ItemEnum, state: &mut ParseState, casing: Option<Case>, debug: bool) {
    let enum_name = exported_enum.ident.to_string();
    if debug {
        println!("[zits][debug]  - unnamed enum");
    }
    let mut temp = String::new();
    let mut succeeded = true;
    /// write each enum variant as type
    let mut variant_types = Vec::new();
    for variant in exported_enum.variants {
        let variant_name = if let Some(case) = casing {
            variant.ident.to_string().to_case(case)
        } else {
            variant.ident.to_string()
        };
        let variant_type_name = format!("{}Variant{}", enum_name, variant.ident.to_string().to_case(Case::Pascal));
        let maybe_variant_type = get_segment_ident(variant.fields, &enum_name);
        if let Err(e) = maybe_variant_type {
            eprintln!("{}", e);
            succeeded = false;
            break;
        }
        variant_types.push(variant_type_name.clone());
        temp.push_str(&format!("export type {variant_type_name} = {{{variant_name}: {variant_type}}}\n",
                               variant_type_name = variant_type_name,
                               variant_name = variant_name, //.to_case(Case::Camel),
                               variant_type = maybe_variant_type.unwrap(),
        ));
    }
    ///
    if !succeeded {
        state.type_defs_output.push_str(&format!("export type {} = unknown\n", enum_name));
        return;
    }
    state.type_defs_output.push_str(&temp);
    /// write enum as type
    state.type_defs_output.push_str(&format!("export type {} = \n", enum_name));
    for variant_type in variant_types.into_iter() {
        state.type_defs_output.push_str(&format!(" | {}", variant_type));
    }
    state.type_defs_output.push_str(";\n");
}


///
fn get_segment_ident(fields: Fields, enum_name: &str) -> Result<String, String> {

    if let Fields::Unit = fields {
        return Ok("null".to_string());
    }

    let Fields::Unnamed(fields) = fields else {
        return Err(format!("[zits][error] variant is not unnamed in enum \"{}\"", enum_name));
    };
    if fields.unnamed.len() != 1 {
        return Err(format!("[zits][error] unnamed variant does not have one field {:?}", fields.unnamed));
    }

    return Ok(convert_type(&fields.unnamed[0].ty, false).ts_type);
    // let Path(typed) = fields.unnamed[0].clone().ty else {
    //     return Err(format!("[zits][error] unnamed variant is not of type Path {:?}", fields.unnamed));
    // };
    // if typed.path.segments.len() != 1 {
    //     return Err(format!("[zits][error] unnamed variant does not have one segment {:?}", fields.unnamed));
    // }
    // Ok(typed.path.segments[0].ident.to_string())
}
