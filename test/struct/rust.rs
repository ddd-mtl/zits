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
