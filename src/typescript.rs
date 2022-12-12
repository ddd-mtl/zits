#[derive(Debug)]
pub struct TsType {
    pub ts_type: String,
    pub is_optional: bool,
}

impl From<String> for TsType {
    fn from(ts_type: String) -> TsType {
        TsType {
            ts_type,
            is_optional: false,
        }
    }
}

fn convert_generic(gen_ty: &syn::GenericArgument) -> TsType {
    println!("convert_generic(): {:?}", gen_ty);
    match gen_ty {
        syn::GenericArgument::Type(ty) => convert_type(ty),
        _ => "unknown".to_string().into(),
    }
}

pub fn convert_type(ty: &syn::Type) -> TsType {
    match ty {
        //syn::Type::Paren(p) => "void".to_string().into(),
        syn::Type::Reference(p) => convert_type(&*p.elem),
        syn::Type::Path(p) => {
            let segment = p.path.segments.last().unwrap();
            let ident = &segment.ident;
            let arguments = &segment.arguments;
            let identifier = ident.to_string();
            match identifier.as_str() {
                "i8" => "number".to_string().into(),
                "u8" => "number".to_string().into(),
                "i16" => "number".to_string().into(),
                "u16" => "number".to_string().into(),
                "i32" => "number".to_string().into(),
                "u32" => "number".to_string().into(),
                "i64" => "number".to_string().into(),
                "u64" => "number".to_string().into(),
                "i128" => "number".to_string().into(),
                "u128" => "number".to_string().into(),
                "isize" => "number".to_string().into(),
                "usize" => "number".to_string().into(),
                "f32" => "number".to_string().into(),
                "f64" => "number".to_string().into(),
                "bool" => "boolean".to_string().into(),
                "char" => "string".to_string().into(),
                "str" => "string".to_string().into(),
                "String" => "string".to_string().into(),
                "NaiveDateTime" => "Date".to_string().into(),
                "DateTime" => "Date".to_string().into(),
                //"()" => "void".to_string().into(),
                "BTreeMap" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!(
                        "Dictionary<{}>",
                        match convert_generic(anglebracketed_argument.args.first().unwrap()) {
                            TsType{ is_optional: true, ts_type } => format!("{} | undefined", ts_type),
                            TsType{ is_optional: false, ts_type } => ts_type
                        }
                    ),
                    _ => "unknown".to_string(),
                }.into(),
                "ExternResult" => TsType {
                    is_optional: true,
                    ts_type: match arguments {
                        syn::PathArguments::Parenthesized(parenthesized_argument) => {
                            format!("Promise<{:?}>", parenthesized_argument)
                        }
                        syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                            format!("Promise<{}>", convert_generic(anglebracketed_argument.args.first().unwrap())
                               .ts_type)
                        }
                        _ => "unknown".to_string(),
                    },
                },
                "Option" => TsType {
                    is_optional: true,
                    ts_type: match arguments {
                        syn::PathArguments::Parenthesized(parenthesized_argument) => {
                            format!("{:?}", parenthesized_argument)
                        }
                        syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                            format!("{} | null", convert_generic(anglebracketed_argument.args.first().unwrap())
                               .ts_type)
                        }
                        _ => "unknown".to_string(),
                    },
                },
                "Vec" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!(
                        "Array<{}>",
                        match convert_generic(anglebracketed_argument.args.first().unwrap()) {
                            TsType{ is_optional: true, ts_type } => format!("{} | undefined", ts_type),
                            TsType{ is_optional: false, ts_type } => ts_type
                        }
                    ),
                    _ => "unknown".to_string(),
                }.into(),
                _ => identifier.to_string().into(),
            }
        }
        syn::Type::Tuple(p) => {
            if p.elems.is_empty() {return "void".to_string().into();}
            let mut str = String::from("[");
            for elem in p.elems.iter() {
                str.push_str(&convert_type(&elem).ts_type);
                str.push_str(", ");
            }
            str.push(']');
            return TsType {
                is_optional: false,
                ts_type: str,
            };
        },
        _ => "unknown".to_string().into(),
    }
}
