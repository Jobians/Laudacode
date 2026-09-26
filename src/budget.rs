//! Token/cost accounting: the `[limits]` ceilings and the model-aware
//! auto-compact threshold.

use crate::config::Limits;

/// Rough per-1M-token pricing in USD (blended in/out assumption), used to
/// turn token usage into a dollar figure for `[limits] max_cost_usd`.
/// ponytail: one blended rate, not a per-model price table — a real table
/// needs a network lookup and this is a guardrail, not billing.
const USD_PER_1M_TOKENS: f64 = 0.75;

/// Estimated cost in USD for the given cumulative token counts.
pub fn estimate_cost(prompt_tokens: u64, completion_tokens: u64) -> f64 {
    let total = prompt_tokens.saturating_add(completion_tokens) as f64;
    (total / 1_000_000.0) * USD_PER_1M_TOKENS
}

/// The ceiling that has been crossed, with a sentence telling the user which
/// one to raise. `None` = no ceiling set, or still under both.
pub fn exceeded(l: &Limits, prompt: u64, completion: u64) -> Option<String> {
    let path = crate::config::Config::toml_path();
    let path = path.display();
    if let Some(cap) = l.max_tokens {
        let total = prompt.saturating_add(completion);
        if total > cap {
            return Some(format!(
                "token budget reached: {total} of {cap} tokens used. \
                 Raise `[limits] max_tokens` in {path} to continue, or start a fresh session."
            ));
        }
    }
    if let Some(cap) = l.max_cost_usd {
        let cost = estimate_cost(prompt, completion);
        if cost > cap {
            return Some(format!(
                "cost budget reached: ~${cost:.2} of ~${cap:.2} used. \
                 Raise `[limits] max_cost_usd` in {path} to continue."
            ));
        }
    }
    None
}

/// Fraction of the context window at which auto-compaction fires.
pub const COMPACT_AT_FRACTION: f64 = 0.8;

/// Model-aware compact threshold: 80% of the assumed context window, with a
/// sane floor so tiny advertised windows don't thrash.
pub fn compact_threshold(ctx_window: u64) -> u64 {
    let threshold = (ctx_window as f64 * COMPACT_AT_FRACTION) as u64;
    threshold.max(4_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits(max_tokens: Option<u64>, max_cost_usd: Option<f64>) -> Limits {
        Limits {
            max_tokens,
            max_cost_usd,
            ..Default::default()
        }
    }

    #[test]
    fn no_ceilings_never_blocks() {
        assert!(exceeded(&Limits::default(), 10_000_000, 10_000_000).is_none());
        assert!(exceeded(&limits(Some(1_000), None), 600, 300).is_none());
        assert!(exceeded(&limits(None, Some(0.001)), 10, 10).is_none());
    }

    #[test]
    fn token_ceiling_blocks_past_the_cap() {
        let l = limits(Some(1_000), None);
        // total 1100 > 1000
        let msg = exceeded(&l, 600, 500).expect("blocked");
        assert!(msg.contains("token budget reached"), "{msg}");
        assert!(msg.contains("max_tokens"), "{msg}");
    }

    #[test]
    fn cost_ceiling_blocks_past_the_cap() {
        // ~$0.75 per 1M blended tokens.
        let l = limits(None, Some(0.0005));
        let msg = exceeded(&l, 1_000, 0).expect("blocked");
        assert!(msg.contains("cost budget reached"), "{msg}");
        assert!(msg.contains("max_cost_usd"), "{msg}");
    }

    #[test]
    fn estimate_scales_with_tokens() {
        assert!(estimate_cost(10_000, 10_000) > estimate_cost(1_000, 1_000));
        assert!(estimate_cost(0, 0).abs() < f64::EPSILON);
    }

    #[test]
    fn compact_threshold_is_fraction_with_floor() {
        assert_eq!(compact_threshold(128_000), 102_400);
        // A tiny window still gets a usable floor to avoid thrash.
        assert_eq!(compact_threshold(1_000), 4_000);
    }
}
