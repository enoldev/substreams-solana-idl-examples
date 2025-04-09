mod idl;
mod pb;

use anchor_lang::prelude::Pubkey;
use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use idl::idl::program::types::ProgramStatus;
use pb::substreams::v1::program::ClaimEventEvent;
use pb::substreams::v1::program::ClaimVestedTokensInstruction;
use pb::substreams::v1::program::CompleteEventEvent;
use pb::substreams::v1::program::CreateBondingCurveInstruction;
use pb::substreams::v1::program::CreateEventEvent;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::GetClaimableTokensInstruction;
use pb::substreams::v1::program::GlobalUpdateEventEvent;
use pb::substreams::v1::program::InitializeInstruction;
use pb::substreams::v1::program::MigrateInstruction;
use pb::substreams::v1::program::SetConfigInstruction;
use pb::substreams::v1::program::SwapInstruction;
use pb::substreams::v1::program::TradeEventEvent;
use pb::substreams::v1::program::VestingEventEvent;
use pb::substreams::v1::program::WithdrawEventEvent;

use pb::substreams::v1::program::CreateBondingCurveParams;

use pb::substreams::v1::program::GlobalSettingsInput;

use pb::substreams::v1::program::ProgramStatusEnum;

use pb::substreams::v1::program::SwapParams;

