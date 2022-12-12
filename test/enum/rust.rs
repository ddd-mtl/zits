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
#[ztsync]
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

/// All Unit Enums go to union of constant strings
/// even if have explicit numeric annotations
/// There is no case renaming on default
#[ztsync]
enum Animal {
    Dog,
    Cat,
}
#[ztsync]
#[serde(renameAll = "snake_case")]
enum AnimalTwo {
    DogLongExtra = 2,
    Cat,
}

/// Integer enums should follow rust discrimination if literals (doesn't evaluate expression)
#[derive(Serialize_repr)]
#[ztsync]
enum Foo {
    Bar,       // 0
    Baz = 123, // 123
    Quux,      // 124
}


#[unit_enum(PlaysetTypes)]
enum Foo {
    //Bar,
    Bar(Boo),       // 0
    Baz = 123, // 123
    Quux,      // 124
}


#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MarkerPiece {
    Svg(EntryHashB64),
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
