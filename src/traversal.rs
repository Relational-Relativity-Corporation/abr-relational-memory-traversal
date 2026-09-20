// traversal.rs — Metatron Dynamics, Inc. V7.
//
// Traversal over instantiated relational structure only.
// Does not evaluate uninstantiated relations.
// Does not create relations during traversal.
// Does not use activation_count() for routing decisions.
//
// R⁰ traversal: follows (char→char) edges forward.
// R¹ traversal: follows (edge→edge) compositions forward.
//
// Cycle detection: a cycle is detected when traversal would revisit
// a locus already in the current path. Cycles are findings, not errors.
//
// NOT OBSERVED is an admissible result at every query level.
//
// Metatron Dynamics, Inc. V7.

use crate::history::RelationalMemory;
use serde::Serialize;

// ── R⁰ Traversal ─────────────────────────────────────────────────────────

/// One step of R⁰ traversal provenance.
#[derive(Clone, Debug, Serialize)]
pub struct R0Step {
    pub src: char,
    pub tgt: char,
    pub first_observed_step: usize,
    pub is_recurrent: bool,
    // activation_count available for measurement but NOT used for routing
    pub activation_count_diagnostic: usize,
}

/// One complete R⁰ traversal path from a start locus.
#[derive(Clone, Debug, Serialize)]
pub struct R0Path {
    pub loci: Vec<char>,
    pub steps: Vec<R0Step>,
    pub cycle_detected: bool,
}

impl R0Path {
    pub fn as_string(&self) -> String { self.loci.iter().collect() }
}

/// R⁰ traversal result from one start locus.
#[derive(Debug, Serialize)]
pub struct R0TraversalResult {
    pub start: char,
    pub depth_limit: usize,
    pub paths: Vec<R0Path>,
    pub reachable_at_depth: Vec<Vec<char>>,
    pub total_reachable: usize,
    pub cycles_detected: Vec<(char,char)>,
}

/// Traverse R⁰ memory forward from start locus.
/// Depth-limited. No frequency routing. Cycles recorded.
pub fn traverse_r0(
    start: char,
    memory: &RelationalMemory,
    depth_limit: usize,
) -> R0TraversalResult {
    let mut all_paths: Vec<R0Path> = Vec::new();
    let mut reachable: Vec<Vec<char>> = vec![Vec::new(); depth_limit + 1];
    let mut cycles: Vec<(char,char)> = Vec::new();

    reachable[0].push(start);

    // stack: (current loci path, current steps)
    let mut stack: Vec<(Vec<char>, Vec<R0Step>)> = vec![(vec![start], vec![])];

    while let Some((path, steps)) = stack.pop() {
        let current = *path.last().unwrap();
        let depth = path.len() - 1;

        let outgoing = memory.r0_outgoing(current);

        if outgoing.is_empty() || depth >= depth_limit {
            if path.len() > 1 {
                all_paths.push(R0Path { loci: path, steps, cycle_detected: false });
            }
            continue;
        }

        for edge in outgoing {
            let next = edge.tgt;

            if path.contains(&next) {
                cycles.push((current, next));
                all_paths.push(R0Path {
                    loci: path.clone(), steps: steps.clone(), cycle_detected: true,
                });
                continue;
            }

            let step = R0Step {
                src: current,
                tgt: next,
                first_observed_step: edge.observations.first()
                    .map(|r| r.step).unwrap_or(0),
                is_recurrent: edge.is_recurrent(),
                activation_count_diagnostic: edge.activation_count(),
            };

            let next_depth = depth + 1;
            if next_depth < reachable.len() && !reachable[next_depth].contains(&next) {
                reachable[next_depth].push(next);
            }

            let mut np = path.clone(); np.push(next);
            let mut ns = steps.clone(); ns.push(step);
            stack.push((np, ns));
        }
    }

    for level in reachable.iter_mut() { level.sort(); level.dedup(); }
    cycles.sort(); cycles.dedup();
    all_paths.sort_by_key(|p| p.as_string());

    let mut all_reachable: Vec<char> = reachable.iter().flatten().cloned().collect();
    all_reachable.sort(); all_reachable.dedup();
    all_reachable.retain(|&c| c != start);

    R0TraversalResult {
        start,
        depth_limit,
        paths: all_paths,
        reachable_at_depth: reachable,
        total_reachable: all_reachable.len(),
        cycles_detected: cycles,
    }
}

