/// Finite per-file inspection budgets. Source previews never limit matching text.
#[derive(Debug, Clone, Copy)]
pub struct DetectorBudget {
    pub max_hits_per_rule: usize,
    pub max_match_candidates: usize,
    pub max_ast_candidates: usize,
    pub max_ast_nodes: usize,
    pub max_ast_match_bytes: usize,
}
impl Default for DetectorBudget {
    fn default() -> Self {
        Self {
            max_hits_per_rule: 4096,
            max_match_candidates: 65_536,
            max_ast_candidates: 262_144,
            max_ast_nodes: 2_000_000,
            max_ast_match_bytes: 256 * 1024 * 1024,
        }
    }
}
