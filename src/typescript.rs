#[derive(Debug)]
pub struct TsType {
    pub ts_type: String,
    pub is_optional: bool,
    pub is_result: bool,
}

impl From<String> for TsType {
    fn from(ts_type: String) -> TsType {
        TsType {
            ts_type,
            is_optional: false,
            is_result: false,
        }
    }
}

fn convert_generic(gen_ty: &syn::GenericArgument, is_return_type: bool) -> TsType {
    //println!("convert_generic(): {:?}", gen_ty);
    match gen_ty {
        syn::GenericArgument::Type(ty) => convert_type(ty, is_return_type),
        _ => "unknown".to_string().into(),
    }
}

pub fn convert_type(ty: &syn::Type, is_return_type: bool) -> TsType {
    match ty {
        //syn::Type::Paren(p) => "void".to_string().into(),
        syn::Type::Reference(p) => convert_type(&*p.elem, is_return_type),
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
                // TODO: should be imported from holochain/client instead
                "XSalsa20Poly1305EncryptedData" => "unknown".to_string().into(),
                "XSalsa20Poly1305Data" => "Uint8Array".to_string().into(),
                "X25519PubKey" => "Uint8Array".to_string().into(),
                "XSalsa20Poly1305KeyRef" => "unknown".to_string().into(),
                "SerializedBytes" => "Uint8Array".to_string().into(),
                "AppEntryBytes" => "Uint8Array".to_string().into(),
                "AnyLinkableHashB64" => "string".to_string().into(),
                /// HoloHash
                "HoloHash" => "HoloArray".to_string().into(),
                "AgentPubKey" => "AgentArray".to_string().into(),
                "WasmHash" => "WasmArray".to_string().into(),
                "EntryHash" => "EntryArray".to_string().into(),
                "ActionHash" => "ActionArray".to_string().into(),
                "AnyDhtHash" => "AnyDhtArray".to_string().into(),
                "ExternalHash" => "ExternalArray".to_string().into(),
                /// Record
                "Record" => "HcRecord".to_string().into(),
                "RecordEntry" => "HcRecordEntry".to_string().into(),
                /// Date
                "NaiveDateTime" => "Date".to_string().into(),
                // "DateTime" => "Date".to_string().into(),
                "DateTime" => "string".to_string().into(),

                //"()" => "void".to_string().into(),
                "BTreeMap" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!(
                        "Record<string, {}>",
                        match convert_generic(anglebracketed_argument.args.first().unwrap(), is_return_type) {
                            TsType{ is_optional: true, is_result: _ ,ts_type } => format!("{} | undefined", ts_type),
                            TsType{ is_optional: false, is_result: _ , ts_type } => ts_type
                        }
                    ),
                    _ => "unknown".to_string(),
                }.into(),
                "ExternResult" => TsType {
                    is_optional: true,
                    is_result: false,
                    ts_type: match arguments {
                        syn::PathArguments::Parenthesized(parenthesized_argument) => {
                            format!("Promise<{:?}>", parenthesized_argument)
                        }
                        syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                            format!("Promise<{}>", convert_generic(anglebracketed_argument.args.first().unwrap(), is_return_type)
                               .ts_type)
                        }
                        _ => "unknown".to_string(),
                    },
                },
                "Result" => TsType {
                    is_optional: false,
                    is_result: true,
                    ts_type: match arguments {
                        syn::PathArguments::Parenthesized(parenthesized_argument) => {
                            format!("{:?}", parenthesized_argument)
                        }
                        syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                            let args = &anglebracketed_argument.args;
                            //let sec = &args[1];
                            let first_tst = convert_generic(&args[0], is_return_type);
                            let second_tst = convert_generic(&args[1], is_return_type);
                            format!("{} | {}", first_tst.ts_type, second_tst.ts_type)
                        }
                        _ => "unknown".to_string(),
                    },
                },
                "Option" => TsType {
                    is_optional: true,
                    is_result: false,
                    ts_type: match arguments {
                        syn::PathArguments::Parenthesized(parenthesized_argument) => {
                            format!("{:?}", parenthesized_argument)
                        }
                        syn::PathArguments::AngleBracketed(anglebracketed_argument) => {
                            let tst = convert_generic(anglebracketed_argument.args.first().unwrap(), is_return_type);
                            if is_return_type {
                                format!("{} | null", tst.ts_type)
                            } else {
                                tst.ts_type
                            }
                        }
                        _ => "unknown".to_string(),
                    },
                },
                "Vec" => match arguments {
                    syn::PathArguments::Parenthesized(parenthesized_argument) => {
                        format!("{:?}", parenthesized_argument)
                    }
                    syn::PathArguments::AngleBracketed(anglebracketed_argument) => format!(
                        //"Array<{}>",
                        "{}[]",
                        match convert_generic(anglebracketed_argument.args.first().unwrap(), is_return_type) {
                            TsType{ is_optional: true, is_result: _, ts_type } => format!("{} | undefined", ts_type),
                            TsType{ is_optional: false, is_result: _, ts_type } => ts_type
                        }
                    ),
                    _ => "unknown".to_string(),
                }.into(),
                _ => identifier.to_string().into(),
            }
        }
        syn::Type::Tuple(p) => {
            if p.elems.is_empty() {
                return "void".to_string().into();
            }
            let mut str = String::from("[");
            let mut iter = p.elems.iter();
            let first = iter.next().unwrap();
            str.push_str(&convert_type(&first, is_return_type).ts_type);
            for elem in iter {
                str.push_str(", ");
                str.push_str(&convert_type(&elem, is_return_type).ts_type);
            }
            str.push(']');
            return TsType {
                is_optional: false,
                is_result: false,
                ts_type: str,
            };
        },
        _ => "unknown".to_string().into(),
    }
}