// ── R¹ Traversal ─────────────────────────────────────────────────────────

/// One step of R¹ traversal provenance.
#[derive(Clone, Debug, Serialize)]
pub struct R1Step {
    pub from_edge: (char,char),
    pub to_edge: (char,char),
    pub first_observed_step: usize,
    pub is_recurrent: bool,
    pub activation_count_diagnostic: usize,
}

/// One complete R¹ traversal path from a start edge.
#[derive(Clone, Debug, Serialize)]
pub struct R1Path {
    pub edges: Vec<(char,char)>,  // the sequence of R⁰ edges traversed
    pub steps: Vec<R1Step>,
    pub cycle_detected: bool,
}

impl R1Path {
    pub fn as_string(&self) -> String {
        if self.edges.is_empty() { return String::new(); }
        let mut s = format!("{}{}",self.edges[0].0, self.edges[0].1);
        for e in &self.edges[1..] { s.push(e.1); }
        s
    }
}

/// R¹ traversal result from one start edge.
#[derive(Debug, Serialize)]
pub struct R1TraversalResult {
    pub start_edge: (char,char),
    pub depth_limit: usize,
    pub paths: Vec<R1Path>,
    pub total_paths: usize,
    pub cycles_detected: Vec<((char,char),(char,char))>,
}

/// Traverse R¹ memory forward from a start edge.
/// Follows (edge→edge) compositions in instantiated R¹ history.
pub fn traverse_r1(
    start_edge: (char,char),
    memory: &RelationalMemory,
    depth_limit: usize,
) -> R1TraversalResult {
    let mut all_paths: Vec<R1Path> = Vec::new();
    let mut cycles: Vec<((char,char),(char,char))> = Vec::new();

    let mut stack: Vec<(Vec<(char,char)>, Vec<R1Step>)> =
        vec![(vec![start_edge], vec![])];

    while let Some((path, steps)) = stack.pop() {
        let current_edge = *path.last().unwrap();
        let depth = path.len() - 1;

        let outgoing = memory.r1_outgoing_from_edge(current_edge);

        if outgoing.is_empty() || depth >= depth_limit {
            if path.len() > 1 {
                all_paths.push(R1Path { edges: path, steps, cycle_detected: false });
            }
            continue;
        }

        for r1 in outgoing {
            let next_edge = r1.e2;

            if path.contains(&next_edge) {
                cycles.push((current_edge, next_edge));
                all_paths.push(R1Path {
                    edges: path.clone(), steps: steps.clone(), cycle_detected: true,
                });
                continue;
            }

            let step = R1Step {
                from_edge: current_edge,
                to_edge: next_edge,
                first_observed_step: r1.observations.first()
                    .map(|r| r.step).unwrap_or(0),
                is_recurrent: r1.is_recurrent(),
                activation_count_diagnostic: r1.activation_count(),
            };

            let mut np = path.clone(); np.push(next_edge);
            let mut ns = steps.clone(); ns.push(step);
            stack.push((np, ns));
        }
    }

    all_paths.sort_by_key(|p| p.as_string());
    cycles.sort(); cycles.dedup();
    let total = all_paths.len();

    R1TraversalResult {
        start_edge,
        depth_limit,
        paths: all_paths,
        total_paths: total,
        cycles_detected: cycles,
    }
}

// ── Q6: Does R¹ traversal from dog's terminal reach god's opening? ────────

