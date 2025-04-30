mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::AdminStateInitializedEvent;
use pb::substreams::v1::program::BetCreatedEvent;
use pb::substreams::v1::program::BetInitializedEvent;
use pb::substreams::v1::program::BuySharesEvent;
use pb::substreams::v1::program::SellSharesEvent;
use pb::substreams::v1::program::BuyInstruction;
use pb::substreams::v1::program::CreateBetInstruction;
use pb::substreams::v1::program::GetPriceInstruction;
use pb::substreams::v1::program::InitAdminInstruction;
use pb::substreams::v1::program::InitBetInstruction;
use pb::substreams::v1::program::SellInstruction;










use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "BhRc1GTpQPtuSVPJG2tzE74jF3afb7NRrLK7JTvBbree";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut admin_state_initialized_event_list: Vec<AdminStateInitializedEvent> = Vec::new();
    let mut bet_created_event_list: Vec<BetCreatedEvent> = Vec::new();
    let mut bet_initialized_event_list: Vec<BetInitializedEvent> = Vec::new();
    let mut buy_shares_event_list: Vec<BuySharesEvent> = Vec::new();
    let mut sell_shares_event_list: Vec<SellSharesEvent> = Vec::new();
    let mut buy_instruction_list: Vec<BuyInstruction> = Vec::new();
    let mut create_bet_instruction_list: Vec<CreateBetInstruction> = Vec::new();
    let mut get_price_instruction_list: Vec<GetPriceInstruction> = Vec::new();
    let mut init_admin_instruction_list: Vec<InitAdminInstruction> = Vec::new();
    let mut init_bet_instruction_list: Vec<InitBetInstruction> = Vec::new();
    let mut sell_instruction_list: Vec<SellInstruction> = Vec::new();

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
                            idl::idl::program::events::AdminStateInitialized::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::AdminStateInitialized::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    admin_state_initialized_event_list.push(AdminStateInitializedEvent {
                                        trx_hash: transaction.id(),
                                        admin: event.admin.to_string(),
                                        is_initialised: event.is_initialised,
                                    });
                                }
                            }
                            idl::idl::program::events::BetCreated::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::BetCreated::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    bet_created_event_list.push(BetCreatedEvent {
                                        trx_hash: transaction.id(),
                                        bet_id: event.bet_id,
                                        creator: event.creator.to_string(),
                                        bet_prompt: event.bet_prompt,
                                        expiration_at: event.expiration_at,
                                        initial_liq: event.initial_liq as u64,
                                        yes_symbol: event.yes_symbol,
                                        yes_uri: event.yes_uri,
                                        no_symbol: event.no_symbol,
                                        no_uri: event.no_uri,
                                    });
                                }
                            }
                            idl::idl::program::events::BetInitialized::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::BetInitialized::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    bet_initialized_event_list.push(BetInitializedEvent {
                                        trx_hash: transaction.id(),
                                        bet_id: event.bet_id,
                                        creator: event.creator.to_string(),
                                        expiration_at: event.expiration_at,
                                    });
                                }
                            }
                            idl::idl::program::events::BuyShares::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::BuyShares::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    buy_shares_event_list.push(BuySharesEvent {
                                        trx_hash: transaction.id(),
                                        bet_id: event.bet_id,
                                        buyer: event.buyer.to_string(),
                                        outcome: event.outcome as u64,
                                        amount: event.amount as u64,
                                        shares: event.shares as u64,
                                    });
                                }
                            }
                            idl::idl::program::events::SellShares::DISCRIMINATOR => {
                                if let Ok(event) =
                                    idl::idl::program::events::SellShares::deserialize(
                                        &mut &slice_u8[8..],
                                    )
                                {
                                    sell_shares_event_list.push(SellSharesEvent {
                                        trx_hash: transaction.id(),
                                        bet_id: event.bet_id,
                                        seller: event.seller.to_string(),
                                        outcome: event.outcome as u64,
                                        shares: event.shares as u64,
                                        amount_withdraw: event.amount_withdraw as u64,
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
            if &slice_u8[8..16] == idl::idl::program::events::AdminStateInitialized::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::AdminStateInitialized::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    admin_state_initialized_event_list.push(AdminStateInitializedEvent {
                        trx_hash: transaction.id(),
                        admin: event.admin.to_string(),
                        is_initialised: event.is_initialised,
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::BetCreated::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::BetCreated::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    bet_created_event_list.push(BetCreatedEvent {
                        trx_hash: transaction.id(),
                        bet_id: event.bet_id,
                        creator: event.creator.to_string(),
                        bet_prompt: event.bet_prompt,
                        expiration_at: event.expiration_at,
                        initial_liq: event.initial_liq as u64,
                        yes_symbol: event.yes_symbol,
                        yes_uri: event.yes_uri,
                        no_symbol: event.no_symbol,
                        no_uri: event.no_uri,
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::BetInitialized::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::BetInitialized::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    bet_initialized_event_list.push(BetInitializedEvent {
                        trx_hash: transaction.id(),
                        bet_id: event.bet_id,
                        creator: event.creator.to_string(),
                        expiration_at: event.expiration_at,
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::BuyShares::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::BuyShares::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    buy_shares_event_list.push(BuySharesEvent {
                        trx_hash: transaction.id(),
                        bet_id: event.bet_id,
                        buyer: event.buyer.to_string(),
                        outcome: event.outcome as u64,
                        amount: event.amount as u64,
                        shares: event.shares as u64,
                    });
                }
            }
            if &slice_u8[8..16] == idl::idl::program::events::SellShares::DISCRIMINATOR {
                if let Ok(event) =
                    idl::idl::program::events::SellShares::deserialize(
                        &mut &slice_u8[16..],
                    )
                {
                    sell_shares_event_list.push(SellSharesEvent {
                        trx_hash: transaction.id(),
                        bet_id: event.bet_id,
                        seller: event.seller.to_string(),
                        outcome: event.outcome as u64,
                        shares: event.shares as u64,
                        amount_withdraw: event.amount_withdraw as u64,
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Buy::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Buy::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    buy_instruction_list.push(BuyInstruction {
                        trx_hash: transaction.id(),
                        bet_id: instruction.bet_id,
                        outcome: instruction.outcome as u64,
                        amt: instruction.amt as u64,
                        acct_signer: accts[0].to_string(),
                        acct_bet: accts[1].to_string(),
                        acct_usdc: accts[2].to_string(),
                        acct_sender_token_account: accts[3].to_string(),
                        acct_recipient_token_account: accts[4].to_string(),
                        acct_mint_yes: accts[5].to_string(),
                        acct_mint_no: accts[6].to_string(),
                        acct_destination_yes: accts[7].to_string(),
                        acct_destination_no: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::CreateBet::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::CreateBet::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    create_bet_instruction_list.push(CreateBetInstruction {
                        trx_hash: transaction.id(),
                        bet_id: instruction.bet_id,
                        initial_liq: instruction.initial_liq as u64,
                        bet_prompt: instruction.bet_prompt,
                        expiration_at: instruction.expiration_at,
                        yes_symbol: instruction.yes_symbol,
                        yes_uri: instruction.yes_uri,
                        no_symbol: instruction.no_symbol,
                        no_uri: instruction.no_uri,
                        acct_signer: accts[0].to_string(),
                        acct_bet: accts[1].to_string(),
                        acct_mint_yes: accts[2].to_string(),
                        acct_mint_no: accts[3].to_string(),
                        acct_metadata_yes: accts[4].to_string(),
                        acct_metadata_no: accts[5].to_string(),
                        acct_token_program: accts[7].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::GetPrice::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::GetPrice::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    get_price_instruction_list.push(GetPriceInstruction {
                        trx_hash: transaction.id(),
                        side: instruction.side as u64,
                        acct_bet: accts[0].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitAdmin::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitAdmin::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    init_admin_instruction_list.push(InitAdminInstruction {
                        trx_hash: transaction.id(),
                        acct_admin_state: accts[0].to_string(),
                        acct_signer: accts[1].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::InitBet::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::InitBet::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    init_bet_instruction_list.push(InitBetInstruction {
                        trx_hash: transaction.id(),
                        bet_id: instruction.bet_id,
                        acct_signer: accts[0].to_string(),
                        acct_bet: accts[1].to_string(),
                        acct_usdc: accts[2].to_string(),
                        acct_sender_token_account: accts[3].to_string(),
                        acct_recipient_token_account: accts[4].to_string(),
                        acct_mint_yes: accts[5].to_string(),
                        acct_mint_no: accts[6].to_string(),
                        acct_destination_yes: accts[7].to_string(),
                        acct_destination_no: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                    });
                }
            }
            if &slice_u8[0..8] == idl::idl::program::client::args::Sell::DISCRIMINATOR {
                if let Ok(instruction) =
                    idl::idl::program::client::args::Sell::deserialize(&mut &slice_u8[8..])
                {
                    let accts = inst.accounts();
                    sell_instruction_list.push(SellInstruction {
                        trx_hash: transaction.id(),
                        bet_id: instruction.bet_id,
                        outcome: instruction.outcome as u64,
                        shares: instruction.shares as u64,
                        acct_signer: accts[0].to_string(),
                        acct_bet: accts[1].to_string(),
                        acct_usdc: accts[2].to_string(),
                        acct_sender_token_account: accts[3].to_string(),
                        acct_recipient_token_account: accts[4].to_string(),
                        acct_mint_yes: accts[5].to_string(),
                        acct_mint_no: accts[6].to_string(),
                        acct_destination_yes: accts[7].to_string(),
                        acct_destination_no: accts[8].to_string(),
                        acct_token_program: accts[9].to_string(),
                    });
                }
            }
        });
    });


    Data {
        admin_state_initialized_event_list,
        bet_created_event_list,
        bet_initialized_event_list,
        buy_shares_event_list,
        sell_shares_event_list,
        buy_instruction_list,
        create_bet_instruction_list,
        get_price_instruction_list,
        init_admin_instruction_list,
        init_bet_instruction_list,
        sell_instruction_list,
    }
}





















