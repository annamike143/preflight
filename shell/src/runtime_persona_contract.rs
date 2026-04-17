use serde::{Deserialize, Serialize};

pub const RUNTIME_PERSONA_CONTRACT_NAME: &str = "RuntimePersonaContract";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimePersonaContractStage {
    FoundationMaterialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct RuntimePersonaContractSnapshot {
    pub stage: RuntimePersonaContractStage,
    pub contract_name: &'static str,
    pub engine_owned_persona_generation: bool,
    pub runtime_generated_from_seed_context: bool,
    pub exactly_one_moderator_required: bool,
    pub stable_persona_labels_required: bool,
    pub favorable_perspective_required: bool,
    pub skeptical_perspective_required: bool,
    pub analytical_or_neutral_perspective_required: bool,
    pub static_user_authored_persona_configuration_allowed: bool,
    pub runtime_persona_generation_materialized: bool,
    pub moderator_basis_materialized: bool,
    pub participant_labeling_materialized: bool,
    pub persona_outputs_attributable: bool,
    pub full_moderated_rounds_implied: bool,
}

impl RuntimePersonaContractSnapshot {
    pub fn summary(self) -> String {
        format!(
            "Runtime persona contract surface is materialized. Stage is {:?}; contract name is {}; engine-owned persona generation is {}; runtime-generated from seed context is {}; exactly one Moderator required is {}; stable persona labels required is {}; favorable perspective required is {}; skeptical perspective required is {}; analytical-or-neutral perspective required is {}; static user-authored persona configuration allowed is {}; runtime persona generation materialized is {}; Moderator basis materialized is {}; participant labeling materialized is {}; persona outputs attributable is {}; full moderated rounds implied is {}.",
            self.stage,
            self.contract_name,
            self.engine_owned_persona_generation,
            self.runtime_generated_from_seed_context,
            self.exactly_one_moderator_required,
            self.stable_persona_labels_required,
            self.favorable_perspective_required,
            self.skeptical_perspective_required,
            self.analytical_or_neutral_perspective_required,
            self.static_user_authored_persona_configuration_allowed,
            self.runtime_persona_generation_materialized,
            self.moderator_basis_materialized,
            self.participant_labeling_materialized,
            self.persona_outputs_attributable,
            self.full_moderated_rounds_implied
        )
    }
}

pub fn runtime_persona_contract_surface() -> RuntimePersonaContractSnapshot {
    RuntimePersonaContractSnapshot {
        stage: RuntimePersonaContractStage::FoundationMaterialized,
        contract_name: RUNTIME_PERSONA_CONTRACT_NAME,
        engine_owned_persona_generation: true,
        runtime_generated_from_seed_context: true,
        exactly_one_moderator_required: true,
        stable_persona_labels_required: true,
        favorable_perspective_required: true,
        skeptical_perspective_required: true,
        analytical_or_neutral_perspective_required: true,
        static_user_authored_persona_configuration_allowed: false,
        runtime_persona_generation_materialized: true,
        moderator_basis_materialized: true,
        participant_labeling_materialized: true,
        persona_outputs_attributable: true,
        full_moderated_rounds_implied: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_persona_contract_surface_requires_seed_driven_personas_and_single_moderator() {
        let snapshot = runtime_persona_contract_surface();

        assert_eq!(snapshot.contract_name, RUNTIME_PERSONA_CONTRACT_NAME);
        assert!(snapshot.engine_owned_persona_generation);
        assert!(snapshot.runtime_generated_from_seed_context);
        assert!(snapshot.exactly_one_moderator_required);
        assert!(snapshot.stable_persona_labels_required);
        assert!(snapshot.favorable_perspective_required);
        assert!(snapshot.skeptical_perspective_required);
        assert!(snapshot.analytical_or_neutral_perspective_required);
        assert!(!snapshot.static_user_authored_persona_configuration_allowed);
        assert!(snapshot.runtime_persona_generation_materialized);
        assert!(snapshot.moderator_basis_materialized);
        assert!(snapshot.participant_labeling_materialized);
        assert!(snapshot.persona_outputs_attributable);
        assert!(snapshot.full_moderated_rounds_implied);
    }
}
