use crate::preflight_contract::{
    PreflightEffectivePlanSnapshot, PreflightEstimatedUsageSnapshot, PreflightRejectionReason,
    PreflightRequestedPlanSnapshot,
};
use crate::provider_configuration_contract::SupportedProvider;

pub const MIN_EFFECTIVE_AGENT_COUNT: u32 = 3;
pub const MAX_EFFECTIVE_AGENT_COUNT: u32 = 5;
pub const HARD_DURATION_CAP_MINUTES: u32 = 120;
pub const BOUNDED_SPEAKER_MODEL_LABEL: &str = "moderated_3_to_5_speakers_per_round";
pub const BOUNDED_CONCURRENCY_POLICY_LABEL: &str = "single_moderated_speaker_set_per_round";
pub const OPENAI_DEFAULT_MODEL: &str = "gpt-4o-mini";
pub const GOOGLE_DEFAULT_MODEL: &str = "gemini-1.5-flash";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightPlanDerivationOutcome {
    pub effective_plan: PreflightEffectivePlanSnapshot,
    pub estimated_usage: PreflightEstimatedUsageSnapshot,
    pub requested_plan_budget_feasible: bool,
    pub requested_plan_duration_feasible: bool,
    pub downgrade_warning: Option<String>,
    pub is_downgraded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProviderPlanningConstants {
    startup_tokens: u64,
    per_agent_round_tokens: u64,
    moderator_tokens_per_round: u64,
    minutes_low_per_round: u32,
    minutes_high_per_round: u32,
}

const OPENAI_PLANNING_CONSTANTS: ProviderPlanningConstants = ProviderPlanningConstants {
    startup_tokens: 1_500,
    per_agent_round_tokens: 430,
    moderator_tokens_per_round: 220,
    minutes_low_per_round: 7,
    minutes_high_per_round: 11,
};

const GOOGLE_PLANNING_CONSTANTS: ProviderPlanningConstants = ProviderPlanningConstants {
    startup_tokens: 1_300,
    per_agent_round_tokens: 380,
    moderator_tokens_per_round: 180,
    minutes_low_per_round: 6,
    minutes_high_per_round: 10,
};

pub fn default_model_for_provider(provider: SupportedProvider) -> &'static str {
    match provider {
        SupportedProvider::OpenAi => OPENAI_DEFAULT_MODEL,
        SupportedProvider::Google => GOOGLE_DEFAULT_MODEL,
    }
}

pub fn derive_effective_plan(
    provider: SupportedProvider,
    requested_plan: &PreflightRequestedPlanSnapshot,
) -> Result<PreflightPlanDerivationOutcome, PreflightRejectionReason> {
    if requested_plan.requested_agent_count < MIN_EFFECTIVE_AGENT_COUNT {
        return Err(PreflightRejectionReason::RequestedAgentCountBelowMinimum);
    }

    let requested_usage = estimate_usage(
        provider,
        requested_plan.requested_agent_count,
        requested_plan.requested_round_count,
    );
    let requested_plan_budget_feasible =
        requested_usage.estimated_total_tokens <= requested_plan.max_token_budget;
    let requested_plan_duration_feasible =
        requested_usage.estimated_duration_minutes_high <= HARD_DURATION_CAP_MINUTES;

    let mut effective_agent_count = requested_plan.requested_agent_count.min(MAX_EFFECTIVE_AGENT_COUNT);
    let mut effective_round_count = requested_plan.requested_round_count;

    loop {
        let estimated_usage = estimate_usage(provider, effective_agent_count, effective_round_count);
        let budget_feasible = estimated_usage.estimated_total_tokens <= requested_plan.max_token_budget;
        let duration_feasible =
            estimated_usage.estimated_duration_minutes_high <= HARD_DURATION_CAP_MINUTES;

        if budget_feasible && duration_feasible {
            let effective_plan = PreflightEffectivePlanSnapshot {
                effective_agent_count,
                effective_round_count,
                provider,
                model: default_model_for_provider(provider).to_string(),
                max_token_budget: requested_plan.max_token_budget,
                bounded_speaker_model: BOUNDED_SPEAKER_MODEL_LABEL.to_string(),
                bounded_concurrency_policy: BOUNDED_CONCURRENCY_POLICY_LABEL.to_string(),
            };
            let is_downgraded = effective_agent_count != requested_plan.requested_agent_count
                || effective_round_count != requested_plan.requested_round_count;

            return Ok(PreflightPlanDerivationOutcome {
                effective_plan,
                estimated_usage,
                requested_plan_budget_feasible,
                requested_plan_duration_feasible,
                downgrade_warning: is_downgraded.then(|| {
                    build_downgrade_warning(
                        requested_plan,
                        effective_agent_count,
                        effective_round_count,
                        requested_plan_budget_feasible,
                        requested_plan_duration_feasible,
                    )
                }),
                is_downgraded,
            });
        }

        if effective_round_count > 1 {
            effective_round_count -= 1;
            continue;
        }

        if effective_agent_count > MIN_EFFECTIVE_AGENT_COUNT {
            effective_agent_count -= 1;
            continue;
        }

        return Err(PreflightRejectionReason::BudgetTooLowForMinimumPlan);
    }
}

