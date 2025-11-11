/// Try initializing tracing subscriber. Useful in context of unit
/// tests where depending on the exact `cargo test` invocation
/// tracing may or may not already be initialized. I.e. when running
/// a single test.
pub fn setup() {
    let _ = tracing_subscriber::fmt::try_init();
}
