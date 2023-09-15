use syn::{Attribute, NestedMeta, __private::ToTokens};



///
pub fn write_comments(target_str: &mut String, comments: &[String], indentation_amount: i8) {
    let indentation = build_indentation(indentation_amount);
    match comments.len() {
        0 => (),
        1 => target_str.push_str(&format!("{}/** {} */\n", indentation, &comments[0])),
        _ => {
            target_str.push_str(&format!("{}/**\n", indentation));
            for comment in comments {
                target_str.push_str(&format!("{} * {}\n", indentation, &comment))
            }
            target_str.push_str(&format!("{} */\n", indentation))
        }
    }
}


const ZITS_NEEDLES: &[&str] = &[
    "hdk_entry_helper",
    "hdk_extern",
    "hdk_entry_defs",
    "serde",
];


///
pub fn has_zits_attribute(attributes: &[syn::Attribute], item_name: &str) -> bool {
    /// Skip if item has #[ignore(zits)]
    if let Some(attr) = get_attribute("ignore", attributes) {
        //println!("IGNORE attribute found: {:?}", attr.tokens.to_string());
        if attr.tokens.to_string() == "(zits)" {
            println!("[zits][Info] Ignored fn \"{}()\"", item_name);
            return false;
        }
    }
    /// Check for ZITS needles
    let has_attr = attributes
       .iter()
       .any(|attr| {
        attr.path.segments
            .iter()
            .any(|segment| ZITS_NEEDLES.contains(&segment.ident.to_string().as_str()))
    });
    return has_attr
       || has_attribute_arg("derive", "Serialize", attributes)
}

// // ///
// pub fn has_zits_attribute(attributes: &[syn::Attribute]) -> bool {
//     //println!("has_zits_attribute(): {:?}", attributes);
//     return has_attribute("hdk_entry_helper", attributes)
//        || has_attribute("hdk_extern", attributes)
//        //|| has_attribute("unit_enum", attributes)
//        || has_attribute("hdk_entry_defs", attributes)
//        || has_attribute("serde", attributes)
//        || has_attribute_arg("derive", "Serialize", &attributes)
// }


///
pub fn has_derive_attribute(needle: &str, attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attr| {
        attr.path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == needle)
    })
}


///
pub fn has_attribute(needle: &str, attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attr| {
        attr.path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == needle)
    })
}


/// Get the value matching an attribute and argument combination
pub fn get_attribute_arg(
    needle: &str,
    arg: &str,
    attributes: &[syn::Attribute],
) -> Option<String> {
    if let Some(attr) = get_attribute(needle, attributes) {
        // check if attribute list contains the argument we are interested in
        if let Ok(syn::Meta::List(args)) = attr.parse_meta() {
            // accept the literal following the argument we want
            for subs in args.nested {
                match subs {
                    NestedMeta::Meta(syn::Meta::NameValue(meta)) => {
                        // check if the meta refers to the argument we want
                        if meta
                            .path
                            .get_ident()
                            .filter(|x| &x.to_string() == arg)
                            .is_some()
                        {
                            if let syn::Lit::Str(out) = meta.lit {
                                return Some(out.value());
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    None
}


/// Check has an attribute arg.
pub fn has_attribute_arg(needle: &str, arg: &str, attributes: &[syn::Attribute]) -> bool {
    if let Some(attr) = get_attribute(needle, attributes) {
        // check if attribute list contains the argument we are interested in
        if let Ok(syn::Meta::List(args)) = attr.parse_meta() {
            // accept the literal following the argument we want
            for subs in args.nested {
                match subs {
                    NestedMeta::Meta(meta) => {
                        // check if the meta refers to the argument we want
                        if meta.to_token_stream().to_string() == arg {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
        }
    }
    false
}


/// Get the doc string comments from the syn::attributes
pub fn get_comments(attributes: &[syn::Attribute]) -> Vec<String> {
    let mut comments: Vec<String> = vec![];

    for attribute in attributes.iter() {
        let mut is_doc = false;
        for segment in attribute.path.segments.iter() {
            if segment.ident.to_string() == "doc" {
                is_doc = true;
                break;
            }
        }

        if is_doc {
            for token in attribute.tokens.clone().into_iter() {
                match token {
                    syn::__private::quote::__private::TokenTree::Literal(comment) => {
                        let comment = comment.to_string();
                        let comment = comment[1..comment.len() - 1].trim();
                        comments.push(comment.to_string());
                    }
                    _ => { /* Do nothing */ }
                }
            }
        }
    }

    comments
}

pub fn build_indentation(indentation_amount: i8) -> String {
    let mut indent = "".to_string();
    for _ in 0..indentation_amount {
        indent.push(' ');
    }
    indent
}

pub fn extract_struct_generics(s: syn::Generics) -> String {
    let mut generic_params: Vec<String> = vec![];

    for generic_param in s.params {
        match generic_param {
            syn::GenericParam::Type(ty) => generic_params.push(ty.ident.to_string()),
            _ => {}
        }
    }

    if generic_params.len() == 0 {
        "".to_string()
    } else {
        format!("<{list}>", list = generic_params.join(", "))
    }
}

/// Get the attribute matching needle name.
pub fn get_attribute(needle: &str, attributes: &[syn::Attribute]) -> Option<Attribute> {
    // if multiple attributes pass the conditions
    // we still want to return the last
    for attr in attributes.iter().rev() {
        // check if correct attribute
        if attr
            .path
            .segments
            .iter()
            .any(|segment| segment.ident.to_string() == needle)
        {
            return Some(attr.clone());
        }
    }
    None
}
