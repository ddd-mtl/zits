
#[hdk_extern]
pub fn get_agent_profile(input: ZomeFnInput<AgentPubKey>) -> ExternResult<Option<Record>> {
    Ok(vec![])
}

#[hdk_extern]
pub fn get_agents_with_profile(input: ZomeFnInput<()>) -> ExternResult<Vec<Record>> {
    Ok(vec![])
}


/// Incredible function
#[hdk_extern]
pub fn export_piece(input: ExportPieceInput) -> ExternResult<()> {
   Ok(())
}


#[hdk_extern]
pub fn get_session_from_eh(session_eh: EntryHashB64) -> ExternResult<Option<PlacementSession>> {
   Ok(None)
}

#[hdk_extern]
#[feature(zits_blocking)]
fn create_new_session(input: CreateNextSessionInput) -> ExternResult<(EntryHashB64, u32)> {
   Ok(("", 42))
}

#[hdk_extern]
#[feature(zits_blocking = "BlogPost")]
fn create_new_post(input: CreateNextSessionInput) -> ExternResult<(EntryHashB64, u32)> {
   Ok(("", 42))
}

#[hdk_extern]
fn create_next_session(input: CreateNextSessionInput) -> ExternResult<(EntryHashB64, u32)> {
   Ok(("", 42))
}


#[hdk_extern]
fn get_all_playsets(_: ()) -> ExternResult<Vec<Record>> {
   Ok(vec![])
}

#[hdk_extern]
fn no_arg() -> ExternResult<Vec<Record>> {
    Ok(vec![])
}

#[ignore(zits)]
#[hdk_extern]
fn unwanted_function(_: ()) -> ExternResult<Vec<Record>> {
   Ok(vec![])
}

#[ignore = "zits"]
#[hdk_extern]
fn unwanted_function2(_: ()) -> ExternResult<Vec<Record>> {
   Ok(vec![])
}

#[hdk_extern]
fn export_playset(ExportPlaysetInput{playset_eh, destination_cell_id}: ExportPlaysetInput) -> ExternResult<Vec<EntryHashB64>> {
   Ok(vec![])
}

#[hdk_extern]
fn recv_remote_signal(signal: ExternIO) -> ExternResult<()> {
   Ok(())
}


#[hdk_link_types]
#[derive(Serialize, Deserialize)]
pub enum TaggingLinkTypes {
}

#[hdk_entry_types]
#[unit_enum(TaggingEntryTypes)]
pub enum TaggingEntry {
}