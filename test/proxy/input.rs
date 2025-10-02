/// test/rust.rs

#[hdk_extern]
fn get_all_playsets(_: ()) -> ExternResult<Vec<Record>> {
    Ok(vec![])
}

#[hdk_link_types]
#[derive(Serialize, Deserialize)]
pub enum TaggingLinkTypes {
}

#[hdk_entry_types]
#[unit_enum(TaggingEntryTypes)]
pub enum TaggingEntry {
}
