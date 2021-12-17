use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Default, BorshSerialize, BorshDeserialize, Debug)]
pub struct Party {
    pub is_initialized: bool,
    pub positive_votes: u32,
    pub negative_votes: u32,
    pub name: String,
    pub voting_state_pubkey: Pubkey,
}

impl Party {
    pub fn serialized_size() -> usize {
        Self::default()
            .try_to_vec()
            .expect("failed to serialize default Party")
            .len()
    }
}