use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "fso2tMtRazNhfZHRRgDz3XUXwZHPJWZFDQPCfGfQJGm";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut claim_event_event_list: Vec<ClaimEventEvent> = Vec::new();
    let mut complete_event_event_list: Vec<CompleteEventEvent> = Vec::new();
    let mut create_event_event_list: Vec<CreateEventEvent> = Vec::new();
    let mut global_update_event_event_list: Vec<GlobalUpdateEventEvent> = Vec::new();
    let mut trade_event_event_list: Vec<TradeEventEvent> = Vec::new();
    let mut vesting_event_event_list: Vec<VestingEventEvent> = Vec::new();
    let mut withdraw_event_event_list: Vec<WithdrawEventEvent> = Vec::new();
    let mut claim_vested_tokens_instruction_list: Vec<ClaimVestedTokensInstruction> = Vec::new();
    let mut create_bonding_curve_instruction_list: Vec<CreateBondingCurveInstruction> = Vec::new();
    let mut get_claimable_tokens_instruction_list: Vec<GetClaimableTokensInstruction> = Vec::new();
    let mut initialize_instruction_list: Vec<InitializeInstruction> = Vec::new();
    let mut migrate_instruction_list: Vec<MigrateInstruction> = Vec::new();
    let mut set_config_instruction_list: Vec<SetConfigInstruction> = Vec::new();
    let mut swap_instruction_list: Vec<SwapInstruction> = Vec::new();

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
                        let static_discriminator_slice: &'static [u8] =
                            Box::leak(Box::new(slice_discriminator));

                        match static_discriminator_slice {
                            idl::idl::program::events::ClaimEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::ClaimEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    claim_event_event_list.push(ClaimEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            idl::idl::program::events::CompleteEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::CompleteEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    complete_event_event_list.push(CompleteEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            idl::idl::program::events::CreateEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::CreateEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    create_event_event_list.push(CreateEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            idl::idl::program::events::GlobalUpdateEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::GlobalUpdateEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    global_update_event_event_list.push(GlobalUpdateEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            idl::idl::program::events::TradeEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::TradeEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    trade_event_event_list.push(TradeEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            idl::idl::program::events::VestingEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::VestingEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    vesting_event_event_list.push(VestingEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            idl::idl::program::events::WithdrawEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::WithdrawEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    withdraw_event_event_list.push(WithdrawEventEvent {
                                        trx_hash: transaction.id(),
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
                });
            }); // ------------- INSTRUCTIONS -------------
        transaction
            .walk_instructions()
            .into_iter()
            .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
            .for_each(|inst| {
                let slice_u8: &[u8] = &inst.data()[..];
                if &slice_u8[0..8]
                    == idl::idl::program::client::args::ClaimVestedTokens::DISCRIMINATOR
                {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::ClaimVestedTokens::deserialize(
                            &mut &slice_u8[8..],
                        )
                    {
                        let accts = inst.accounts();
                        claim_vested_tokens_instruction_list.push(ClaimVestedTokensInstruction {
                            trx_hash: transaction.id(),
                            acct_global: accts[0].to_string(),
                            acct_bonding_curve: accts[1].to_string(),
                            acct_creator: accts[2].to_string(),
                            acct_creator_token_account: accts[3].to_string(),
                            acct_mint: accts[4].to_string(),
                            acct_bonding_curve_token_account: accts[5].to_string(),
                            acct_event_authority: accts[7].to_string(),
                            acct_program: accts[8].to_string(),
                        });
                    }
                }
                if &slice_u8[0..8]
                    == idl::idl::program::client::args::CreateBondingCurve::DISCRIMINATOR
                {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::CreateBondingCurve::deserialize(
                            &mut &slice_u8[8..],
                        )
                    {
                        let accts = inst.accounts();
                        create_bonding_curve_instruction_list.push(CreateBondingCurveInstruction {
                            trx_hash: transaction.id(),
                            params: Some(CreateBondingCurveParams {
                                name: instruction.params.name,
                                symbol: instruction.params.symbol,
                                uri: instruction.params.uri,
                                start_time: instruction.params.start_time,
                            }),
                            acct_mint: accts[0].to_string(),
                            acct_creator: accts[1].to_string(),
                            acct_fren_mint: accts[2].to_string(),
                            acct_bonding_curve: accts[3].to_string(),
                            acct_bonding_curve_token_account: accts[4].to_string(),
                            acct_bonding_curve_fren_account: accts[5].to_string(),
                            acct_global: accts[6].to_string(),
                            acct_whitelist: accts[7].to_string(),
                            acct_metadata: accts[8].to_string(),
                            acct_system_program: accts[9].to_string(),
                            acct_associated_token_program: accts[11].to_string(),
                            acct_token_metadata_program: accts[12].to_string(),
                            acct_rent: accts[13].to_string(),
                            acct_event_authority: accts[14].to_string(),
                            acct_program: accts[15].to_string(),
                        });
                    }
                }
                if &slice_u8[0..8]
                    == idl::idl::program::client::args::GetClaimableTokens::DISCRIMINATOR
                {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::GetClaimableTokens::deserialize(
                            &mut &slice_u8[8..],
                        )
                    {
                        let accts = inst.accounts();
                        get_claimable_tokens_instruction_list.push(GetClaimableTokensInstruction {
                            trx_hash: transaction.id(),
                            acct_bonding_curve: accts[0].to_string(),
                            acct_creator: accts[1].to_string(),
                        });
                    }
                }
                if &slice_u8[0..8] == idl::idl::program::client::args::Initialize::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::Initialize::deserialize(
                            &mut &slice_u8[8..],
                        )
                    {
                        let accts = inst.accounts();
                        initialize_instruction_list.push(InitializeInstruction {
                            trx_hash: transaction.id(),
                            params: Some(GlobalSettingsInput {
                                initial_virtual_token_reserves: instruction
                                    .params
                                    .initial_virtual_token_reserves,
                                initial_virtual_sol_reserves: instruction
                                    .params
                                    .initial_virtual_sol_reserves,
                                initial_real_token_reserves: instruction
                                    .params
                                    .initial_real_token_reserves,
                                token_total_supply: instruction.params.token_total_supply,
                                creator_reserves: instruction.params.creator_reserves,
                                escrow_reserves: instruction.params.escrow_reserves,
                                mint_decimals: map_option_u8(instruction.params.mint_decimals),
                                migrate_fee_amount: instruction.params.migrate_fee_amount,
                                cliff_duration: instruction.params.cliff_duration,
                                vesting_duration: instruction.params.vesting_duration,
                                fee_receiver: map_option_pub_key(instruction.params.fee_receiver),
                                status: map_option_program_status(instruction.params.status),
                                whitelist_enabled: instruction.params.whitelist_enabled,
                                raydiyum_config: map_option_pub_key(
                                    instruction.params.raydiyum_config,
                                ),
                            }),
                            acct_authority: accts[0].to_string(),
                            acct_global: accts[1].to_string(),
                            acct_event_authority: accts[3].to_string(),
                            acct_program: accts[4].to_string(),
                        });
                    }
                }
                if &slice_u8[0..8] == idl::idl::program::client::args::Migrate::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::Migrate::deserialize(&mut &slice_u8[8..])
                    {
                        let accts = inst.accounts();
                        migrate_instruction_list.push(MigrateInstruction {
                            trx_hash: transaction.id(),
                            acct_global: accts[0].to_string(),
                            acct_bonding_curve: accts[1].to_string(),
                            acct_mint: accts[2].to_string(),
                            acct_bonding_curve_token_account: accts[3].to_string(),
                            acct_bonding_curve_fren_account: accts[4].to_string(),
                            acct_migration_vault: accts[5].to_string(),
                            acct_creator: accts[7].to_string(),
                            acct_amm_config: accts[8].to_string(),
                            acct_authority: accts[9].to_string(),
                            acct_pool_state: accts[10].to_string(),
                            acct_token_0_mint: accts[11].to_string(),
                            acct_token_1_mint: accts[12].to_string(),
                            acct_lp_mint: accts[13].to_string(),
                            acct_creator_token_0: accts[14].to_string(),
                            acct_creator_token_1: accts[15].to_string(),
                            acct_creator_lp_token: accts[16].to_string(),
                            acct_token_0_vault: accts[17].to_string(),
                            acct_token_1_vault: accts[18].to_string(),
                            acct_observation_state: accts[20].to_string(),
                            acct_token_0_program: accts[22].to_string(),
                            acct_token_1_program: accts[23].to_string(),
                            acct_event_authority: accts[27].to_string(),
                            acct_program: accts[28].to_string(),
                        });
                    }
                }
                if &slice_u8[0..8] == idl::idl::program::client::args::SetConfig::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::SetConfig::deserialize(&mut &slice_u8[8..])
                    {
                        let accts = inst.accounts();
                        set_config_instruction_list.push(SetConfigInstruction {
                            trx_hash: transaction.id(),
                            params: Some(GlobalSettingsInput {
                                initial_virtual_token_reserves: instruction
                                    .params
                                    .initial_virtual_token_reserves,
                                initial_virtual_sol_reserves: instruction
                                    .params
                                    .initial_virtual_sol_reserves,
                                initial_real_token_reserves: instruction
                                    .params
                                    .initial_real_token_reserves,
                                token_total_supply: instruction.params.token_total_supply,
                                creator_reserves: instruction.params.creator_reserves,
                                escrow_reserves: instruction.params.escrow_reserves,
                                mint_decimals: map_option_u8(instruction.params.mint_decimals),
                                migrate_fee_amount: instruction.params.migrate_fee_amount,
                                cliff_duration: instruction.params.cliff_duration,
                                vesting_duration: instruction.params.vesting_duration,
                                fee_receiver: map_option_pub_key(instruction.params.fee_receiver),
                                status: map_option_program_status(instruction.params.status),
                                whitelist_enabled: instruction.params.whitelist_enabled,
                                raydiyum_config: map_option_pub_key(
                                    instruction.params.raydiyum_config,
                                ),
                            }),
                            acct_authority: accts[0].to_string(),
                            acct_global: accts[1].to_string(),
                            acct_new_authority: accts[2].to_string(),
                            acct_new_migration_authority: accts[3].to_string(),
                            acct_event_authority: accts[5].to_string(),
                            acct_program: accts[6].to_string(),
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
                            params: Some(SwapParams {
                                base_in: instruction.params.base_in,
                                exact_in_amount: instruction.params.exact_in_amount,
                                min_out_amount: instruction.params.min_out_amount,
                            }),
                            acct_user: accts[0].to_string(),
                            acct_global: accts[1].to_string(),
                            acct_fee_receiver: accts[2].to_string(),
                            acct_mint: accts[3].to_string(),
                            acct_fren_mint: accts[4].to_string(),
                            acct_bonding_curve: accts[5].to_string(),
                            acct_bonding_curve_token_account: accts[6].to_string(),
                            acct_bonding_curve_fren_account: accts[7].to_string(),
                            acct_user_token_account: accts[8].to_string(),
                            acct_user_fren_account: accts[9].to_string(),
                            acct_fee_receiver_fren_account: accts[10].to_string(),
                            acct_event_authority: accts[15].to_string(),
                            acct_program: accts[16].to_string(),
                        });
                    }
                }
            });
    });

    Data {
        claim_event_event_list,
        complete_event_event_list,
        create_event_event_list,
        global_update_event_event_list,
        trade_event_event_list,
        vesting_event_event_list,
        withdraw_event_event_list,
        claim_vested_tokens_instruction_list,
        create_bonding_curve_instruction_list,
        get_claimable_tokens_instruction_list,
        initialize_instruction_list,
        migrate_instruction_list,
        set_config_instruction_list,
        swap_instruction_list,
    }
}

