//! Agent shadowing — monitor, replay, and analyze agent behavior.

use std::collections::HashMap;

/// A recorded action by an agent.
#[derive(Debug, Clone)]
pub struct Action {
    pub timestamp: f64,
    pub action_type: String,
    pub payload: Vec<u8>,
    pub duration: f64,
    pub success: bool,
}

impl Action {
    pub fn new(ts: f64, action_type: &str) -> Self {
        Self { timestamp: ts, action_type: action_type.to_string(), payload: vec![], duration: 0.0, success: true }
    }
    pub fn with_duration(mut self, d: f64) -> Self { self.duration = d; self }
    pub fn with_success(mut self, s: bool) -> Self { self.success = s; self }
}

/// A trace of an agent's behavior over time.
#[derive(Debug, Clone)]
pub struct Trace {
    pub agent_id: String,
    pub actions: Vec<Action>,
    pub start_time: f64,
    pub end_time: f64,
}

impl Trace {
    pub fn new(agent_id: &str) -> Self {
        Self { agent_id: agent_id.to_string(), actions: vec![], start_time: 0.0, end_time: 0.0 }
    }

    pub fn record(&mut self, action: Action) {
        if self.actions.is_empty() { self.start_time = action.timestamp; }
        self.end_time = action.timestamp;
        self.actions.push(action);
    }

    pub fn duration(&self) -> f64 { self.end_time - self.start_time }
    pub fn action_count(&self) -> usize { self.actions.len() }

    pub fn success_rate(&self) -> f64 {
        if self.actions.is_empty() { return 0.0; }
        self.actions.iter().filter(|a| a.success).count() as f64 / self.actions.len() as f64
    }

    pub fn action_types(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for a in &self.actions { *counts.entry(a.action_type.clone()).or_insert(0) += 1; }
        counts
    }

    pub fn filter_by_type(&self, action_type: &str) -> Vec<&Action> {
        self.actions.iter().filter(|a| a.action_type == action_type).collect()
    }

    pub fn avg_duration(&self) -> f64 {
        if self.actions.is_empty() { return 0.0; }
        self.actions.iter().map(|a| a.duration).sum::<f64>() / self.actions.len() as f64
    }
}

/// Shadow mode: silently record without affecting the live agent.
#[derive(Debug, Clone)]
pub struct Shadow {
    pub traces: HashMap<String, Trace>,
    pub max_traces: usize,
}

impl Default for Shadow {
    fn default() -> Self { Self::new() }
}

impl Shadow {
    pub fn new() -> Self { Self { traces: HashMap::new(), max_traces: 100 } }
    pub fn with_max_traces(mut self, n: usize) -> Self { self.max_traces = n; self }

    pub fn record(&mut self, agent_id: &str, action: Action) {
        let trace = self.traces.entry(agent_id.to_string()).or_insert_with(|| Trace::new(agent_id));
        trace.record(action);
        if self.traces.len() > self.max_traces {
            // Remove oldest trace
            if let Some(oldest) = self.traces.iter().min_by(|a, b| a.1.start_time.partial_cmp(&b.1.start_time).unwrap()).map(|(k, _)| k.clone()) {
                self.traces.remove(&oldest);
            }
        }
    }

    pub fn get_trace(&self, agent_id: &str) -> Option<&Trace> { self.traces.get(agent_id) }

    pub fn compare(&self, agent_a: &str, agent_b: &str) -> Option<ComparisonResult> {
        let a = self.traces.get(agent_a)?;
        let b = self.traces.get(agent_b)?;
        Some(ComparisonResult {
            action_diff: a.action_count() as i64 - b.action_count() as i64,
            success_diff: a.success_rate() - b.success_rate(),
            duration_diff: a.avg_duration() - b.avg_duration(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComparisonResult {
    pub action_diff: i64,
    pub success_diff: f64,
    pub duration_diff: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_recording() {
        let mut trace = Trace::new("agent-1");
        trace.record(Action::new(0.0, "search").with_duration(0.5));
        trace.record(Action::new(1.0, "fetch").with_duration(0.3));
        assert_eq!(trace.action_count(), 2);
        assert!((trace.duration() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_success_rate() {
        let mut trace = Trace::new("agent-1");
        trace.record(Action::new(0.0, "a").with_success(true));
        trace.record(Action::new(1.0, "b").with_success(false));
        trace.record(Action::new(2.0, "c").with_success(true));
        assert!((trace.success_rate() - 0.667).abs() < 0.01);
    }

    #[test]
    fn test_action_types() {
        let mut trace = Trace::new("agent-1");
        trace.record(Action::new(0.0, "search"));
        trace.record(Action::new(1.0, "search"));
        trace.record(Action::new(2.0, "fetch"));
        let types = trace.action_types();
        assert_eq!(types.get("search"), Some(&2));
        assert_eq!(types.get("fetch"), Some(&1));
    }

    #[test]
    fn test_shadow_record() {
        let mut shadow = Shadow::new();
        shadow.record("a", Action::new(0.0, "x"));
        shadow.record("a", Action::new(1.0, "y"));
        shadow.record("b", Action::new(0.5, "z"));
        assert_eq!(shadow.get_trace("a").unwrap().action_count(), 2);
        assert_eq!(shadow.get_trace("b").unwrap().action_count(), 1);
    }

    #[test]
    fn test_compare() {
        let mut shadow = Shadow::new();
        shadow.record("a", Action::new(0.0, "x").with_success(true).with_duration(1.0));
        shadow.record("a", Action::new(1.0, "y").with_success(false).with_duration(2.0));
        shadow.record("b", Action::new(0.0, "x").with_success(true).with_duration(0.5));
        let cmp = shadow.compare("a", "b").unwrap();
        assert_eq!(cmp.action_diff, 1);
    }

    #[test]
    fn test_filter() {
        let mut trace = Trace::new("agent-1");
        trace.record(Action::new(0.0, "search"));
        trace.record(Action::new(1.0, "fetch"));
        trace.record(Action::new(2.0, "search"));
        assert_eq!(trace.filter_by_type("search").len(), 2);
    }
}
