/* This file is generated by zits. Do not edit manually */

/**
 * test/rust.rs
 * Variants should to discriminated unions
 * The last serde/attribute combo matching the tag should be taken
 */
export type Message =
  /** Per Enum case Docs One */
  | {
      last_precedent: "UnitCaseLeft",
    }
  /** Per Enum case Docs Two */
  | {
      last_precedent: "RequestLongTake",
      id: string
      method: string
      params: number
    }
  | {
      last_precedent: "Response",
      id: string
      result: Date
    };

/** The default enum conversion uses external tagging */
export type ExternalMessage =
  /** Per Enum case Docs One */
  | {
      "UnitCaseLeft": {}
    }
  /** Per Enum case Docs Two */
  | {
      "RequestLongTake": {
        id: string
        method: string
        params: number
      }
    }
  | {
      "Response": {
        id: string
        result: Date
      }
    };

/**
 * All Unit Enums go to union of constant strings
 * even if have explicit numeric annotations
 * There is no case renaming on default
 */
export type Animal =
  | "Dog" | "Cat";

export type AnimalTwo =
  | "dog_long_extra" | "cat";

/** Integer enums should follow rust discrimination if literals (doesn't evaluate expression) */
enum Foo {
  Bar = 0,
  Baz = 123,
  Quux = 124,
}

export enum MarkerPieceType {
	Svg = 'Svg',
	EmojiGroup = 'EmojiGroup',
}
export type MarkerPieceVariantSvg = {svg: number}
export type MarkerPieceVariantEmojiGroup = {emojiGroup: EntryHashB64}
export type MarkerPiece = 
 | MarkerPieceVariantSvg | MarkerPieceVariantEmojiGroup;

export enum PlaysetEntryType {
	SvgMarker = 'SvgMarker',
	EmojiGroup = 'EmojiGroup',
	Template = 'Template',
	Space = 'Space',
}
export type PlaysetEntryVariantSvgMarker = {svgMarker: SvgMarker}
export type PlaysetEntryVariantEmojiGroup = {emojiGroup: EmojiGroup}
export type PlaysetEntryVariantTemplate = {template: Template}
export type PlaysetEntryVariantSpace = {space: Space}
export type PlaysetEntry = 
 | PlaysetEntryVariantSvgMarker | PlaysetEntryVariantEmojiGroup | PlaysetEntryVariantTemplate | PlaysetEntryVariantSpace;

export enum MessageType {
	Ping = 'Ping',
	Pong = 'Pong',
	NewHere = 'NewHere',
	DeleteHere = 'DeleteHere',
	UpdateHere = 'UpdateHere',
	NewSession = 'NewSession',
	NewSpace = 'NewSpace',
	NewTemplate = 'NewTemplate',
	NewSvgMarker = 'NewSvgMarker',
	NewEmojiGroup = 'NewEmojiGroup',
}
export type Message = 
 | {type: "Ping", content: AgentPubKeyB64}
 | {type: "Pong", content: AgentPubKeyB64}
 | {type: "NewHere", content: HereOutput}
 | {type: "DeleteHere", content: [EntryHashB64, ActionHashB64]}
 | {type: "UpdateHere", content: [number, ActionHashB64, Here]}
 | {type: "NewSession", content: [EntryHashB64, PlacementSession]}
 | {type: "NewSpace", content: EntryHashB64}
 | {type: "NewTemplate", content: EntryHashB64}
 | {type: "NewSvgMarker", content: EntryHashB64}
 | {type: "NewEmojiGroup", content: EntryHashB64}


export enum FakeMessageType {
	Ping = 'Ping',
	Pong = 'Pong',
	NewHere = 'NewHere',
	DeleteHere = 'DeleteHere',
}
export type FakeMessage = 
 | {bob: "Ping", marley: AgentPubKeyB64}
 | {bob: "Pong", marley: AgentPubKeyB64}
 | {bob: "NewHere", marley: null}
 | {bob: "DeleteHere", marley: [EntryHashB64, ActionHashB64]}


export enum MembraneThresholdType {
	CreateEntryCount = 'CreateEntryCount',
	Vouch = 'Vouch',
	Progenitor = 'Progenitor',
}
export type MembraneThresholdVariantCreateEntryCount = {createEntryCount: CreateEntryCountThreshold}
export type MembraneThresholdVariantVouch = {vouch: VouchThreshold}
export type MembraneThresholdVariantProgenitor = {progenitor: null}
export type MembraneThreshold = 
 | MembraneThresholdVariantCreateEntryCount | MembraneThresholdVariantVouch | MembraneThresholdVariantProgenitor;

export enum DirectMessageProtocolType {
	Failure = 'Failure',
	Success = 'Success',
	Mail = 'Mail',
	Ack = 'Ack',
	Chunk = 'Chunk',
	FileManifest = 'FileManifest',
	RequestChunk = 'RequestChunk',
	RequestManifest = 'RequestManifest',
	UnknownEntry = 'UnknownEntry',
	Ping = 'Ping',
}
export type DirectMessageProtocolVariantFailure = {failure: string}
export type DirectMessageProtocolVariantSuccess = {success: string}
export type DirectMessageProtocolVariantMail = {mail: MailMessage}
export type DirectMessageProtocolVariantAck = {ack: AckMessage}
export type DirectMessageProtocolVariantChunk = {chunk: FileChunk}
export type DirectMessageProtocolVariantFileManifest = {fileManifest: FileManifest}
export type DirectMessageProtocolVariantRequestChunk = {requestChunk: EntryHash}
export type DirectMessageProtocolVariantRequestManifest = {requestManifest: EntryHash}
export type DirectMessageProtocolVariantUnknownEntry = {unknownEntry: null}
export type DirectMessageProtocolVariantPing = {ping: null}
export type DirectMessageProtocol = 
 | DirectMessageProtocolVariantFailure | DirectMessageProtocolVariantSuccess | DirectMessageProtocolVariantMail | DirectMessageProtocolVariantAck | DirectMessageProtocolVariantChunk | DirectMessageProtocolVariantFileManifest | DirectMessageProtocolVariantRequestChunk | DirectMessageProtocolVariantRequestManifest | DirectMessageProtocolVariantUnknownEntry | DirectMessageProtocolVariantPing;
