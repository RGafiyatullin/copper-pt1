use crate::error::Error;
use crate::pdu::*;
use crate::state::State;
use crate::value::Value;

#[derive(Debug)]
pub struct Protocol<const N: usize, V> {
    state: State<N, V>,
}

impl<const N: usize, V> Protocol<N, V>
where
    V: Value,
{
    pub fn new(secret: V) -> Self {
        let state = State::init(secret);
        Self { state }
    }

    #[cfg(test)]
    pub(super) fn from_state(state: State<N, V>) -> Self {
        Self { state }
    }

    pub fn take_outbound(&mut self) -> Option<Outbound<V>> {
        loop {
            if let Some(outbound_opt) = self.take_outbound_inner() {
                break outbound_opt;
            }
        }
    }

    fn take_outbound_inner(&mut self) -> Option<Option<Outbound<V>>> {
        match &mut self.state {
            State::Round1Sending(shares_to_send) => {
                if let Some(outbound) = shares_to_send.iter_mut().enumerate().find_map(|(to, s)| {
                    s.take().map(|v| Outbound {
                        to,
                        payload: Payload::Share(v),
                    })
                }) {
                    Some(Some(outbound))
                } else {
                    self.state = State::Round2Receiving([None; N]);
                    None
                }
            }
            State::Round2Receiving(shares_received) => {
                if shares_received.iter().all(|s| s.is_some()) {
                    let mut sum = V::zero();
                    shares_received.iter().copied().flatten().for_each(|s| {
                        sum.wrapping_add_assign(&s);
                    });
                    self.state = State::Round2Sending([Some(sum); N]);
                    None
                } else {
                    Some(None)
                }
            }
            State::Round2Sending(candidates_to_send) => {
                if let Some(outbound) =
                    candidates_to_send
                        .iter_mut()
                        .enumerate()
                        .find_map(|(to, s)| {
                            s.take().map(|v| Outbound {
                                to,
                                payload: Payload::Candidate(v),
                            })
                        })
                {
                    Some(Some(outbound))
                } else {
                    self.state = State::Round3Receiving([None; N]);
                    None
                }
            }
            State::Round3Receiving(candidates_received) => {
                if candidates_received.iter().copied().any(|c| c.is_none()) {
                    return Some(None);
                }

                let mut sum = V::zero();
                candidates_received
                    .into_iter()
                    .flatten()
                    .for_each(|v| sum.wrapping_add_assign(v));

                self.state = State::Round4Sending([Some(sum); N]);
                None
            }
            State::Round4Sending(sums_to_send) => {
                if let Some(outbound) = sums_to_send.iter_mut().enumerate().find_map(|(to, s)| {
                    s.take().map(|v| Outbound {
                        to,
                        payload: Payload::Sum(v),
                    })
                }) {
                    Some(Some(outbound))
                } else {
                    self.state = State::Round4Receiving([None; N]);
                    None
                }
            }
            State::Round4Receiving { .. } => Some(None),
        }
    }

    pub fn process_inbound(&mut self, inbound: Inbound<V>) -> Result<(), Error> {
        match (&mut self.state, inbound) {
            (
                State::Round2Receiving(shares_received),
                Inbound {
                    from,
                    payload: Payload::Share(share),
                },
            ) => {
                if let Some(_existing_share) = shares_received[from].replace(share) {
                    return Err(Error::UnexpectedPacket);
                }
            }

            (
                State::Round3Receiving(candidates_received),
                Inbound {
                    from,
                    payload: Payload::Candidate(candidate),
                },
            ) => {
                if let Some(_existing_candidate) = candidates_received[from].replace(candidate) {
                    return Err(Error::UnexpectedPacket);
                }
            }

            (
                State::Round4Receiving(sums_received),
                Inbound {
                    from,
                    payload: Payload::Sum(status),
                },
            ) => {
                if let Some(_existing_status) = sums_received[from].replace(status) {
                    return Err(Error::UnexpectedPacket);
                }
            }

            _ => return Err(Error::UnexpectedPacket),
        }
        Ok(())
    }

    pub fn outcome(&self) -> Result<Option<V>, Error> {
        let State::Round4Receiving(sums) = &self.state else { return Ok(None) };

        let mut first = Option::<V>::None;

        for s in sums {
            match (first, *s) {
                (_, None) => return Ok(None),
                (None, Some(this)) => {
                    first = Some(this);
                }
                (Some(prev), Some(this)) => {
                    if prev != this {
                        return Err(Error::PeerReportedFailure);
                    }
                }
            }
        }

        Ok(first)
    }
}
