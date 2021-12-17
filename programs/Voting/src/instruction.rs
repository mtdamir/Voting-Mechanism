#![allow(clippy::use_self)]

use crate::error::VotingError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum VotingInstruction {
    InitVoting,

    AddVoter {
        voter_pubkey: Pubkey,
        voter_votes_bump_seed: u8,
    },

    AddParty{name: String, party_bump_seed: u8 },

    Vote {
        positive: bool,
        voter_votes_bump_seed: u8,
    },
}

impl VotingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|error| {
            msg!(&error.to_string());
            VotingError::InvalidInstruction.into()
        })
    }
}

fn voting_state_pubkey(voting_owner_pubkey: &Pubkey) -> Pubkey {
    Pubkey::create_with_seed(voting_owner_pubkey, "voting_state", &create::id())
        .expect("failed to create voting_state_pubkey")
}

pub fn init_voting(voting_owner_pubkey: &Pubkey) -> Instructions {
    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new(voting_state_pubkey(voting_owner_pubkey), false),
    ];
    Instruction::new_with_borsh(crate::id(), &VotingInstruction::InitVoting, account_metas)

}

pub fn add_voter(voting_owner_pubkey: &Pubkey, voter_pubkey: &Pubkey) -> (Instruction, Pubkey) {
    let voting_state_pubkey = voting_state_pubkey(voting_owner_pubkey);

    let seeds = &[
        b"voter_votes".as_ref(),
        voter_pubkey.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let (voter_votes_pubkey, voter_votes_bump_seed) =
        Pubkey::find_program_address(seeds, &crate::id());

    let account_metas = vec![
        AccountMeta::new(*voting_owner_pubkey, true),
        AccountMeta::new_readonly(voting_state_pubkey, false),
        AccountMeta::new(voter_votes_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddVoter {
            voter_pubkey: *voter_pubkey,
            voter_votes_bump_seed,
        },
        account_metas,
    );
    (ix, voter_votes_pubkey)
}

pub fn add_party(
    fee_payer: &Pubkey,
    party_name: &str,
    party_count: u32,
    voting_state_pubkey: &Pubkey,
) -> (Instruction, Pubkey) {
    let new_party_index_bytes = party_count.to_le_bytes();
    let seeds = &[
        b"party",
        new_party_index_bytes.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let (party_pubkey, party_bump_seed) = Pubkey::find_program_address(seeds, &crate::id());

    let account_metas = vec![
        AccountMeta::new(*fee_payer, true),
        AccountMeta::new(party_pubkey, false),
        AccountMeta::new(*voting_state_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::AddParty {
            name: party_name.to_owned(),
            party_bump_seed,
        },
        account_metas,
    );
    (ix, party_pubkey)
}


pub fn vote(
    voter_pubkey: &Pubkey,
    voting_state_pubkey: &Pubkey,
    party_pubkey: &Pubkey,
    positive: bool,
) -> (Instruction, Pubkey, Pubkey) {
    let seeds = &[
        b"voter_votes".as_ref(),
        voter_pubkey.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let voter_votes_pubkey = Pubkey::find_program_address(seeds, &crate::id()).0;

    let seeds = &[
        b"voter_voted".as_ref(),
        voter_pubkey.as_ref(),
        party_pubkey.as_ref(),
        voting_state_pubkey.as_ref(),
    ];
    let (voter_voted_pubkey, voter_votes_bump_seed) =
        Pubkey::find_program_address(seeds, &crate::id());

    let account_metas = vec![
        AccountMeta::new(*voter_pubkey, true),
        AccountMeta::new_readonly(*voting_state_pubkey, false),
        AccountMeta::new(voter_voted_pubkey, false),
        AccountMeta::new(voter_votes_pubkey, false),
        AccountMeta::new(*party_pubkey, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let ix = Instruction::new_with_borsh(
        crate::id(),
        &VotingInstruction::Vote {
            positive,
            voter_votes_bump_seed,
        },
        account_metas,
    );
    (ix, voter_votes_pubkey, voter_voted_pubkey)
}