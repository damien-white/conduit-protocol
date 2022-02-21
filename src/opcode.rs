use bytes::Bytes;

// TODO: Create Opcode -> Command (function) via VTable or similar technique
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    /// Invalid opcode; stop processing request
    Invalid = 0x00,
    /// Send _start_ signal to server
    Start = 0x01,
    /// Send _shutdown_ signal; args: u64 (seconds before shutdown signal)
    Shutdown = 0x02,
    /// Send _restart_ signal; args: u64 (seconds before shutdown signal)
    Restart = 0x04,
    // time before shutdown
    Command = 0x08, // `Get` `Set` `List` Del`
}

#[derive(Clone, Debug)]
pub enum Command {
    Get { key: String, value: Bytes },
}
