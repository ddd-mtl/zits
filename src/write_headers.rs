use std::path::PathBuf;
use convert_case::{Case, Casing};
use crate::holochain_imports::{HOD_CORE_TYPES_IMPORTS, HOLOCHAIN_CLIENT_IMPORTS};
use crate::MAGIC_FIRST_LINE;



pub fn write_type_defs_header(target_str: &mut String, can_hc_imports: bool) {
   target_str.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
   if can_hc_imports {
      target_str.push_str(HOLOCHAIN_CLIENT_IMPORTS);
      target_str.push_str(HOD_CORE_TYPES_IMPORTS);
   }
}


///
pub(crate) fn write_zome_proxy_header(
   target_str: &mut String,
   types_path: &PathBuf,
   zome_name: &str,
   can_hc_imports: bool,
) {
   target_str.push_str(&format!("{}\n", MAGIC_FIRST_LINE));
   if can_hc_imports {
      target_str.push_str(HOLOCHAIN_CLIENT_IMPORTS);
      target_str.push_str(HOD_CORE_TYPES_IMPORTS);
   }
   target_str.push_str(&format!("import * from './{}';", types_path.file_name().unwrap().to_str().unwrap()));

   target_str.push_str(&format!("
import {{ZomeProxy}} from '@ddd-qc/lit-happ';

/**
 *
 */
export class {zome_name}Proxy extends ZomeProxy {{
  static readonly DEFAULT_ZOME_NAME = \"z{zome_name}\"
 "
   , zome_name = zome_name.to_case(Case::Pascal)
   ));
}