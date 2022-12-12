
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
fn create_next_session(input: CreateNextSessionInput) -> ExternResult<(EntryHashB64, u32)> {
   Ok(("", 42))
}