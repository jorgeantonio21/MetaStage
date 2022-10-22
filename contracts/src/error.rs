use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Error)]
#[serde(crate = "near_sdk::serde")]
pub enum MetaDaoError {
    #[error("Invalid Admin call")]
    InvalidAdminCall,
    #[error("Unable to create a new epoch, while previous epoch is still ongoing")]
    UnableToCreateNewEpoch,
    #[error("Creator is not registered for current epoch")]
    CreatorIsNotRegistered,
    #[error("Invalid current epoch")]
    InvalidCurrentEpoch,
    #[error("Currently, epoch is off")]
    EpochIsOff,
    #[error("Not in funding period")]
    NotInFundingPeriod,
    #[error("User did not attach enough funds to contract call")]
    UserDidNotAttachEnoughFunds,
    #[error("User already registered funds to creator")]
    UserAlreadyRegisteredFundsToCreator,
}

impl AsRef<str> for NearConLiqError {
    fn as_ref(&self) -> &str {
        match self {
            Self::InvalidAdminCall => "Invalid Admin call",
            Self::UnableToCreateNewEpoch => {
                "Unable to create a new epoch, while previous epoch is still ongoing"
            }
            Self::CreatorIsNotRegistered => "Creator is not registered for current epoch",
            Self::InvalidCurrentEpoch => "Invalid current epoch",
            Self::EpochIsOff => "Currently, epoch is off",
            Self::NotInFundingPeriod => "Not in funding period",
            Self::UserDidNotAttachEnoughFunds => "User did not attach enough funds to contract call",
            Self::UserAlreadyRegisteredfundsToCreator => "User already registered funds to creator",
        }
    }
}