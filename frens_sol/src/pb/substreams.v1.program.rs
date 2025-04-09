// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub claim_event_event_list: ::prost::alloc::vec::Vec<ClaimEventEvent>,
    #[prost(message, repeated, tag="2")]
    pub complete_event_event_list: ::prost::alloc::vec::Vec<CompleteEventEvent>,
    #[prost(message, repeated, tag="3")]
    pub create_event_event_list: ::prost::alloc::vec::Vec<CreateEventEvent>,
    #[prost(message, repeated, tag="4")]
    pub global_update_event_event_list: ::prost::alloc::vec::Vec<GlobalUpdateEventEvent>,
    #[prost(message, repeated, tag="5")]
    pub trade_event_event_list: ::prost::alloc::vec::Vec<TradeEventEvent>,
    #[prost(message, repeated, tag="6")]
    pub vesting_event_event_list: ::prost::alloc::vec::Vec<VestingEventEvent>,
    #[prost(message, repeated, tag="7")]
    pub withdraw_event_event_list: ::prost::alloc::vec::Vec<WithdrawEventEvent>,
    #[prost(message, repeated, tag="8")]
    pub claim_vested_tokens_instruction_list: ::prost::alloc::vec::Vec<ClaimVestedTokensInstruction>,
    #[prost(message, repeated, tag="9")]
    pub create_bonding_curve_instruction_list: ::prost::alloc::vec::Vec<CreateBondingCurveInstruction>,
    #[prost(message, repeated, tag="10")]
    pub get_claimable_tokens_instruction_list: ::prost::alloc::vec::Vec<GetClaimableTokensInstruction>,
    #[prost(message, repeated, tag="11")]
    pub initialize_instruction_list: ::prost::alloc::vec::Vec<InitializeInstruction>,
    #[prost(message, repeated, tag="12")]
    pub migrate_instruction_list: ::prost::alloc::vec::Vec<MigrateInstruction>,
    #[prost(message, repeated, tag="13")]
    pub set_config_instruction_list: ::prost::alloc::vec::Vec<SetConfigInstruction>,
    #[prost(message, repeated, tag="14")]
    pub swap_instruction_list: ::prost::alloc::vec::Vec<SwapInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalUpdateEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VestingEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawEventEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimVestedTokensInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_global: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_creator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_creator_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_bonding_curve_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBondingCurveInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<CreateBondingCurveParams>,
    #[prost(string, tag="3")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_creator: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_fren_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_bonding_curve_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_bonding_curve_fren_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_global: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_whitelist: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_metadata: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_associated_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_metadata_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClaimableTokensInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_creator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<GlobalSettingsInput>,
    #[prost(string, tag="3")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_global: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_global: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bonding_curve_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bonding_curve_fren_account: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_migration_vault: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_creator: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_amm_config: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_pool_state: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_0_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_1_mint: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_lp_mint: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_creator_token_0: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_creator_token_1: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_creator_lp_token: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_token_0_vault: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_token_1_vault: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_observation_state: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub acct_token_0_program: ::prost::alloc::string::String,
    #[prost(string, tag="22")]
    pub acct_token_1_program: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="24")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<GlobalSettingsInput>,
    #[prost(string, tag="3")]
    pub acct_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_global: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_new_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_new_migration_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<SwapParams>,
    #[prost(string, tag="3")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_global: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_fee_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_fren_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_bonding_curve_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_bonding_curve_fren_account: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_user_fren_account: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_fee_receiver_fren_account: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBondingCurveParams {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(int64, optional, tag="4")]
    pub start_time: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalSettingsInput {
    #[prost(uint64, optional, tag="1")]
    pub initial_virtual_token_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub initial_virtual_sol_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub initial_real_token_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub token_total_supply: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub creator_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="6")]
    pub escrow_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub mint_decimals: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="8")]
    pub migrate_fee_amount: ::core::option::Option<u64>,
    #[prost(int64, optional, tag="9")]
    pub cliff_duration: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="10")]
    pub vesting_duration: ::core::option::Option<i64>,
    #[prost(string, optional, tag="11")]
    pub fee_receiver: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="ProgramStatusEnum", optional, tag="12")]
    pub status: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="13")]
    pub whitelist_enabled: ::core::option::Option<bool>,
    #[prost(string, optional, tag="14")]
    pub raydiyum_config: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapParams {
    #[prost(bool, tag="1")]
    pub base_in: bool,
    #[prost(uint64, tag="2")]
    pub exact_in_amount: u64,
    #[prost(uint64, tag="3")]
    pub min_out_amount: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProgramStatusEnum {
    ProgramStatusRunning = 0,
    ProgramStatusSwapOnly = 1,
    ProgramStatusSwapOnlyNoLaunch = 2,
    ProgramStatusPaused = 3,
}
impl ProgramStatusEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProgramStatusEnum::ProgramStatusRunning => "PROGRAM_STATUS_RUNNING",
            ProgramStatusEnum::ProgramStatusSwapOnly => "PROGRAM_STATUS_SWAP_ONLY",
            ProgramStatusEnum::ProgramStatusSwapOnlyNoLaunch => "PROGRAM_STATUS_SWAP_ONLY_NO_LAUNCH",
            ProgramStatusEnum::ProgramStatusPaused => "PROGRAM_STATUS_PAUSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROGRAM_STATUS_RUNNING" => Some(Self::ProgramStatusRunning),
            "PROGRAM_STATUS_SWAP_ONLY" => Some(Self::ProgramStatusSwapOnly),
            "PROGRAM_STATUS_SWAP_ONLY_NO_LAUNCH" => Some(Self::ProgramStatusSwapOnlyNoLaunch),
            "PROGRAM_STATUS_PAUSED" => Some(Self::ProgramStatusPaused),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
