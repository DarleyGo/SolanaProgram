use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::{PrintProgramError,ProgramError},
    pubkey::Pubkey,
    //sysvar,
};
use num_derive::FromPrimitive;
use thiserror::Error;

/// Errors that may be returned by the Metadata program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum RaceError {
    /// Player Already exists!
    #[error("Player Already exists!")]
    PlayerFoundError,

    /// Slot not available!
    #[error("Slot not available!")]
    SlotNotAvailableError,
}

impl PrintProgramError for RaceError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<RaceError> for ProgramError {
    fn from(e: RaceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RaceAccount {
    pub status: u8,
    pub level: u8,
    pub r#type: u8,
    pub date: u64,
    pub name: String,
    pub location: String,
    pub distance: u16,
    pub entry_fee: u16,
    pub prize_pool: u16,
    pub game_url: String,
    pub end_date: u64,
    pub players: Option<Vec<Player>>,
}

impl RaceAccount {
    pub fn from_account_info(a: &AccountInfo) -> Result<RaceAccount, ProgramError> {
        let md: RaceAccount =
            try_from_slice_unchecked(&a.data.borrow_mut())?;
            //try_from_slice_checked(&a.data.borrow_mut(), Key::MetadataV1, MAX_METADATA_LEN)?;

        Ok(md)
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Copy)]
pub struct Player {
    pub address: Pubkey,
    pub slot: u8,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
/// Args for create call
pub struct UpdateRaceArgs {
    pub status: u8,
    pub level: u8,
    pub r#type: u8,
    pub date: u64,
    pub name: String,
    pub location: String,
    pub distance: u16,
    pub entry_fee: u16,
    pub prize_pool: u16,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
/// Args for create call
pub struct UpdateGameArgs {
    pub game_url: String,
    pub end_date: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
/// Args for create call
pub struct JoinRaceArgs {
    pub player: Player,
}

/// Instructions supported by the Race program.
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum RaceInstruction {
    UpdateRace(UpdateRaceArgs),
    UpdateGame(UpdateGameArgs),
    JoinRace(JoinRaceArgs),
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction<'a>(
    program_id: &'a Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &'a [AccountInfo<'a>], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Race Rust program entrypoint");
    let instruction = RaceInstruction::try_from_slice(_instruction_data)?;
    match instruction {
        RaceInstruction::UpdateRace(args) => {
            msg!("Instruction: UpdateRace");
            msg!("Name: {}", &args.name);
            process_update_race(
                program_id,
                accounts,
                args
            )
        }
        RaceInstruction::UpdateGame(args) => {
            msg!("Instruction: UpdateGame: {}", &args.game_url);
            process_update_game(
                program_id,
                accounts,
                args
            )
        }
        RaceInstruction::JoinRace(args) => {
            msg!("Instruction: JoinRace: {}", &args.player.address);
            process_join_race(
                program_id,
                accounts,
                args
            )
        }
    }
}

pub fn process_update_race<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: UpdateRaceArgs,
) -> ProgramResult {
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Race Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    //let mut race_account = RaceAccount::try_from_slice(&account.data.borrow())?;
    let mut race_account : RaceAccount = try_from_slice_unchecked(&account.data.borrow())?;
    msg!("Current Name: {}", &race_account.name);
    race_account.date = args.date;
    race_account.level = args.level;
    race_account.name = args.name;
    race_account.location = args.location;
    race_account.distance = args.distance;
    race_account.entry_fee = args.entry_fee;
    race_account.prize_pool = args.prize_pool;
    race_account.status = args.status;
    //race_account.players = args.name;
    race_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn process_update_game<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: UpdateGameArgs,
) -> ProgramResult {
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Race Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    //let mut race_account = RaceAccount::try_from_slice(&account.data.borrow())?;
    let mut race_account : RaceAccount = try_from_slice_unchecked(&account.data.borrow())?;
    race_account.game_url = args.game_url;
    race_account.end_date = args.end_date;
    race_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn process_join_race<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: JoinRaceArgs,
) -> ProgramResult {
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Race Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    //let mut race_account = RaceAccount::try_from_slice(&account.data.borrow())?;
    let mut race_account : RaceAccount = try_from_slice_unchecked(&account.data.borrow())?;

    if let Some(players) = &mut race_account.players {
        let mut new_players = Vec::<Player>::new();
        for player in players {
            if player.address == args.player.address {
                return Err(RaceError::PlayerFoundError.into());
            }
            if player.slot == args.player.slot {
                return Err(RaceError::SlotNotAvailableError.into());
            }
            new_players.push(*player);
        }
        new_players.push(args.player);
    } else {
        //return Err(MetadataError::NoCreatorsPresentOnMetadata.into());
        let mut players = Vec::<Player>::new();
        players.push(args.player);
        race_account.players = Some(players);
    }

    race_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            RaceAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            RaceAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            RaceAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
