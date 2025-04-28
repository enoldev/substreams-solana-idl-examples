mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::ClaimVestedEventEvent;
use pb::substreams::v1::program::CreateVestingEventEvent;
use pb::substreams::v1::program::PoolCreateEventEvent;
use pb::substreams::v1::program::TradeEventEvent;
use pb::substreams::v1::program::BuyExactInInstruction;
use pb::substreams::v1::program::BuyExactOutInstruction;
use pb::substreams::v1::program::ClaimPlatformFeeInstruction;
use pb::substreams::v1::program::ClaimVestedTokenInstruction;
use pb::substreams::v1::program::CollectFeeInstruction;
use pb::substreams::v1::program::CollectMigrateFeeInstruction;
use pb::substreams::v1::program::CreateConfigInstruction;
use pb::substreams::v1::program::CreatePlatformConfigInstruction;
use pb::substreams::v1::program::CreateVestingAccountInstruction;
use pb::substreams::v1::program::InitializeInstruction;
use pb::substreams::v1::program::MigrateToAmmInstruction;
use pb::substreams::v1::program::MigrateToCpswapInstruction;
use pb::substreams::v1::program::SellExactInInstruction;
use pb::substreams::v1::program::SellExactOutInstruction;
use pb::substreams::v1::program::UpdateConfigInstruction;
use pb::substreams::v1::program::UpdatePlatformConfigInstruction;
use pb::substreams::v1::program::MigrateNftInfo;
use pb::substreams::v1::program::MintParams;
use pb::substreams::v1::program::PlatformParams;
use pb::substreams::v1::program::VestingParams;
use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut claim_vested_event_event_list: Vec<ClaimVestedEventEvent> = Vec::new();
    let mut create_vesting_event_event_list: Vec<CreateVestingEventEvent> = Vec::new();
    let mut pool_create_event_event_list: Vec<PoolCreateEventEvent> = Vec::new();
    let mut trade_event_event_list: Vec<TradeEventEvent> = Vec::new();
    let mut buy_exact_in_instruction_list: Vec<BuyExactInInstruction> = Vec::new();
    let mut buy_exact_out_instruction_list: Vec<BuyExactOutInstruction> = Vec::new();
    let mut claim_platform_fee_instruction_list: Vec<ClaimPlatformFeeInstruction> = Vec::new();
    let mut claim_vested_token_instruction_list: Vec<ClaimVestedTokenInstruction> = Vec::new();
    let mut collect_fee_instruction_list: Vec<CollectFeeInstruction> = Vec::new();
    let mut collect_migrate_fee_instruction_list: Vec<CollectMigrateFeeInstruction> = Vec::new();
    let mut create_config_instruction_list: Vec<CreateConfigInstruction> = Vec::new();
    let mut create_platform_config_instruction_list: Vec<CreatePlatformConfigInstruction> = Vec::new();
    let mut create_vesting_account_instruction_list: Vec<CreateVestingAccountInstruction> = Vec::new();
    let mut initialize_instruction_list: Vec<InitializeInstruction> = Vec::new();
    let mut migrate_to_amm_instruction_list: Vec<MigrateToAmmInstruction> = Vec::new();
    let mut migrate_to_cpswap_instruction_list: Vec<MigrateToCpswapInstruction> = Vec::new();
    let mut sell_exact_in_instruction_list: Vec<SellExactInInstruction> = Vec::new();
    let mut sell_exact_out_instruction_list: Vec<SellExactOutInstruction> = Vec::new();
    let mut update_config_instruction_list: Vec<UpdateConfigInstruction> = Vec::new();
    let mut update_platform_config_instruction_list: Vec<UpdatePlatformConfigInstruction> = Vec::new();

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
                            idl::idl::program::events::ClaimVestedEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::ClaimVestedEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    claim_vested_event_event_list.push(ClaimVestedEventEvent {
                                        trx_hash: transaction.id(),
                                        pool_state: event.pool_state.to_string(),
                                        beneficiary: event.beneficiary.to_string(),
                                        claim_amount: event.claim_amount,
                                    });
                                }
                            }
                            idl::idl::program::events::CreateVestingEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::CreateVestingEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    create_vesting_event_event_list.push(CreateVestingEventEvent {
                                        trx_hash: transaction.id(),
                                        pool_state: event.pool_state.to_string(),
                                        beneficiary: event.beneficiary.to_string(),
                                        share_amount: event.share_amount,
                                    });
                                }
                            }
                            idl::idl::program::events::PoolCreateEvent::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::PoolCreateEvent::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    pool_create_event_event_list.push(PoolCreateEventEvent {
                                        trx_hash: transaction.id(),
                                        pool_state: event.pool_state.to_string(),
                                        creator: event.creator.to_string(),
                                        config: event.config.to_string(),
                                        base_mint_param: Some(MintParams {
						decimals: event.base_mint_param.decimals as u64,name: event.base_mint_param.name,symbol: event.base_mint_param.symbol,uri: event.base_mint_param.uri,
					}),
                                        curve_param: map_enum_curve_params(event.curve_param),
                                        vesting_param: Some(VestingParams {
						total_locked_amount: event.vesting_param.total_locked_amount,cliff_period: event.vesting_param.cliff_period,unlock_period: event.vesting_param.unlock_period,
					}),
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
                                        pool_state: event.pool_state.to_string(),
                                        total_base_sell: event.total_base_sell,
                                        virtual_base: event.virtual_base,
                                        virtual_quote: event.virtual_quote,
                                        real_base_before: event.real_base_before,
                                        real_quote_before: event.real_quote_before,
                                        real_base_after: event.real_base_after,
                                        real_quote_after: event.real_quote_after,
                                        amount_in: event.amount_in,
                                        amount_out: event.amount_out,
                                        protocol_fee: event.protocol_fee,
                                        platform_fee: event.platform_fee,
                                        share_fee: event.share_fee,
                                        trade_direction: map_enum_trade_direction(event.trade_direction),
                                        pool_status: map_enum_pool_status(event.pool_status),
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
            if &slice_u8[8..16] == idl::idl::program::events::ClaimVestedEvent::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::ClaimVestedEvent::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    claim_vested_event_event_list.push(ClaimVestedEventEvent {
                        trx_hash: transaction.id(),
                        pool_state: event.pool_state.to_string(),
                        beneficiary: event.beneficiary.to_string(),
                        claim_amount: event.claim_amount,
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::CreateVestingEvent::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::CreateVestingEvent::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    create_vesting_event_event_list.push(CreateVestingEventEvent {
                        trx_hash: transaction.id(),
                        pool_state: event.pool_state.to_string(),
                        beneficiary: event.beneficiary.to_string(),
                        share_amount: event.share_amount,
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::PoolCreateEvent::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::PoolCreateEvent::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    pool_create_event_event_list.push(PoolCreateEventEvent {
                        trx_hash: transaction.id(),
                        pool_state: event.pool_state.to_string(),
                        creator: event.creator.to_string(),
                        config: event.config.to_string(),
                        base_mint_param: Some(MintParams {
						decimals: event.base_mint_param.decimals as u64,name: event.base_mint_param.name,symbol: event.base_mint_param.symbol,uri: event.base_mint_param.uri,
					}),
                        curve_param: map_enum_curve_params(event.curve_param),
                        vesting_param: Some(VestingParams {
						total_locked_amount: event.vesting_param.total_locked_amount,cliff_period: event.vesting_param.cliff_period,unlock_period: event.vesting_param.unlock_period,
					}),
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::TradeEvent::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::TradeEvent::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    trade_event_event_list.push(TradeEventEvent {
                        trx_hash: transaction.id(),
                        pool_state: event.pool_state.to_string(),
                        total_base_sell: event.total_base_sell,
                        virtual_base: event.virtual_base,
                        virtual_quote: event.virtual_quote,
                        real_base_before: event.real_base_before,
                        real_quote_before: event.real_quote_before,
                        real_base_after: event.real_base_after,
                        real_quote_after: event.real_quote_after,
                        amount_in: event.amount_in,
                        amount_out: event.amount_out,
                        protocol_fee: event.protocol_fee,
                        platform_fee: event.platform_fee,
                        share_fee: event.share_fee,
                        trade_direction: map_enum_trade_direction(event.trade_direction),
                        pool_status: map_enum_pool_status(event.pool_status),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::BuyExactIn::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::BuyExactIn::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    buy_exact_in_instruction_list.push(BuyExactInInstruction {
                        trx_hash: transaction.id(),
                        amount_in: instruction.amount_in,
                        minimum_amount_out: instruction.minimum_amount_out,
                        share_fee_rate: instruction.share_fee_rate,
                        acct_payer: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_global_config: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_pool_state: accts[4].to_string(),
                        acct_user_base_token: accts[5].to_string(),
                        acct_user_quote_token: accts[6].to_string(),
                        acct_base_vault: accts[7].to_string(),
                        acct_quote_vault: accts[8].to_string(),
                        acct_base_token_mint: accts[9].to_string(),
                        acct_quote_token_mint: accts[10].to_string(),
                        acct_base_token_program: accts[11].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::BuyExactOut::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::BuyExactOut::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    buy_exact_out_instruction_list.push(BuyExactOutInstruction {
                        trx_hash: transaction.id(),
                        amount_out: instruction.amount_out,
                        maximum_amount_in: instruction.maximum_amount_in,
                        share_fee_rate: instruction.share_fee_rate,
                        acct_payer: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_global_config: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_pool_state: accts[4].to_string(),
                        acct_user_base_token: accts[5].to_string(),
                        acct_user_quote_token: accts[6].to_string(),
                        acct_base_vault: accts[7].to_string(),
                        acct_quote_vault: accts[8].to_string(),
                        acct_base_token_mint: accts[9].to_string(),
                        acct_quote_token_mint: accts[10].to_string(),
                        acct_base_token_program: accts[11].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClaimPlatformFee::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClaimPlatformFee::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_platform_fee_instruction_list.push(ClaimPlatformFeeInstruction {
                        trx_hash: transaction.id(),
                        acct_platform_fee_wallet: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_pool_state: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_quote_vault: accts[4].to_string(),
                        acct_recipient_token_account: accts[5].to_string(),
                        acct_quote_mint: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::ClaimVestedToken::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::ClaimVestedToken::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    claim_vested_token_instruction_list.push(ClaimVestedTokenInstruction {
                        trx_hash: transaction.id(),
                        acct_beneficiary: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_pool_state: accts[2].to_string(),
                        acct_vesting_record: accts[3].to_string(),
                        acct_base_vault: accts[4].to_string(),
                        acct_user_base_token: accts[5].to_string(),
                        acct_base_token_mint: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CollectFee::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CollectFee::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    collect_fee_instruction_list.push(CollectFeeInstruction {
                        trx_hash: transaction.id(),
                        acct_owner: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_pool_state: accts[2].to_string(),
                        acct_global_config: accts[3].to_string(),
                        acct_quote_vault: accts[4].to_string(),
                        acct_quote_mint: accts[5].to_string(),
                        acct_recipient_token_account: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CollectMigrateFee::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CollectMigrateFee::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    collect_migrate_fee_instruction_list.push(CollectMigrateFeeInstruction {
                        trx_hash: transaction.id(),
                        acct_owner: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_pool_state: accts[2].to_string(),
                        acct_global_config: accts[3].to_string(),
                        acct_quote_vault: accts[4].to_string(),
                        acct_quote_mint: accts[5].to_string(),
                        acct_recipient_token_account: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CreateConfig::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CreateConfig::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    create_config_instruction_list.push(CreateConfigInstruction {
                        trx_hash: transaction.id(),
                        curve_type: instruction.curve_type as u64,
                        index: instruction.index as u64,
                        migrate_fee: instruction.migrate_fee,
                        trade_fee_rate: instruction.trade_fee_rate,
                        acct_global_config: accts[1].to_string(),
                        acct_quote_token_mint: accts[2].to_string(),
                        acct_protocol_fee_owner: accts[3].to_string(),
                        acct_migrate_fee_owner: accts[4].to_string(),
                        acct_migrate_to_amm_wallet: accts[5].to_string(),
                        acct_migrate_to_cpswap_wallet: accts[6].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CreatePlatformConfig::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CreatePlatformConfig::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    create_platform_config_instruction_list.push(CreatePlatformConfigInstruction {
                        trx_hash: transaction.id(),
                        platform_params: Some(PlatformParams {
						migrate_nft_info: Some(MigrateNftInfo {
						platform_scale: instruction.platform_params.migrate_nft_info.platform_scale,creator_scale: instruction.platform_params.migrate_nft_info.creator_scale,burn_scale: instruction.platform_params.migrate_nft_info.burn_scale,
					}),fee_rate: instruction.platform_params.fee_rate,name: instruction.platform_params.name,web: instruction.platform_params.web,img: instruction.platform_params.img,
					}),
                        acct_platform_admin: accts[0].to_string(),
                        acct_platform_fee_wallet: accts[1].to_string(),
                        acct_platform_nft_wallet: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CreateVestingAccount::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CreateVestingAccount::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    create_vesting_account_instruction_list.push(CreateVestingAccountInstruction {
                        trx_hash: transaction.id(),
                        share_amount: instruction.share_amount,
                        acct_creator: accts[0].to_string(),
                        acct_beneficiary: accts[1].to_string(),
                        acct_pool_state: accts[2].to_string(),
                        acct_vesting_record: accts[3].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Initialize::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Initialize::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    initialize_instruction_list.push(InitializeInstruction {
                        trx_hash: transaction.id(),
                        base_mint_param: Some(MintParams {
						decimals: instruction.base_mint_param.decimals as u64,name: instruction.base_mint_param.name,symbol: instruction.base_mint_param.symbol,uri: instruction.base_mint_param.uri,
					}),
                        curve_param: map_enum_curve_params(instruction.curve_param),
                        vesting_param: Some(VestingParams {
						total_locked_amount: instruction.vesting_param.total_locked_amount,cliff_period: instruction.vesting_param.cliff_period,unlock_period: instruction.vesting_param.unlock_period,
					}),
                        acct_payer: accts[0].to_string(),
                        acct_creator: accts[1].to_string(),
                        acct_global_config: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_authority: accts[4].to_string(),
                        acct_pool_state: accts[5].to_string(),
                        acct_base_mint: accts[6].to_string(),
                        acct_quote_mint: accts[7].to_string(),
                        acct_base_vault: accts[8].to_string(),
                        acct_quote_vault: accts[9].to_string(),
                        acct_metadata_account: accts[10].to_string(),
                        acct_event_authority: accts[16].to_string(),
                        acct_program: accts[17].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::MigrateToAmm::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::MigrateToAmm::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    migrate_to_amm_instruction_list.push(MigrateToAmmInstruction {
                        trx_hash: transaction.id(),
                        base_lot_size: instruction.base_lot_size,
                        quote_lot_size: instruction.quote_lot_size,
                        market_vault_signer_nonce: instruction.market_vault_signer_nonce as u64,
                        acct_payer: accts[0].to_string(),
                        acct_base_mint: accts[1].to_string(),
                        acct_quote_mint: accts[2].to_string(),
                        acct_market: accts[4].to_string(),
                        acct_request_queue: accts[5].to_string(),
                        acct_event_queue: accts[6].to_string(),
                        acct_bids: accts[7].to_string(),
                        acct_asks: accts[8].to_string(),
                        acct_market_vault_signer: accts[9].to_string(),
                        acct_market_base_vault: accts[10].to_string(),
                        acct_market_quote_vault: accts[11].to_string(),
                        acct_amm_pool: accts[13].to_string(),
                        acct_amm_authority: accts[14].to_string(),
                        acct_amm_open_orders: accts[15].to_string(),
                        acct_amm_lp_mint: accts[16].to_string(),
                        acct_amm_base_vault: accts[17].to_string(),
                        acct_amm_quote_vault: accts[18].to_string(),
                        acct_amm_target_orders: accts[19].to_string(),
                        acct_amm_config: accts[20].to_string(),
                        acct_amm_create_fee_destination: accts[21].to_string(),
                        acct_authority: accts[22].to_string(),
                        acct_pool_state: accts[23].to_string(),
                        acct_global_config: accts[24].to_string(),
                        acct_base_vault: accts[25].to_string(),
                        acct_quote_vault: accts[26].to_string(),
                        acct_pool_lp_token: accts[27].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::MigrateToCpswap::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::MigrateToCpswap::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    migrate_to_cpswap_instruction_list.push(MigrateToCpswapInstruction {
                        trx_hash: transaction.id(),
                        acct_payer: accts[0].to_string(),
                        acct_base_mint: accts[1].to_string(),
                        acct_quote_mint: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_cpswap_pool: accts[5].to_string(),
                        acct_cpswap_authority: accts[6].to_string(),
                        acct_cpswap_lp_mint: accts[7].to_string(),
                        acct_cpswap_base_vault: accts[8].to_string(),
                        acct_cpswap_quote_vault: accts[9].to_string(),
                        acct_cpswap_config: accts[10].to_string(),
                        acct_cpswap_create_pool_fee: accts[11].to_string(),
                        acct_cpswap_observation: accts[12].to_string(),
                        acct_lock_authority: accts[14].to_string(),
                        acct_lock_lp_vault: accts[15].to_string(),
                        acct_authority: accts[16].to_string(),
                        acct_pool_state: accts[17].to_string(),
                        acct_global_config: accts[18].to_string(),
                        acct_base_vault: accts[19].to_string(),
                        acct_quote_vault: accts[20].to_string(),
                        acct_pool_lp_token: accts[21].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SellExactIn::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SellExactIn::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    sell_exact_in_instruction_list.push(SellExactInInstruction {
                        trx_hash: transaction.id(),
                        amount_in: instruction.amount_in,
                        minimum_amount_out: instruction.minimum_amount_out,
                        share_fee_rate: instruction.share_fee_rate,
                        acct_payer: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_global_config: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_pool_state: accts[4].to_string(),
                        acct_user_base_token: accts[5].to_string(),
                        acct_user_quote_token: accts[6].to_string(),
                        acct_base_vault: accts[7].to_string(),
                        acct_quote_vault: accts[8].to_string(),
                        acct_base_token_mint: accts[9].to_string(),
                        acct_quote_token_mint: accts[10].to_string(),
                        acct_base_token_program: accts[11].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::SellExactOut::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::SellExactOut::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    sell_exact_out_instruction_list.push(SellExactOutInstruction {
                        trx_hash: transaction.id(),
                        amount_out: instruction.amount_out,
                        maximum_amount_in: instruction.maximum_amount_in,
                        share_fee_rate: instruction.share_fee_rate,
                        acct_payer: accts[0].to_string(),
                        acct_authority: accts[1].to_string(),
                        acct_global_config: accts[2].to_string(),
                        acct_platform_config: accts[3].to_string(),
                        acct_pool_state: accts[4].to_string(),
                        acct_user_base_token: accts[5].to_string(),
                        acct_user_quote_token: accts[6].to_string(),
                        acct_base_vault: accts[7].to_string(),
                        acct_quote_vault: accts[8].to_string(),
                        acct_base_token_mint: accts[9].to_string(),
                        acct_quote_token_mint: accts[10].to_string(),
                        acct_base_token_program: accts[11].to_string(),
                        acct_event_authority: accts[13].to_string(),
                        acct_program: accts[14].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdateConfig::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdateConfig::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_config_instruction_list.push(UpdateConfigInstruction {
                        trx_hash: transaction.id(),
                        param: instruction.param as u64,
                        value: instruction.value,
                        acct_global_config: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::UpdatePlatformConfig::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::UpdatePlatformConfig::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    update_platform_config_instruction_list.push(UpdatePlatformConfigInstruction {
                        trx_hash: transaction.id(),
                        param: map_enum_platform_config_param(instruction.param),
                        acct_platform_admin: accts[0].to_string(),
                        acct_platform_config: accts[1].to_string(),
                    });
                }
            }
        });
    });


    Data {
        claim_vested_event_event_list,
        create_vesting_event_event_list,
        pool_create_event_event_list,
        trade_event_event_list,
        buy_exact_in_instruction_list,
        buy_exact_out_instruction_list,
        claim_platform_fee_instruction_list,
        claim_vested_token_instruction_list,
        collect_fee_instruction_list,
        collect_migrate_fee_instruction_list,
        create_config_instruction_list,
        create_platform_config_instruction_list,
        create_vesting_account_instruction_list,
        initialize_instruction_list,
        migrate_to_amm_instruction_list,
        migrate_to_cpswap_instruction_list,
        sell_exact_in_instruction_list,
        sell_exact_out_instruction_list,
        update_config_instruction_list,
        update_platform_config_instruction_list,
    }
}


fn map_enum_curve_params(value: idl::idl::program::types::CurveParams) -> i32 {
    match value {
        idl::idl::program::types::CurveParams::Constant { data: _} => return 0,
        idl::idl::program::types::CurveParams::Fixed { data: _} => return 1,
        idl::idl::program::types::CurveParams::Linear { data: _} => return 2,
        _ => 0,
    }
}

fn map_enum_platform_config_param(value: idl::idl::program::types::PlatformConfigParam) -> i32 {
    match value {
        idl::idl::program::types::PlatformConfigParam::FeeWallet(_) => return 0,
        idl::idl::program::types::PlatformConfigParam::NFTWallet(_) => return 1,
        idl::idl::program::types::PlatformConfigParam::MigrateNftInfo(_) => return 2,
        idl::idl::program::types::PlatformConfigParam::FeeRate(_) => return 3,
        idl::idl::program::types::PlatformConfigParam::Name(_) => return 4,
        idl::idl::program::types::PlatformConfigParam::Web(_) => return 5,
        idl::idl::program::types::PlatformConfigParam::Img(_) => return 6,
        _ => 0,
    }
}

fn map_enum_pool_status(value: idl::idl::program::types::PoolStatus) -> i32 {
    match value {
        idl::idl::program::types::PoolStatus::Fund => return 0,
        idl::idl::program::types::PoolStatus::Migrate => return 1,
        idl::idl::program::types::PoolStatus::Trade => return 2,
        _ => 0,
    }
}

fn map_enum_trade_direction(value: idl::idl::program::types::TradeDirection) -> i32 {
    match value {
        idl::idl::program::types::TradeDirection::Buy => return 0,
        idl::idl::program::types::TradeDirection::Sell => return 1,
        _ => 0,
    }
}