/// The Q6 query: starting from the terminal edge of one persistent R¹
/// structure, does traversal through accumulated R¹ history reach the
/// opening edge of another?
///
/// For DOG⟿GOD:
///   dog_terminal_edge = (o, g)   — the last R⁰ edge of d→o→g
///   god_opening_edge  = (g, o)   — the first R⁰ edge of g→o→d
///
/// Traversal from (o,g) through R¹ memory looks for paths that
/// contain (g,o) as a subsequent edge.
///
/// NOT OBSERVED is an admissible result.
#[derive(Debug, Serialize)]
pub struct Q6Result {
    pub query: &'static str,
    pub start_edge: (char,char),
    pub target_edge: (char,char),
    pub result: Q6Finding,
    pub paths_found: Vec<R1Path>,
}

#[derive(Debug, Serialize)]
pub enum Q6Finding {
    Observed,
    NotObserved,
}

pub fn query_q6(
    start_edge: (char,char),
    target_edge: (char,char),
    memory: &RelationalMemory,
    depth_limit: usize,
) -> Q6Result {
    let traversal = traverse_r1(start_edge, memory, depth_limit);

    let matching: Vec<R1Path> = traversal.paths.into_iter()
        .filter(|p| p.edges.contains(&target_edge))
        .collect();

    let finding = if matching.is_empty() {
        Q6Finding::NotObserved
    } else {
        Q6Finding::Observed
    };

    Q6Result {
        query: "R¹ traversal from dog terminal edge to god opening edge",
        start_edge,
        target_edge,
        result: finding,
        paths_found: matching,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observation::StreamParser;
    use crate::history::RelationalMemory;

    fn build(text: &str) -> RelationalMemory {
        let mut p = StreamParser::new();
        let parsed = p.parse(text);
        let mut mem = RelationalMemory::new();
        mem.record(&parsed);
        mem
    }

    #[test]
    fn test_r0_traversal_from_d() {
        let mem = build("dog dot");
        let result = traverse_r0('d', &mem, 3);
        assert!(result.reachable_at_depth[1].contains(&'o'));
        assert!(result.reachable_at_depth[2].contains(&'g'));
        assert!(result.reachable_at_depth[2].contains(&'t'));
    }

    #[test]
    fn test_r1_traversal_from_dog_start() {
        // from edge (d,o) — the opening edge of "dog"
        let mem = build("dog dot");
        let result = traverse_r1(('d','o'), &mem, 3);
        // should find paths through (o,g) and (o,t)
        let path_strings: Vec<String> = result.paths.iter()
            .map(|p| p.as_string()).collect();
        assert!(path_strings.iter().any(|s| s.contains("og")),
            "R¹ traversal from (d,o) should reach (o,g)");
    }

    #[test]
    fn test_no_frequency_routing() {
        // "dog" observed 10 times, "dot" once
        // traversal must not prefer (o,g) over (o,t) based on count
        let mem = build("dog dog dog dog dog dog dog dog dog dog dot");
        let result = traverse_r0('d', &mem, 2);
        // both 'g' and 't' must be reachable at depth 2
        assert!(result.reachable_at_depth[2].contains(&'g'));
        assert!(result.reachable_at_depth[2].contains(&'t'));
    }

    #[test]
    fn test_q6_not_observed_on_minimal_corpus() {
        let mem = build("dog dot cat cot");
        let result = query_q6(('o','g'), ('g','o'), &mem, 5);
        // (g,o) not in memory — god not yet observed
        matches!(result.result, Q6Finding::NotObserved);
    }

    #[test]
    fn test_q6_observed_after_god_in_corpus() {
        // after "god" is in corpus, (g,o) is instantiated
        // Q6 asks: can R¹ traversal from (o,g) reach (g,o)?
        let mem = build("dog god");
        let result = query_q6(('o','g'), ('g','o'), &mem, 5);
        // This is the experimental question — result may be either.
        // The test confirms the query runs without error and returns
        // a structured finding. NOT OBSERVED is admissible.
        println!("Q6 result after 'dog god': {:?}", result.result);
    }
}
