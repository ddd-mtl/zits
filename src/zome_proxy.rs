use std::path::PathBuf;
use convert_case::{Case, Casing};
use crate::holochain_imports::{HOD_CORE_TYPES_IMPORTS, HOLOCHAIN_CLIENT_IMPORTS};
use crate::MAGIC_FIRST_LINE;



pub fn write_type_defs_header(target_str: &mut String){
   target_str.push_str(&format!("{}\n\n", MAGIC_FIRST_LINE));
   target_str.push_str(&format!("import * from '@holochain-open-dev/core-types';
import * from '@holochain/client';
"));
}


///
pub(crate) fn write_zome_proxy_header(target_str: &mut String, types_path: &PathBuf, zome_name: &str) {

   target_str.push_str(&format!("{}\n\n", MAGIC_FIRST_LINE));
   target_str.push_str(HOLOCHAIN_CLIENT_IMPORTS);
   target_str.push_str(HOD_CORE_TYPES_IMPORTS);
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