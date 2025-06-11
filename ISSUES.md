thread 'tokio-runtime-worker' panicked at crates/comet/src/main.rs:140:70:
called `Result::unwrap()` on an `Err` value: Backend(ProtocolError(AMQPError { kind: Hard(NOTALLOWED), message: ShortString("NOT_ALLOWED - vhost  not found") }))
