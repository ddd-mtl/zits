/// test/rust.rs

/// Variants should to discriminated unions
/// The last serde/attribute combo matching the tag should be taken
#[derive(Serialize, Deserialize)]
#[serde(tag = "somethingelse")]
#[serde(renameAll = "kebab-case")]
#[serde(tag = "last_precedent")]
#[unit_enum]
enum Message {
    /// Per Enum case Docs One
    UnitCaseLeft,
    /// Per Enum case Docs Two
    RequestLongTake {
        id: String,
        method: String,
        params: i32,
    },
    Response {
        id: String,
        result: NaiveDateTime,
    },
}

/// The default enum conversion uses external tagging
#[serde]
enum ExternalMessage {
    /// Per Enum case Docs One
    UnitCaseLeft,
    /// Per Enum case Docs Two
    RequestLongTake {
        id: String,
        method: String,
        params: i32,
    },
    Response {
        id: String,
        result: NaiveDateTime,
    },
}

/// All Unit Enums go to union of constant strings (or null types)
/// even if have explicit numeric annotations
/// There is no case renaming on default
#[serde]
enum Animal {
    Dog,
    Cat,
}

#[serde(renameAll = "snake_case")]
enum AnimalTwo {
    DogLongExtra = 2,
    Cat,
}

/// Integer enums should follow rust discrimination if literals (doesn't evaluate expression)
#[derive(Serialize_repr)]
#[serde]
enum Foo {
    Bar,       // 0
    Baz = 123, // 123
    Quux,      // 124
}


#[unit_enum(FooTypes)]
enum Foo {
    //Bar,
    Bar(Boo),       // 0
    Baz = 123, // 123
    Quux,      // 124
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MarkerPiece {
    Svg(u32),
    EmojiGroup(EntryHashB64),
}


#[hdk_entry_defs]
#[unit_enum(PlaysetTypes)]
pub enum PlaysetEntry {
    #[entry_def(required_validations = 2, visibility = "public")]
    SvgMarker(SvgMarker),
    #[entry_def(required_validations = 2, visibility = "public")]
    EmojiGroup(EmojiGroup),
    #[entry_def(required_validations = 2, visibility = "public")]
    Template(Template),
    #[entry_def(required_validations = 2, visibility = "public")]
    Space(Space),
}


#[derive(Serialize, Deserialize, SerializedBytes, Debug)]
#[serde(tag = "type")]
pub enum Message {
    Ping(AgentPubKeyB64),
    Pong(AgentPubKeyB64),
    NewHere(HereOutput),
    DeleteHere((EntryHashB64, ActionHashB64)), /// sessionEh, hereLinkHh
    UpdateHere((u32, ActionHashB64, Here)),    ///[index, newLinkAh, newHereEntry]}
    NewSession((EntryHashB64, PlacementSession)),
    /// - with entry hash of entries
    NewSpace(EntryHashB64),
    NewTemplate(EntryHashB64),
    NewSvgMarker(EntryHashB64),
    NewEmojiGroup(EntryHashB64),
}


#[serde(tag = "bob", content = "marley")]
pub enum FakeMessage {
    Ping(AgentPubKeyB64),
    Pong(AgentPubKeyB64),
    NewHere,
    DeleteHere((EntryHashB64, ActionHashB64)),
}


#[serde(rename_all = "camelCase")]
pub enum MembraneThreshold {
    CreateEntryCount(CreateEntryCountThreshold),
    Vouch(VouchThreshold),
    Progenitor,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, SerializedBytes)]
pub enum DirectMessageProtocol {
    Failure(String),
    Success(String),
    Mail(MailMessage),
    Ack(AckMessage),
    Chunk(FileChunk),
    FileManifest(FileManifest),
    RequestChunk(EntryHash),
    RequestManifest(EntryHash),
    UnknownEntry,
    Ping,
}
