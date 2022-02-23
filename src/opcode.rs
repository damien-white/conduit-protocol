use bytes::Bytes;

// TODO: Update this solution to something more optimal and easy to reason about
#[derive(Debug, Clone)]
pub enum Opcode {
    /// Send a known command to the database (`get`, `set`, `remove`, `list`)
    Query { command: Command },
    /// Send "shutdown" signal with a delay (in seconds)
    Shutdown { delay: u64 },
    /// Send "restart" signal with a delay (in seconds)
    Restart { delay: u64 },
    /// Send "status" check, performing a similar function as a health check
    Status { message: Bytes },
    /// Invalid opcode / Error
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
