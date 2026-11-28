#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum PartialCause {
    ParserRecovery = 1,
    ConditionalSource = 2,
    UnresolvedCryptoBinding = 4,
    OtherSourceUncertainty = 8,
    OpaqueMacro = 16,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct PartialCauses(u8);
impl PartialCauses {
    pub fn insert(&mut self, cause: PartialCause) {
        self.0 |= cause as u8;
    }
    pub fn contains(self, cause: PartialCause) -> bool {
        self.0 & cause as u8 != 0
    }
    pub fn remove(&mut self, cause: PartialCause) {
        self.0 &= !(cause as u8);
    }
    pub fn extend(&mut self, causes: Self) {
        self.0 |= causes.0;
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub fn explanation(self) -> &'static str {
        static MESSAGES: std::sync::LazyLock<[String; 32]> = std::sync::LazyLock::new(|| {
            std::array::from_fn(|bits| {
                let causes = PartialCauses(bits as u8);
                let mut message = String::from("Source inspection incomplete. ");
                for cause in [
                    PartialCause::ParserRecovery,
                    PartialCause::ConditionalSource,
                    PartialCause::UnresolvedCryptoBinding,
                    PartialCause::OpaqueMacro,
                    PartialCause::OtherSourceUncertainty,
                ] {
                    if causes.contains(cause)
                        || (bits == 0 && cause == PartialCause::OtherSourceUncertainty)
                    {
                        message.push_str(match cause {
                            PartialCause::ParserRecovery => "Parser recovery or missing syntax tokens: parsed regions were inspected; unresolved syntax remains unknown. ",
                            PartialCause::ConditionalSource => "Conditional source: active branches and build definitions remain unknown. ",
                            PartialCause::UnresolvedCryptoBinding => "Unresolved supported crypto binding: the receiver or selected algorithm remains unknown. ",
                            PartialCause::OpaqueMacro => "Opaque macro syntax: source references were retained; macro expansion and build semantics remain unknown. ",
                            PartialCause::OtherSourceUncertainty => "Other source uncertainty: some supported source facts could not be resolved. ",
                        });
                    }
                }
                message.push_str("Review the affected source and build configuration separately.");
                message
            })
        });
        &MESSAGES[self.0 as usize]
    }
}
impl From<PartialCause> for PartialCauses {
    fn from(cause: PartialCause) -> Self {
        Self(cause as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn combined_explanations_preserve_each_cause_in_stable_order() {
        let mut causes = PartialCauses::from(PartialCause::UnresolvedCryptoBinding);
        causes.insert(PartialCause::ConditionalSource);
        let text = causes.explanation();
        assert!(text.contains("Conditional source:"));
        assert!(text.contains("Unresolved supported crypto binding:"));
        assert!(!text.contains("Parser recovery"));
        assert!(
            text.find("Conditional source:") < text.find("Unresolved supported crypto binding:")
        );
        causes.insert(PartialCause::ParserRecovery);
        assert!(causes
            .explanation()
            .contains("Parser recovery or missing syntax tokens:"));
        causes.remove(PartialCause::ParserRecovery);
        causes.insert(PartialCause::OpaqueMacro);
        assert!(!causes.explanation().contains("Parser recovery"));
        assert!(causes.explanation().contains("Opaque macro syntax:"));
        assert!(causes.explanation().contains("Conditional source:"));
    }
    #[test]
    fn every_finite_cause_combination_has_a_safe_nonempty_explanation() {
        for bits in 0..32 {
            let message = PartialCauses(bits).explanation();
            assert!(message.starts_with("Source inspection incomplete. "));
            assert!(
                message.ends_with("Review the affected source and build configuration separately.")
            );
            assert!(!message.contains("\n"));
        }
    }
}
