mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::CompositionFeeEvent;
use pb::substreams::v1::program::AddLiquidityEvent;
use pb::substreams::v1::program::RemoveLiquidityEvent;
use pb::substreams::v1::program::SwapEvent;
use pb::substreams::v1::program::ClaimRewardEvent;
use pb::substreams::v1::program::FundRewardEvent;
use pb::substreams::v1::program::InitializeRewardEvent;
use pb::substreams::v1::program::UpdateRewardDurationEvent;
use pb::substreams::v1::program::UpdateRewardFunderEvent;
use pb::substreams::v1::program::PositionCloseEvent;
use pb::substreams::v1::program::ClaimFeeEvent;
use pb::substreams::v1::program::LbPairCreateEvent;
use pb::substreams::v1::program::PositionCreateEvent;
use pb::substreams::v1::program::IncreasePositionLengthEvent;
use pb::substreams::v1::program::DecreasePositionLengthEvent;
use pb::substreams::v1::program::FeeParameterUpdateEvent;
use pb::substreams::v1::program::DynamicFeeParameterUpdateEvent;
use pb::substreams::v1::program::IncreaseObservationEvent;
use pb::substreams::v1::program::WithdrawIneligibleRewardEvent;
use pb::substreams::v1::program::UpdatePositionOperatorEvent;
use pb::substreams::v1::program::UpdatePositionLockReleasePointEvent;
use pb::substreams::v1::program::GoToABinEvent;
use pb::substreams::v1::program::InitializeLbPairInstruction;
use pb::substreams::v1::program::InitializePermissionLbPairInstruction;
use pb::substreams::v1::program::InitializeCustomizablePermissionlessLbPairInstruction;
use pb::substreams::v1::program::InitializeBinArrayBitmapExtensionInstruction;
use pb::substreams::v1::program::InitializeBinArrayInstruction;
use pb::substreams::v1::program::AddLiquidityInstruction;
use pb::substreams::v1::program::AddLiquidityByWeightInstruction;
use pb::substreams::v1::program::AddLiquidityByStrategyInstruction;
use pb::substreams::v1::program::AddLiquidityByStrategyOneSideInstruction;
use pb::substreams::v1::program::AddLiquidityOneSideInstruction;
use pb::substreams::v1::program::RemoveLiquidityInstruction;
use pb::substreams::v1::program::InitializePositionInstruction;
use pb::substreams::v1::program::InitializePositionPdaInstruction;
use pb::substreams::v1::program::InitializePositionByOperatorInstruction;
use pb::substreams::v1::program::UpdatePositionOperatorInstruction;
use pb::substreams::v1::program::SwapInstruction;
use pb::substreams::v1::program::SwapExactOutInstruction;
use pb::substreams::v1::program::SwapWithPriceImpactInstruction;
use pb::substreams::v1::program::WithdrawProtocolFeeInstruction;
use pb::substreams::v1::program::InitializeRewardInstruction;
use pb::substreams::v1::program::FundRewardInstruction;
use pb::substreams::v1::program::UpdateRewardFunderInstruction;
use pb::substreams::v1::program::UpdateRewardDurationInstruction;
use pb::substreams::v1::program::ClaimRewardInstruction;
use pb::substreams::v1::program::ClaimFeeInstruction;
use pb::substreams::v1::program::ClosePositionInstruction;
use pb::substreams::v1::program::UpdateBaseFeeParametersInstruction;
use pb::substreams::v1::program::UpdateDynamicFeeParametersInstruction;
use pb::substreams::v1::program::IncreaseOracleLengthInstruction;
use pb::substreams::v1::program::InitializePresetParameterInstruction;
use pb::substreams::v1::program::ClosePresetParameterInstruction;
use pb::substreams::v1::program::ClosePresetParameter2Instruction;
use pb::substreams::v1::program::RemoveAllLiquidityInstruction;
use pb::substreams::v1::program::SetPairStatusInstruction;
use pb::substreams::v1::program::MigratePositionInstruction;
use pb::substreams::v1::program::MigrateBinArrayInstruction;
use pb::substreams::v1::program::UpdateFeesAndRewardsInstruction;
use pb::substreams::v1::program::WithdrawIneligibleRewardInstruction;
use pb::substreams::v1::program::SetActivationPointInstruction;
use pb::substreams::v1::program::RemoveLiquidityByRangeInstruction;
use pb::substreams::v1::program::AddLiquidityOneSidePreciseInstruction;
use pb::substreams::v1::program::GoToAbInInstruction;
use pb::substreams::v1::program::SetPreActivationDurationInstruction;
use pb::substreams::v1::program::SetPreActivationSwapAddressInstruction;
use pb::substreams::v1::program::SetPairStatusPermissionlessInstruction;
use pb::substreams::v1::program::InitializeTokenBadgeInstruction;
use pb::substreams::v1::program::CreateClaimProtocolFeeOperatorInstruction;
use pb::substreams::v1::program::CloseClaimProtocolFeeOperatorInstruction;
use pb::substreams::v1::program::InitializePresetParameter2Instruction;
use pb::substreams::v1::program::InitializeLbPair2Instruction;
use pb::substreams::v1::program::InitializeCustomizablePermissionlessLbPair2Instruction;
use pb::substreams::v1::program::ClaimFee2Instruction;
use pb::substreams::v1::program::ClaimReward2Instruction;
use pb::substreams::v1::program::AddLiquidity2Instruction;
use pb::substreams::v1::program::AddLiquidityByStrategy2Instruction;
use pb::substreams::v1::program::AddLiquidityOneSidePrecise2Instruction;
use pb::substreams::v1::program::RemoveLiquidity2Instruction;
use pb::substreams::v1::program::RemoveLiquidityByRange2Instruction;
use pb::substreams::v1::program::Swap2Instruction;
use pb::substreams::v1::program::SwapExactOut2Instruction;
use pb::substreams::v1::program::SwapWithPriceImpact2Instruction;
use pb::substreams::v1::program::ClosePosition2Instruction;
use pb::substreams::v1::program::UpdateFeesAndReward2Instruction;
use pb::substreams::v1::program::ClosePositionIfEmptyInstruction;


use pb::substreams::v1::program::InitPresetParameters2Ix;



use pb::substreams::v1::program::InitPresetParametersIx;



use pb::substreams::v1::program::BaseFeeParameter;



use pb::substreams::v1::program::DynamicFeeParameter;



use pb::substreams::v1::program::LiquidityParameterByStrategyOneSide;



use pb::substreams::v1::program::LiquidityParameterByStrategy;



use pb::substreams::v1::program::StrategyParameters;



use pb::substreams::v1::program::LiquidityOneSideParameter;



use pb::substreams::v1::program::BinLiquidityDistributionByWeight;



use pb::substreams::v1::program::LiquidityParameterByWeight;



use pb::substreams::v1::program::AddLiquiditySingleSidePreciseParameter;



use pb::substreams::v1::program::CompressedBinDepositAmount;



use pb::substreams::v1::program::BinLiquidityDistribution;



use pb::substreams::v1::program::LiquidityParameter;



use pb::substreams::v1::program::CustomizableParams;



use pb::substreams::v1::program::InitPermissionPairIx;



use pb::substreams::v1::program::AddLiquiditySingleSidePreciseParameter2;




use pb::substreams::v1::program::InitializeLbPair2Params;



use pb::substreams::v1::program::BinLiquidityReduction;











use pb::substreams::v1::program::RemainingAccountsSlice;



use pb::substreams::v1::program::RemainingAccountsInfo;



use pb::substreams::v1::program::StrategyTypeEnum;









use pb::substreams::v1::program::AccountsTypeEnum;












