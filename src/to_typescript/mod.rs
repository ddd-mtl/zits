use syn::{Attribute, Ident};


pub mod consts;
pub mod enums;
pub mod structs;
pub mod type_item;
pub mod fns;


pub trait ToTypescript {
    fn convert_to_ts(self, state: &mut crate::ParseState, debug: bool, uses_typeinterface: bool, is_blocking: bool);

    fn attrs(&self) -> Vec<Attribute>;

    fn ident(&self) -> Ident;

    fn kind(&self) -> &'static str;
}
