use gmeta::{metawasm, Metadata};
use gstd::{prelude::*, ActorId};
use onchain_nft_io::*;

#[metawasm]
pub trait Metawasm {
    type State = <ContractMetadata as Metadata>::State;
}
