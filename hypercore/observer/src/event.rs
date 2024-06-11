use gprimitives::{ActorId, CodeId, H256};
pub use hypercore_ethereum::event::{ClaimValue, CreateProgram, SendMessage, SendReply};

use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub enum BlockEvent {
    CreateProgram(CreateProgram),
    SendMessage(SendMessage),
    SendReply(SendReply),
    ClaimValue(ClaimValue),
}

#[derive(Debug, Encode, Decode)]
pub enum Event {
    UploadCode {
        origin: ActorId,
        code_id: CodeId,
        code: Vec<u8>,
    },
    Block {
        block_hash: H256,
        events: Vec<BlockEvent>,
    },
}
