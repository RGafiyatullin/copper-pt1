#[derive(Debug, Clone, Copy)]
pub struct Outbound<V> {
    pub to: usize,
    pub payload: Payload<V>,
}

#[derive(Debug, Clone, Copy)]
pub struct Inbound<V> {
    pub from: usize,
    pub payload: Payload<V>,
}

#[derive(Debug, Clone, Copy)]
pub enum Payload<V> {
    Share(V),
    Candidate(V),
    Sum(V),
}
