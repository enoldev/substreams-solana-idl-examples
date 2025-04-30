// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub admin_state_initialized_event_list: ::prost::alloc::vec::Vec<AdminStateInitializedEvent>,
    #[prost(message, repeated, tag="2")]
    pub bet_created_event_list: ::prost::alloc::vec::Vec<BetCreatedEvent>,
    #[prost(message, repeated, tag="3")]
    pub bet_initialized_event_list: ::prost::alloc::vec::Vec<BetInitializedEvent>,
    #[prost(message, repeated, tag="4")]
    pub buy_shares_event_list: ::prost::alloc::vec::Vec<BuySharesEvent>,
    #[prost(message, repeated, tag="5")]
    pub sell_shares_event_list: ::prost::alloc::vec::Vec<SellSharesEvent>,
    #[prost(message, repeated, tag="6")]
    pub buy_instruction_list: ::prost::alloc::vec::Vec<BuyInstruction>,
    #[prost(message, repeated, tag="7")]
    pub create_bet_instruction_list: ::prost::alloc::vec::Vec<CreateBetInstruction>,
    #[prost(message, repeated, tag="8")]
    pub get_price_instruction_list: ::prost::alloc::vec::Vec<GetPriceInstruction>,
    #[prost(message, repeated, tag="9")]
    pub init_admin_instruction_list: ::prost::alloc::vec::Vec<InitAdminInstruction>,
    #[prost(message, repeated, tag="10")]
    pub init_bet_instruction_list: ::prost::alloc::vec::Vec<InitBetInstruction>,
    #[prost(message, repeated, tag="11")]
    pub sell_instruction_list: ::prost::alloc::vec::Vec<SellInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminStateInitializedEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub admin: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_initialised: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetCreatedEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(string, tag="3")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bet_prompt: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub expiration_at: i64,
    #[prost(uint64, tag="6")]
    pub initial_liq: u64,
    #[prost(string, tag="7")]
    pub yes_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub yes_uri: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub no_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub no_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetInitializedEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(string, tag="3")]
    pub creator: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub expiration_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuySharesEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(string, tag="3")]
    pub buyer: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub outcome: u64,
    #[prost(uint64, tag="5")]
    pub amount: u64,
    #[prost(uint64, tag="6")]
    pub shares: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellSharesEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(string, tag="3")]
    pub seller: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub outcome: u64,
    #[prost(uint64, tag="5")]
    pub shares: u64,
    #[prost(uint64, tag="6")]
    pub amount_withdraw: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(uint64, tag="3")]
    pub outcome: u64,
    #[prost(uint64, tag="4")]
    pub amt: u64,
    #[prost(string, tag="5")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bet: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_usdc: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_sender_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_recipient_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_mint_yes: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_mint_no: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_destination_yes: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_destination_no: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBetInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(uint64, tag="3")]
    pub initial_liq: u64,
    #[prost(string, tag="4")]
    pub bet_prompt: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub expiration_at: i64,
    #[prost(string, tag="6")]
    pub yes_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub yes_uri: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub no_symbol: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub no_uri: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_bet: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_mint_yes: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_mint_no: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_metadata_yes: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_metadata_no: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPriceInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub side: u64,
    #[prost(string, tag="3")]
    pub acct_bet: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitAdminInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_admin_state: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_signer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitBetInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(string, tag="3")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bet: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_usdc: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_sender_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_recipient_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_mint_yes: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_mint_no: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_destination_yes: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_destination_no: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub bet_id: u64,
    #[prost(uint64, tag="3")]
    pub outcome: u64,
    #[prost(uint64, tag="4")]
    pub shares: u64,
    #[prost(string, tag="5")]
    pub acct_signer: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bet: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_usdc: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_sender_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_recipient_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_mint_yes: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_mint_no: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_destination_yes: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_destination_no: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_program: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
