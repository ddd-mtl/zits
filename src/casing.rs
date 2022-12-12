use convert_case::Case;
use crate::utils;


static RENAME_RULES: &[(&str, convert_case::Case)] = &[
   ("lowercase", Case::Lower),
   ("UPPERCASE", Case::Upper),
   ("PascalCase", Case::Pascal),
   ("camelCase", Case::Camel),
   ("snake_case", Case::Snake),
   ("SCREAMING_SNAKE_CASE", Case::ScreamingSnake),
   ("kebab-case", Case::Kebab),
   // ("SCREAMING-KEBAB-CASE", _), // not supported by convert_case
];


pub fn get_serde_casing(attributes: &Vec<syn::Attribute>) -> Option<Case> {
   let mut casing = utils::get_attribute_arg("serde", "renameAll", &attributes);
   if casing.is_none() {
      casing = utils::get_attribute_arg("serde", "rename_all", &attributes);
   }
   return to_case(casing);
}


fn to_case(val: impl Into<Option<String>>) -> Option<Case> {
   val.into().and_then(|x| {
      for (name, rule) in RENAME_RULES {
         if x == *name {
            return Some(*rule);
         }
      }
      None
   })
}
