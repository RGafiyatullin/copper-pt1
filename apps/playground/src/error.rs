#[derive(Debug, Clone, Copy, thiserror::Error)]
pub enum Error {
    #[error("Unexpected Packet")]
    UnexpectedPacket,
    #[error("Peer Reported Failure")]
    PeerReportedFailure,
}
