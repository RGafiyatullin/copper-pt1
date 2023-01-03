#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnexpectedPacket,
    PeerReportedFailure,
}

#[derive(Debug, Clone)]
pub struct Protocol<const N: usize> {
    secret: i128,
    this_party_id: usize,
    state: State<N>,
}

impl<const N: usize> Protocol<N> {
    pub fn new(this_party_id: usize, secret: i128) -> Self {
        let state = init_state(this_party_id, secret);
        Self {
            this_party_id,
            secret,
            state,
        }
    }

    pub fn pending_events(&mut self) -> Option<[Outbound; N]> {
        match self.state {
            State::Round1(shares) => {
                self.state = State::Round2([None; N]);

                let out = std::array::from_fn(|to_party| Outbound {
                    to_party,
                    payload: Payload::Share(shares[to_party]),
                });

                Some(out)
            }
            State::Round2(shares) => {
                if shares.iter().copied().all(|maybe_sum| maybe_sum.is_some()) {
                    self.state = State::Round3([None; N]);

                    let sum = shares
                        .into_iter()
                        .filter_map(std::convert::identity)
                        .sum::<i128>();

                    let out = std::array::from_fn(|to_party| Outbound {
                        to_party,
                        payload: Payload::Candidate(sum),
                    });

                    Some(out)
                } else {
                    None
                }
            }
            State::Round3(candidates) => {
                if candidates
                    .iter()
                    .copied()
                    .all(|maybe_candidate| maybe_candidate.is_some())
                {
                    unimplemented!()
                } else {
                    None
                }
            }
        }
    }

    pub fn inbound_event(&mut self, inbound: Inbound) -> Result<(), Error> {
        match (&mut self.state, inbound) {
            (
                State::Round2(shares),
                Inbound {
                    payload: Payload::Share(share),
                    from_party,
                },
            ) => {
                shares[from_party] = Some(share);
                Ok(())
            }
            (
                State::Round3(candidates),
                Inbound {
                    payload: Payload::Candidate(candidate),
                    from_party,
                },
            ) => {
                candidates[from_party] = Some(candidate);
                Ok(())
            }
            (
                State::Round4(_candidate, acks),
                Inbound {
                    payload,
                    from_party,
                },
            ) if matches!(payload, Payload::Success | Payload::Error) => {
                acks[from_party] = Some(matches!(payload, Payload::Success));
                Ok(())
            }

            _ => Err(Error::UnexpectedPacket),
        }
    }

    pub fn outcome(&self) -> Result<i128, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Outbound {
    pub to_party: usize,
    pub payload: Payload,
}

#[derive(Debug, Clone, Copy)]
pub struct Inbound {
    pub from_party: usize,
    pub payload: Payload,
}

#[derive(Debug, Clone, Copy)]
pub enum Payload {
    Share(i128),
    Candidate(i128),
    Success,
    Error,
}

#[derive(Debug, Clone, Copy)]
enum State<const N: usize> {
    Round1([i128; N]), // Shares generated here
    Round2([Option<i128>; N]), // Shares collected from inbound packets
    Round3([Option<i128>; N]), // Sum-candidates
    Round4(i128, [Option<bool>; N]), // Sum; Acks
}

fn init_state<const N: usize>(this_party_id: usize, secret: i128) -> State<N> {
    State::Round1(unimplemented!())
}

#[test]
fn example() {
    let mut p1 = Protocol::<2>::new(0, 1000);
    let p2 = Protocol::<2>::new(1, 1500);

    loop {
        if let Ok(outcome) = p1.outcome() {
            break outcome;
        }

        if let Some(outbound_events) = p1.pending_events() {
            // send the events here
        }

        if let Some(inbound_event) = unimplemented!("receive another event") {
            p1.inbound_event(inbound_event);
        }
    }
}