use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut composition_fee_event_list: Vec<CompositionFeeEvent> = Vec::new();
    let mut add_liquidity_event_list: Vec<AddLiquidityEvent> = Vec::new();
    let mut remove_liquidity_event_list: Vec<RemoveLiquidityEvent> = Vec::new();
    let mut swap_event_list: Vec<SwapEvent> = Vec::new();
    let mut claim_reward_event_list: Vec<ClaimRewardEvent> = Vec::new();
    let mut fund_reward_event_list: Vec<FundRewardEvent> = Vec::new();
    let mut initialize_reward_event_list: Vec<InitializeRewardEvent> = Vec::new();
    let mut update_reward_duration_event_list: Vec<UpdateRewardDurationEvent> = Vec::new();
    let mut update_reward_funder_event_list: Vec<UpdateRewardFunderEvent> = Vec::new();
    let mut position_close_event_list: Vec<PositionCloseEvent> = Vec::new();
    let mut claim_fee_event_list: Vec<ClaimFeeEvent> = Vec::new();
    let mut lb_pair_create_event_list: Vec<LbPairCreateEvent> = Vec::new();
    let mut position_create_event_list: Vec<PositionCreateEvent> = Vec::new();
    let mut increase_position_length_event_list: Vec<IncreasePositionLengthEvent> = Vec::new();
    let mut decrease_position_length_event_list: Vec<DecreasePositionLengthEvent> = Vec::new();
    let mut fee_parameter_update_event_list: Vec<FeeParameterUpdateEvent> = Vec::new();
    let mut dynamic_fee_parameter_update_event_list: Vec<DynamicFeeParameterUpdateEvent> = Vec::new();
    let mut increase_observation_event_list: Vec<IncreaseObservationEvent> = Vec::new();
    let mut withdraw_ineligible_reward_event_list: Vec<WithdrawIneligibleRewardEvent> = Vec::new();
    let mut update_position_operator_event_list: Vec<UpdatePositionOperatorEvent> = Vec::new();
    let mut update_position_lock_release_point_event_list: Vec<UpdatePositionLockReleasePointEvent> = Vec::new();
    let mut go_to_a_bin_event_list: Vec<GoToABinEvent> = Vec::new();
    let mut initialize_lb_pair_instruction_list: Vec<InitializeLbPairInstruction> = Vec::new();
    let mut initialize_permission_lb_pair_instruction_list: Vec<InitializePermissionLbPairInstruction> = Vec::new();
    let mut initialize_customizable_permissionless_lb_pair_instruction_list: Vec<InitializeCustomizablePermissionlessLbPairInstruction> = Vec::new();
    let mut initialize_bin_array_bitmap_extension_instruction_list: Vec<InitializeBinArrayBitmapExtensionInstruction> = Vec::new();
    let mut initialize_bin_array_instruction_list: Vec<InitializeBinArrayInstruction> = Vec::new();
    let mut add_liquidity_instruction_list: Vec<AddLiquidityInstruction> = Vec::new();
    let mut add_liquidity_by_weight_instruction_list: Vec<AddLiquidityByWeightInstruction> = Vec::new();
    let mut add_liquidity_by_strategy_instruction_list: Vec<AddLiquidityByStrategyInstruction> = Vec::new();
    let mut add_liquidity_by_strategy_one_side_instruction_list: Vec<AddLiquidityByStrategyOneSideInstruction> = Vec::new();
    let mut add_liquidity_one_side_instruction_list: Vec<AddLiquidityOneSideInstruction> = Vec::new();
    let mut remove_liquidity_instruction_list: Vec<RemoveLiquidityInstruction> = Vec::new();
    let mut initialize_position_instruction_list: Vec<InitializePositionInstruction> = Vec::new();
    let mut initialize_position_pda_instruction_list: Vec<InitializePositionPdaInstruction> = Vec::new();
    let mut initialize_position_by_operator_instruction_list: Vec<InitializePositionByOperatorInstruction> = Vec::new();
    let mut update_position_operator_instruction_list: Vec<UpdatePositionOperatorInstruction> = Vec::new();
    let mut swap_instruction_list: Vec<SwapInstruction> = Vec::new();
    let mut swap_exact_out_instruction_list: Vec<SwapExactOutInstruction> = Vec::new();
    let mut swap_with_price_impact_instruction_list: Vec<SwapWithPriceImpactInstruction> = Vec::new();
    let mut withdraw_protocol_fee_instruction_list: Vec<WithdrawProtocolFeeInstruction> = Vec::new();
    let mut initialize_reward_instruction_list: Vec<InitializeRewardInstruction> = Vec::new();
    let mut fund_reward_instruction_list: Vec<FundRewardInstruction> = Vec::new();
    let mut update_reward_funder_instruction_list: Vec<UpdateRewardFunderInstruction> = Vec::new();
    let mut update_reward_duration_instruction_list: Vec<UpdateRewardDurationInstruction> = Vec::new();
    let mut claim_reward_instruction_list: Vec<ClaimRewardInstruction> = Vec::new();
    let mut claim_fee_instruction_list: Vec<ClaimFeeInstruction> = Vec::new();
    let mut close_position_instruction_list: Vec<ClosePositionInstruction> = Vec::new();
    let mut update_base_fee_parameters_instruction_list: Vec<UpdateBaseFeeParametersInstruction> = Vec::new();
    let mut update_dynamic_fee_parameters_instruction_list: Vec<UpdateDynamicFeeParametersInstruction> = Vec::new();
    let mut increase_oracle_length_instruction_list: Vec<IncreaseOracleLengthInstruction> = Vec::new();
    let mut initialize_preset_parameter_instruction_list: Vec<InitializePresetParameterInstruction> = Vec::new();
    let mut close_preset_parameter_instruction_list: Vec<ClosePresetParameterInstruction> = Vec::new();
    let mut close_preset_parameter2_instruction_list: Vec<ClosePresetParameter2Instruction> = Vec::new();
    let mut remove_all_liquidity_instruction_list: Vec<RemoveAllLiquidityInstruction> = Vec::new();
    let mut set_pair_status_instruction_list: Vec<SetPairStatusInstruction> = Vec::new();
    let mut migrate_position_instruction_list: Vec<MigratePositionInstruction> = Vec::new();
    let mut migrate_bin_array_instruction_list: Vec<MigrateBinArrayInstruction> = Vec::new();
    let mut update_fees_and_rewards_instruction_list: Vec<UpdateFeesAndRewardsInstruction> = Vec::new();
    let mut withdraw_ineligible_reward_instruction_list: Vec<WithdrawIneligibleRewardInstruction> = Vec::new();
    let mut set_activation_point_instruction_list: Vec<SetActivationPointInstruction> = Vec::new();
    let mut remove_liquidity_by_range_instruction_list: Vec<RemoveLiquidityByRangeInstruction> = Vec::new();
    let mut add_liquidity_one_side_precise_instruction_list: Vec<AddLiquidityOneSidePreciseInstruction> = Vec::new();
    let mut go_to_a_bin_instruction_list: Vec<GoToAbInInstruction> = Vec::new();
    let mut set_pre_activation_duration_instruction_list: Vec<SetPreActivationDurationInstruction> = Vec::new();
    let mut set_pre_activation_swap_address_instruction_list: Vec<SetPreActivationSwapAddressInstruction> = Vec::new();
    let mut set_pair_status_permissionless_instruction_list: Vec<SetPairStatusPermissionlessInstruction> = Vec::new();
    let mut initialize_token_badge_instruction_list: Vec<InitializeTokenBadgeInstruction> = Vec::new();
    let mut create_claim_protocol_fee_operator_instruction_list: Vec<CreateClaimProtocolFeeOperatorInstruction> = Vec::new();
    let mut close_claim_protocol_fee_operator_instruction_list: Vec<CloseClaimProtocolFeeOperatorInstruction> = Vec::new();
    let mut initialize_preset_parameter2_instruction_list: Vec<InitializePresetParameter2Instruction> = Vec::new();
    let mut initialize_lb_pair2_instruction_list: Vec<InitializeLbPair2Instruction> = Vec::new();
    let mut initialize_customizable_permissionless_lb_pair2_instruction_list: Vec<InitializeCustomizablePermissionlessLbPair2Instruction> = Vec::new();
    let mut claim_fee2_instruction_list: Vec<ClaimFee2Instruction> = Vec::new();
    let mut claim_reward2_instruction_list: Vec<ClaimReward2Instruction> = Vec::new();
    let mut add_liquidity2_instruction_list: Vec<AddLiquidity2Instruction> = Vec::new();
    let mut add_liquidity_by_strategy2_instruction_list: Vec<AddLiquidityByStrategy2Instruction> = Vec::new();
    let mut add_liquidity_one_side_precise2_instruction_list: Vec<AddLiquidityOneSidePrecise2Instruction> = Vec::new();
    let mut remove_liquidity2_instruction_list: Vec<RemoveLiquidity2Instruction> = Vec::new();
    let mut remove_liquidity_by_range2_instruction_list: Vec<RemoveLiquidityByRange2Instruction> = Vec::new();
    let mut swap2_instruction_list: Vec<Swap2Instruction> = Vec::new();
    let mut swap_exact_out2_instruction_list: Vec<SwapExactOut2Instruction> = Vec::new();
    let mut swap_with_price_impact2_instruction_list: Vec<SwapWithPriceImpact2Instruction> = Vec::new();
    let mut close_position2_instruction_list: Vec<ClosePosition2Instruction> = Vec::new();
    let mut update_fees_and_reward2_instruction_list: Vec<UpdateFeesAndReward2Instruction> = Vec::new();
    let mut close_position_if_empty_instruction_list: Vec<ClosePositionIfEmptyInstruction> = Vec::new();

    blk.transactions().for_each(|transaction| {

        // ------------- EVENTS -------------
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();
        let programs_selector: ProgramsSelector = ProgramsSelector::new(&["*".to_string()]);
        let log_contexts = LogContext::parse_logs_basic(&meta.log_messages, &programs_selector);

        log_contexts
            .iter()
            .filter(|context| context.program_id == PROGRAM_ID)
            .for_each(|context| {
                context.data_logs.iter().for_each(|data| {
                    if let Ok(decoded) = BASE64_STANDARD.decode(data) {
                        let slice_u8: &mut &[u8] = &mut &decoded[..];
                        let slice_discriminator: [u8; 8] =
                            slice_u8[0..8].try_into().expect("error");
                        let static_discriminator_slice: &'static [u8] = Box::leak(Box::new(slice_discriminator));

                        match static_discriminator_slice {
                            idl::idl::program::events::CompositionFee::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::CompositionFee::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    composition_fee_event_list.push(CompositionFeeEvent {
                                        trx_hash: transaction.id(),
                                        from: event.from.to_string(),
                                        bin_id: event.bin_id as i32,
                                        token_x_fee_amount: event.token_x_fee_amount,
                                        token_y_fee_amount: event.token_y_fee_amount,
                                        protocol_token_x_fee_amount: event.protocol_token_x_fee_amount,
                                        protocol_token_y_fee_amount: event.protocol_token_y_fee_amount,
                                    });
                                }
                            }
                            idl::idl::program::events::AddLiquidity::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::AddLiquidity::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    add_liquidity_event_list.push(AddLiquidityEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        from: event.from.to_string(),
                                        position: event.position.to_string(),
                                        amounts: event.amounts.to_vec(),
                                        active_bin_id: event.active_bin_id,
                                    });
                                }
                            }
                            idl::idl::program::events::RemoveLiquidity::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::RemoveLiquidity::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    remove_liquidity_event_list.push(RemoveLiquidityEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        from: event.from.to_string(),
                                        position: event.position.to_string(),
                                        amounts: event.amounts.to_vec(),
                                        active_bin_id: event.active_bin_id,
                                    });
                                }
                            }
                            idl::idl::program::events::Swap::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::Swap::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    swap_event_list.push(SwapEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        from: event.from.to_string(),
                                        start_bin_id: event.start_bin_id,
                                        end_bin_id: event.end_bin_id,
                                        amount_in: event.amount_in,
                                        amount_out: event.amount_out,
                                        swap_for_y: event.swap_for_y,
                                        fee: event.fee,
                                        protocol_fee: event.protocol_fee,
                                        fee_bps: event.fee_bps as u64,
                                        host_fee: event.host_fee,
                                    });
                                }
                            }
                            idl::idl::program::events::ClaimReward::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::ClaimReward::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    claim_reward_event_list.push(ClaimRewardEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        position: event.position.to_string(),
                                        owner: event.owner.to_string(),
                                        reward_index: event.reward_index,
                                        total_reward: event.total_reward,
                                    });
                                }
                            }
                            idl::idl::program::events::FundReward::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::FundReward::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    fund_reward_event_list.push(FundRewardEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        funder: event.funder.to_string(),
                                        reward_index: event.reward_index,
                                        amount: event.amount,
                                    });
                                }
                            }
                            idl::idl::program::events::InitializeReward::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::InitializeReward::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    initialize_reward_event_list.push(InitializeRewardEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        reward_mint: event.reward_mint.to_string(),
                                        funder: event.funder.to_string(),
                                        reward_index: event.reward_index,
                                        reward_duration: event.reward_duration,
                                    });
                                }
                            }
                            idl::idl::program::events::UpdateRewardDuration::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UpdateRewardDuration::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    update_reward_duration_event_list.push(UpdateRewardDurationEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        reward_index: event.reward_index,
                                        old_reward_duration: event.old_reward_duration,
                                        new_reward_duration: event.new_reward_duration,
                                    });
                                }
                            }
                            idl::idl::program::events::UpdateRewardFunder::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UpdateRewardFunder::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    update_reward_funder_event_list.push(UpdateRewardFunderEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        reward_index: event.reward_index,
                                        old_funder: event.old_funder.to_string(),
                                        new_funder: event.new_funder.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::PositionClose::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::PositionClose::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    position_close_event_list.push(PositionCloseEvent {
                                        trx_hash: transaction.id(),
                                        position: event.position.to_string(),
                                        owner: event.owner.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::ClaimFee::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::ClaimFee::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    claim_fee_event_list.push(ClaimFeeEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        position: event.position.to_string(),
                                        owner: event.owner.to_string(),
                                        fee_x: event.fee_x,
                                        fee_y: event.fee_y,
                                    });
                                }
                            }
                            idl::idl::program::events::LbPairCreate::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::LbPairCreate::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    lb_pair_create_event_list.push(LbPairCreateEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        bin_step: event.bin_step as u64,
                                        token_x: event.token_x.to_string(),
                                        token_y: event.token_y.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::PositionCreate::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::PositionCreate::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    position_create_event_list.push(PositionCreateEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        position: event.position.to_string(),
                                        owner: event.owner.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::IncreasePositionLength::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::IncreasePositionLength::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    increase_position_length_event_list.push(IncreasePositionLengthEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        position: event.position.to_string(),
                                        owner: event.owner.to_string(),
                                        length_to_add: event.length_to_add as u64,
                                        side: event.side as u64,
                                    });
                                }
                            }
                            idl::idl::program::events::DecreasePositionLength::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::DecreasePositionLength::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    decrease_position_length_event_list.push(DecreasePositionLengthEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        position: event.position.to_string(),
                                        owner: event.owner.to_string(),
                                        length_to_remove: event.length_to_remove as u64,
                                        side: event.side as u64,
                                    });
                                }
                            }
                            idl::idl::program::events::FeeParameterUpdate::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::FeeParameterUpdate::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    fee_parameter_update_event_list.push(FeeParameterUpdateEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        protocol_share: event.protocol_share as u64,
                                        base_factor: event.base_factor as u64,
                                    });
                                }
                            }
                            idl::idl::program::events::DynamicFeeParameterUpdate::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::DynamicFeeParameterUpdate::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    dynamic_fee_parameter_update_event_list.push(DynamicFeeParameterUpdateEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        filter_period: event.filter_period as u64,
                                        decay_period: event.decay_period as u64,
                                        reduction_factor: event.reduction_factor as u64,
                                        variable_fee_control: event.variable_fee_control,
                                        max_volatility_accumulator: event.max_volatility_accumulator,
                                    });
                                }
                            }
                            idl::idl::program::events::IncreaseObservation::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::IncreaseObservation::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    increase_observation_event_list.push(IncreaseObservationEvent {
                                        trx_hash: transaction.id(),
                                        oracle: event.oracle.to_string(),
                                        new_observation_length: event.new_observation_length,
                                    });
                                }
                            }
                            idl::idl::program::events::WithdrawIneligibleReward::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WithdrawIneligibleReward::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    withdraw_ineligible_reward_event_list.push(WithdrawIneligibleRewardEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        reward_mint: event.reward_mint.to_string(),
                                        amount: event.amount,
                                    });
                                }
                            }
                            idl::idl::program::events::UpdatePositionOperator::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UpdatePositionOperator::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    update_position_operator_event_list.push(UpdatePositionOperatorEvent {
                                        trx_hash: transaction.id(),
                                        position: event.position.to_string(),
                                        old_operator: event.old_operator.to_string(),
                                        new_operator: event.new_operator.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::UpdatePositionLockReleasePoint::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::UpdatePositionLockReleasePoint::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    update_position_lock_release_point_event_list.push(UpdatePositionLockReleasePointEvent {
                                        trx_hash: transaction.id(),
                                        position: event.position.to_string(),
                                        current_point: event.current_point,
                                        new_lock_release_point: event.new_lock_release_point,
                                        old_lock_release_point: event.old_lock_release_point,
                                        sender: event.sender.to_string(),
                                    });
                                }
                            }
                            idl::idl::program::events::GoToABin::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::GoToABin::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    go_to_a_bin_event_list.push(GoToABinEvent {
                                        trx_hash: transaction.id(),
                                        lb_pair: event.lb_pair.to_string(),
                                        from_bin_id: event.from_bin_id,
                                        to_bin_id: event.to_bin_id,
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                });
            });// ------------- INSTRUCTIONS -------------
        transaction
        .walk_instructions()
        .into_iter()
        .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
        .for_each(|inst| {
            let slice_u8: &[u8] = &inst.data()[..];

            /*
                CPI events are contained inside the instruction data
            */
            if slice_u8.len() >= 16 {
                if &slice_u8[8..16] == idl::idl::program::events::CompositionFee::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::CompositionFee::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        composition_fee_event_list.push(CompositionFeeEvent {
                            trx_hash: transaction.id(),
                            from: event.from.to_string(),
                            bin_id: event.bin_id as i32,
                            token_x_fee_amount: event.token_x_fee_amount,
                            token_y_fee_amount: event.token_y_fee_amount,
                            protocol_token_x_fee_amount: event.protocol_token_x_fee_amount,
                            protocol_token_y_fee_amount: event.protocol_token_y_fee_amount,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::AddLiquidity::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::AddLiquidity::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        add_liquidity_event_list.push(AddLiquidityEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            from: event.from.to_string(),
                            position: event.position.to_string(),
                            amounts: event.amounts.to_vec(),
                            active_bin_id: event.active_bin_id,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::RemoveLiquidity::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::RemoveLiquidity::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        remove_liquidity_event_list.push(RemoveLiquidityEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            from: event.from.to_string(),
                            position: event.position.to_string(),
                            amounts: event.amounts.to_vec(),
                            active_bin_id: event.active_bin_id,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::Swap::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::Swap::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        swap_event_list.push(SwapEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            from: event.from.to_string(),
                            start_bin_id: event.start_bin_id,
                            end_bin_id: event.end_bin_id,
                            amount_in: event.amount_in,
                            amount_out: event.amount_out,
                            swap_for_y: event.swap_for_y,
                            fee: event.fee,
                            protocol_fee: event.protocol_fee,
                            fee_bps: event.fee_bps as u64,
                            host_fee: event.host_fee,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::ClaimReward::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::ClaimReward::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        claim_reward_event_list.push(ClaimRewardEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            position: event.position.to_string(),
                            owner: event.owner.to_string(),
                            reward_index: event.reward_index,
                            total_reward: event.total_reward,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::FundReward::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::FundReward::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        fund_reward_event_list.push(FundRewardEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            funder: event.funder.to_string(),
                            reward_index: event.reward_index,
                            amount: event.amount,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::InitializeReward::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::InitializeReward::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        initialize_reward_event_list.push(InitializeRewardEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            reward_mint: event.reward_mint.to_string(),
                            funder: event.funder.to_string(),
                            reward_index: event.reward_index,
                            reward_duration: event.reward_duration,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::UpdateRewardDuration::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::UpdateRewardDuration::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        update_reward_duration_event_list.push(UpdateRewardDurationEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            reward_index: event.reward_index,
                            old_reward_duration: event.old_reward_duration,
                            new_reward_duration: event.new_reward_duration,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::UpdateRewardFunder::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::UpdateRewardFunder::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        update_reward_funder_event_list.push(UpdateRewardFunderEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            reward_index: event.reward_index,
                            old_funder: event.old_funder.to_string(),
                            new_funder: event.new_funder.to_string(),
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::PositionClose::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::PositionClose::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        position_close_event_list.push(PositionCloseEvent {
                            trx_hash: transaction.id(),
                            position: event.position.to_string(),
                            owner: event.owner.to_string(),
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::ClaimFee::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::ClaimFee::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        claim_fee_event_list.push(ClaimFeeEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            position: event.position.to_string(),
                            owner: event.owner.to_string(),
                            fee_x: event.fee_x,
                            fee_y: event.fee_y,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::LbPairCreate::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::LbPairCreate::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        lb_pair_create_event_list.push(LbPairCreateEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            bin_step: event.bin_step as u64,
                            token_x: event.token_x.to_string(),
                            token_y: event.token_y.to_string(),
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::PositionCreate::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::PositionCreate::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        position_create_event_list.push(PositionCreateEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            position: event.position.to_string(),
                            owner: event.owner.to_string(),
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::IncreasePositionLength::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::IncreasePositionLength::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        increase_position_length_event_list.push(IncreasePositionLengthEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            position: event.position.to_string(),
                            owner: event.owner.to_string(),
                            length_to_add: event.length_to_add as u64,
                            side: event.side as u64,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::DecreasePositionLength::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::DecreasePositionLength::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        decrease_position_length_event_list.push(DecreasePositionLengthEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            position: event.position.to_string(),
                            owner: event.owner.to_string(),
                            length_to_remove: event.length_to_remove as u64,
                            side: event.side as u64,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::FeeParameterUpdate::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::FeeParameterUpdate::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        fee_parameter_update_event_list.push(FeeParameterUpdateEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            protocol_share: event.protocol_share as u64,
                            base_factor: event.base_factor as u64,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::DynamicFeeParameterUpdate::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::DynamicFeeParameterUpdate::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        dynamic_fee_parameter_update_event_list.push(DynamicFeeParameterUpdateEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            filter_period: event.filter_period as u64,
                            decay_period: event.decay_period as u64,
                            reduction_factor: event.reduction_factor as u64,
                            variable_fee_control: event.variable_fee_control,
                            max_volatility_accumulator: event.max_volatility_accumulator,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::IncreaseObservation::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::IncreaseObservation::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        increase_observation_event_list.push(IncreaseObservationEvent {
                            trx_hash: transaction.id(),
                            oracle: event.oracle.to_string(),
                            new_observation_length: event.new_observation_length,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::WithdrawIneligibleReward::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::WithdrawIneligibleReward::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        withdraw_ineligible_reward_event_list.push(WithdrawIneligibleRewardEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            reward_mint: event.reward_mint.to_string(),
                            amount: event.amount,
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::UpdatePositionOperator::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::UpdatePositionOperator::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        update_position_operator_event_list.push(UpdatePositionOperatorEvent {
                            trx_hash: transaction.id(),
                            position: event.position.to_string(),
                            old_operator: event.old_operator.to_string(),
                            new_operator: event.new_operator.to_string(),
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::UpdatePositionLockReleasePoint::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::UpdatePositionLockReleasePoint::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        update_position_lock_release_point_event_list.push(UpdatePositionLockReleasePointEvent {
                            trx_hash: transaction.id(),
                            position: event.position.to_string(),
                            current_point: event.current_point,
                            new_lock_release_point: event.new_lock_release_point,
                            old_lock_release_point: event.old_lock_release_point,
                            sender: event.sender.to_string(),
                        });
                    }
                }
                if &slice_u8[8..16] == idl::idl::program::events::GoToABin::DISCRIMINATOR {
                    if let Ok(event) =
                        idl::idl::program::events::GoToABin::deserialize(
                            &mut &slice_u8[16..],
                        )
                    {
                        go_to_a_bin_event_list.push(GoToABinEvent {
                            trx_hash: transaction.id(),
                            lb_pair: event.lb_pair.to_string(),
                            from_bin_id: event.from_bin_id,
                            to_bin_id: event.to_bin_id,
                        });
                    }
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeLbPair::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeLbPair::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_lb_pair_instruction_list.push(InitializeLbPairInstruction {
                        trx_hash: transaction.id(),
                        active_id: instruction.active_id,
                        bin_step: instruction.bin_step as u64,
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_token_mint_x: accts[2].to_string(),
                        acct_token_mint_y: accts[3].to_string(),
                        acct_reserve_x: accts[4].to_string(),
                        acct_reserve_y: accts[5].to_string(),
                        acct_oracle: accts[6].to_string(),
                        acct_preset_parameter: accts[7].to_string(),
                        acct_funder: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                        acct_system_program: accts[10].to_string(),
                        acct_rent: accts[11].to_string(),
                        acct_event_authority: accts[12].to_string(),
                        acct_program: accts[13].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializePermissionLbPair::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializePermissionLbPair::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_permission_lb_pair_instruction_list.push(InitializePermissionLbPairInstruction {
                        trx_hash: transaction.id(),
                        ix_data: Some(InitPermissionPairIx {
						active_id: instruction.ix_data.active_id,bin_step: instruction.ix_data.bin_step as u64,base_factor: instruction.ix_data.base_factor as u64,base_fee_power_factor: instruction.ix_data.base_fee_power_factor as u64,activation_type: instruction.ix_data.activation_type as u64,protocol_share: instruction.ix_data.protocol_share as u64,
					}),
                        acct_base: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_token_mint_x: accts[3].to_string(),
                        acct_token_mint_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_oracle: accts[7].to_string(),
                        acct_admin: accts[8].to_string(),
                        acct_token_badge_x: accts[9].to_string(),
                        acct_token_badge_y: accts[10].to_string(),
                        acct_token_program_x: accts[11].to_string(),
                        acct_token_program_y: accts[12].to_string(),
                        acct_system_program: accts[13].to_string(),
                        acct_rent: accts[14].to_string(),
                        acct_event_authority: accts[15].to_string(),
                        acct_program: accts[16].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeCustomizablePermissionlessLbPair::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeCustomizablePermissionlessLbPair::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_customizable_permissionless_lb_pair_instruction_list.push(InitializeCustomizablePermissionlessLbPairInstruction {
                        trx_hash: transaction.id(),
                        params: Some(CustomizableParams {
						active_id: instruction.params.active_id,bin_step: instruction.params.bin_step as u64,base_factor: instruction.params.base_factor as u64,activation_type: instruction.params.activation_type as u64,has_alpha_vault: instruction.params.has_alpha_vault,activation_point: instruction.params.activation_point,creator_pool_on_off_control: instruction.params.creator_pool_on_off_control,base_fee_power_factor: instruction.params.base_fee_power_factor as u64,padding: instruction.params.padding.into_iter().map(|f| f as u64).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_token_mint_x: accts[2].to_string(),
                        acct_token_mint_y: accts[3].to_string(),
                        acct_reserve_x: accts[4].to_string(),
                        acct_reserve_y: accts[5].to_string(),
                        acct_oracle: accts[6].to_string(),
                        acct_user_token_x: accts[7].to_string(),
                        acct_funder: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                        acct_system_program: accts[10].to_string(),
                        acct_user_token_y: accts[11].to_string(),
                        acct_event_authority: accts[12].to_string(),
                        acct_program: accts[13].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeBinArrayBitmapExtension::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeBinArrayBitmapExtension::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_bin_array_bitmap_extension_instruction_list.push(InitializeBinArrayBitmapExtensionInstruction {
                        trx_hash: transaction.id(),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_funder: accts[2].to_string(),
                        acct_system_program: accts[3].to_string(),
                        acct_rent: accts[4].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeBinArray::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeBinArray::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_bin_array_instruction_list.push(InitializeBinArrayInstruction {
                        trx_hash: transaction.id(),
                        index: instruction.index,
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array: accts[1].to_string(),
                        acct_funder: accts[2].to_string(),
                        acct_system_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidity::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidity::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_instruction_list.push(AddLiquidityInstruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityParameter {
						amount_x: instruction.liquidity_parameter.amount_x,amount_y: instruction.liquidity_parameter.amount_y,bin_liquidity_dist: instruction.liquidity_parameter.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistribution {
						bin_id: bin_liquidity_dist.bin_id,distribution_x: bin_liquidity_dist.distribution_x as u64,distribution_y: bin_liquidity_dist.distribution_y as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_bin_array_lower: accts[9].to_string(),
                        acct_bin_array_upper: accts[10].to_string(),
                        acct_sender: accts[11].to_string(),
                        acct_token_x_program: accts[12].to_string(),
                        acct_token_y_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityByWeight::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityByWeight::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_by_weight_instruction_list.push(AddLiquidityByWeightInstruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityParameterByWeight {
						amount_x: instruction.liquidity_parameter.amount_x,amount_y: instruction.liquidity_parameter.amount_y,active_id: instruction.liquidity_parameter.active_id,max_active_bin_slippage: instruction.liquidity_parameter.max_active_bin_slippage,bin_liquidity_dist: instruction.liquidity_parameter.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistributionByWeight {
						bin_id: bin_liquidity_dist.bin_id,weight: bin_liquidity_dist.weight as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_bin_array_lower: accts[9].to_string(),
                        acct_bin_array_upper: accts[10].to_string(),
                        acct_sender: accts[11].to_string(),
                        acct_token_x_program: accts[12].to_string(),
                        acct_token_y_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityByStrategy::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityByStrategy::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_by_strategy_instruction_list.push(AddLiquidityByStrategyInstruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityParameterByStrategy {
						amount_x: instruction.liquidity_parameter.amount_x,amount_y: instruction.liquidity_parameter.amount_y,active_id: instruction.liquidity_parameter.active_id,max_active_bin_slippage: instruction.liquidity_parameter.max_active_bin_slippage,strategy_parameters: Some(StrategyParameters {
						min_bin_id: instruction.liquidity_parameter.strategy_parameters.min_bin_id,max_bin_id: instruction.liquidity_parameter.strategy_parameters.max_bin_id,strategy_type: map_enum_strategy_type(instruction.liquidity_parameter.strategy_parameters.strategy_type),parameteres: instruction.liquidity_parameter.strategy_parameters.parameteres.into_iter().map(|f| f as u64).collect(),
					}),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_bin_array_lower: accts[9].to_string(),
                        acct_bin_array_upper: accts[10].to_string(),
                        acct_sender: accts[11].to_string(),
                        acct_token_x_program: accts[12].to_string(),
                        acct_token_y_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityByStrategyOneSide::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityByStrategyOneSide::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_by_strategy_one_side_instruction_list.push(AddLiquidityByStrategyOneSideInstruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityParameterByStrategyOneSide {
						amount: instruction.liquidity_parameter.amount,active_id: instruction.liquidity_parameter.active_id,max_active_bin_slippage: instruction.liquidity_parameter.max_active_bin_slippage,strategy_parameters: Some(StrategyParameters {
						min_bin_id: instruction.liquidity_parameter.strategy_parameters.min_bin_id,max_bin_id: instruction.liquidity_parameter.strategy_parameters.max_bin_id,strategy_type: map_enum_strategy_type(instruction.liquidity_parameter.strategy_parameters.strategy_type),parameteres: instruction.liquidity_parameter.strategy_parameters.parameteres.into_iter().map(|f| f as u64).collect(),
					}),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token: accts[3].to_string(),
                        acct_reserve: accts[4].to_string(),
                        acct_token_mint: accts[5].to_string(),
                        acct_bin_array_lower: accts[6].to_string(),
                        acct_bin_array_upper: accts[7].to_string(),
                        acct_sender: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                        acct_event_authority: accts[10].to_string(),
                        acct_program: accts[11].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityOneSide::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityOneSide::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_one_side_instruction_list.push(AddLiquidityOneSideInstruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityOneSideParameter {
						amount: instruction.liquidity_parameter.amount,active_id: instruction.liquidity_parameter.active_id,max_active_bin_slippage: instruction.liquidity_parameter.max_active_bin_slippage,bin_liquidity_dist: instruction.liquidity_parameter.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistributionByWeight {
						bin_id: bin_liquidity_dist.bin_id,weight: bin_liquidity_dist.weight as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token: accts[3].to_string(),
                        acct_reserve: accts[4].to_string(),
                        acct_token_mint: accts[5].to_string(),
                        acct_bin_array_lower: accts[6].to_string(),
                        acct_bin_array_upper: accts[7].to_string(),
                        acct_sender: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                        acct_event_authority: accts[10].to_string(),
                        acct_program: accts[11].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::RemoveLiquidity::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::RemoveLiquidity::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    remove_liquidity_instruction_list.push(RemoveLiquidityInstruction {
                        trx_hash: transaction.id(),
                        bin_liquidity_removal: instruction.bin_liquidity_removal.into_iter().map(|bin_liquidity_removal| BinLiquidityReduction {
						bin_id: bin_liquidity_removal.bin_id,bps_to_remove: bin_liquidity_removal.bps_to_remove as u64,
					}).collect(),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_bin_array_lower: accts[9].to_string(),
                        acct_bin_array_upper: accts[10].to_string(),
                        acct_sender: accts[11].to_string(),
                        acct_token_x_program: accts[12].to_string(),
                        acct_token_y_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializePosition::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializePosition::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_position_instruction_list.push(InitializePositionInstruction {
                        trx_hash: transaction.id(),
                        lower_bin_id: instruction.lower_bin_id,
                        width: instruction.width,
                        acct_payer: accts[0].to_string(),
                        acct_position: accts[1].to_string(),
                        acct_lb_pair: accts[2].to_string(),
                        acct_owner: accts[3].to_string(),
                        acct_system_program: accts[4].to_string(),
                        acct_rent: accts[5].to_string(),
                        acct_event_authority: accts[6].to_string(),
                        acct_program: accts[7].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializePositionPda::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializePositionPda::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_position_pda_instruction_list.push(InitializePositionPdaInstruction {
                        trx_hash: transaction.id(),
                        lower_bin_id: instruction.lower_bin_id,
                        width: instruction.width,
                        acct_payer: accts[0].to_string(),
                        acct_base: accts[1].to_string(),
                        acct_position: accts[2].to_string(),
                        acct_lb_pair: accts[3].to_string(),
                        acct_owner: accts[4].to_string(),
                        acct_system_program: accts[5].to_string(),
                        acct_rent: accts[6].to_string(),
                        acct_event_authority: accts[7].to_string(),
                        acct_program: accts[8].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializePositionByOperator::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializePositionByOperator::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_position_by_operator_instruction_list.push(InitializePositionByOperatorInstruction {
                        trx_hash: transaction.id(),
                        lower_bin_id: instruction.lower_bin_id,
                        width: instruction.width,
                        fee_owner: instruction.fee_owner.to_string(),
                        lock_release_point: instruction.lock_release_point,
                        acct_payer: accts[0].to_string(),
                        acct_base: accts[1].to_string(),
                        acct_position: accts[2].to_string(),
                        acct_lb_pair: accts[3].to_string(),
                        acct_owner: accts[4].to_string(),
                        acct_operator: accts[5].to_string(),
                        acct_operator_token_x: accts[6].to_string(),
                        acct_owner_token_x: accts[7].to_string(),
                        acct_system_program: accts[8].to_string(),
                        acct_event_authority: accts[9].to_string(),
                        acct_program: accts[10].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdatePositionOperator::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdatePositionOperator::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_position_operator_instruction_list.push(UpdatePositionOperatorInstruction {
                        trx_hash: transaction.id(),
                        operator: instruction.operator.to_string(),
                        acct_position: accts[0].to_string(),
                        acct_owner: accts[1].to_string(),
                        acct_event_authority: accts[2].to_string(),
                        acct_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Swap::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Swap::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    swap_instruction_list.push(SwapInstruction {
                        trx_hash: transaction.id(),
                        amount_in: instruction.amount_in,
                        min_amount_out: instruction.min_amount_out,
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_reserve_x: accts[2].to_string(),
                        acct_reserve_y: accts[3].to_string(),
                        acct_user_token_in: accts[4].to_string(),
                        acct_user_token_out: accts[5].to_string(),
                        acct_token_x_mint: accts[6].to_string(),
                        acct_token_y_mint: accts[7].to_string(),
                        acct_oracle: accts[8].to_string(),
                        acct_host_fee_in: accts[9].to_string(),
                        acct_user: accts[10].to_string(),
                        acct_token_x_program: accts[11].to_string(),
                        acct_token_y_program: accts[12].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SwapExactOut::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SwapExactOut::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    swap_exact_out_instruction_list.push(SwapExactOutInstruction {
                        trx_hash: transaction.id(),
                        max_in_amount: instruction.max_in_amount,
                        out_amount: instruction.out_amount,
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_reserve_x: accts[2].to_string(),
                        acct_reserve_y: accts[3].to_string(),
                        acct_user_token_in: accts[4].to_string(),
                        acct_user_token_out: accts[5].to_string(),
                        acct_token_x_mint: accts[6].to_string(),
                        acct_token_y_mint: accts[7].to_string(),
                        acct_oracle: accts[8].to_string(),
                        acct_host_fee_in: accts[9].to_string(),
                        acct_user: accts[10].to_string(),
                        acct_token_x_program: accts[11].to_string(),
                        acct_token_y_program: accts[12].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SwapWithPriceImpact::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SwapWithPriceImpact::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    swap_with_price_impact_instruction_list.push(SwapWithPriceImpactInstruction {
                        trx_hash: transaction.id(),
                        amount_in: instruction.amount_in,
                        active_id: instruction.active_id,
                        max_price_impact_bps: instruction.max_price_impact_bps as u64,
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_reserve_x: accts[2].to_string(),
                        acct_reserve_y: accts[3].to_string(),
                        acct_user_token_in: accts[4].to_string(),
                        acct_user_token_out: accts[5].to_string(),
                        acct_token_x_mint: accts[6].to_string(),
                        acct_token_y_mint: accts[7].to_string(),
                        acct_oracle: accts[8].to_string(),
                        acct_host_fee_in: accts[9].to_string(),
                        acct_user: accts[10].to_string(),
                        acct_token_x_program: accts[11].to_string(),
                        acct_token_y_program: accts[12].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::WithdrawProtocolFee::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::WithdrawProtocolFee::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    withdraw_protocol_fee_instruction_list.push(WithdrawProtocolFeeInstruction {
                        trx_hash: transaction.id(),
                        amount_x: instruction.amount_x,
                        amount_y: instruction.amount_y,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_reserve_x: accts[1].to_string(),
                        acct_reserve_y: accts[2].to_string(),
                        acct_token_x_mint: accts[3].to_string(),
                        acct_token_y_mint: accts[4].to_string(),
                        acct_receiver_token_x: accts[5].to_string(),
                        acct_receiver_token_y: accts[6].to_string(),
                        acct_claim_fee_operator: accts[7].to_string(),
                        acct_operator: accts[8].to_string(),
                        acct_token_x_program: accts[9].to_string(),
                        acct_token_y_program: accts[10].to_string(),
                        acct_memo_program: accts[11].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeReward::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeReward::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_reward_instruction_list.push(InitializeRewardInstruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        reward_duration: instruction.reward_duration,
                        funder: instruction.funder.to_string(),
                        acct_lb_pair: accts[0].to_string(),
                        acct_reward_vault: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_token_badge: accts[3].to_string(),
                        acct_admin: accts[4].to_string(),
                        acct_token_program: accts[5].to_string(),
                        acct_system_program: accts[6].to_string(),
                        acct_rent: accts[7].to_string(),
                        acct_event_authority: accts[8].to_string(),
                        acct_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::FundReward::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::FundReward::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    fund_reward_instruction_list.push(FundRewardInstruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        amount: instruction.amount,
                        carry_forward: instruction.carry_forward,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_reward_vault: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_funder_token_account: accts[3].to_string(),
                        acct_funder: accts[4].to_string(),
                        acct_bin_array: accts[5].to_string(),
                        acct_token_program: accts[6].to_string(),
                        acct_event_authority: accts[7].to_string(),
                        acct_program: accts[8].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateRewardFunder::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateRewardFunder::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_reward_funder_instruction_list.push(UpdateRewardFunderInstruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        new_funder: instruction.new_funder.to_string(),
                        acct_lb_pair: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_event_authority: accts[2].to_string(),
                        acct_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateRewardDuration::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateRewardDuration::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_reward_duration_instruction_list.push(UpdateRewardDurationInstruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        new_duration: instruction.new_duration,
                        acct_lb_pair: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_bin_array: accts[2].to_string(),
                        acct_event_authority: accts[3].to_string(),
                        acct_program: accts[4].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClaimReward::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClaimReward::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_reward_instruction_list.push(ClaimRewardInstruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        acct_lb_pair: accts[0].to_string(),
                        acct_position: accts[1].to_string(),
                        acct_bin_array_lower: accts[2].to_string(),
                        acct_bin_array_upper: accts[3].to_string(),
                        acct_sender: accts[4].to_string(),
                        acct_reward_vault: accts[5].to_string(),
                        acct_reward_mint: accts[6].to_string(),
                        acct_user_token_account: accts[7].to_string(),
                        acct_token_program: accts[8].to_string(),
                        acct_event_authority: accts[9].to_string(),
                        acct_program: accts[10].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClaimFee::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClaimFee::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_fee_instruction_list.push(ClaimFeeInstruction {
                        trx_hash: transaction.id(),
                        acct_lb_pair: accts[0].to_string(),
                        acct_position: accts[1].to_string(),
                        acct_bin_array_lower: accts[2].to_string(),
                        acct_bin_array_upper: accts[3].to_string(),
                        acct_sender: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_user_token_x: accts[7].to_string(),
                        acct_user_token_y: accts[8].to_string(),
                        acct_token_x_mint: accts[9].to_string(),
                        acct_token_y_mint: accts[10].to_string(),
                        acct_token_program: accts[11].to_string(),
                        acct_event_authority: accts[12].to_string(),
                        acct_program: accts[13].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClosePosition::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClosePosition::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    close_position_instruction_list.push(ClosePositionInstruction {
                        trx_hash: transaction.id(),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_lower: accts[2].to_string(),
                        acct_bin_array_upper: accts[3].to_string(),
                        acct_sender: accts[4].to_string(),
                        acct_rent_receiver: accts[5].to_string(),
                        acct_event_authority: accts[6].to_string(),
                        acct_program: accts[7].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateBaseFeeParameters::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateBaseFeeParameters::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_base_fee_parameters_instruction_list.push(UpdateBaseFeeParametersInstruction {
                        trx_hash: transaction.id(),
                        fee_parameter: Some(BaseFeeParameter {
						protocol_share: instruction.fee_parameter.protocol_share as u64,base_factor: instruction.fee_parameter.base_factor as u64,base_fee_power_factor: instruction.fee_parameter.base_fee_power_factor as u64,
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_event_authority: accts[2].to_string(),
                        acct_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateDynamicFeeParameters::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateDynamicFeeParameters::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_dynamic_fee_parameters_instruction_list.push(UpdateDynamicFeeParametersInstruction {
                        trx_hash: transaction.id(),
                        fee_parameter: Some(DynamicFeeParameter {
						filter_period: instruction.fee_parameter.filter_period as u64,decay_period: instruction.fee_parameter.decay_period as u64,reduction_factor: instruction.fee_parameter.reduction_factor as u64,variable_fee_control: instruction.fee_parameter.variable_fee_control,max_volatility_accumulator: instruction.fee_parameter.max_volatility_accumulator,
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_event_authority: accts[2].to_string(),
                        acct_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::IncreaseOracleLength::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::IncreaseOracleLength::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    increase_oracle_length_instruction_list.push(IncreaseOracleLengthInstruction {
                        trx_hash: transaction.id(),
                        length_to_add: instruction.length_to_add,
                        acct_oracle: accts[0].to_string(),
                        acct_funder: accts[1].to_string(),
                        acct_system_program: accts[2].to_string(),
                        acct_event_authority: accts[3].to_string(),
                        acct_program: accts[4].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializePresetParameter::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializePresetParameter::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_preset_parameter_instruction_list.push(InitializePresetParameterInstruction {
                        trx_hash: transaction.id(),
                        ix: Some(InitPresetParametersIx {
						bin_step: instruction.ix.bin_step as u64,base_factor: instruction.ix.base_factor as u64,filter_period: instruction.ix.filter_period as u64,decay_period: instruction.ix.decay_period as u64,reduction_factor: instruction.ix.reduction_factor as u64,variable_fee_control: instruction.ix.variable_fee_control,max_volatility_accumulator: instruction.ix.max_volatility_accumulator,protocol_share: instruction.ix.protocol_share as u64,
					}),
                        acct_preset_parameter: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_system_program: accts[2].to_string(),
                        acct_rent: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClosePresetParameter::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClosePresetParameter::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    close_preset_parameter_instruction_list.push(ClosePresetParameterInstruction {
                        trx_hash: transaction.id(),
                        acct_preset_parameter: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_rent_receiver: accts[2].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClosePresetParameter2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClosePresetParameter2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    close_preset_parameter2_instruction_list.push(ClosePresetParameter2Instruction {
                        trx_hash: transaction.id(),
                        acct_preset_parameter: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_rent_receiver: accts[2].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::RemoveAllLiquidity::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::RemoveAllLiquidity::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    remove_all_liquidity_instruction_list.push(RemoveAllLiquidityInstruction {
                        trx_hash: transaction.id(),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_bin_array_lower: accts[9].to_string(),
                        acct_bin_array_upper: accts[10].to_string(),
                        acct_sender: accts[11].to_string(),
                        acct_token_x_program: accts[12].to_string(),
                        acct_token_y_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetPairStatus::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetPairStatus::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_pair_status_instruction_list.push(SetPairStatusInstruction {
                        trx_hash: transaction.id(),
                        status: instruction.status as u64,
                        acct_lb_pair: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::MigratePosition::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::MigratePosition::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    migrate_position_instruction_list.push(MigratePositionInstruction {
                        trx_hash: transaction.id(),
                        acct_position_v2: accts[0].to_string(),
                        acct_position_v1: accts[1].to_string(),
                        acct_lb_pair: accts[2].to_string(),
                        acct_bin_array_lower: accts[3].to_string(),
                        acct_bin_array_upper: accts[4].to_string(),
                        acct_owner: accts[5].to_string(),
                        acct_system_program: accts[6].to_string(),
                        acct_rent_receiver: accts[7].to_string(),
                        acct_event_authority: accts[8].to_string(),
                        acct_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::MigrateBinArray::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::MigrateBinArray::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    migrate_bin_array_instruction_list.push(MigrateBinArrayInstruction {
                        trx_hash: transaction.id(),
                        acct_lb_pair: accts[0].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateFeesAndRewards::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateFeesAndRewards::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_fees_and_rewards_instruction_list.push(UpdateFeesAndRewardsInstruction {
                        trx_hash: transaction.id(),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_lower: accts[2].to_string(),
                        acct_bin_array_upper: accts[3].to_string(),
                        acct_owner: accts[4].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::WithdrawIneligibleReward::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::WithdrawIneligibleReward::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    withdraw_ineligible_reward_instruction_list.push(WithdrawIneligibleRewardInstruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_reward_vault: accts[1].to_string(),
                        acct_reward_mint: accts[2].to_string(),
                        acct_funder_token_account: accts[3].to_string(),
                        acct_funder: accts[4].to_string(),
                        acct_bin_array: accts[5].to_string(),
                        acct_token_program: accts[6].to_string(),
                        acct_memo_program: accts[7].to_string(),
                        acct_event_authority: accts[8].to_string(),
                        acct_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetActivationPoint::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetActivationPoint::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_activation_point_instruction_list.push(SetActivationPointInstruction {
                        trx_hash: transaction.id(),
                        activation_point: instruction.activation_point,
                        acct_lb_pair: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::RemoveLiquidityByRange::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::RemoveLiquidityByRange::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    remove_liquidity_by_range_instruction_list.push(RemoveLiquidityByRangeInstruction {
                        trx_hash: transaction.id(),
                        from_bin_id: instruction.from_bin_id,
                        to_bin_id: instruction.to_bin_id,
                        bps_to_remove: instruction.bps_to_remove as u64,
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_bin_array_lower: accts[9].to_string(),
                        acct_bin_array_upper: accts[10].to_string(),
                        acct_sender: accts[11].to_string(),
                        acct_token_x_program: accts[12].to_string(),
                        acct_token_y_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityOneSidePrecise::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityOneSidePrecise::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_one_side_precise_instruction_list.push(AddLiquidityOneSidePreciseInstruction {
                        trx_hash: transaction.id(),
                        parameter: Some(AddLiquiditySingleSidePreciseParameter {
						bins: instruction.parameter.bins.into_iter().map(|bins| CompressedBinDepositAmount {
						bin_id: bins.bin_id,amount: bins.amount,
					}).collect(),decompress_multiplier: instruction.parameter.decompress_multiplier,
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token: accts[3].to_string(),
                        acct_reserve: accts[4].to_string(),
                        acct_token_mint: accts[5].to_string(),
                        acct_bin_array_lower: accts[6].to_string(),
                        acct_bin_array_upper: accts[7].to_string(),
                        acct_sender: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                        acct_event_authority: accts[10].to_string(),
                        acct_program: accts[11].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::GoToABin::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::GoToABin::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    go_to_a_bin_instruction_list.push(GoToAbInInstruction {
                        trx_hash: transaction.id(),
                        bin_id: instruction.bin_id,
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_from_bin_array: accts[2].to_string(),
                        acct_to_bin_array: accts[3].to_string(),
                        acct_event_authority: accts[4].to_string(),
                        acct_program: accts[5].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetPreActivationDuration::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetPreActivationDuration::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_pre_activation_duration_instruction_list.push(SetPreActivationDurationInstruction {
                        trx_hash: transaction.id(),
                        pre_activation_duration: instruction.pre_activation_duration,
                        acct_lb_pair: accts[0].to_string(),
                        acct_creator: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetPreActivationSwapAddress::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetPreActivationSwapAddress::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_pre_activation_swap_address_instruction_list.push(SetPreActivationSwapAddressInstruction {
                        trx_hash: transaction.id(),
                        pre_activation_swap_address: instruction.pre_activation_swap_address.to_string(),
                        acct_lb_pair: accts[0].to_string(),
                        acct_creator: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SetPairStatusPermissionless::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SetPairStatusPermissionless::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    set_pair_status_permissionless_instruction_list.push(SetPairStatusPermissionlessInstruction {
                        trx_hash: transaction.id(),
                        status: instruction.status as u64,
                        acct_lb_pair: accts[0].to_string(),
                        acct_creator: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeTokenBadge::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeTokenBadge::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_token_badge_instruction_list.push(InitializeTokenBadgeInstruction {
                        trx_hash: transaction.id(),
                        acct_token_mint: accts[0].to_string(),
                        acct_token_badge: accts[1].to_string(),
                        acct_admin: accts[2].to_string(),
                        acct_system_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CreateClaimProtocolFeeOperator::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CreateClaimProtocolFeeOperator::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    create_claim_protocol_fee_operator_instruction_list.push(CreateClaimProtocolFeeOperatorInstruction {
                        trx_hash: transaction.id(),
                        acct_claim_fee_operator: accts[0].to_string(),
                        acct_operator: accts[1].to_string(),
                        acct_admin: accts[2].to_string(),
                        acct_system_program: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CloseClaimProtocolFeeOperator::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CloseClaimProtocolFeeOperator::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    close_claim_protocol_fee_operator_instruction_list.push(CloseClaimProtocolFeeOperatorInstruction {
                        trx_hash: transaction.id(),
                        acct_claim_fee_operator: accts[0].to_string(),
                        acct_rent_receiver: accts[1].to_string(),
                        acct_admin: accts[2].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializePresetParameter2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializePresetParameter2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_preset_parameter2_instruction_list.push(InitializePresetParameter2Instruction {
                        trx_hash: transaction.id(),
                        ix: Some(InitPresetParameters2Ix {
						index: instruction.ix.index as u64,bin_step: instruction.ix.bin_step as u64,base_factor: instruction.ix.base_factor as u64,filter_period: instruction.ix.filter_period as u64,decay_period: instruction.ix.decay_period as u64,reduction_factor: instruction.ix.reduction_factor as u64,variable_fee_control: instruction.ix.variable_fee_control,max_volatility_accumulator: instruction.ix.max_volatility_accumulator,protocol_share: instruction.ix.protocol_share as u64,base_fee_power_factor: instruction.ix.base_fee_power_factor as u64,
					}),
                        acct_preset_parameter: accts[0].to_string(),
                        acct_admin: accts[1].to_string(),
                        acct_system_program: accts[2].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeLbPair2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeLbPair2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_lb_pair2_instruction_list.push(InitializeLbPair2Instruction {
                        trx_hash: transaction.id(),
                        params: Some(InitializeLbPair2Params {
						active_id: instruction.params.active_id,padding: instruction.params.padding.into_iter().map(|f| f as u64).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_token_mint_x: accts[2].to_string(),
                        acct_token_mint_y: accts[3].to_string(),
                        acct_reserve_x: accts[4].to_string(),
                        acct_reserve_y: accts[5].to_string(),
                        acct_oracle: accts[6].to_string(),
                        acct_preset_parameter: accts[7].to_string(),
                        acct_funder: accts[8].to_string(),
                        acct_token_badge_x: accts[9].to_string(),
                        acct_token_badge_y: accts[10].to_string(),
                        acct_token_program_x: accts[11].to_string(),
                        acct_token_program_y: accts[12].to_string(),
                        acct_system_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitializeCustomizablePermissionlessLbPair2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitializeCustomizablePermissionlessLbPair2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_customizable_permissionless_lb_pair2_instruction_list.push(InitializeCustomizablePermissionlessLbPair2Instruction {
                        trx_hash: transaction.id(),
                        params: Some(CustomizableParams {
						active_id: instruction.params.active_id,bin_step: instruction.params.bin_step as u64,base_factor: instruction.params.base_factor as u64,activation_type: instruction.params.activation_type as u64,has_alpha_vault: instruction.params.has_alpha_vault,activation_point: instruction.params.activation_point,creator_pool_on_off_control: instruction.params.creator_pool_on_off_control,base_fee_power_factor: instruction.params.base_fee_power_factor as u64,padding: instruction.params.padding.into_iter().map(|f| f as u64).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_token_mint_x: accts[2].to_string(),
                        acct_token_mint_y: accts[3].to_string(),
                        acct_reserve_x: accts[4].to_string(),
                        acct_reserve_y: accts[5].to_string(),
                        acct_oracle: accts[6].to_string(),
                        acct_user_token_x: accts[7].to_string(),
                        acct_funder: accts[8].to_string(),
                        acct_token_badge_x: accts[9].to_string(),
                        acct_token_badge_y: accts[10].to_string(),
                        acct_token_program_x: accts[11].to_string(),
                        acct_token_program_y: accts[12].to_string(),
                        acct_system_program: accts[13].to_string(),
                        acct_user_token_y: accts[14].to_string(),
                        acct_event_authority: accts[15].to_string(),
                        acct_program: accts[16].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClaimFee2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClaimFee2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_fee2_instruction_list.push(ClaimFee2Instruction {
                        trx_hash: transaction.id(),
                        min_bin_id: instruction.min_bin_id,
                        max_bin_id: instruction.max_bin_id,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_position: accts[1].to_string(),
                        acct_sender: accts[2].to_string(),
                        acct_reserve_x: accts[3].to_string(),
                        acct_reserve_y: accts[4].to_string(),
                        acct_user_token_x: accts[5].to_string(),
                        acct_user_token_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_token_program_x: accts[9].to_string(),
                        acct_token_program_y: accts[10].to_string(),
                        acct_memo_program: accts[11].to_string(),
                        acct_event_authority: accts[12].to_string(),
                        acct_program: accts[13].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClaimReward2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClaimReward2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_reward2_instruction_list.push(ClaimReward2Instruction {
                        trx_hash: transaction.id(),
                        reward_index: instruction.reward_index,
                        min_bin_id: instruction.min_bin_id,
                        max_bin_id: instruction.max_bin_id,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_position: accts[1].to_string(),
                        acct_sender: accts[2].to_string(),
                        acct_reward_vault: accts[3].to_string(),
                        acct_reward_mint: accts[4].to_string(),
                        acct_user_token_account: accts[5].to_string(),
                        acct_token_program: accts[6].to_string(),
                        acct_memo_program: accts[7].to_string(),
                        acct_event_authority: accts[8].to_string(),
                        acct_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidity2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidity2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity2_instruction_list.push(AddLiquidity2Instruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityParameter {
						amount_x: instruction.liquidity_parameter.amount_x,amount_y: instruction.liquidity_parameter.amount_y,bin_liquidity_dist: instruction.liquidity_parameter.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistribution {
						bin_id: bin_liquidity_dist.bin_id,distribution_x: bin_liquidity_dist.distribution_x as u64,distribution_y: bin_liquidity_dist.distribution_y as u64,
					}).collect(),
					}),
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_sender: accts[9].to_string(),
                        acct_token_x_program: accts[10].to_string(),
                        acct_token_y_program: accts[11].to_string(),
                        acct_event_authority: accts[12].to_string(),
                        acct_program: accts[13].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityByStrategy2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityByStrategy2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_by_strategy2_instruction_list.push(AddLiquidityByStrategy2Instruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(LiquidityParameterByStrategy {
						amount_x: instruction.liquidity_parameter.amount_x,amount_y: instruction.liquidity_parameter.amount_y,active_id: instruction.liquidity_parameter.active_id,max_active_bin_slippage: instruction.liquidity_parameter.max_active_bin_slippage,strategy_parameters: Some(StrategyParameters {
						min_bin_id: instruction.liquidity_parameter.strategy_parameters.min_bin_id,max_bin_id: instruction.liquidity_parameter.strategy_parameters.max_bin_id,strategy_type: map_enum_strategy_type(instruction.liquidity_parameter.strategy_parameters.strategy_type),parameteres: instruction.liquidity_parameter.strategy_parameters.parameteres.into_iter().map(|f| f as u64).collect(),
					}),
					}),
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_sender: accts[9].to_string(),
                        acct_token_x_program: accts[10].to_string(),
                        acct_token_y_program: accts[11].to_string(),
                        acct_event_authority: accts[12].to_string(),
                        acct_program: accts[13].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::AddLiquidityOneSidePrecise2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::AddLiquidityOneSidePrecise2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    add_liquidity_one_side_precise2_instruction_list.push(AddLiquidityOneSidePrecise2Instruction {
                        trx_hash: transaction.id(),
                        liquidity_parameter: Some(AddLiquiditySingleSidePreciseParameter2 {
						bins: instruction.liquidity_parameter.bins.into_iter().map(|bins| CompressedBinDepositAmount {
						bin_id: bins.bin_id,amount: bins.amount,
					}).collect(),decompress_multiplier: instruction.liquidity_parameter.decompress_multiplier,max_amount: instruction.liquidity_parameter.max_amount,
					}),
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token: accts[3].to_string(),
                        acct_reserve: accts[4].to_string(),
                        acct_token_mint: accts[5].to_string(),
                        acct_sender: accts[6].to_string(),
                        acct_token_program: accts[7].to_string(),
                        acct_event_authority: accts[8].to_string(),
                        acct_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::RemoveLiquidity2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::RemoveLiquidity2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    remove_liquidity2_instruction_list.push(RemoveLiquidity2Instruction {
                        trx_hash: transaction.id(),
                        bin_liquidity_removal: instruction.bin_liquidity_removal.into_iter().map(|bin_liquidity_removal| BinLiquidityReduction {
						bin_id: bin_liquidity_removal.bin_id,bps_to_remove: bin_liquidity_removal.bps_to_remove as u64,
					}).collect(),
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_sender: accts[9].to_string(),
                        acct_token_x_program: accts[10].to_string(),
                        acct_token_y_program: accts[11].to_string(),
                        acct_memo_program: accts[12].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::RemoveLiquidityByRange2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::RemoveLiquidityByRange2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    remove_liquidity_by_range2_instruction_list.push(RemoveLiquidityByRange2Instruction {
                        trx_hash: transaction.id(),
                        from_bin_id: instruction.from_bin_id,
                        to_bin_id: instruction.to_bin_id,
                        bps_to_remove: instruction.bps_to_remove as u64,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_bin_array_bitmap_extension: accts[2].to_string(),
                        acct_user_token_x: accts[3].to_string(),
                        acct_user_token_y: accts[4].to_string(),
                        acct_reserve_x: accts[5].to_string(),
                        acct_reserve_y: accts[6].to_string(),
                        acct_token_x_mint: accts[7].to_string(),
                        acct_token_y_mint: accts[8].to_string(),
                        acct_sender: accts[9].to_string(),
                        acct_token_x_program: accts[10].to_string(),
                        acct_token_y_program: accts[11].to_string(),
                        acct_memo_program: accts[12].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Swap2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Swap2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    swap2_instruction_list.push(Swap2Instruction {
                        trx_hash: transaction.id(),
                        amount_in: instruction.amount_in,
                        min_amount_out: instruction.min_amount_out,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_reserve_x: accts[2].to_string(),
                        acct_reserve_y: accts[3].to_string(),
                        acct_user_token_in: accts[4].to_string(),
                        acct_user_token_out: accts[5].to_string(),
                        acct_token_x_mint: accts[6].to_string(),
                        acct_token_y_mint: accts[7].to_string(),
                        acct_oracle: accts[8].to_string(),
                        acct_host_fee_in: accts[9].to_string(),
                        acct_user: accts[10].to_string(),
                        acct_token_x_program: accts[11].to_string(),
                        acct_token_y_program: accts[12].to_string(),
                        acct_memo_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SwapExactOut2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SwapExactOut2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    swap_exact_out2_instruction_list.push(SwapExactOut2Instruction {
                        trx_hash: transaction.id(),
                        max_in_amount: instruction.max_in_amount,
                        out_amount: instruction.out_amount,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_reserve_x: accts[2].to_string(),
                        acct_reserve_y: accts[3].to_string(),
                        acct_user_token_in: accts[4].to_string(),
                        acct_user_token_out: accts[5].to_string(),
                        acct_token_x_mint: accts[6].to_string(),
                        acct_token_y_mint: accts[7].to_string(),
                        acct_oracle: accts[8].to_string(),
                        acct_host_fee_in: accts[9].to_string(),
                        acct_user: accts[10].to_string(),
                        acct_token_x_program: accts[11].to_string(),
                        acct_token_y_program: accts[12].to_string(),
                        acct_memo_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SwapWithPriceImpact2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SwapWithPriceImpact2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    swap_with_price_impact2_instruction_list.push(SwapWithPriceImpact2Instruction {
                        trx_hash: transaction.id(),
                        amount_in: instruction.amount_in,
                        active_id: instruction.active_id,
                        max_price_impact_bps: instruction.max_price_impact_bps as u64,
                        remaining_accounts_info: Some(RemainingAccountsInfo {
						slices: instruction.remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
					}),
                        acct_lb_pair: accts[0].to_string(),
                        acct_bin_array_bitmap_extension: accts[1].to_string(),
                        acct_reserve_x: accts[2].to_string(),
                        acct_reserve_y: accts[3].to_string(),
                        acct_user_token_in: accts[4].to_string(),
                        acct_user_token_out: accts[5].to_string(),
                        acct_token_x_mint: accts[6].to_string(),
                        acct_token_y_mint: accts[7].to_string(),
                        acct_oracle: accts[8].to_string(),
                        acct_host_fee_in: accts[9].to_string(),
                        acct_user: accts[10].to_string(),
                        acct_token_x_program: accts[11].to_string(),
                        acct_token_y_program: accts[12].to_string(),
                        acct_memo_program: accts[13].to_string(),
                        acct_event_authority: accts[14].to_string(),
                        acct_program: accts[15].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClosePosition2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClosePosition2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    close_position2_instruction_list.push(ClosePosition2Instruction {
                        trx_hash: transaction.id(),
                        acct_position: accts[0].to_string(),
                        acct_sender: accts[1].to_string(),
                        acct_rent_receiver: accts[2].to_string(),
                        acct_event_authority: accts[3].to_string(),
                        acct_program: accts[4].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateFeesAndReward2::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateFeesAndReward2::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_fees_and_reward2_instruction_list.push(UpdateFeesAndReward2Instruction {
                        trx_hash: transaction.id(),
                        min_bin_id: instruction.min_bin_id,
                        max_bin_id: instruction.max_bin_id,
                        acct_position: accts[0].to_string(),
                        acct_lb_pair: accts[1].to_string(),
                        acct_owner: accts[2].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClosePositionIfEmpty::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClosePositionIfEmpty::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    close_position_if_empty_instruction_list.push(ClosePositionIfEmptyInstruction {
                        trx_hash: transaction.id(),
                        acct_position: accts[0].to_string(),
                        acct_sender: accts[1].to_string(),
                        acct_rent_receiver: accts[2].to_string(),
                        acct_event_authority: accts[3].to_string(),
                        acct_program: accts[4].to_string(),
                    });
                }
            }
        });
    });


    Data {
        composition_fee_event_list,
        add_liquidity_event_list,
        remove_liquidity_event_list,
        swap_event_list,
        claim_reward_event_list,
        fund_reward_event_list,
        initialize_reward_event_list,
        update_reward_duration_event_list,
        update_reward_funder_event_list,
        position_close_event_list,
        claim_fee_event_list,
        lb_pair_create_event_list,
        position_create_event_list,
        increase_position_length_event_list,
        decrease_position_length_event_list,
        fee_parameter_update_event_list,
        dynamic_fee_parameter_update_event_list,
        increase_observation_event_list,
        withdraw_ineligible_reward_event_list,
        update_position_operator_event_list,
        update_position_lock_release_point_event_list,
        go_to_a_bin_event_list,
        initialize_lb_pair_instruction_list,
        initialize_permission_lb_pair_instruction_list,
        initialize_customizable_permissionless_lb_pair_instruction_list,
        initialize_bin_array_bitmap_extension_instruction_list,
        initialize_bin_array_instruction_list,
        add_liquidity_instruction_list,
        add_liquidity_by_weight_instruction_list,
        add_liquidity_by_strategy_instruction_list,
        add_liquidity_by_strategy_one_side_instruction_list,
        add_liquidity_one_side_instruction_list,
        remove_liquidity_instruction_list,
        initialize_position_instruction_list,
        initialize_position_pda_instruction_list,
        initialize_position_by_operator_instruction_list,
        update_position_operator_instruction_list,
        swap_instruction_list,
        swap_exact_out_instruction_list,
        swap_with_price_impact_instruction_list,
        withdraw_protocol_fee_instruction_list,
        initialize_reward_instruction_list,
        fund_reward_instruction_list,
        update_reward_funder_instruction_list,
        update_reward_duration_instruction_list,
        claim_reward_instruction_list,
        claim_fee_instruction_list,
        close_position_instruction_list,
        update_base_fee_parameters_instruction_list,
        update_dynamic_fee_parameters_instruction_list,
        increase_oracle_length_instruction_list,
        initialize_preset_parameter_instruction_list,
        close_preset_parameter_instruction_list,
        close_preset_parameter2_instruction_list,
        remove_all_liquidity_instruction_list,
        set_pair_status_instruction_list,
        migrate_position_instruction_list,
        migrate_bin_array_instruction_list,
        update_fees_and_rewards_instruction_list,
        withdraw_ineligible_reward_instruction_list,
        set_activation_point_instruction_list,
        remove_liquidity_by_range_instruction_list,
        add_liquidity_one_side_precise_instruction_list,
        go_to_a_bin_instruction_list,
        set_pre_activation_duration_instruction_list,
        set_pre_activation_swap_address_instruction_list,
        set_pair_status_permissionless_instruction_list,
        initialize_token_badge_instruction_list,
        create_claim_protocol_fee_operator_instruction_list,
        close_claim_protocol_fee_operator_instruction_list,
        initialize_preset_parameter2_instruction_list,
        initialize_lb_pair2_instruction_list,
        initialize_customizable_permissionless_lb_pair2_instruction_list,
        claim_fee2_instruction_list,
        claim_reward2_instruction_list,
        add_liquidity2_instruction_list,
        add_liquidity_by_strategy2_instruction_list,
        add_liquidity_one_side_precise2_instruction_list,
        remove_liquidity2_instruction_list,
        remove_liquidity_by_range2_instruction_list,
        swap2_instruction_list,
        swap_exact_out2_instruction_list,
        swap_with_price_impact2_instruction_list,
        close_position2_instruction_list,
        update_fees_and_reward2_instruction_list,
        close_position_if_empty_instruction_list,
    }
}



fn map_option_init_preset_parameters2_ix(value: Option<idl::idl::program::types::InitPresetParameters2Ix>) -> Option<InitPresetParameters2Ix> {
    match value {
        Some(init_preset_parameters2_ix) => {
            return Some(InitPresetParameters2Ix {
                index: init_preset_parameters2_ix.index as u64,
                bin_step: init_preset_parameters2_ix.bin_step as u64,
                base_factor: init_preset_parameters2_ix.base_factor as u64,
                filter_period: init_preset_parameters2_ix.filter_period as u64,
                decay_period: init_preset_parameters2_ix.decay_period as u64,
                reduction_factor: init_preset_parameters2_ix.reduction_factor as u64,
                variable_fee_control: init_preset_parameters2_ix.variable_fee_control,
                max_volatility_accumulator: init_preset_parameters2_ix.max_volatility_accumulator,
                protocol_share: init_preset_parameters2_ix.protocol_share as u64,
                base_fee_power_factor: init_preset_parameters2_ix.base_fee_power_factor as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_init_preset_parameters_ix(value: Option<idl::idl::program::types::InitPresetParametersIx>) -> Option<InitPresetParametersIx> {
    match value {
        Some(init_preset_parameters_ix) => {
            return Some(InitPresetParametersIx {
                bin_step: init_preset_parameters_ix.bin_step as u64,
                base_factor: init_preset_parameters_ix.base_factor as u64,
                filter_period: init_preset_parameters_ix.filter_period as u64,
                decay_period: init_preset_parameters_ix.decay_period as u64,
                reduction_factor: init_preset_parameters_ix.reduction_factor as u64,
                variable_fee_control: init_preset_parameters_ix.variable_fee_control,
                max_volatility_accumulator: init_preset_parameters_ix.max_volatility_accumulator,
                protocol_share: init_preset_parameters_ix.protocol_share as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_base_fee_parameter(value: Option<idl::idl::program::types::BaseFeeParameter>) -> Option<BaseFeeParameter> {
    match value {
        Some(base_fee_parameter) => {
            return Some(BaseFeeParameter {
                protocol_share: base_fee_parameter.protocol_share as u64,
                base_factor: base_fee_parameter.base_factor as u64,
                base_fee_power_factor: base_fee_parameter.base_fee_power_factor as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_dynamic_fee_parameter(value: Option<idl::idl::program::types::DynamicFeeParameter>) -> Option<DynamicFeeParameter> {
    match value {
        Some(dynamic_fee_parameter) => {
            return Some(DynamicFeeParameter {
                filter_period: dynamic_fee_parameter.filter_period as u64,
                decay_period: dynamic_fee_parameter.decay_period as u64,
                reduction_factor: dynamic_fee_parameter.reduction_factor as u64,
                variable_fee_control: dynamic_fee_parameter.variable_fee_control,
                max_volatility_accumulator: dynamic_fee_parameter.max_volatility_accumulator,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_liquidity_parameter_by_strategy_one_side(value: Option<idl::idl::program::types::LiquidityParameterByStrategyOneSide>) -> Option<LiquidityParameterByStrategyOneSide> {
    match value {
        Some(liquidity_parameter_by_strategy_one_side) => {
            return Some(LiquidityParameterByStrategyOneSide {
                amount: liquidity_parameter_by_strategy_one_side.amount,
                active_id: liquidity_parameter_by_strategy_one_side.active_id,
                max_active_bin_slippage: liquidity_parameter_by_strategy_one_side.max_active_bin_slippage,
                strategy_parameters: Some(StrategyParameters {
						min_bin_id: liquidity_parameter_by_strategy_one_side.strategy_parameters.min_bin_id,max_bin_id: liquidity_parameter_by_strategy_one_side.strategy_parameters.max_bin_id,strategy_type: map_enum_strategy_type(liquidity_parameter_by_strategy_one_side.strategy_parameters.strategy_type),parameteres: liquidity_parameter_by_strategy_one_side.strategy_parameters.parameteres.into_iter().map(|f| f as u64).collect(),
					}),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_liquidity_parameter_by_strategy(value: Option<idl::idl::program::types::LiquidityParameterByStrategy>) -> Option<LiquidityParameterByStrategy> {
    match value {
        Some(liquidity_parameter_by_strategy) => {
            return Some(LiquidityParameterByStrategy {
                amount_x: liquidity_parameter_by_strategy.amount_x,
                amount_y: liquidity_parameter_by_strategy.amount_y,
                active_id: liquidity_parameter_by_strategy.active_id,
                max_active_bin_slippage: liquidity_parameter_by_strategy.max_active_bin_slippage,
                strategy_parameters: Some(StrategyParameters {
						min_bin_id: liquidity_parameter_by_strategy.strategy_parameters.min_bin_id,max_bin_id: liquidity_parameter_by_strategy.strategy_parameters.max_bin_id,strategy_type: map_enum_strategy_type(liquidity_parameter_by_strategy.strategy_parameters.strategy_type),parameteres: liquidity_parameter_by_strategy.strategy_parameters.parameteres.into_iter().map(|f| f as u64).collect(),
					}),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_strategy_parameters(value: Option<idl::idl::program::types::StrategyParameters>) -> Option<StrategyParameters> {
    match value {
        Some(strategy_parameters) => {
            return Some(StrategyParameters {
                min_bin_id: strategy_parameters.min_bin_id,
                max_bin_id: strategy_parameters.max_bin_id,
                strategy_type: map_enum_strategy_type(strategy_parameters.strategy_type),
                parameteres: strategy_parameters.parameteres.into_iter().map(|f| f as u64).collect(),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_liquidity_one_side_parameter(value: Option<idl::idl::program::types::LiquidityOneSideParameter>) -> Option<LiquidityOneSideParameter> {
    match value {
        Some(liquidity_one_side_parameter) => {
            return Some(LiquidityOneSideParameter {
                amount: liquidity_one_side_parameter.amount,
                active_id: liquidity_one_side_parameter.active_id,
                max_active_bin_slippage: liquidity_one_side_parameter.max_active_bin_slippage,
                bin_liquidity_dist: liquidity_one_side_parameter.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistributionByWeight {
						bin_id: bin_liquidity_dist.bin_id,weight: bin_liquidity_dist.weight as u64,
					}).collect(),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_bin_liquidity_distribution_by_weight(value: Option<idl::idl::program::types::BinLiquidityDistributionByWeight>) -> Option<BinLiquidityDistributionByWeight> {
    match value {
        Some(bin_liquidity_distribution_by_weight) => {
            return Some(BinLiquidityDistributionByWeight {
                bin_id: bin_liquidity_distribution_by_weight.bin_id,
                weight: bin_liquidity_distribution_by_weight.weight as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_liquidity_parameter_by_weight(value: Option<idl::idl::program::types::LiquidityParameterByWeight>) -> Option<LiquidityParameterByWeight> {
    match value {
        Some(liquidity_parameter_by_weight) => {
            return Some(LiquidityParameterByWeight {
                amount_x: liquidity_parameter_by_weight.amount_x,
                amount_y: liquidity_parameter_by_weight.amount_y,
                active_id: liquidity_parameter_by_weight.active_id,
                max_active_bin_slippage: liquidity_parameter_by_weight.max_active_bin_slippage,
                bin_liquidity_dist: liquidity_parameter_by_weight.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistributionByWeight {
						bin_id: bin_liquidity_dist.bin_id,weight: bin_liquidity_dist.weight as u64,
					}).collect(),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_add_liquidity_single_side_precise_parameter(value: Option<idl::idl::program::types::AddLiquiditySingleSidePreciseParameter>) -> Option<AddLiquiditySingleSidePreciseParameter> {
    match value {
        Some(add_liquidity_single_side_precise_parameter) => {
            return Some(AddLiquiditySingleSidePreciseParameter {
                bins: add_liquidity_single_side_precise_parameter.bins.into_iter().map(|bins| CompressedBinDepositAmount {
						bin_id: bins.bin_id,amount: bins.amount,
					}).collect(),
                decompress_multiplier: add_liquidity_single_side_precise_parameter.decompress_multiplier,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_compressed_bin_deposit_amount(value: Option<idl::idl::program::types::CompressedBinDepositAmount>) -> Option<CompressedBinDepositAmount> {
    match value {
        Some(compressed_bin_deposit_amount) => {
            return Some(CompressedBinDepositAmount {
                bin_id: compressed_bin_deposit_amount.bin_id,
                amount: compressed_bin_deposit_amount.amount,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_bin_liquidity_distribution(value: Option<idl::idl::program::types::BinLiquidityDistribution>) -> Option<BinLiquidityDistribution> {
    match value {
        Some(bin_liquidity_distribution) => {
            return Some(BinLiquidityDistribution {
                bin_id: bin_liquidity_distribution.bin_id,
                distribution_x: bin_liquidity_distribution.distribution_x as u64,
                distribution_y: bin_liquidity_distribution.distribution_y as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_liquidity_parameter(value: Option<idl::idl::program::types::LiquidityParameter>) -> Option<LiquidityParameter> {
    match value {
        Some(liquidity_parameter) => {
            return Some(LiquidityParameter {
                amount_x: liquidity_parameter.amount_x,
                amount_y: liquidity_parameter.amount_y,
                bin_liquidity_dist: liquidity_parameter.bin_liquidity_dist.into_iter().map(|bin_liquidity_dist| BinLiquidityDistribution {
						bin_id: bin_liquidity_dist.bin_id,distribution_x: bin_liquidity_dist.distribution_x as u64,distribution_y: bin_liquidity_dist.distribution_y as u64,
					}).collect(),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_customizable_params(value: Option<idl::idl::program::types::CustomizableParams>) -> Option<CustomizableParams> {
    match value {
        Some(customizable_params) => {
            return Some(CustomizableParams {
                active_id: customizable_params.active_id,
                bin_step: customizable_params.bin_step as u64,
                base_factor: customizable_params.base_factor as u64,
                activation_type: customizable_params.activation_type as u64,
                has_alpha_vault: customizable_params.has_alpha_vault,
                activation_point: customizable_params.activation_point,
                creator_pool_on_off_control: customizable_params.creator_pool_on_off_control,
                base_fee_power_factor: customizable_params.base_fee_power_factor as u64,
                padding: customizable_params.padding.into_iter().map(|f| f as u64).collect(),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_init_permission_pair_ix(value: Option<idl::idl::program::types::InitPermissionPairIx>) -> Option<InitPermissionPairIx> {
    match value {
        Some(init_permission_pair_ix) => {
            return Some(InitPermissionPairIx {
                active_id: init_permission_pair_ix.active_id,
                bin_step: init_permission_pair_ix.bin_step as u64,
                base_factor: init_permission_pair_ix.base_factor as u64,
                base_fee_power_factor: init_permission_pair_ix.base_fee_power_factor as u64,
                activation_type: init_permission_pair_ix.activation_type as u64,
                protocol_share: init_permission_pair_ix.protocol_share as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_add_liquidity_single_side_precise_parameter2(value: Option<idl::idl::program::types::AddLiquiditySingleSidePreciseParameter2>) -> Option<AddLiquiditySingleSidePreciseParameter2> {
    match value {
        Some(add_liquidity_single_side_precise_parameter2) => {
            return Some(AddLiquiditySingleSidePreciseParameter2 {
                bins: add_liquidity_single_side_precise_parameter2.bins.into_iter().map(|bins| CompressedBinDepositAmount {
						bin_id: bins.bin_id,amount: bins.amount,
					}).collect(),
                decompress_multiplier: add_liquidity_single_side_precise_parameter2.decompress_multiplier,
                max_amount: add_liquidity_single_side_precise_parameter2.max_amount,
            })
        },
        None => {
            return None;
        }
    }
}





fn map_option_initialize_lb_pair2_params(value: Option<idl::idl::program::types::InitializeLbPair2Params>) -> Option<InitializeLbPair2Params> {
    match value {
        Some(initialize_lb_pair2_params) => {
            return Some(InitializeLbPair2Params {
                active_id: initialize_lb_pair2_params.active_id,
                padding: initialize_lb_pair2_params.padding.into_iter().map(|f| f as u64).collect(),
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_bin_liquidity_reduction(value: Option<idl::idl::program::types::BinLiquidityReduction>) -> Option<BinLiquidityReduction> {
    match value {
        Some(bin_liquidity_reduction) => {
            return Some(BinLiquidityReduction {
                bin_id: bin_liquidity_reduction.bin_id,
                bps_to_remove: bin_liquidity_reduction.bps_to_remove as u64,
            })
        },
        None => {
            return None;
        }
    }
}



















fn map_option_remaining_accounts_slice(value: Option<idl::idl::program::types::RemainingAccountsSlice>) -> Option<RemainingAccountsSlice> {
    match value {
        Some(remaining_accounts_slice) => {
            return Some(RemainingAccountsSlice {
                accounts_type: map_enum_accounts_type(remaining_accounts_slice.accounts_type),
                length: remaining_accounts_slice.length as u64,
            })
        },
        None => {
            return None;
        }
    }
}



fn map_option_remaining_accounts_info(value: Option<idl::idl::program::types::RemainingAccountsInfo>) -> Option<RemainingAccountsInfo> {
    match value {
        Some(remaining_accounts_info) => {
            return Some(RemainingAccountsInfo {
                slices: remaining_accounts_info.slices.into_iter().map(|slices| RemainingAccountsSlice {
						accounts_type: map_enum_accounts_type(slices.accounts_type),length: slices.length as u64,
					}).collect(),
            })
        },
        None => {
            return None;
        }
    }
}

fn map_enum_strategy_type(value: idl::idl::program::types::StrategyType) -> i32 {
    match value {
        idl::idl::program::types::StrategyType::SpotOneSide => return 0,
        idl::idl::program::types::StrategyType::CurveOneSide => return 1,
        idl::idl::program::types::StrategyType::BidAskOneSide => return 2,
        idl::idl::program::types::StrategyType::SpotBalanced => return 3,
        idl::idl::program::types::StrategyType::CurveBalanced => return 4,
        idl::idl::program::types::StrategyType::BidAskBalanced => return 5,
        idl::idl::program::types::StrategyType::SpotImBalanced => return 6,
        idl::idl::program::types::StrategyType::CurveImBalanced => return 7,
        idl::idl::program::types::StrategyType::BidAskImBalanced => return 8,
        _ => 0,
    }
}
fn map_enum_rounding(value: idl::idl::program::types::Rounding) -> i32 {
    match value {
        idl::idl::program::types::Rounding::Up => return 0,
        idl::idl::program::types::Rounding::Down => return 1,
        _ => 0,
    }
}
fn map_enum_activation_type(value: idl::idl::program::types::ActivationType) -> i32 {
    match value {
        idl::idl::program::types::ActivationType::Slot => return 0,
        idl::idl::program::types::ActivationType::Timestamp => return 1,
        _ => 0,
    }
}
fn map_enum_layout_version(value: idl::idl::program::types::LayoutVersion) -> i32 {
    match value {
        idl::idl::program::types::LayoutVersion::V0 => return 0,
        idl::idl::program::types::LayoutVersion::V1 => return 1,
        _ => 0,
    }
}
fn map_enum_pair_type(value: idl::idl::program::types::PairType) -> i32 {
    match value {
        idl::idl::program::types::PairType::Permissionless => return 0,
        idl::idl::program::types::PairType::Permission => return 1,
        idl::idl::program::types::PairType::CustomizablePermissionless => return 2,
        idl::idl::program::types::PairType::PermissionlessV2 => return 3,
        _ => 0,
    }
}
fn map_enum_pair_status(value: idl::idl::program::types::PairStatus) -> i32 {
    match value {
        idl::idl::program::types::PairStatus::Enabled => return 0,
        idl::idl::program::types::PairStatus::Disabled => return 1,
        _ => 0,
    }
}
fn map_enum_token_program_flags(value: idl::idl::program::types::TokenProgramFlags) -> i32 {
    match value {
        idl::idl::program::types::TokenProgramFlags::TokenProgram => return 0,
        idl::idl::program::types::TokenProgramFlags::TokenProgram2022 => return 1,
        _ => 0,
    }
}
fn map_enum_accounts_type(value: idl::idl::program::types::AccountsType) -> i32 {
    match value {
        idl::idl::program::types::AccountsType::TransferHookX => return 0,
        idl::idl::program::types::AccountsType::TransferHookY => return 1,
        idl::idl::program::types::AccountsType::TransferHookReward => return 2,
        _ => 0,
    }
}






















