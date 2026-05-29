# agent-shadow-rs

Rust port of [agent-shadow](https://github.com/SuperInstance/agent-shadow) — agent behavior monitoring and replay.

## Features

- **Action recording**: timestamped actions with duration and success tracking
- **Trace analysis**: success rate, action type breakdown, filtering
- **Shadow mode**: silently monitor multiple agents without interference
- **Behavior comparison**: compare two agents' traces quantitatively

## Usage

```rust
use agent_shadow::{Shadow, Action};

let mut shadow = Shadow::new();

// Record actions
shadow.record("agent-a", Action::new(0.0, "search").with_duration(0.5).with_success(true));
shadow.record("agent-a", Action::new(1.0, "fetch").with_duration(0.3).with_success(true));
shadow.record("agent-b", Action::new(0.0, "search").with_duration(0.8).with_success(false));

// Analyze
let trace = shadow.get_trace("agent-a").unwrap();
println!("Success rate: {:.1}%", trace.success_rate() * 100.0);
println!("Actions: {:?}", trace.action_types());

// Compare agents
let cmp = shadow.compare("agent-a", "agent-b").unwrap();
println!("Success rate diff: {:.3}", cmp.success_diff);
```

## License

MIT

Part of the [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct) ecosystem.
