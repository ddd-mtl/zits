# zits

<a href="https://crates.io/crates/zits"><img src="https://img.shields.io/crates/v/zits.svg?style=for-the-badge" height="20" alt="License: MIT OR Apache-2.0" /></a>

**Compatible with:**
 - **HDK v0.1.0-beta-rc.3** & **HDI v0.2.0-beta-rc.3**
 - **@holochain/client v0.11.15**
 - **@holochain-open-dev/core-types v0.6.1**
 - **@ddd-qc/cell-proxy v0.12.0**


A utility to generate Typescript bindings for Zome code in Rust (**Z**ome **I**nto **T**ype**S**cript).


# Install

The CLI can be installed from [crates.io](https://crates.io):

```
cargo install zits
```

Or by building the source-code locally:

```
git clone https://github.com/ddd-mtl/zits.git
cd zits
cargo install --path ./
```


# Usage

**No modification of the zome rust code is required!**

Use the CLI tool on the folders of your zome code:

```sh
zits -i ./zomes/profiles -i ./zomes/profiles_integrity -o ./bindings/profiles.ts
```

Typescript bindings will be generated for all types, structs, enums, and consts marked with holochain or serde specific attributes (`#[hdk_entry_helper]`, `#[hdk_entry_defs]`, etc). The bindings file will be named `*.types.ts`.
Bindings will also be generated for public consts that don't have those attributes.
The serde `rename_all` attribute argument is supported.

All functions marked with holochain attributes `#[hdk_extern]`, except the Holochain callbacks, will be listed in a typescript array to be consumed by Holochain's credential authorizing mechanism. The array will be written in a file named `*.fn.ts`. 

A `ZomeProxy` subclass for [cell-proxy](https://npmjs.org/@ddd-qc/cell-proxy) will be generated in its own file. It will have a method for each function marked with `[hdk_extern]`, excluding the holochain callbacks like `init()` or `validate()`. It will be named after the filename given as output. The file will also have the same name with `*.proxy.ts` as extension.


## Multiple Inputs

You can specify many inputs (directories and/or files) using the `-i` flag multiple times, like so:

```sh
zits -i directory1 -i directory2 -o types.ts
```

## Multiple Outputs

It might help to create multiple typing files for your project. It's easy, just call zits multiple times:

```sh
zits -i src/models -o models.ts
zits -i src/api -o api.ts
```

# Usage as a library

In the case that installing the `zits` CLI isn't an option, you can use it as a library:

1. Add the library to your project:

   ```sh
   cargo add zits@1
   ```

2. Create a new binary in your project which uses the crate (for example, `bin/zits.rs`):
   
   ```rust
   // bin/zits.rs

   use std::path::PathBuf;
   
   pub fn main() {
   let dir = env!("CARGO_MANIFEST_DIR");
   
       let inputs = vec![PathBuf::from_iter([dir, "backend"])];
       let output = PathBuf::from_iter([dir, "frontend/src/types/rust.ts"]);
   
       zits::generate_typescript_defs(inputs, output, false);
   }
   ```

3. Create a `Cargo.toml` binary entry:
   
   ```toml
   [[bin]]
   name = "zits"
   path = "bin/zits.rs"
   ```

4. Execute!

   ```sh
   cargo run --bin zits
   ```

**Protip**: to use `cargo zits`, create an alias in `.cargo/config`:

   ```toml
   [alias]
   zits="run --bin zits"
   ```

# Errors

A list of files which can't be opened or parsed successfully are listed after executing `zits`. For other errors, try using the `--debug` flag to pinpoint issues. Please use the Github issue tracker to report any issues.

# Docs

See `zits --help` for more information on parameters, options & flags.


# Supported Conversions & Examples

`zits` is a fork of [tsync](https://github.com/wulf/tsync). See its documentations for details on how conversions works.

Support added for types defined in `@holochain/client` and `@holochain-open-dev/core-types`.


## Types

Support includes, but is not limited to, the following "base" types:
- `ExternResult<T>` converts to `Promise<T>`
- `BTreeMap<T>` converts to `Record<string, T>`
- `Result<A, B>` converts to `A | B`
- `X25519PubKey` converts to `Uint8Array`

**Holochain's `Record` and `RecordEntry` types are renamed `HcRecord` and `HcRecordEntry` respectively as to not conflict with typescript's native `Record` utility type.**


## Functions
Support has been added for functions, but only the first argument is considered since as this is a limitation of zome functions. 
- `Option<T>` converts to `T | null` when it's a function return type
- A destructured argument will be converted to `input`


## Structs

Added support for non-tuple newtypes.

### Example

Input:
```rust
#[serde]
pub struct GetMailOutput(pub Option<Result<u32, String>>);
```
Output:
```javascript
export type GetMailOutput = number | string | null;
```

## Enums
 Additionnaly support for enums of unnamed variants has been added and converts to a string enum and a ORed type.

### Example

#### unnamed variants
 Input:
```rust
#[hdk_entry_defs]
pub enum PlaysetEntry {
    SvgMarker(SvgMarker),
    EmojiGroup(EmojiGroup),
    Template(Template),
    Space,
}
```

Output:
```javascript
export enum PlaysetEntryType {
	SvgMarker = 'SvgMarker',
	EmojiGroup = 'EmojiGroup',
	Template = 'Template',
	Space = 'Space',
}

export type PlaysetEntryVariantSvgMarker = {svgMarker: SvgMarker}
export type PlaysetEntryVariantEmojiGroup = {emojiGroup: EmojiGroup}
export type PlaysetEntryVariantTemplate = {template: Template}
export type PlaysetEntryVariantSpace = {space: null}
export type PlaysetEntry = 
 | PlaysetEntryVariantSvgMarker | PlaysetEntryVariantEmojiGroup | PlaysetEntryVariantTemplate | PlaysetEntryVariantSpace;

```

#### tagged unnamed variants

 Serde's tag and content attributes are considered for enums. When provided, type declarations for each variant is omitted.

 Input:
```rust
#[serde(tag = "type", content = "content")]
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
```

Output:
```javascript
export enum MessageType {
	Ping = 'Ping',
	Pong = 'Pong',
	NewHere = 'NewHere',
	DeleteHere = 'DeleteHere',
	UpdateHere = 'UpdateHere',
}
export type Message = 
 | {type: "Ping", content: AgentPubKeyB64}
 | {type: "Pong", content: AgentPubKeyB64}
 | {type: "NewHere", content: HereOutput}
 | {type: "DeleteHere", content: [EntryHashB64, ActionHashB64, ]}
 | {type: "UpdateHere", content: [number, ActionHashB64, Here, ]}


```

# Development/Testing

Use `./test/test_all.sh` to run tests.
After running the test, there should be no unexpected changes to files in `./test` (use `git status` and `git diff` to see if there were any changes).

# License

This tool is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See LICENSE-APACHE, LICENSE-MIT, and COPYRIGHT for details.
