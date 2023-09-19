#![no_std]
use gstd::{debug, exec, msg, prelude::*, ActorId, MessageId, ReservationId};
use io::Action;
static mut PANIC_ADDRESS: ActorId = ActorId::zero();

#[gstd::async_main]
async fn main() {
    let action: Action = msg::load().expect("Unable to load the msg");
    let address = unsafe { PANIC_ADDRESS };
    match action {
        Action::SendWithoutReplyDeposit => {
            msg::send_with_gas_for_reply(address, "", 500_000_000_000, 0, 0)
                .expect("Error in sending a message `NFTAction::Mint`")
                .await
                .expect("Error in receiving reply");
        }
        Action::SendWithReplyDeposit => {
            msg::send_with_gas_for_reply(address, "", 500_000_000_000, 0, 5_000_000_000)
                .expect("Error in sending a message `NFTAction::Mint`")
                .await
                .expect("Error in receiving reply");
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let address: ActorId = msg::load().expect("Unable to load the msg");
    unsafe { PANIC_ADDRESS = address };
}