fn estimate_usage(
    provider: SupportedProvider,
    effective_agent_count: u32,
    effective_round_count: u32,
) -> PreflightEstimatedUsageSnapshot {
    let constants = planning_constants(provider);
    let per_round_tokens = (effective_agent_count as u64 * constants.per_agent_round_tokens)
        + constants.moderator_tokens_per_round;

    PreflightEstimatedUsageSnapshot {
        estimated_total_tokens: constants.startup_tokens
            + (per_round_tokens * effective_round_count as u64),
        estimated_duration_minutes_low: constants.minutes_low_per_round * effective_round_count,
        estimated_duration_minutes_high: constants.minutes_high_per_round * effective_round_count,
    }
}

fn planning_constants(provider: SupportedProvider) -> ProviderPlanningConstants {
    match provider {
        SupportedProvider::OpenAi => OPENAI_PLANNING_CONSTANTS,
        SupportedProvider::Google => GOOGLE_PLANNING_CONSTANTS,
    }
}

fn build_downgrade_warning(
    requested_plan: &PreflightRequestedPlanSnapshot,
    effective_agent_count: u32,
    effective_round_count: u32,
    requested_plan_budget_feasible: bool,
    requested_plan_duration_feasible: bool,
) -> String {
    let mut reasons = Vec::new();

    if requested_plan.requested_agent_count > MAX_EFFECTIVE_AGENT_COUNT {
        reasons.push("the requested agent count exceeded the locked 3-to-5 speaker model");
    }

    if !requested_plan_budget_feasible {
        reasons.push("the requested budget could not cover the bounded local estimate");
    }

    if !requested_plan_duration_feasible {
        reasons.push("the requested scope exceeded the hard two-hour duration cap");
    }

    let reason_text = if reasons.is_empty() {
        "the bounded MVP effective plan is smaller than the requested scope".to_string()
    } else {
        reasons.join("; ")
    };

    format!(
        "Requested plan was downgraded before run start. Effective agent count is {}; effective round count is {}; reason: {}.",
        effective_agent_count,
        effective_round_count,
        reason_text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effective_plan_can_approve_requested_scope_without_downgrade() {
        let requested_plan = PreflightRequestedPlanSnapshot {
            requested_agent_count: 5,
            requested_round_count: 4,
            max_token_budget: 12_000,
        };

        let outcome = derive_effective_plan(SupportedProvider::OpenAi, &requested_plan)
            .expect("plan should be feasible");

        assert_eq!(outcome.effective_plan.effective_agent_count, 5);
        assert_eq!(outcome.effective_plan.effective_round_count, 4);
        assert_eq!(outcome.effective_plan.model, OPENAI_DEFAULT_MODEL);
        assert!(outcome.requested_plan_budget_feasible);
        assert!(outcome.requested_plan_duration_feasible);
        assert!(!outcome.is_downgraded);
        assert!(outcome.downgrade_warning.is_none());
    }

    #[test]
    fn effective_plan_can_downgrade_requested_scope_to_fit_budget() {
        let requested_plan = PreflightRequestedPlanSnapshot {
            requested_agent_count: 7,
            requested_round_count: 8,
            max_token_budget: 12_000,
        };

        let outcome = derive_effective_plan(SupportedProvider::OpenAi, &requested_plan)
            .expect("plan should be downgraded rather than rejected");

        assert_eq!(outcome.effective_plan.effective_agent_count, 5);
        assert!(outcome.effective_plan.effective_round_count < 8);
        assert!(outcome.estimated_usage.estimated_total_tokens <= 12_000);
        assert!(outcome.is_downgraded);
        assert!(outcome.downgrade_warning.is_some());
    }

    #[test]
    fn effective_plan_rejects_budget_below_minimum_bounded_scope() {
        let requested_plan = PreflightRequestedPlanSnapshot {
            requested_agent_count: 3,
            requested_round_count: 1,
            max_token_budget: 2_000,
        };

        let error = derive_effective_plan(SupportedProvider::Google, &requested_plan)
            .expect_err("plan should be rejected");

        assert_eq!(error, PreflightRejectionReason::BudgetTooLowForMinimumPlan);
    }
}
