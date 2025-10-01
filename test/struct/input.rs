// test/rust.rs

/// Doc comments are preserved too!
#[hdk_entry_helper]
struct Book {
    /// Name of the book.
    name: String,
    /// Chapters of the book.
    chapters: Vec<Chapter>,
    /// Reviews of the book
    /// by users.
    user_reviews: Option<Vec<String>>,
}

/// Multiple line comments
/// are formatted on
/// separate lines
#[hdk_entry_helper]
struct Chapter {
    title: String,
    pages: u32,
}

/// Test duplicate
#[hdk_entry_helper]
struct Chapter {
    title: String,
    pages: u32,
}


#[hdk_entry_helper]
/// Generic struct test
struct PaginationResult<T> {
    items: Vec<T>,
    total_items: number,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportPieceInput {
    pub cell_id: CellId,
    pub piece_eh: EntryHashB64,
    pub piece_type_name: String,
    pub maybe_name: Option<String>,
}


/// NewTypes

#[serde]
pub struct FindManifestOutput(FileManifest);

#[serde]
pub struct FindOutput(pub Option<u32>);

#[serde]
pub struct GetMailOutput(pub Option<Result<u32, String>>);


#[hdk_entry_helper]
pub struct Link {
    pub author: HoloHash<Agent>,
    pub target: HoloHash<AnyLinkable>,
    pub timestamp: Timestamp,
    pub zome_index: ZomeIndex,
    pub link_type: LinkType,
    pub tag: LinkTag,
    pub create_link_hash: HoloHash<Action>,
}

#[hdk_entry_helper]
pub struct LinkTag(pub Vec<u8>);

#[hdk_entry_helper]
pub struct LinkType(pub u8);


#[hdk_entry_helper]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZomeFnInput<T> {
    pub input: T,
    pub local: Option<bool>,
}