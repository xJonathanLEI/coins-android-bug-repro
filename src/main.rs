use coins_ledger::{common::APDUData, transports::LedgerAsync, APDUCommand, Ledger};
use futures_executor::block_on;
use hex_literal::hex;

const P1_FIRST: u8 = 0x00;
const P1_MORE: u8 = 0x80;

const P2_NO_CHAINCODE: u8 = 0x00;

const SIGN: u8 = 0x04;

/// m/44'/60'/0'/0/0
const PATH: &[u8] = &hex!("058000002c8000003c800000000000000000000000");

/// Legacy (non-1559) transaction sending 0 ETH to the zero address
const TX: &[u8] = &hex!("df80808252089400000000000000000000000000000000000000008080018080");

#[tokio::main]
async fn main() {
    let transport = Ledger::init().await.expect("Ledger init failed");

    let mut payload = PATH.to_vec();
    payload.extend_from_slice(TX);

    let mut command = APDUCommand {
        ins: SIGN,
        p1: P1_FIRST,
        p2: P2_NO_CHAINCODE,
        data: APDUData::new(&[]),
        response_len: None,
    };

    let mut answer = None;

    let chunk_size = (0..=255)
        .rev()
        .find(|i| payload.len() % i != 3)
        .expect("true for any length");

    // Iterate in 255 byte chunks
    for chunk in payload.chunks(chunk_size) {
        command.data = APDUData::new(chunk);

        answer = Some(block_on(transport.exchange(&command)).expect("transport exchange failed"));

        let data = answer.as_ref().expect("just assigned").data();
        if data.is_none() {
            panic!("unexpected null response");
        }

        // We need more data
        command.p1 = P1_MORE;
    }

    let answer = answer.expect("payload is non-empty, therefore loop ran");
    let result = answer.data().expect("check in loop");
    if result.len() < 65 {
        panic!("short response: {}", result.len());
    }

    println!("Signature:");
    println!("v = {}", result[0] as u64);
    println!("r = 0x{}", hex::encode(&result[1..33]));
    println!("s = 0x{}", hex::encode(&result[33..]));
}
