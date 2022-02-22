use bytes::Bytes;

#[derive(Debug, Clone)]
pub enum Opcode {
    /// Send a known command to the database (`get`, `set`, `remove`, `list`)
    Query { command: Command },
    /// Send _start_ signal to server
    Start,
    /// Send _shutdown_ signal; args: u64 (seconds before shutdown signal)
    Shutdown { delay: u64 },
    /// Send _restart_ signal; args: u64 (seconds before shutdown signal)
    Restart { delay: u64 },
    /// Send _status_ check, useful for performing a health check
    Status { message: Bytes },
    /// Invalid opcode; stop processing request
    Invalid { message: Bytes },
}

// TODO: Change `key` to `String` type or decide upon an alternative solution.
#[derive(Debug, Clone)]
pub enum Command {
    Get {
        key: Bytes,
        value: Bytes,
    },
    Set {
        key: Bytes,
        value: Bytes,
        ttl: Option<u64>,
    },
    Del {
        key: Bytes,
    },
    List,
}
