
/// from @Holochain/client
pub const HOLOCHAIN_CLIENT_IMPORTS: &str = "
import {
// Types
HoloHash,
AgentPubKey,
DnaHash,
WasmHash,
EntryHash,
ActionHash,
AnyDhtHash,
KitsuneAgent,
KitsuneSpace,
InstalledAppId,
Signature,
CellId,
DnaProperties,
RoleId,
InstalledCell,
Timestamp,
HoloHashed,
// Action
SignedActionHashed,
ActionHashed,
ActionType,
Action,
NewEntryAction,
Dna,
AgentValidationPkg,
InitZomesComplete,
CreateLink,
DeleteLink,
OpenChain,
CloseChain,
Update,
Delete,
Create,
// Capabilities,
CapSecret,
CapClaim,
ZomeCallCapGrant,
CapAccess,
CapGrant,
// CounterSigning,
CounterSigningSessionData,
PreflightRequest,
CounterSigningSessionTimes,
ActionBase,
ActionBase,
CounterSigningAgents,
PreflightBytes,
Role,
CountersigningAgentState,
// DhtOps
DhtOpType,
DhtOp,
getDhtOpType,
getDhtOpAction,
getDhtOpEntry,
getDhtOpSignature,
// Entry
EntryVisibility,
AppEntryType,
EntryType,
EntryContent,
Entry,
// Record
Record,
RecordEntry,
   } from '@holochain/client';
";


/// from @Holochain-open-dev/core-types
pub const HOD_CORE_TYPES_IMPORTS: &str = "
import {
// Common
Dictionary,
HoloHashB64,
EntryHashB64,
ActionHashB64,
DhtOpHashB64,
DnaHashB64,
AgentPubKeyB64,
AnyDhtHashB64,
DhtOpHash,
// DnaFile
DnaFile,
DnaDef,
Zomes,
WasmCode,
// entry-details
EntryDetails,
RecordDetails,
Details,
DetailsType,
EntryDhtStatus,
// Validation
ValidationStatus,
ValidationReceipt,
   } from '@holochain-open-dev/core-types';
";