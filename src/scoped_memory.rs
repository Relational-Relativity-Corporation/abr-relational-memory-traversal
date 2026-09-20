// scoped_memory.rs — Metatron Dynamics, Inc. V7.
//
// Two declared memory scopes for V0.3.0.
//
// long_term: accumulates across all observations and conversations.
//   Never reset by X*X*X. Source of persistent historical experience H_L.
//
// conversation: accumulates within the current conversation only.
//   Reset to empty when X*X*X is observed.
//   Source of immediate conversational experience H_C.
//
// Both scopes use the identical EdgeHistory structure from V0.1.1.
// No new scalar fields. No ρ_conv, ρ_long, η, λ, or combining function.
//
// Declared boundary token: X*X*X
//   Not a locus. Not passed to any operator.
//   When observed: resets conversation scope only.
//   Long-term scope is unaffected.
//
// No selection mechanism. No scalar encoding of history.
// NOT OBSERVED is admissible at every measurement.
//
// Metatron Dynamics, Inc. V7.


use crate::locus::is_declared;
use crate::observation::{R0Relation, R1Structure};
use crate::history::RelationalMemory;

// ── Boundary token ────────────────────────────────────────────────────────

/// The declared conversation boundary token.
/// When this exact sequence appears in the input stream, it triggers
/// a conversation scope reset. It is not a locus. It produces no edges.
pub const BOUNDARY_TOKEN: &str = "X*X*X";

/// A stream event — either a character observation or a boundary.
#[derive(Clone, Debug)]
pub enum StreamEvent {
    /// A declared locus character at a stream position.
    Character { ch: char, position: usize, step: usize },
    /// The X*X*X boundary token. Not a locus.
    ConversationEnd { position: usize },
    /// A character outside D — recorded, not processed.
    Unresolved { ch: char, position: usize },
}

/// Parse a raw text stream into StreamEvents.
/// Recognizes X*X*X before character-level processing.
/// X*X*X produces ConversationEnd — not five Character events.
pub fn parse_stream(text: &str) -> Vec<StreamEvent> {
    let mut events = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let boundary: Vec<char> = BOUNDARY_TOKEN.chars().collect();
    let blen = boundary.len();
    let mut i = 0usize;
    let mut step = 1usize;
    let mut stream_pos = 0usize;

    while i < chars.len() {
        // Check for boundary token
        if i + blen <= chars.len() && chars[i..i+blen] == boundary[..] {
            events.push(StreamEvent::ConversationEnd { position: stream_pos });
            i += blen;
            stream_pos += blen;
            continue;
        }

        let ch = chars[i];
        if is_declared(ch) {
            events.push(StreamEvent::Character { ch, position: stream_pos, step });
            step += 1;
        } else {
            events.push(StreamEvent::Unresolved { ch, position: stream_pos });
        }
        i += 1;
        stream_pos += 1;
    }

    events
}

/// Extract consecutive R⁰ relations from a sequence of Character events.
/// ConversationEnd does not interrupt the locus sequence for R⁰ purposes —
/// the boundary resets the conversation scope but does not produce an edge.
pub fn r0_from_events(events: &[StreamEvent]) -> Vec<R0Relation> {
    use crate::locus::Locus;
    let mut relations = Vec::new();
    let mut prev: Option<(char, usize, usize)> = None; // (ch, pos, step)

    for event in events {
        match event {
            StreamEvent::Character { ch, position, step } => {
                if let Some((prev_ch, prev_pos, prev_step)) = prev {
                    relations.push(R0Relation {
                        src: Locus::new(prev_ch),
                        tgt: Locus::new(*ch),
                        src_position: prev_pos,
                        tgt_position: *position,
                        step: prev_step,
                    });
                }
                prev = Some((*ch, *position, *step));
            }
            StreamEvent::ConversationEnd { .. } => {
                // Boundary does not produce an edge and breaks the chain
                prev = None;
            }
            StreamEvent::Unresolved { .. } => {
                // Unresolved character breaks the chain
                prev = None;
            }
        }
    }

    relations
}

/// Extract R¹ structures from R⁰ relations.
pub fn r1_from_r0(r0_relations: &[R0Relation]) -> Vec<R1Structure> {
    let mut structures = Vec::new();
    for window in r0_relations.windows(2) {
        let (e1, e2) = (&window[0], &window[1]);
        if e1.tgt == e2.src {
            let context: String = [e1.src.ch(), e1.tgt.ch(), e2.tgt.ch()].iter().collect();
            structures.push(R1Structure {
                e1: e1.edge(),
                e2: e2.edge(),
                positions: [e1.src_position, e1.tgt_position, e2.tgt_position],
                step: e1.step,
                source_context: context,
            });
        }
    }
    structures
}

// ── Scoped relational memory ──────────────────────────────────────────────

/// Two-scope relational memory.
///
/// long_term: persists across X*X*X boundaries.
/// conversation: resets at each X*X*X boundary.
///
/// Both scopes use identical RelationalMemory from V0.1.1.
/// No scalar fields added. No combining function.
pub struct ScopedRelationalMemory {
    pub long_term: RelationalMemory,
    pub conversation: RelationalMemory,
    pub boundary_count: usize,       // how many X*X*X tokens observed
    pub total_steps: usize,
}

impl ScopedRelationalMemory {
    pub fn new() -> Self {
        ScopedRelationalMemory {
            long_term: RelationalMemory::new(),
            conversation: RelationalMemory::new(),
            boundary_count: 0,
            total_steps: 0,
        }
    }

