// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub composition_fee_event_list: ::prost::alloc::vec::Vec<CompositionFeeEvent>,
    #[prost(message, repeated, tag="2")]
    pub add_liquidity_event_list: ::prost::alloc::vec::Vec<AddLiquidityEvent>,
    #[prost(message, repeated, tag="3")]
    pub remove_liquidity_event_list: ::prost::alloc::vec::Vec<RemoveLiquidityEvent>,
    #[prost(message, repeated, tag="4")]
    pub swap_event_list: ::prost::alloc::vec::Vec<SwapEvent>,
    #[prost(message, repeated, tag="5")]
    pub claim_reward_event_list: ::prost::alloc::vec::Vec<ClaimRewardEvent>,
    #[prost(message, repeated, tag="6")]
    pub fund_reward_event_list: ::prost::alloc::vec::Vec<FundRewardEvent>,
    #[prost(message, repeated, tag="7")]
    pub initialize_reward_event_list: ::prost::alloc::vec::Vec<InitializeRewardEvent>,
    #[prost(message, repeated, tag="8")]
    pub update_reward_duration_event_list: ::prost::alloc::vec::Vec<UpdateRewardDurationEvent>,
    #[prost(message, repeated, tag="9")]
    pub update_reward_funder_event_list: ::prost::alloc::vec::Vec<UpdateRewardFunderEvent>,
    #[prost(message, repeated, tag="10")]
    pub position_close_event_list: ::prost::alloc::vec::Vec<PositionCloseEvent>,
    #[prost(message, repeated, tag="11")]
    pub claim_fee_event_list: ::prost::alloc::vec::Vec<ClaimFeeEvent>,
    #[prost(message, repeated, tag="12")]
    pub lb_pair_create_event_list: ::prost::alloc::vec::Vec<LbPairCreateEvent>,
    #[prost(message, repeated, tag="13")]
    pub position_create_event_list: ::prost::alloc::vec::Vec<PositionCreateEvent>,
    #[prost(message, repeated, tag="14")]
    pub increase_position_length_event_list: ::prost::alloc::vec::Vec<IncreasePositionLengthEvent>,
    #[prost(message, repeated, tag="15")]
    pub decrease_position_length_event_list: ::prost::alloc::vec::Vec<DecreasePositionLengthEvent>,
    #[prost(message, repeated, tag="16")]
    pub fee_parameter_update_event_list: ::prost::alloc::vec::Vec<FeeParameterUpdateEvent>,
    #[prost(message, repeated, tag="17")]
    pub dynamic_fee_parameter_update_event_list: ::prost::alloc::vec::Vec<DynamicFeeParameterUpdateEvent>,
    #[prost(message, repeated, tag="18")]
    pub increase_observation_event_list: ::prost::alloc::vec::Vec<IncreaseObservationEvent>,
    #[prost(message, repeated, tag="19")]
    pub withdraw_ineligible_reward_event_list: ::prost::alloc::vec::Vec<WithdrawIneligibleRewardEvent>,
    #[prost(message, repeated, tag="20")]
    pub update_position_operator_event_list: ::prost::alloc::vec::Vec<UpdatePositionOperatorEvent>,
    #[prost(message, repeated, tag="21")]
    pub update_position_lock_release_point_event_list: ::prost::alloc::vec::Vec<UpdatePositionLockReleasePointEvent>,
    #[prost(message, repeated, tag="22")]
    pub go_to_a_bin_event_list: ::prost::alloc::vec::Vec<GoToABinEvent>,
    #[prost(message, repeated, tag="23")]
    pub initialize_lb_pair_instruction_list: ::prost::alloc::vec::Vec<InitializeLbPairInstruction>,
    #[prost(message, repeated, tag="24")]
    pub initialize_permission_lb_pair_instruction_list: ::prost::alloc::vec::Vec<InitializePermissionLbPairInstruction>,
    #[prost(message, repeated, tag="25")]
    pub initialize_customizable_permissionless_lb_pair_instruction_list: ::prost::alloc::vec::Vec<InitializeCustomizablePermissionlessLbPairInstruction>,
    #[prost(message, repeated, tag="26")]
    pub initialize_bin_array_bitmap_extension_instruction_list: ::prost::alloc::vec::Vec<InitializeBinArrayBitmapExtensionInstruction>,
    #[prost(message, repeated, tag="27")]
    pub initialize_bin_array_instruction_list: ::prost::alloc::vec::Vec<InitializeBinArrayInstruction>,
    #[prost(message, repeated, tag="28")]
    pub add_liquidity_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityInstruction>,
    #[prost(message, repeated, tag="29")]
    pub add_liquidity_by_weight_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityByWeightInstruction>,
    #[prost(message, repeated, tag="30")]
    pub add_liquidity_by_strategy_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityByStrategyInstruction>,
    #[prost(message, repeated, tag="31")]
    pub add_liquidity_by_strategy_one_side_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityByStrategyOneSideInstruction>,
    #[prost(message, repeated, tag="32")]
    pub add_liquidity_one_side_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityOneSideInstruction>,
    #[prost(message, repeated, tag="33")]
    pub remove_liquidity_instruction_list: ::prost::alloc::vec::Vec<RemoveLiquidityInstruction>,
    #[prost(message, repeated, tag="34")]
    pub initialize_position_instruction_list: ::prost::alloc::vec::Vec<InitializePositionInstruction>,
    #[prost(message, repeated, tag="35")]
    pub initialize_position_pda_instruction_list: ::prost::alloc::vec::Vec<InitializePositionPdaInstruction>,
    #[prost(message, repeated, tag="36")]
    pub initialize_position_by_operator_instruction_list: ::prost::alloc::vec::Vec<InitializePositionByOperatorInstruction>,
    #[prost(message, repeated, tag="37")]
    pub update_position_operator_instruction_list: ::prost::alloc::vec::Vec<UpdatePositionOperatorInstruction>,
    #[prost(message, repeated, tag="38")]
    pub swap_instruction_list: ::prost::alloc::vec::Vec<SwapInstruction>,
    #[prost(message, repeated, tag="39")]
    pub swap_exact_out_instruction_list: ::prost::alloc::vec::Vec<SwapExactOutInstruction>,
    #[prost(message, repeated, tag="40")]
    pub swap_with_price_impact_instruction_list: ::prost::alloc::vec::Vec<SwapWithPriceImpactInstruction>,
    #[prost(message, repeated, tag="41")]
    pub withdraw_protocol_fee_instruction_list: ::prost::alloc::vec::Vec<WithdrawProtocolFeeInstruction>,
    #[prost(message, repeated, tag="42")]
    pub initialize_reward_instruction_list: ::prost::alloc::vec::Vec<InitializeRewardInstruction>,
    #[prost(message, repeated, tag="43")]
    pub fund_reward_instruction_list: ::prost::alloc::vec::Vec<FundRewardInstruction>,
    #[prost(message, repeated, tag="44")]
    pub update_reward_funder_instruction_list: ::prost::alloc::vec::Vec<UpdateRewardFunderInstruction>,
    #[prost(message, repeated, tag="45")]
    pub update_reward_duration_instruction_list: ::prost::alloc::vec::Vec<UpdateRewardDurationInstruction>,
    #[prost(message, repeated, tag="46")]
    pub claim_reward_instruction_list: ::prost::alloc::vec::Vec<ClaimRewardInstruction>,
    #[prost(message, repeated, tag="47")]
    pub claim_fee_instruction_list: ::prost::alloc::vec::Vec<ClaimFeeInstruction>,
    #[prost(message, repeated, tag="48")]
    pub close_position_instruction_list: ::prost::alloc::vec::Vec<ClosePositionInstruction>,
    #[prost(message, repeated, tag="49")]
    pub update_base_fee_parameters_instruction_list: ::prost::alloc::vec::Vec<UpdateBaseFeeParametersInstruction>,
    #[prost(message, repeated, tag="50")]
    pub update_dynamic_fee_parameters_instruction_list: ::prost::alloc::vec::Vec<UpdateDynamicFeeParametersInstruction>,
    #[prost(message, repeated, tag="51")]
    pub increase_oracle_length_instruction_list: ::prost::alloc::vec::Vec<IncreaseOracleLengthInstruction>,
    #[prost(message, repeated, tag="52")]
    pub initialize_preset_parameter_instruction_list: ::prost::alloc::vec::Vec<InitializePresetParameterInstruction>,
    #[prost(message, repeated, tag="53")]
    pub close_preset_parameter_instruction_list: ::prost::alloc::vec::Vec<ClosePresetParameterInstruction>,
    #[prost(message, repeated, tag="54")]
    pub close_preset_parameter2_instruction_list: ::prost::alloc::vec::Vec<ClosePresetParameter2Instruction>,
    #[prost(message, repeated, tag="55")]
    pub remove_all_liquidity_instruction_list: ::prost::alloc::vec::Vec<RemoveAllLiquidityInstruction>,
    #[prost(message, repeated, tag="56")]
    pub set_pair_status_instruction_list: ::prost::alloc::vec::Vec<SetPairStatusInstruction>,
    #[prost(message, repeated, tag="57")]
    pub migrate_position_instruction_list: ::prost::alloc::vec::Vec<MigratePositionInstruction>,
    #[prost(message, repeated, tag="58")]
    pub migrate_bin_array_instruction_list: ::prost::alloc::vec::Vec<MigrateBinArrayInstruction>,
    #[prost(message, repeated, tag="59")]
    pub update_fees_and_rewards_instruction_list: ::prost::alloc::vec::Vec<UpdateFeesAndRewardsInstruction>,
    #[prost(message, repeated, tag="60")]
    pub withdraw_ineligible_reward_instruction_list: ::prost::alloc::vec::Vec<WithdrawIneligibleRewardInstruction>,
    #[prost(message, repeated, tag="61")]
    pub set_activation_point_instruction_list: ::prost::alloc::vec::Vec<SetActivationPointInstruction>,
    #[prost(message, repeated, tag="62")]
    pub remove_liquidity_by_range_instruction_list: ::prost::alloc::vec::Vec<RemoveLiquidityByRangeInstruction>,
    #[prost(message, repeated, tag="63")]
    pub add_liquidity_one_side_precise_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityOneSidePreciseInstruction>,
    #[prost(message, repeated, tag="64")]
    pub go_to_a_bin_instruction_list: ::prost::alloc::vec::Vec<GoToAbInInstruction>,
    #[prost(message, repeated, tag="65")]
    pub set_pre_activation_duration_instruction_list: ::prost::alloc::vec::Vec<SetPreActivationDurationInstruction>,
    #[prost(message, repeated, tag="66")]
    pub set_pre_activation_swap_address_instruction_list: ::prost::alloc::vec::Vec<SetPreActivationSwapAddressInstruction>,
    #[prost(message, repeated, tag="67")]
    pub set_pair_status_permissionless_instruction_list: ::prost::alloc::vec::Vec<SetPairStatusPermissionlessInstruction>,
    #[prost(message, repeated, tag="68")]
    pub initialize_token_badge_instruction_list: ::prost::alloc::vec::Vec<InitializeTokenBadgeInstruction>,
    #[prost(message, repeated, tag="69")]
    pub create_claim_protocol_fee_operator_instruction_list: ::prost::alloc::vec::Vec<CreateClaimProtocolFeeOperatorInstruction>,
    #[prost(message, repeated, tag="70")]
    pub close_claim_protocol_fee_operator_instruction_list: ::prost::alloc::vec::Vec<CloseClaimProtocolFeeOperatorInstruction>,
    #[prost(message, repeated, tag="71")]
    pub initialize_preset_parameter2_instruction_list: ::prost::alloc::vec::Vec<InitializePresetParameter2Instruction>,
    #[prost(message, repeated, tag="72")]
    pub initialize_lb_pair2_instruction_list: ::prost::alloc::vec::Vec<InitializeLbPair2Instruction>,
    #[prost(message, repeated, tag="73")]
    pub initialize_customizable_permissionless_lb_pair2_instruction_list: ::prost::alloc::vec::Vec<InitializeCustomizablePermissionlessLbPair2Instruction>,
    #[prost(message, repeated, tag="74")]
    pub claim_fee2_instruction_list: ::prost::alloc::vec::Vec<ClaimFee2Instruction>,
    #[prost(message, repeated, tag="75")]
    pub claim_reward2_instruction_list: ::prost::alloc::vec::Vec<ClaimReward2Instruction>,
    #[prost(message, repeated, tag="76")]
    pub add_liquidity2_instruction_list: ::prost::alloc::vec::Vec<AddLiquidity2Instruction>,
    #[prost(message, repeated, tag="77")]
    pub add_liquidity_by_strategy2_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityByStrategy2Instruction>,
    #[prost(message, repeated, tag="78")]
    pub add_liquidity_one_side_precise2_instruction_list: ::prost::alloc::vec::Vec<AddLiquidityOneSidePrecise2Instruction>,
    #[prost(message, repeated, tag="79")]
    pub remove_liquidity2_instruction_list: ::prost::alloc::vec::Vec<RemoveLiquidity2Instruction>,
    #[prost(message, repeated, tag="80")]
    pub remove_liquidity_by_range2_instruction_list: ::prost::alloc::vec::Vec<RemoveLiquidityByRange2Instruction>,
    #[prost(message, repeated, tag="81")]
    pub swap2_instruction_list: ::prost::alloc::vec::Vec<Swap2Instruction>,
    #[prost(message, repeated, tag="82")]
    pub swap_exact_out2_instruction_list: ::prost::alloc::vec::Vec<SwapExactOut2Instruction>,
    #[prost(message, repeated, tag="83")]
    pub swap_with_price_impact2_instruction_list: ::prost::alloc::vec::Vec<SwapWithPriceImpact2Instruction>,
    #[prost(message, repeated, tag="84")]
    pub close_position2_instruction_list: ::prost::alloc::vec::Vec<ClosePosition2Instruction>,
    #[prost(message, repeated, tag="85")]
    pub update_fees_and_reward2_instruction_list: ::prost::alloc::vec::Vec<UpdateFeesAndReward2Instruction>,
    #[prost(message, repeated, tag="86")]
    pub close_position_if_empty_instruction_list: ::prost::alloc::vec::Vec<ClosePositionIfEmptyInstruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompositionFeeEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub bin_id: i32,
    #[prost(uint64, tag="4")]
    pub token_x_fee_amount: u64,
    #[prost(uint64, tag="5")]
    pub token_y_fee_amount: u64,
    #[prost(uint64, tag="6")]
    pub protocol_token_x_fee_amount: u64,
    #[prost(uint64, tag="7")]
    pub protocol_token_y_fee_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub position: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="5")]
    pub amounts: ::prost::alloc::vec::Vec<u64>,
    #[prost(int32, tag="6")]
    pub active_bin_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub position: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="5")]
    pub amounts: ::prost::alloc::vec::Vec<u64>,
    #[prost(int32, tag="6")]
    pub active_bin_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub from: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub start_bin_id: i32,
    #[prost(int32, tag="5")]
    pub end_bin_id: i32,
    #[prost(uint64, tag="6")]
    pub amount_in: u64,
    #[prost(uint64, tag="7")]
    pub amount_out: u64,
    #[prost(bool, tag="8")]
    pub swap_for_y: bool,
    #[prost(uint64, tag="9")]
    pub fee: u64,
    #[prost(uint64, tag="10")]
    pub protocol_fee: u64,
    #[prost(uint64, tag="11")]
    pub fee_bps: u64,
    #[prost(uint64, tag="12")]
    pub host_fee: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimRewardEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub reward_index: u64,
    #[prost(uint64, tag="6")]
    pub total_reward: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundRewardEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub funder: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub reward_index: u64,
    #[prost(uint64, tag="5")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRewardEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub funder: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub reward_index: u64,
    #[prost(uint64, tag="6")]
    pub reward_duration: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRewardDurationEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub reward_index: u64,
    #[prost(uint64, tag="4")]
    pub old_reward_duration: u64,
    #[prost(uint64, tag="5")]
    pub new_reward_duration: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRewardFunderEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub reward_index: u64,
    #[prost(string, tag="4")]
    pub old_funder: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub new_funder: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionCloseEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimFeeEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub fee_x: u64,
    #[prost(uint64, tag="6")]
    pub fee_y: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbPairCreateEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub bin_step: u64,
    #[prost(string, tag="4")]
    pub token_x: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub token_y: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionCreateEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreasePositionLengthEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub length_to_add: u64,
    #[prost(uint64, tag="6")]
    pub side: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecreasePositionLengthEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub length_to_remove: u64,
    #[prost(uint64, tag="6")]
    pub side: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeParameterUpdateEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub protocol_share: u64,
    #[prost(uint64, tag="4")]
    pub base_factor: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFeeParameterUpdateEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub filter_period: u64,
    #[prost(uint64, tag="4")]
    pub decay_period: u64,
    #[prost(uint64, tag="5")]
    pub reduction_factor: u64,
    #[prost(uint32, tag="6")]
    pub variable_fee_control: u32,
    #[prost(uint32, tag="7")]
    pub max_volatility_accumulator: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseObservationEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub oracle: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub new_observation_length: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawIneligibleRewardEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub reward_mint: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePositionOperatorEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub old_operator: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_operator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePositionLockReleasePointEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub position: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub current_point: u64,
    #[prost(uint64, tag="4")]
    pub new_lock_release_point: u64,
    #[prost(uint64, tag="5")]
    pub old_lock_release_point: u64,
    #[prost(string, tag="6")]
    pub sender: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoToABinEvent {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub lb_pair: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub from_bin_id: i32,
    #[prost(int32, tag="4")]
    pub to_bin_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeLbPairInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub active_id: i32,
    #[prost(uint64, tag="3")]
    pub bin_step: u64,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_token_mint_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_token_mint_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_preset_parameter: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePermissionLbPairInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub ix_data: ::core::option::Option<InitPermissionPairIx>,
    #[prost(string, tag="3")]
    pub acct_base: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_token_mint_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_token_mint_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_badge_x: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_badge_y: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_program_x: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_program_y: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeCustomizablePermissionlessLbPairInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<CustomizableParams>,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_token_mint_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_token_mint_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeBinArrayBitmapExtensionInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_rent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeBinArrayInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub index: i64,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_system_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityParameter>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityByWeightInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityParameterByWeight>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityByStrategyInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityParameterByStrategy>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityByStrategyOneSideInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityParameterByStrategyOneSide>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityOneSideInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityOneSideParameter>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub bin_liquidity_removal: ::prost::alloc::vec::Vec<BinLiquidityReduction>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePositionInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub lower_bin_id: i32,
    #[prost(int32, tag="3")]
    pub width: i32,
    #[prost(string, tag="4")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePositionPdaInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub lower_bin_id: i32,
    #[prost(int32, tag="3")]
    pub width: i32,
    #[prost(string, tag="4")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_base: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePositionByOperatorInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub lower_bin_id: i32,
    #[prost(int32, tag="3")]
    pub width: i32,
    #[prost(string, tag="4")]
    pub fee_owner: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub lock_release_point: u64,
    #[prost(string, tag="6")]
    pub acct_payer: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_base: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_operator_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_owner_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePositionOperatorInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub operator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub min_amount_out: u64,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_token_in: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_out: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_host_fee_in: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactOutInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub max_in_amount: u64,
    #[prost(uint64, tag="3")]
    pub out_amount: u64,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_token_in: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_out: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_host_fee_in: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapWithPriceImpactInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(int32, optional, tag="3")]
    pub active_id: ::core::option::Option<i32>,
    #[prost(uint64, tag="4")]
    pub max_price_impact_bps: u64,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_in: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_out: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_host_fee_in: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawProtocolFeeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_x: u64,
    #[prost(uint64, tag="3")]
    pub amount_y: u64,
    #[prost(message, optional, tag="4")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_receiver_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_receiver_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_claim_fee_operator: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_memo_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeRewardInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(uint64, tag="3")]
    pub reward_duration: u64,
    #[prost(string, tag="4")]
    pub funder: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_reward_vault: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_token_badge: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_rent: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundRewardInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(uint64, tag="3")]
    pub amount: u64,
    #[prost(bool, tag="4")]
    pub carry_forward: bool,
    #[prost(message, optional, tag="5")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="6")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reward_vault: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_funder_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_bin_array: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRewardFunderInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(string, tag="3")]
    pub new_funder: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRewardDurationInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(uint64, tag="3")]
    pub new_duration: u64,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimRewardInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reward_vault: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimFeeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePositionInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBaseFeeParametersInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub fee_parameter: ::core::option::Option<BaseFeeParameter>,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDynamicFeeParametersInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub fee_parameter: ::core::option::Option<DynamicFeeParameter>,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncreaseOracleLengthInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub length_to_add: u64,
    #[prost(string, tag="3")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePresetParameterInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub ix: ::core::option::Option<InitPresetParametersIx>,
    #[prost(string, tag="3")]
    pub acct_preset_parameter: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_rent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePresetParameterInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_preset_parameter: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePresetParameter2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_preset_parameter: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAllLiquidityInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPairStatusInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub status: u64,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigratePositionInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_position_v2: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_position_v1: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_owner: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateBinArrayInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_lb_pair: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeesAndRewardsInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawIneligibleRewardInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(message, optional, tag="3")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_reward_vault: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_funder_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_bin_array: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetActivationPointInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub activation_point: u64,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityByRangeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub from_bin_id: i32,
    #[prost(int32, tag="3")]
    pub to_bin_id: i32,
    #[prost(uint64, tag="4")]
    pub bps_to_remove: u64,
    #[prost(string, tag="5")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityOneSidePreciseInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub parameter: ::core::option::Option<AddLiquiditySingleSidePreciseParameter>,
    #[prost(string, tag="3")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_user_token: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_bin_array_lower: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_bin_array_upper: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoToAbInInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub bin_id: i32,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_from_bin_array: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_to_bin_array: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPreActivationDurationInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub pre_activation_duration: u64,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_creator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPreActivationSwapAddressInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pre_activation_swap_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_creator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPairStatusPermissionlessInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub status: u64,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_creator: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeTokenBadgeInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_token_badge: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClaimProtocolFeeOperatorInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_claim_fee_operator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_operator: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseClaimProtocolFeeOperatorInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_claim_fee_operator: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializePresetParameter2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub ix: ::core::option::Option<InitPresetParameters2Ix>,
    #[prost(string, tag="3")]
    pub acct_preset_parameter: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_admin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_system_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeLbPair2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<InitializeLbPair2Params>,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_token_mint_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_token_mint_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_preset_parameter: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_badge_x: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_badge_y: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_program_x: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_program_y: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeCustomizablePermissionlessLbPair2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<CustomizableParams>,
    #[prost(string, tag="3")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_token_mint_x: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_token_mint_y: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_funder: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_badge_x: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_badge_y: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_program_x: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_program_y: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_system_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimFee2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub min_bin_id: i32,
    #[prost(int32, tag="3")]
    pub max_bin_id: i32,
    #[prost(message, optional, tag="4")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_program_x: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_program_y: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimReward2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub reward_index: u64,
    #[prost(int32, tag="3")]
    pub min_bin_id: i32,
    #[prost(int32, tag="4")]
    pub max_bin_id: i32,
    #[prost(message, optional, tag="5")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="6")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reward_vault: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_reward_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_token_account: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidity2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityParameter>,
    #[prost(message, optional, tag="3")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="4")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityByStrategy2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<LiquidityParameterByStrategy>,
    #[prost(message, optional, tag="3")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="4")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquidityOneSidePrecise2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub liquidity_parameter: ::core::option::Option<AddLiquiditySingleSidePreciseParameter2>,
    #[prost(message, optional, tag="3")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="4")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_token_mint: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_program: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidity2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub bin_liquidity_removal: ::prost::alloc::vec::Vec<BinLiquidityReduction>,
    #[prost(message, optional, tag="3")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="4")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityByRange2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub from_bin_id: i32,
    #[prost(int32, tag="3")]
    pub to_bin_id: i32,
    #[prost(uint64, tag="4")]
    pub bps_to_remove: u64,
    #[prost(message, optional, tag="5")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="6")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_x: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_y: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(uint64, tag="3")]
    pub min_amount_out: u64,
    #[prost(message, optional, tag="4")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_in: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_out: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_host_fee_in: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapExactOut2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub max_in_amount: u64,
    #[prost(uint64, tag="3")]
    pub out_amount: u64,
    #[prost(message, optional, tag="4")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_user_token_in: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_out: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_host_fee_in: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapWithPriceImpact2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount_in: u64,
    #[prost(int32, optional, tag="3")]
    pub active_id: ::core::option::Option<i32>,
    #[prost(uint64, tag="4")]
    pub max_price_impact_bps: u64,
    #[prost(message, optional, tag="5")]
    pub remaining_accounts_info: ::core::option::Option<RemainingAccountsInfo>,
    #[prost(string, tag="6")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acct_bin_array_bitmap_extension: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub acct_reserve_x: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub acct_reserve_y: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub acct_user_token_in: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub acct_user_token_out: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub acct_token_x_mint: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub acct_token_y_mint: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub acct_oracle: ::prost::alloc::string::String,
    #[prost(string, tag="15")]
    pub acct_host_fee_in: ::prost::alloc::string::String,
    #[prost(string, tag="16")]
    pub acct_user: ::prost::alloc::string::String,
    #[prost(string, tag="17")]
    pub acct_token_x_program: ::prost::alloc::string::String,
    #[prost(string, tag="18")]
    pub acct_token_y_program: ::prost::alloc::string::String,
    #[prost(string, tag="19")]
    pub acct_memo_program: ::prost::alloc::string::String,
    #[prost(string, tag="20")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="21")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePosition2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeesAndReward2Instruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub min_bin_id: i32,
    #[prost(int32, tag="3")]
    pub max_bin_id: i32,
    #[prost(string, tag="4")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_lb_pair: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClosePositionIfEmptyInstruction {
    #[prost(string, tag="1")]
    pub trx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub acct_position: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub acct_sender: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub acct_rent_receiver: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub acct_event_authority: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub acct_program: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPresetParameters2Ix {
    #[prost(uint64, tag="1")]
    pub index: u64,
    #[prost(uint64, tag="2")]
    pub bin_step: u64,
    #[prost(uint64, tag="3")]
    pub base_factor: u64,
    #[prost(uint64, tag="4")]
    pub filter_period: u64,
    #[prost(uint64, tag="5")]
    pub decay_period: u64,
    #[prost(uint64, tag="6")]
    pub reduction_factor: u64,
    #[prost(uint32, tag="7")]
    pub variable_fee_control: u32,
    #[prost(uint32, tag="8")]
    pub max_volatility_accumulator: u32,
    #[prost(uint64, tag="9")]
    pub protocol_share: u64,
    #[prost(uint64, tag="10")]
    pub base_fee_power_factor: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPresetParametersIx {
    #[prost(uint64, tag="1")]
    pub bin_step: u64,
    #[prost(uint64, tag="2")]
    pub base_factor: u64,
    #[prost(uint64, tag="3")]
    pub filter_period: u64,
    #[prost(uint64, tag="4")]
    pub decay_period: u64,
    #[prost(uint64, tag="5")]
    pub reduction_factor: u64,
    #[prost(uint32, tag="6")]
    pub variable_fee_control: u32,
    #[prost(uint32, tag="7")]
    pub max_volatility_accumulator: u32,
    #[prost(uint64, tag="8")]
    pub protocol_share: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseFeeParameter {
    #[prost(uint64, tag="1")]
    pub protocol_share: u64,
    #[prost(uint64, tag="2")]
    pub base_factor: u64,
    #[prost(uint64, tag="3")]
    pub base_fee_power_factor: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicFeeParameter {
    #[prost(uint64, tag="1")]
    pub filter_period: u64,
    #[prost(uint64, tag="2")]
    pub decay_period: u64,
    #[prost(uint64, tag="3")]
    pub reduction_factor: u64,
    #[prost(uint32, tag="4")]
    pub variable_fee_control: u32,
    #[prost(uint32, tag="5")]
    pub max_volatility_accumulator: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityParameterByStrategyOneSide {
    #[prost(uint64, tag="1")]
    pub amount: u64,
    #[prost(int32, tag="2")]
    pub active_id: i32,
    #[prost(int32, tag="3")]
    pub max_active_bin_slippage: i32,
    #[prost(message, optional, tag="4")]
    pub strategy_parameters: ::core::option::Option<StrategyParameters>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityParameterByStrategy {
    #[prost(uint64, tag="1")]
    pub amount_x: u64,
    #[prost(uint64, tag="2")]
    pub amount_y: u64,
    #[prost(int32, tag="3")]
    pub active_id: i32,
    #[prost(int32, tag="4")]
    pub max_active_bin_slippage: i32,
    #[prost(message, optional, tag="5")]
    pub strategy_parameters: ::core::option::Option<StrategyParameters>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyParameters {
    #[prost(int32, tag="1")]
    pub min_bin_id: i32,
    #[prost(int32, tag="2")]
    pub max_bin_id: i32,
    #[prost(enumeration="StrategyTypeEnum", tag="3")]
    pub strategy_type: i32,
    #[prost(uint64, repeated, tag="4")]
    pub parameteres: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityOneSideParameter {
    #[prost(uint64, tag="1")]
    pub amount: u64,
    #[prost(int32, tag="2")]
    pub active_id: i32,
    #[prost(int32, tag="3")]
    pub max_active_bin_slippage: i32,
    #[prost(message, repeated, tag="4")]
    pub bin_liquidity_dist: ::prost::alloc::vec::Vec<BinLiquidityDistributionByWeight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinLiquidityDistributionByWeight {
    #[prost(int32, tag="1")]
    pub bin_id: i32,
    #[prost(uint64, tag="2")]
    pub weight: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityParameterByWeight {
    #[prost(uint64, tag="1")]
    pub amount_x: u64,
    #[prost(uint64, tag="2")]
    pub amount_y: u64,
    #[prost(int32, tag="3")]
    pub active_id: i32,
    #[prost(int32, tag="4")]
    pub max_active_bin_slippage: i32,
    #[prost(message, repeated, tag="5")]
    pub bin_liquidity_dist: ::prost::alloc::vec::Vec<BinLiquidityDistributionByWeight>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquiditySingleSidePreciseParameter {
    #[prost(message, repeated, tag="1")]
    pub bins: ::prost::alloc::vec::Vec<CompressedBinDepositAmount>,
    #[prost(uint64, tag="2")]
    pub decompress_multiplier: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompressedBinDepositAmount {
    #[prost(int32, tag="1")]
    pub bin_id: i32,
    #[prost(uint32, tag="2")]
    pub amount: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinLiquidityDistribution {
    #[prost(int32, tag="1")]
    pub bin_id: i32,
    #[prost(uint64, tag="2")]
    pub distribution_x: u64,
    #[prost(uint64, tag="3")]
    pub distribution_y: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityParameter {
    #[prost(uint64, tag="1")]
    pub amount_x: u64,
    #[prost(uint64, tag="2")]
    pub amount_y: u64,
    #[prost(message, repeated, tag="3")]
    pub bin_liquidity_dist: ::prost::alloc::vec::Vec<BinLiquidityDistribution>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizableParams {
    #[prost(int32, tag="1")]
    pub active_id: i32,
    #[prost(uint64, tag="2")]
    pub bin_step: u64,
    #[prost(uint64, tag="3")]
    pub base_factor: u64,
    #[prost(uint64, tag="4")]
    pub activation_type: u64,
    #[prost(bool, tag="5")]
    pub has_alpha_vault: bool,
    #[prost(uint64, optional, tag="6")]
    pub activation_point: ::core::option::Option<u64>,
    #[prost(bool, tag="7")]
    pub creator_pool_on_off_control: bool,
    #[prost(uint64, tag="8")]
    pub base_fee_power_factor: u64,
    #[prost(uint64, repeated, tag="9")]
    pub padding: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPermissionPairIx {
    #[prost(int32, tag="1")]
    pub active_id: i32,
    #[prost(uint64, tag="2")]
    pub bin_step: u64,
    #[prost(uint64, tag="3")]
    pub base_factor: u64,
    #[prost(uint64, tag="4")]
    pub base_fee_power_factor: u64,
    #[prost(uint64, tag="5")]
    pub activation_type: u64,
    #[prost(uint64, tag="6")]
    pub protocol_share: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLiquiditySingleSidePreciseParameter2 {
    #[prost(message, repeated, tag="1")]
    pub bins: ::prost::alloc::vec::Vec<CompressedBinDepositAmount>,
    #[prost(uint64, tag="2")]
    pub decompress_multiplier: u64,
    #[prost(uint64, tag="3")]
    pub max_amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeLbPair2Params {
    #[prost(int32, tag="1")]
    pub active_id: i32,
    #[prost(uint64, repeated, tag="2")]
    pub padding: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinLiquidityReduction {
    #[prost(int32, tag="1")]
    pub bin_id: i32,
    #[prost(uint64, tag="2")]
    pub bps_to_remove: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemainingAccountsSlice {
    #[prost(enumeration="AccountsTypeEnum", tag="1")]
    pub accounts_type: i32,
    #[prost(uint64, tag="2")]
    pub length: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemainingAccountsInfo {
    #[prost(message, repeated, tag="1")]
    pub slices: ::prost::alloc::vec::Vec<RemainingAccountsSlice>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StrategyTypeEnum {
    StrategyTypeSpotOneSide = 0,
    StrategyTypeCurveOneSide = 1,
    StrategyTypeBidAskOneSide = 2,
    StrategyTypeSpotBalanced = 3,
    StrategyTypeCurveBalanced = 4,
    StrategyTypeBidAskBalanced = 5,
    StrategyTypeSpotImBalanced = 6,
    StrategyTypeCurveImBalanced = 7,
    StrategyTypeBidAskImBalanced = 8,
}
impl StrategyTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StrategyTypeEnum::StrategyTypeSpotOneSide => "STRATEGY_TYPE_SPOT_ONE_SIDE",
            StrategyTypeEnum::StrategyTypeCurveOneSide => "STRATEGY_TYPE_CURVE_ONE_SIDE",
            StrategyTypeEnum::StrategyTypeBidAskOneSide => "STRATEGY_TYPE_BID_ASK_ONE_SIDE",
            StrategyTypeEnum::StrategyTypeSpotBalanced => "STRATEGY_TYPE_SPOT_BALANCED",
            StrategyTypeEnum::StrategyTypeCurveBalanced => "STRATEGY_TYPE_CURVE_BALANCED",
            StrategyTypeEnum::StrategyTypeBidAskBalanced => "STRATEGY_TYPE_BID_ASK_BALANCED",
            StrategyTypeEnum::StrategyTypeSpotImBalanced => "STRATEGY_TYPE_SPOT_IM_BALANCED",
            StrategyTypeEnum::StrategyTypeCurveImBalanced => "STRATEGY_TYPE_CURVE_IM_BALANCED",
            StrategyTypeEnum::StrategyTypeBidAskImBalanced => "STRATEGY_TYPE_BID_ASK_IM_BALANCED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STRATEGY_TYPE_SPOT_ONE_SIDE" => Some(Self::StrategyTypeSpotOneSide),
            "STRATEGY_TYPE_CURVE_ONE_SIDE" => Some(Self::StrategyTypeCurveOneSide),
            "STRATEGY_TYPE_BID_ASK_ONE_SIDE" => Some(Self::StrategyTypeBidAskOneSide),
            "STRATEGY_TYPE_SPOT_BALANCED" => Some(Self::StrategyTypeSpotBalanced),
            "STRATEGY_TYPE_CURVE_BALANCED" => Some(Self::StrategyTypeCurveBalanced),
            "STRATEGY_TYPE_BID_ASK_BALANCED" => Some(Self::StrategyTypeBidAskBalanced),
            "STRATEGY_TYPE_SPOT_IM_BALANCED" => Some(Self::StrategyTypeSpotImBalanced),
            "STRATEGY_TYPE_CURVE_IM_BALANCED" => Some(Self::StrategyTypeCurveImBalanced),
            "STRATEGY_TYPE_BID_ASK_IM_BALANCED" => Some(Self::StrategyTypeBidAskImBalanced),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountsTypeEnum {
    AccountsTypeTransferHookX = 0,
    AccountsTypeTransferHookY = 1,
    AccountsTypeTransferHookReward = 2,
}
impl AccountsTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountsTypeEnum::AccountsTypeTransferHookX => "ACCOUNTS_TYPE_TRANSFER_HOOK_X",
            AccountsTypeEnum::AccountsTypeTransferHookY => "ACCOUNTS_TYPE_TRANSFER_HOOK_Y",
            AccountsTypeEnum::AccountsTypeTransferHookReward => "ACCOUNTS_TYPE_TRANSFER_HOOK_REWARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNTS_TYPE_TRANSFER_HOOK_X" => Some(Self::AccountsTypeTransferHookX),
            "ACCOUNTS_TYPE_TRANSFER_HOOK_Y" => Some(Self::AccountsTypeTransferHookY),
            "ACCOUNTS_TYPE_TRANSFER_HOOK_REWARD" => Some(Self::AccountsTypeTransferHookReward),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
