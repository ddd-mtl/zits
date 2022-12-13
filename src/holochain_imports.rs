
/// from @Holochain/client v0.9.3
pub const HOLOCHAIN_CLIENT_IMPORTS: &str = "
import {
/** Types */
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
/** Action */
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
/** Capabilities */
CapSecret,
CapClaim,
ZomeCallCapGrant,
CapAccess,
CapGrant,
/** CounterSigning */
//CounterSigningSessionData,
//PreflightRequest,
//CounterSigningSessionTimes,
//ActionBase,
//CounterSigningAgents,
//PreflightBytes,
//Role,
//CountersigningAgentState,
/** DhtOps */
DhtOpType,
DhtOp,
getDhtOpType,
getDhtOpAction,
getDhtOpEntry,
getDhtOpSignature,
/** Entry */
EntryVisibility,
AppEntryType,
EntryType,
EntryContent,
Entry,
/** Record */
Record,
RecordEntry,
/** admin types */
ZomeName,
MembraneProof,
ZomeDefinition,
IntegrityZome,
CoordinatorZome,
DnaDefinition,
ResourceBytes,
ResourceMap,
CellProvisioning,
HoloHashB64,
DnaVersionSpec,
DnaVersionFlexible,
NetworkSeed,
ZomeLocation,
   } from '@holochain/client';
";


/// from @Holochain-open-dev/core-types v0.6.1
pub const HOD_CORE_TYPES_IMPORTS: &str = "
import {
// Common
Dictionary,
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