    /// Process a full text stream through both scopes.
    /// X*X*X resets conversation but not long_term.
    pub fn process_stream(&mut self, text: &str) {
        let events = parse_stream(text);
        self.process_events(&events);
    }

    /// Process pre-parsed events through both scopes.
    pub fn process_events(&mut self, events: &[StreamEvent]) {
        // Split events at conversation boundaries
        let mut current_segment: Vec<StreamEvent> = Vec::new();

        for event in events {
            match event {
                StreamEvent::ConversationEnd { .. } => {
                    // Process current segment into both scopes
                    self.record_segment(&current_segment);
                    // Reset conversation scope — G2: long_term unaffected
                    self.conversation = RelationalMemory::new();
                    self.boundary_count += 1;
                    current_segment.clear();
                }
                other => current_segment.push(other.clone()),
            }
        }

        // Process any remaining segment (no final boundary)
        if !current_segment.is_empty() {
            self.record_segment(&current_segment);
        }
    }

    /// Record one segment (between boundaries) into both scopes.
    fn record_segment(&mut self, events: &[StreamEvent]) {
        let r0 = r0_from_events(events);
        let r1 = r1_from_r0(&r0);

        // Build a ParseResult-equivalent for both scopes
        use crate::observation::ParseResult;
        let parsed = ParseResult {
            observations: events.iter().filter_map(|e| match e {
                StreamEvent::Character { ch, position, step } => {
                    Some(crate::observation::CharObservation {
                        step: *step,
                        position: *position,
                        locus: crate::locus::Locus::new(*ch),
                        source_excerpt: *ch,
                    })
                }
                _ => None,
            }).collect(),
            r0_relations: r0,
            r1_structures: r1,
            unresolved: vec![],
            stream_length: events.len(),
        };

        self.long_term.record(&parsed);
        self.conversation.record(&parsed);

        if parsed.observations.last().is_some() {
            self.total_steps = parsed.observations.last()
                .map(|o| o.step).unwrap_or(self.total_steps);
        }
    }

    /// Summary of both scopes.
    pub fn summary(&self) -> ScopeSummary {
        ScopeSummary {
            long_term_r0: self.long_term.r0_count(),
            long_term_r1: self.long_term.r1_count(),
            long_term_persistent_r1: self.long_term.persistent_r1().len(),
            conversation_r0: self.conversation.r0_count(),
            conversation_r1: self.conversation.r1_count(),
            conversation_persistent_r1: self.conversation.persistent_r1().len(),
            boundary_count: self.boundary_count,
        }
    }
}

#[derive(Debug)]
pub struct ScopeSummary {
    pub long_term_r0: usize,
    pub long_term_r1: usize,
    pub long_term_persistent_r1: usize,
    pub conversation_r0: usize,
    pub conversation_r1: usize,
    pub conversation_persistent_r1: usize,
    pub boundary_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boundary_token_not_a_locus() {
        let events = parse_stream("X*X*X");
        assert_eq!(events.len(), 1);
        matches!(events[0], StreamEvent::ConversationEnd { .. });
    }

    #[test]
    fn test_boundary_resets_conversation_not_long_term() {
        let mut mem = ScopedRelationalMemory::new();
        mem.process_stream("dog runs");
        let lt_r0_before = mem.long_term.r0_count();
        let conv_r0_before = mem.conversation.r0_count();
        assert!(lt_r0_before > 0);
        assert!(conv_r0_before > 0);

        mem.process_stream("X*X*X");

        // long_term unchanged
        assert_eq!(mem.long_term.r0_count(), lt_r0_before);
        // conversation reset
        assert_eq!(mem.conversation.r0_count(), 0);
        assert_eq!(mem.boundary_count, 1);
    }

    #[test]
    fn test_boundary_in_stream() {
        let mut mem = ScopedRelationalMemory::new();
        mem.process_stream("dog runs X*X*X dog eats");

        // long_term has both segments
        assert!(mem.long_term.get_r0('d', 'o').is_some());

        // conversation has only second segment (after boundary)
        // d→o present in both — check eats-specific edges
        // 'e' from 'eats' should be in conversation
        assert!(mem.conversation.get_r0('e', 'a').is_some());
        assert_eq!(mem.boundary_count, 1);
    }

    #[test]
    fn test_boundary_breaks_r0_chain() {
        // X*X*X should not produce an edge between characters on either side
        let events = parse_stream("g X*X*X r");
        let r0 = r0_from_events(&events);
        // g→r should NOT exist — boundary broke the chain
        assert!(!r0.iter().any(|e| e.edge() == ('g', 'r')));
    }

    #[test]
    fn test_multiple_boundaries() {
        let mut mem = ScopedRelationalMemory::new();
        mem.process_stream("dog runs X*X*X cat eats X*X*X bird flies");
        assert_eq!(mem.boundary_count, 2);
        // long_term has all three segments
        assert!(mem.long_term.get_r0('d', 'o').is_some());
        assert!(mem.long_term.get_r0('c', 'a').is_some());
        assert!(mem.long_term.get_r0('b', 'i').is_some());
        // conversation has only third segment
        assert!(mem.conversation.get_r0('b', 'i').is_some());
        assert!(mem.conversation.get_r0('d', 'o').is_none());
        assert!(mem.conversation.get_r0('c', 'a').is_none());
    }
}
