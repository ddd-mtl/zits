/// test/rust.rs

#[serde]
/// Time in UTC seconds
type UTC = usize;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportPieceInput {
   pub piece_type_name: String, // FIXME use UnitEntryTypes and AppEntryTypeName
   pub piece_entry: Entry,
}


#[serde]
type MyResult = Result<u32, String>;
