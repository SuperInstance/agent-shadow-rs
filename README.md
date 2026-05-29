# agent-shadow-rs

Agent behavior monitoring and replay — record timestamped actions, analyze success rates, compare agents quantitatively, and run in shadow mode without interfering with production.

## What This Gives You

- **Action recording**: timestamped actions with duration, success/failure, and binary payloads
- **Trace analysis**: success rate, action type breakdown, filtering by type or time range
- **Shadow mode**: silently monitor multiple agents without affecting their behavior
- **Behavior comparison**: quantitatively compare two agents' traces (success rate diff, duration diff, action overlap)
- **Replay**: replay a recorded trace for debugging or regression testing

## Quick Start

```rust
use agent_shadow::{Shadow, Action};

let mut shadow = Shadow::new();

// Record actions for multiple agents
shadow.record("agent-a", Action::new(0.0, "search").with_duration(0.5).with_success(true));
shadow.record("agent-a", Action::new(1.0, "fetch").with_duration(0.3).with_success(true));
shadow.record("agent-a", Action::new(2.0, "search").with_duration(0.4).with_success(true));
shadow.record("agent-b", Action::new(0.0, "search").with_duration(0.8).with_success(false));

// Per-agent analysis
let trace = shadow.get_trace("agent-a").unwrap();
println!("Success rate: {:.1}%", trace.success_rate() * 100.0);
println!("Action types: {:?}", trace.action_types());

// Compare two agents
let cmp = shadow.compare("agent-a", "agent-b").unwrap();
println!("Success diff: {:.3}", cmp.success_diff);
println!("Duration diff: {:.3}s", cmp.duration_diff);
```

## API Reference

### Action

```rust
pub struct Action {
    pub timestamp: f64,
    pub action_type: String,
    pub payload: Vec<u8>,
    pub duration: f64,
    pub success: bool,
}

impl Action {
    pub fn new(ts: f64, action_type: &str) -> Self;
    pub fn with_duration(self, d: f64) -> Self;
    pub fn with_success(self, s: bool) -> Self;
    pub fn with_payload(self, p: Vec<u8>) -> Self;
}
```

### Trace

```rust
impl Trace {
    pub fn success_rate(&self) -> f64;
    pub fn action_count(&self) -> usize;
    pub fn duration(&self) -> f64;
    pub fn action_types(&self) -> HashMap<String, usize>;
    pub fn filter_by_type(&self, action_type: &str) -> Trace;
    pub fn filter_by_time(&self, from: f64, to: f64) -> Trace;
}
```

### Shadow

```rust
impl Shadow {
    pub fn new() -> Self;
    pub fn record(&mut self, agent_id: &str, action: Action);
    pub fn get_trace(&self, agent_id: &str) -> Option<&Trace>;
    pub fn compare(&self, a: &str, b: &str) -> Option<ComparisonResult>;
    pub fn agent_ids(&self) -> Vec<&str>;
}
```

### ComparisonResult

```rust
pub struct ComparisonResult {
    pub success_diff: f64,
    pub duration_diff: f64,
    pub action_overlap: f64,  // Jaccard similarity of action types
}
```

## How It Fits

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem. Works with:

- **agent-handshake-rs** — begin shadowing after handshake establishes a session
- **agent-identity-rs** — traces are keyed by agent identity
- **agent-dna-rs** — use shadow traces as fitness signals for genetic evolution

## Testing

**6 tests** covering action recording, trace analysis, success rate calculation, behavior comparison, and filtering.

## Installation

```toml
# Cargo.toml
[dependencies]
agent-shadow = { git = "https://github.com/SuperInstance/agent-shadow-rs" }
```

Requires Rust 2021 edition. No external dependencies.
