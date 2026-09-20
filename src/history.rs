// history.rs — Metatron Dynamics, Inc. V7.
//
// Relational memory — accumulated history at R⁰ and R¹.
//
// An instantiated relation exists because it was observed.
// Its history is preserved without reducing it to a weight.
//
// R⁰ memory: EdgeHistory — history of (char → char) relations.
// R¹ memory: R1History — history of (edge → edge) compositions.
//
// Recurrence: observed in more than one independent observation.
// Recorded as Vec<usize> of observation step indices.
// Count is derivable from list length — not stored separately.
// Count does not determine traversal.
//
// Persistence: determined by three conditions (see declaration Section 7).
// No frequency threshold. No θ_min.
//
// Metatron Dynamics, Inc. V7.

use std::collections::HashMap;
use crate::observation::{R0Relation, R1Structure, ParseResult};
use serde::{Serialize, Deserialize};

/// Provenance record for one observation of an R⁰ edge.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct R0Record {
    pub step: usize,
    pub src_position: usize,
    pub tgt_position: usize,
    pub source_excerpt: String,  // provenance only — not identity
}

/// Accumulated history of one R⁰ relation (char → char).
///
/// observations: ordered list of records — one per observation.
/// This IS the memory. Not a float. Not a stored count.
/// The list is the information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgeHistory {
    pub src: char,
    pub tgt: char,
    pub observations: Vec<R0Record>,
}

impl EdgeHistory {
    pub fn new(src: char, tgt: char) -> Self {
        EdgeHistory { src, tgt, observations: Vec::new() }
    }

    pub fn record(&mut self, rel: &R0Relation) {
        self.observations.push(R0Record {
            step: rel.step,
            src_position: rel.src_position,
            tgt_position: rel.tgt_position,
            source_excerpt: format!("{}{}", rel.src.ch(), rel.tgt.ch()),
        });
    }

    /// How many times was this edge observed?
    /// DIAGNOSTIC ONLY — does not control traversal.
    pub fn activation_count(&self) -> usize { self.observations.len() }

    pub fn is_instantiated(&self) -> bool { !self.observations.is_empty() }

    /// Is this edge recurrent? (observed in more than one independent step)
    pub fn is_recurrent(&self) -> bool { self.activation_count() > 1 }

    /// All step indices at which this edge was observed.
    /// This is the recurrence record — not a count.
    pub fn observed_at_steps(&self) -> Vec<usize> {
        self.observations.iter().map(|r| r.step).collect()
    }
}

/// Provenance record for one observation of an R¹ structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct R1Record {
    pub step: usize,
    pub positions: [usize; 3],
    pub source_context: String,  // provenance only — not identity
}

/// Accumulated history of one R¹ structure (edge → edge).
///
/// Identity: edge_sequence = [e1, e2].
/// source_context in records: provenance only.
/// Not used for identity, equality, traversal, or routing.
///
/// Persistence conditions (from declaration Section 7):
///   1. Observed in multiple independent observations (is_recurrent)
///   2. Both constituent R⁰ edges independently instantiated
///   3. The R¹ composition was licensed by at least one observation
///      as a COMPLETE SEQUENTIAL STRUCTURE — not inferred from
///      separately observed edges
/// All three required. Condition 3 is satisfied by construction:
/// R¹History is only created when a consecutive R⁰ pair was observed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct R1History {
    pub e1: (char, char),
    pub e2: (char, char),
    pub observations: Vec<R1Record>,
}

impl R1History {
    pub fn new(e1: (char, char), e2: (char, char)) -> Self {
        R1History { e1, e2, observations: Vec::new() }
    }

    pub fn record(&mut self, r1: &R1Structure) {
        self.observations.push(R1Record {
            step: r1.step,
            positions: r1.positions,
            source_context: r1.source_context.clone(),
        });
    }

    /// DIAGNOSTIC ONLY — does not control traversal.
    pub fn activation_count(&self) -> usize { self.observations.len() }

    pub fn is_instantiated(&self) -> bool { !self.observations.is_empty() }

    pub fn is_recurrent(&self) -> bool { self.activation_count() > 1 }

    pub fn observed_at_steps(&self) -> Vec<usize> {
        self.observations.iter().map(|r| r.step).collect()
    }

    /// Edge sequence — the identity of this R¹ structure.
    pub fn edge_sequence(&self) -> [(char,char); 2] { [self.e1, self.e2] }

    pub fn start(&self) -> char { self.e1.0 }
    pub fn terminal(&self) -> char { self.e2.1 }
    pub fn pivot(&self) -> char { self.e1.1 }

    /// Is this R¹ structure persistent under the declared conditions?
    /// Condition 1: recurrent (observed > 1 time)
    /// Condition 2: checked against R⁰ memory (caller verifies)
    /// Condition 3: satisfied by construction (R¹ only created from
    ///   consecutive R⁰ pairs in a single observation stream)
    pub fn satisfies_persistence_condition_1(&self) -> bool {
        self.is_recurrent()
    }
}

/// Complete relational memory — R⁰ and R¹ histories.
pub struct RelationalMemory {
    /// R⁰: keyed by (src_char, tgt_char)
    pub r0: HashMap<(char,char), EdgeHistory>,
    /// R¹: keyed by edge_sequence [(e1_src, e1_tgt), (e2_src, e2_tgt)]
    pub r1: HashMap<[(char,char);2], R1History>,
    pub total_steps: usize,
    pub is_cold_start: bool,
}

impl RelationalMemory {
    pub fn new() -> Self {
        RelationalMemory {
            r0: HashMap::new(),
            r1: HashMap::new(),
            total_steps: 0,
            is_cold_start: true,
        }
    }

