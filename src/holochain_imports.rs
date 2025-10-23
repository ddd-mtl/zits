/// from @Holochain/client 0.20.0-dev.2
pub const HOLOCHAIN_CLIENT_IMPORTS: &str = "
// @ts-ignore
import {
/** api/common.ts */
// @ts-ignore
WebsocketConnectionOptions, WsClientOptions, HolochainError, CloneIdHelper,
/** types.ts */
// @ts-ignore
KitsuneAgent, KitsuneSpace, HoloHashB64, AgentPubKeyB64, DnaHashB64, WasmHashB64, EntryHashB64, ActionHashB64, AnyDhtHashB64, InstalledAppId, Signature, CellId, DnaProperties, RoleName, InstalledCell, Timestamp, HoloHashed, FetchPoolInfo,
/** hdk/action.ts */
// @ts-ignore
SignedAction, SignedActionHashed, RegisterAgentActivity, ActionHashed, ActionType, Action, NewEntryAction, Dna, AgentValidationPkg, InitZomesComplete, CreateLink, DeleteLink, OpenChain, CloseChain, Update, Delete, Create,
/** hdk/capabilities.ts */
// @ts-ignore
CapSecret, CapClaim, GrantedFunctionsType, GrantedFunctions, ZomeCallCapGrant, CapAccess, CapGrant,
///** hdk/countersigning.ts */
//CounterSigningSessionData,
//PreflightRequest,
//CounterSigningSessionTimes,
//ActionBase,
//CounterSigningAgents,
//PreflightBytes,
//Role,
//CountersigningAgentState,
/** hdk/dht-ops.ts */
// @ts-ignore
ChainOpType, DhtOp, WarrantOp, ChainOp, Warrant, ChainIntegrityWarrant, ValidationType, ActionHashAndSig,
/** hdk/entry.ts */
// @ts-ignore
EntryVisibility, AppEntryDef, EntryType, EntryContent, Entry,
/** hdk/record.ts */
// @ts-ignore
Record as HcRecord, RecordEntry as HcRecordEntry,
/** hdk/link.ts */
//AnyLinkableHash,
// @ts-ignore
ZomeIndex, LinkType, LinkTag, RateWeight, RateBucketId, RateUnits, Link,
/** api/admin/types.ts */
// @ts-ignore
// YamlProperties
Zomes, WasmCode, CapGrantInfo, PeerMetaInfo, DisabledAppReason, AppStatus, StemCell, ProvisionedCell, ClonedCell, CellType, CellInfo, AppInfo, MembraneProof, MemproofMap, RoleSettingsMap, RoleSettings, DnaModifiersOpt, DnaModifiers, FunctionName, ZomeName, ZomeDefinition, IntegrityZome, CoordinatorZome, DnaDefinition, ResourceBytes, ResourceMap, CellProvisioningStrategy, CellProvisioning, DnaVersionSpec, DnaVersionFlexible, AppRoleDnaManifest, AppRoleManifest, AppManifest, AppBundle, AppBundleSource, NetworkSeed, AppStatusFilter, AppInterfaceInfo, AgentInfoSigned, ZomeLocation, DnaManifest,
} from '@holochain/client';


/// Simple Hashes
// @ts-ignore
type AgentArray = Uint8Array;
// @ts-ignore
type DnaArray = Uint8Array;
// @ts-ignore
type WasmArray = Uint8Array;
// @ts-ignore
type EntryArray = Uint8Array;
// @ts-ignore
type ActionArray = Uint8Array;
// @ts-ignore
type AnyDhtArray = Uint8Array;
// @ts-ignore
type AnyLinkableArray = Uint8Array;
// @ts-ignore
type ExternalArray = Uint8Array;
";


/// from @Holochain-open-dev/core-types v0.7.0
pub const HOD_CORE_TYPES_IMPORTS: &str = "
// @ts-ignore
import {
/** Common */
// @ts-ignore
DhtOpHashB64, DhtOpHash,
/** DnaFile */
// @ts-ignore
DnaFile, DnaDef,
/** entry-details */
// @ts-ignore
EntryDetails, RecordDetails, Details, DetailsType, EntryDhtStatus,
/** Validation */
// @ts-ignore
ValidationStatus, ValidationReceipt,
} from '@holochain-open-dev/core-types';
";
