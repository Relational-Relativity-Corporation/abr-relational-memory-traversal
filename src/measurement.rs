// measurement.rs — Metatron Dynamics, Inc. V7.
//
// Seven experimental questions from the Origin declaration.
// Each returns structured output. NOT OBSERVED is admissible.
// No interpretation beyond what the data shows.
//
// Q1: What R⁰ structures are instantiated?
// Q2: What R¹ structures arise from consecutive R⁰ pairs?
// Q3: Which structures recur across independent observations?
// Q4: Which recurrent structures satisfy persistence conditions?
// Q5: Do persistent R¹ structures develop R¹-level neighborhoods?
// Q6: Does R¹ traversal reach another R¹ structure?
// Q7: Downstream human comparison only — after run complete.
//
// Metatron Dynamics, Inc. V7.

use crate::history::RelationalMemory;
use crate::traversal::{traverse_r1, query_q6, Q6Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Q1Result {
    pub instantiated_r0_count: usize,
    pub possible_r0_count: usize,
    pub edges: Vec<Q1Edge>,
}

#[derive(Debug, Serialize)]
pub struct Q1Edge {
    pub src: char,
    pub tgt: char,
    pub observation_count_diagnostic: usize,
    pub first_step: usize,
}

pub fn measure_q1(memory: &RelationalMemory) -> Q1Result {
    let mut edges: Vec<Q1Edge> = memory.r0.values().map(|h| Q1Edge {
        src: h.src,
        tgt: h.tgt,
        observation_count_diagnostic: h.activation_count(),
        first_step: h.observations.first().map(|r| r.step).unwrap_or(0),
    }).collect();
    edges.sort_by_key(|e| (e.src, e.tgt));

    Q1Result {
        instantiated_r0_count: edges.len(),
        possible_r0_count: 3600,
        edges,
    }
}

#[derive(Debug, Serialize)]
pub struct Q2Result {
    pub instantiated_r1_count: usize,
    pub structures: Vec<Q2Structure>,
}

#[derive(Debug, Serialize)]
pub struct Q2Structure {
    pub e1: (char,char),
    pub e2: (char,char),
    pub pivot: char,
    pub observation_count_diagnostic: usize,
    pub first_step: usize,
}

pub fn measure_q2(memory: &RelationalMemory) -> Q2Result {
    let mut structures: Vec<Q2Structure> = memory.r1.values().map(|h| Q2Structure {
        e1: h.e1,
        e2: h.e2,
        pivot: h.pivot(),
        observation_count_diagnostic: h.activation_count(),
        first_step: h.observations.first().map(|r| r.step).unwrap_or(0),
    }).collect();
    structures.sort_by_key(|s| (s.e1, s.e2));

    Q2Result {
        instantiated_r1_count: structures.len(),
        structures,
    }
}

#[derive(Debug, Serialize)]
pub struct Q3Result {
    pub recurrent_r0: Vec<Q3RecurrentEdge>,
    pub recurrent_r1: Vec<Q3RecurrentR1>,
}

#[derive(Debug, Serialize)]
pub struct Q3RecurrentEdge {
    pub src: char,
    pub tgt: char,
    pub observed_at_steps: Vec<usize>,
    pub recurrence_count: usize,
}

#[derive(Debug, Serialize)]
pub struct Q3RecurrentR1 {
    pub e1: (char,char),
    pub e2: (char,char),
    pub observed_at_steps: Vec<usize>,
    pub recurrence_count: usize,
}

pub fn measure_q3(memory: &RelationalMemory) -> Q3Result {
    let mut r0: Vec<Q3RecurrentEdge> = memory.r0.values()
        .filter(|h| h.is_recurrent())
        .map(|h| Q3RecurrentEdge {
            src: h.src, tgt: h.tgt,
            observed_at_steps: h.observed_at_steps(),
            recurrence_count: h.activation_count(),
        }).collect();
    r0.sort_by_key(|e| (e.src, e.tgt));

    let mut r1: Vec<Q3RecurrentR1> = memory.r1.values()
        .filter(|h| h.is_recurrent())
        .map(|h| Q3RecurrentR1 {
            e1: h.e1, e2: h.e2,
            observed_at_steps: h.observed_at_steps(),
            recurrence_count: h.activation_count(),
        }).collect();
    r1.sort_by_key(|s| (s.e1, s.e2));

    Q3Result { recurrent_r0: r0, recurrent_r1: r1 }
}

#[derive(Debug, Serialize)]
pub struct Q4Result {
    pub persistent_r1: Vec<Q4Persistent>,
    pub non_persistent_r1: Vec<Q4NonPersistent>,
}

#[derive(Debug, Serialize)]
pub struct Q4Persistent {
    pub e1: (char,char),
    pub e2: (char,char),
    pub conditions_met: [bool; 3],
    pub observation_steps: Vec<usize>,
}

#[derive(Debug, Serialize)]
pub struct Q4NonPersistent {
    pub e1: (char,char),
    pub e2: (char,char),
    pub condition_1_recurrent: bool,
    pub reason: &'static str,
}

pub fn measure_q4(memory: &RelationalMemory) -> Q4Result {
    let mut persistent = Vec::new();
    let mut non_persistent = Vec::new();

    for h in memory.r1.values() {
        let c1 = h.satisfies_persistence_condition_1();
        // Condition 2: constituent R⁰ edges independently instantiated
        let c2 = memory.get_r0(h.e1.0, h.e1.1).is_some()
               && memory.get_r0(h.e2.0, h.e2.1).is_some();
        // Condition 3: satisfied by construction (R¹ only created from
        //   consecutive R⁰ pairs in a single observation)
        let c3 = true;

        if c1 && c2 && c3 {
            persistent.push(Q4Persistent {
                e1: h.e1, e2: h.e2,
                conditions_met: [c1, c2, c3],
                observation_steps: h.observed_at_steps(),
            });
        } else {
            let reason = if !c1 { "not recurrent (observed only once)" }
                         else if !c2 { "constituent R⁰ edges not both instantiated" }
                         else { "condition 3 not met" };
            non_persistent.push(Q4NonPersistent {
                e1: h.e1, e2: h.e2,
                condition_1_recurrent: c1,
                reason,
            });
        }
    }

    persistent.sort_by_key(|p| (p.e1, p.e2));
    non_persistent.sort_by_key(|p| (p.e1, p.e2));
    Q4Result { persistent_r1: persistent, non_persistent_r1: non_persistent }
}

/// Q5: Do persistent R¹ structures develop relational neighborhoods
/// at the R¹ level? I.e., does one persistent R¹ structure consistently
/// precede or follow another in the stream?
/// NOT OBSERVED is admissible.
#[derive(Debug, Serialize)]
pub struct Q5Result {
    pub finding: Q5Finding,
    pub r1_neighborhoods: Vec<Q5Neighborhood>,
}

#[derive(Debug, Serialize)]
pub enum Q5Finding {
    Observed,
    NotObserved,
}

#[derive(Debug, Serialize)]
pub struct Q5Neighborhood {
    pub structure: [(char,char);2],
    pub outgoing_r1_count: usize,
}

pub fn measure_q5(memory: &RelationalMemory) -> Q5Result {
    // For each persistent R¹ structure, check how many other
    // R¹ structures follow it (via r1_outgoing_from_edge on its terminal)
    let persistent = memory.persistent_r1();
    let mut neighborhoods: Vec<Q5Neighborhood> = Vec::new();

    for h in &persistent {
        let terminal_edge = h.e2;
        let outgoing = memory.r1_outgoing_from_edge(terminal_edge);
        neighborhoods.push(Q5Neighborhood {
            structure: h.edge_sequence(),
            outgoing_r1_count: outgoing.len(),
        });
    }

    neighborhoods.sort_by_key(|n| n.structure);

    let observed = neighborhoods.iter().any(|n| n.outgoing_r1_count > 0);
    let finding = if observed { Q5Finding::Observed } else { Q5Finding::NotObserved };

    Q5Result { finding, r1_neighborhoods: neighborhoods }
}

/// Q6: R¹ traversal between persistent structures.
/// Wraps traversal::query_q6 for the DOG⟿GOD specific case
/// and generalizes to all persistent pairs.
#[derive(Debug, Serialize)]
pub struct Q6FullResult {
    pub dog_god_specific: Q6Result,
    pub persistent_pairs_checked: usize,
    pub pairs_with_r1_connection: usize,
}

pub fn measure_q6(memory: &RelationalMemory, depth_limit: usize) -> Q6FullResult {
    // specific DOG⟿GOD query
    let dog_god = query_q6(('o','g'), ('g','o'), memory, depth_limit);

    // general: check all persistent R¹ pairs
    let persistent = memory.persistent_r1();
    let mut pairs_connected = 0usize;

    for h in &persistent {
        let result = traverse_r1(h.e2, memory, depth_limit);
        if !result.paths.is_empty() { pairs_connected += 1; }
    }

    Q6FullResult {
        dog_god_specific: dog_god,
        persistent_pairs_checked: persistent.len(),
        pairs_with_r1_connection: pairs_connected,
    }
}