fn map_option_create_bonding_curve_params(
    value: Option<idl::idl::program::types::CreateBondingCurveParams>,
) -> Option<CreateBondingCurveParams> {
    match value {
        Some(create_bonding_curve_params) => {
            return Some(CreateBondingCurveParams {
                name: create_bonding_curve_params.name,
                symbol: create_bonding_curve_params.symbol,
                uri: create_bonding_curve_params.uri,
                start_time: create_bonding_curve_params.start_time,
            })
        }
        None => {
            return None;
        }
    }
}

fn map_option_global_settings_input(
    value: Option<idl::idl::program::types::GlobalSettingsInput>,
) -> Option<GlobalSettingsInput> {
    match value {
        Some(global_settings_input) => {
            return Some(GlobalSettingsInput {
                initial_virtual_token_reserves: global_settings_input
                    .initial_virtual_token_reserves,
                initial_virtual_sol_reserves: global_settings_input.initial_virtual_sol_reserves,
                initial_real_token_reserves: global_settings_input.initial_real_token_reserves,
                token_total_supply: global_settings_input.token_total_supply,
                creator_reserves: global_settings_input.creator_reserves,
                escrow_reserves: global_settings_input.escrow_reserves,
                mint_decimals: map_option_u8(global_settings_input.mint_decimals),
                migrate_fee_amount: global_settings_input.migrate_fee_amount,
                cliff_duration: global_settings_input.cliff_duration,
                vesting_duration: global_settings_input.vesting_duration,
                fee_receiver: map_option_pub_key(global_settings_input.fee_receiver),
                status: map_option_program_status(global_settings_input.status),
                whitelist_enabled: global_settings_input.whitelist_enabled,
                raydiyum_config: map_option_pub_key(global_settings_input.raydiyum_config),
            })
        }
        None => {
            return None;
        }
    }
}

