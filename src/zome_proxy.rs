use std::path::PathBuf;
use convert_case::{Case, Casing};
use crate::MAGIC_FIRST_LINE;


///
pub(crate) fn write_zome_proxy_header(target_str: &mut String, types_path: &PathBuf, zome_name: &str) {

   let pascal_name = zome_name.to_case(Case::Pascal);

   target_str.push_str(&format!("{}\n\n", MAGIC_FIRST_LINE));
   target_str.push_str(&format!("import {{ZomeProxy}} from '@ddd-qc/lit-happ';
import * from '@holochain-open-dev/core-types';
import * from '@holochain/client';
import * from './{types_path}';

/**
 *
 */
export class {zome_name}Proxy extends ZomeProxy {{
  static readonly DEFAULT_ZOME_NAME = \"z{zome_name}\"
 "
                                , zome_name = pascal_name
   , types_path = types_path.file_name().unwrap().to_str().unwrap()
   ));
}