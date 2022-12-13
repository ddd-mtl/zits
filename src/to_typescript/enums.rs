use crate::{utils, ParseState, write_comments};
use convert_case::{Case, Casing};
use syn::__private::ToTokens;
use syn::Fields;
use crate::casing::get_serde_casing;


/// Conversion of Rust Enum to Typescript using external tagging as per https://serde.rs/enum-representations.html
/// however conversion will adhere to the `serde` `tag` such that enums are intenrally tagged
/// (while the other forms such as adjacent tagging aren't supported).
/// `renameAll` attributes for the name of the tag will also be adhered to.
impl super::ToTypescript for syn::ItemEnum {

    ///
    fn convert_to_ts(self, state: &mut ParseState, _debug: bool, uses_typeinterface: bool) {

        // Tuple structs not allowed as that could mess things up if we do ignore this struct
        let have_one_unnamed = self.variants.iter().any(|x| {
            if let Fields::Unnamed(_) = x.fields { return true; }
            return false;
        });
        let have_all_unnamed = !self.variants.iter().any(|x| {
            if let Fields::Unnamed(_) = x.fields { return false; }
            return true;
        });

        if have_one_unnamed && !have_all_unnamed {
            println!("[zits][warn] Failed for mixte enum {}", self.ident.to_string());
            return;
        }


        state.types_file.push('\n');

        let comments = utils::get_comments(self.clone().attrs);
        let casing = get_serde_casing(&self.attrs);


        write_comments(&mut state.types_file, &comments, 0);

        let have_all_single = !self.variants.iter().any(|x| x.fields.len() > 0);

        if have_all_single {
            if utils::has_attribute_arg("derive", "Serialize_repr", &self.attrs) {
                make_numeric_enum(self, state, casing, uses_typeinterface)
            } else {
                make_unit_enum(self, state, casing)
            }
            return;
        }


        if have_all_unnamed {
            make_unnamed_enum(self, state/*, casing*/);
            return;
        }

        if let Some(tag_name) = utils::get_attribute_arg("serde", "tag", &self.attrs) {
            make_variant_enum(tag_name, self, state, casing)
        } else {
            make_externally_tagged_variant_enum(self, state, casing)
        }
    }
}


/// This convert an all unit enums to a union of const strings in Typescript.
/// It will ignore any discriminants.  
fn make_unit_enum(exported_enum: syn::ItemEnum, state: &mut ParseState, casing: Option<Case>) {
    state.types_file.push_str(&format!(
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
        state.types_file.push_str(&format!(" | \"{}\"", field_name));
    }

    state.types_file.push_str(";\n");
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
) {
    let declare = if uses_typeinterface { "declare " } else { "" };
    state.types_file.push_str(&format!(
        "{declare}enum {interface_name} {{",
        interface_name = exported_enum.ident.to_string()
    ));

    let mut num = 0;

    for variant in exported_enum.variants {
        state.types_file.push('\n');
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
            .types_file
            .push_str(&format!("  {} = {},", field_name, num));
        num += 1;
    }

    state.types_file.push_str("\n}\n");
}

/// Conversion of Rust Enum to Typescript using internal tagging as per https://serde.rs/enum-representations.html
/// meaning tuple structs will not be support e.g.
/// ```ignore
/// #[derive(Serialize, Deserialize)]
/// #[serde(tag = "type")]
/// #[tsync]
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
) {
    state.types_file.push_str(&format!(
        "export type {interface_name}{generics} =",
        interface_name = exported_enum.ident.to_string(),
        generics = utils::extract_struct_generics(exported_enum.generics.clone())
    ));

    for variant in exported_enum.variants {
        state.types_file.push('\n');
        let comments = utils::get_comments(variant.attrs);
        write_comments(&mut state.types_file, &comments, 2);
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        // add discriminant
        state.types_file.push_str(&format!(
            "  | {{\n{}{}: \"{}\",\n",
            utils::build_indentation(6),
            tag_name,
            field_name,
        ));
        super::structs::process_fields(variant.fields, state, 6, casing);
        state.types_file.push_str("    }");
    }
    state.types_file.push_str(";\n");
}


/// This follows serde's default approach of external tagging
fn make_externally_tagged_variant_enum(
    exported_enum: syn::ItemEnum,
    state: &mut ParseState,
    casing: Option<Case>,
) {
    state.types_file.push_str(&format!(
        "export type {interface_name}{generics} =",
        interface_name = exported_enum.ident.to_string(),
        generics = utils::extract_struct_generics(exported_enum.generics.clone())
    ));

    for variant in exported_enum.variants {
        state.types_file.push('\n');
        let comments = utils::get_comments(variant.attrs);
        write_comments(&mut state.types_file, &comments, 2);
        let field_name = if let Some(casing) = casing {
            variant.ident.to_string().to_case(casing)
        } else {
            variant.ident.to_string()
        };
        // add discriminant
        state.types_file.push_str(&format!(
            "  | {{\n{}\"{}\": {{",
            utils::build_indentation(6),
            field_name,
        ));
        let prepend;
        if variant.fields.len() == 0 {
            prepend = "".into();
        } else {
            prepend = utils::build_indentation(6);
            state.types_file.push('\n');
            super::structs::process_fields(variant.fields, state, 8, casing);
        }
        state
            .types_file
            .push_str(&format!("{}}}\n{}}}", prepend, utils::build_indentation(4)));
    }
    state.types_file.push_str(";\n");
}


fn make_unnamed_enum(
    exported_enum: syn::ItemEnum,
    state: &mut ParseState,
    //casing: Option<Case>,
) {
    println!("[zits][debug] Making unnamed enum {}", exported_enum.ident.to_string());

    state.types_file.push_str(&format!(
        "export enum {interface_name} {{\n",
        interface_name = exported_enum.ident.to_string(),
    ));

    for variant in exported_enum.variants {
        let field_name = variant.ident.to_string().to_case(Case::Pascal);
        state.types_file.push_str(&format!("\t{field_name} = '{field_name}',\n", field_name = field_name));
    }

    state.types_file.push_str("}\n");
}