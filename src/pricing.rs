#[derive(Clone, Copy, Debug, Default)]
pub struct PriceEstimate {
    pub usd: f64,
    pub priced: bool,
}

#[derive(Clone, Copy, Debug)]
struct PricePerMillion {
    input_usd: f64,
    cached_input_usd: f64,
    output_usd: f64,
}

#[derive(Clone, Copy, Debug)]
struct ClaudePricePerMillion {
    input_usd: f64,
    cache_write_5m_usd: f64,
    cache_write_1h_usd: f64,
    cache_read_usd: f64,
    output_usd: f64,
}

// Official API price points verified on March 25, 2026 from:
// OpenAI pricing: https://openai.com/api/pricing/
// Anthropic pricing/docs: https://www.anthropic.com/pricing and https://docs.anthropic.com/
pub fn estimate_codex_cost(
    model: Option<&str>,
    input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
) -> PriceEstimate {
    let Some(pricing) = codex_pricing(model) else {
        return PriceEstimate::default();
    };

    let cached_tokens = cached_input_tokens.min(input_tokens);
    let uncached_tokens = input_tokens.saturating_sub(cached_tokens);
    PriceEstimate {
        usd: tokens_to_usd(uncached_tokens, pricing.input_usd)
            + tokens_to_usd(cached_tokens, pricing.cached_input_usd)
            + tokens_to_usd(output_tokens, pricing.output_usd),
        priced: true,
    }
}

pub fn estimate_claude_cost(
    model: Option<&str>,
    input_tokens: u64,
    cache_write_5m_tokens: u64,
    cache_write_1h_tokens: u64,
    cache_read_tokens: u64,
    output_tokens: u64,
) -> PriceEstimate {
    let Some(pricing) = claude_pricing(model) else {
        return PriceEstimate::default();
    };

    PriceEstimate {
        usd: tokens_to_usd(input_tokens, pricing.input_usd)
            + tokens_to_usd(cache_write_5m_tokens, pricing.cache_write_5m_usd)
            + tokens_to_usd(cache_write_1h_tokens, pricing.cache_write_1h_usd)
            + tokens_to_usd(cache_read_tokens, pricing.cache_read_usd)
            + tokens_to_usd(output_tokens, pricing.output_usd),
        priced: true,
    }
}

fn tokens_to_usd(tokens: u64, price_per_million: f64) -> f64 {
    (tokens as f64 / 1_000_000.0) * price_per_million
}

fn codex_pricing(model: Option<&str>) -> Option<PricePerMillion> {
    let normalized = model?.trim().to_ascii_lowercase();
    if normalized.contains("codex") && normalized.starts_with("gpt-5") {
        // OpenAI's current Codex-family pricing is aligned across listed GPT-5 Codex variants.
        return Some(PricePerMillion {
            input_usd: 1.25,
            cached_input_usd: 0.125,
            output_usd: 10.0,
        });
    }

    None
}

fn claude_pricing(model: Option<&str>) -> Option<ClaudePricePerMillion> {
    let normalized = model?.trim().to_ascii_lowercase();

    if normalized.contains("haiku") {
        return Some(ClaudePricePerMillion {
            input_usd: 1.0,
            cache_write_5m_usd: 1.25,
            cache_write_1h_usd: 2.0,
            cache_read_usd: 0.10,
            output_usd: 5.0,
        });
    }

    if normalized.contains("sonnet") {
        return Some(ClaudePricePerMillion {
            input_usd: 3.0,
            cache_write_5m_usd: 3.75,
            cache_write_1h_usd: 6.0,
            cache_read_usd: 0.30,
            output_usd: 15.0,
        });
    }

    if normalized.contains("opus") {
        return Some(ClaudePricePerMillion {
            input_usd: 15.0,
            cache_write_5m_usd: 18.75,
            cache_write_1h_usd: 30.0,
            cache_read_usd: 1.50,
            output_usd: 75.0,
        });
    }

    None
}