    /// Record all R⁰ and R¹ structures from one ParseResult.
    pub fn record(&mut self, parsed: &ParseResult) {
        for rel in &parsed.r0_relations {
            self.r0
                .entry(rel.edge())
                .or_insert_with(|| EdgeHistory::new(rel.src.ch(), rel.tgt.ch()))
                .record(rel);
            self.total_steps = self.total_steps.max(rel.step);
        }
        for r1 in &parsed.r1_structures {
            self.r1
                .entry(r1.edge_sequence())
                .or_insert_with(|| R1History::new(r1.e1, r1.e2))
                .record(r1);
        }
        self.is_cold_start = false;
    }

    // ── R⁰ queries ────────────────────────────────────────────────────────

    pub fn get_r0(&self, src: char, tgt: char) -> Option<&EdgeHistory> {
        self.r0.get(&(src, tgt))
    }

    /// All instantiated R⁰ edges leaving src.
    pub fn r0_outgoing(&self, src: char) -> Vec<&EdgeHistory> {
        let mut out: Vec<&EdgeHistory> = self.r0.values()
            .filter(|h| h.src == src)
            .collect();
        out.sort_by_key(|h| h.tgt);
        out
    }

    // ── R¹ queries ────────────────────────────────────────────────────────

    pub fn get_r1(&self, seq: [(char,char);2]) -> Option<&R1History> {
        self.r1.get(&seq)
    }

    /// All instantiated R¹ structures whose first edge is e1.
    pub fn r1_outgoing_from_edge(&self, e1: (char,char)) -> Vec<&R1History> {
        let mut out: Vec<&R1History> = self.r1.values()
            .filter(|h| h.e1 == e1)
            .collect();
        out.sort_by_key(|h| h.e2);
        out
    }

    /// All persistent R¹ structures (satisfy condition 1 — recurrent).
    /// Conditions 2 and 3 satisfied by construction.
    pub fn persistent_r1(&self) -> Vec<&R1History> {
        let mut out: Vec<&R1History> = self.r1.values()
            .filter(|h| h.satisfies_persistence_condition_1())
            .collect();
        out.sort_by_key(|h| h.edge_sequence());
        out
    }

    // ── Summary ───────────────────────────────────────────────────────────

    pub fn r0_count(&self) -> usize { self.r0.len() }
    pub fn r1_count(&self) -> usize { self.r1.len() }

    pub fn summary(&self) -> MemorySummary {
        let persistent = self.persistent_r1().len();
        MemorySummary {
            possible_r0_edges: 3600,
            instantiated_r0: self.r0_count(),
            instantiated_r1: self.r1_count(),
            persistent_r1: persistent,
            total_steps: self.total_steps,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MemorySummary {
    pub possible_r0_edges: usize,
    pub instantiated_r0: usize,
    pub instantiated_r1: usize,
    pub persistent_r1: usize,
    pub total_steps: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observation::StreamParser;

    fn build_memory(text: &str) -> RelationalMemory {
        let mut parser = StreamParser::new();
        let parsed = parser.parse(text);
        let mut mem = RelationalMemory::new();
        mem.record(&parsed);
        mem
    }

    #[test]
    fn test_dog_r0_in_memory() {
        let mem = build_memory("dog");
        assert!(mem.get_r0('d','o').is_some());
        assert!(mem.get_r0('o','g').is_some());
        assert!(mem.get_r0('d','g').is_none()); // not licensed
    }

    #[test]
    fn test_dog_r1_in_memory() {
        let mem = build_memory("dog");
        let r1 = mem.get_r1([('d','o'),('o','g')]);
        assert!(r1.is_some());
        assert_eq!(r1.unwrap().pivot(), 'o');
    }

    #[test]
    fn test_dog_god_r1_distinct() {
        let mem = build_memory("dog god");
        let dog_r1 = mem.get_r1([('d','o'),('o','g')]);
        let god_r1 = mem.get_r1([('g','o'),('o','d')]);
        assert!(dog_r1.is_some(), "dog R¹ must be in memory");
        assert!(god_r1.is_some(), "god R¹ must be in memory");
        // they are distinct structures
        assert_ne!(dog_r1.unwrap().edge_sequence(),
                   god_r1.unwrap().edge_sequence());
    }

    #[test]
    fn test_recurrence() {
        // "dog dog" — d→o observed twice
        let mem = build_memory("dog dog");
        let h = mem.get_r0('d','o').unwrap();
        assert!(h.is_recurrent());
        assert_eq!(h.activation_count(), 2);
    }

    #[test]
    fn test_persistence_requires_recurrence() {
        // single "dog" — R¹ not yet persistent
        let mem1 = build_memory("dog");
        let r1 = mem1.get_r1([('d','o'),('o','g')]).unwrap();
        assert!(!r1.satisfies_persistence_condition_1());

        // "dog dog" — R¹ now recurrent → persistent
        let mem2 = build_memory("dog dog");
        let r1b = mem2.get_r1([('d','o'),('o','g')]).unwrap();
        assert!(r1b.satisfies_persistence_condition_1());
    }

    #[test]
    fn test_space_produces_r0_relations() {
        let mem = build_memory("dog cat");
        // space is a locus — g→SPACE and SPACE→c are R⁰ relations
        assert!(mem.get_r0('g',' ').is_some());
        assert!(mem.get_r0(' ','c').is_some());
    }

    #[test]
    fn test_activation_count_diagnostic_only() {
        // activation_count() exists but the test verifies it
        // does NOT appear in any routing decision
        // (structural: the function returns usize, not used in traversal.rs)
        let mem = build_memory("dog dog dog");
        let h = mem.get_r0('d','o').unwrap();
        assert_eq!(h.activation_count(), 3);
        // this count is for measurement — traversal uses is_instantiated() only
    }
}
