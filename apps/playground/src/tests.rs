use std::vec;

use crate::{Inbound, Outbound, Protocol, Value};

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

use either::Either;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_all() {
    run_test::<1, u8>().await;
    run_test::<2, u8>().await;
    run_test::<10, i128>().await;
    run_test::<10, u128>().await;
}

#[tokio::test]
async fn test_manual() {
    use crate::state::State;

    let (tx0, rx0) = mpsc::unbounded_channel();
    let (tx1, rx1) = mpsc::unbounded_channel();

    let protocol_0 = Protocol::<2, u16>::from_state(State::Round1Sending([Some(44), Some(100)]));
    let protocol_1 = Protocol::<2, u16>::from_state(State::Round1Sending([Some(3), Some(19)]));

    let txs = [tx0, tx1];

    let running_0 = run_protocol(0, Either::Right(protocol_0), rx0, &txs);
    let running_1 = run_protocol(1, Either::Right(protocol_1), rx1, &txs);

    let outcomes = futures::future::try_join_all([running_0, running_1])
        .await
        .expect("ew...");
    eprintln!("outcomes: {:?}", outcomes)
}

async fn run_test<const N: usize, V: Value>() {
    let rng = &mut rand::rngs::OsRng;
    let mut pipes: [_; N] = std::array::from_fn(|_| {
        let (tx, rx) = mpsc::unbounded_channel();
        (Some(tx), Some(rx))
    });

    let rxs: [_; N] = std::array::from_fn(|idx| pipes[idx].1.take().unwrap());
    let txs: [_; N] = std::array::from_fn(|idx| pipes[idx].0.take().unwrap());
    let secrets: [_; N] = std::array::from_fn(|_| V::pick_random(rng));

    let mut workers = vec![];
    for (party_id, rx) in rxs.into_iter().enumerate() {
        let running = run_protocol(party_id, Either::Left(secrets[party_id]), rx, &txs);
        workers.push(running);
    }

    let outcomes = futures::future::try_join_all(workers).await.expect("ew...");

    eprintln!("outcomes: {:?}", outcomes);
}

async fn run_protocol<const N: usize, V: Value>(
    party_id: usize,
    init: Either<V, Protocol<N, V>>,
    mut rx: mpsc::UnboundedReceiver<Inbound<V>>,
    txs: &[mpsc::UnboundedSender<Inbound<V>>; N],
) -> Result<V, AnyError> {
    eprintln!("[{}] starting protocol [init: {:?}]", party_id, init);

    let mut protocol = init.either(Protocol::new, std::convert::identity);

    let outcome = loop {
        eprintln!("[{}] begin loop", party_id);

        if let Some(outcome) = protocol.outcome()? {
            eprintln!("[{}] outcome: {:?}", party_id, outcome);
            break outcome;
        }

        while let Some(Outbound { to, payload }) = protocol.take_outbound() {
            eprintln!(
                "[{}] outbound [to: {}; payload: {:?}]",
                party_id, to, payload
            );
            txs[to]
                .send(Inbound {
                    from: party_id,
                    payload,
                })
                .map_err(|_| "tx-failure")?;
        }

        eprintln!("[{}] receiving...", party_id);
        let Some(inbound) = rx.recv().await else {
            Err("recv error")?
        };
        eprintln!(
            "[{}] received [from: {}; payload: {:?}]",
            party_id, inbound.from, inbound.payload
        );
        protocol.process_inbound(inbound)?;
        eprintln!("[{}] end loop", party_id);
    };

    eprintln!("[{}] outcome: {:?}", party_id, outcome);

    Ok(outcome)
}
