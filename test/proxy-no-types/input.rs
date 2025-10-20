/// test/rust.rs

#[hdk_extern]
fn get_all_playsets(_: ()) -> ExternResult<Vec<Record>> {
    Ok(vec![])
}

#[derive(Serialize, Deserialize)]
pub enum TaggingLinkTypes {
}

#[derive(Serialize, Deserialize)]
pub enum TaggingEntry {
}
