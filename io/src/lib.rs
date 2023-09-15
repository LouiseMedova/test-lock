//! Data types for the contract input/output.

#![no_std]

use gmeta::{In, Metadata};
use gstd::{prelude::*, ActorId};

pub struct ContractMetadata;

impl Metadata for ContractMetadata {  
    type Init = In<ActorId>;
    type Handle = In<Action>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = ();
}

/// The main type used as an input message.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    SendWithoutReplyDeposit,
    SendWithReplyDeposit,
}