fn map_option_u8(option: Option<u8>) -> Option<u64> {
    match option {
        Some(value) => Some(value as u64),
        None => None,
    }
}

fn map_option_pub_key(option: Option<Pubkey>) -> Option<String> {
    match option {
        Some(value) => Some(value.to_string()),
        None => None,
    }
}

fn map_option_program_status(option: Option<ProgramStatus>) -> Option<i32> {
    match option {
        Some(value) => Some(map_enum_program_status(value)),
        None => None,
    }
}

fn map_enum_program_status(value: idl::idl::program::types::ProgramStatus) -> i32 {
    match value {
        idl::idl::program::types::ProgramStatus::Running => return 0,
        idl::idl::program::types::ProgramStatus::SwapOnly => return 1,
        idl::idl::program::types::ProgramStatus::SwapOnlyNoLaunch => return 2,
        idl::idl::program::types::ProgramStatus::Paused => return 3,
        _ => 0,
    }
}

fn map_option_swap_params(
    value: Option<idl::idl::program::types::SwapParams>,
) -> Option<SwapParams> {
    match value {
        Some(swap_params) => {
            return Some(SwapParams {
                base_in: swap_params.base_in,
                exact_in_amount: swap_params.exact_in_amount,
                min_out_amount: swap_params.min_out_amount,
            })
        }
        None => {
            return None;
        }
    }
}
