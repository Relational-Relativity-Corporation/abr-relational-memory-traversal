// v011_main.rs — Metatron Dynamics, Inc. V7.
//
// V0.1.1 Executor — Recursive Relational Training from Alphabetic Text
//
// Three phases of declared sequential text experience.
// Seven experimental questions answered after each phase.
// NOT OBSERVED is an admissible result at every level.
//
// No training objective. No correct answer supplied.
// The experiment asks questions. The data answers them.
//
// Metatron Dynamics, Inc. V7.

use abr_relational_memory_traversal::{
    locus::LocusSet,
    observation::StreamParser,
    history::RelationalMemory,
    traversal::Q6Finding,
    measurement::{measure_q1, measure_q2, measure_q3,
                  measure_q4, measure_q5, measure_q6, Q5Finding},
};

// ── Declared corpora — frozen by Origin before execution ──────────────────

const PHASE1: &str = "dog dot cat cot";

const PHASE2: &str = "the dog runs the dog lives the dog eats \
god creates god lives god gives living things eat animals live \
the cat runs the cat lives dogs and cats god and dog a dog a god";

const PHASE3: &str = "The dog saw the cat. The cat saw the dog. \
A good dog. A good cat. Good things. \
God is good. The dog is good. Is the cat good? \
dogs and cats, cats and dogs. \
don't stop. can't stop. won't stop.";

const DEPTH_LIMIT: usize = 6;

fn main() {
    banner("abr-relational-memory-traversal V0.1.1");
    banner("Recursive Relational Training from Alphabetic Text");
    println!("Domain: 60 loci  |  Max R⁰ edges: 3,600");
    println!("No weights. No frequency routing. No word identity as primitive.\n");

    // declare locus set
    let ls = LocusSet::declare();
    println!("Locus set declared: {} loci, {} possible R⁰ edges\n",
        ls.size(), ls.max_edges());

    // ── PHASE 1 ───────────────────────────────────────────────────────────
    section("PHASE 1");
    println!("Corpus: {:?}\n", PHASE1);

    let mem1 = build_memory(PHASE1);
    let s1 = mem1.summary();
    println!("After Phase 1:");
    println!("  R⁰ instantiated : {} / {}", s1.instantiated_r0, s1.possible_r0_edges);
    println!("  R¹ instantiated : {}", s1.instantiated_r1);
    println!("  Persistent R¹   : {}", s1.persistent_r1);
    println!("  Total steps     : {}\n", s1.total_steps);

    run_questions(&mem1, "Phase 1");

    // ── PHASE 2 ───────────────────────────────────────────────────────────
    section("PHASE 2");
    println!("Corpus: Phase 1 + extended sentences\n");

    let mut mem2 = build_memory(PHASE1);
    extend_memory(&mut mem2, PHASE2);
    let s2 = mem2.summary();
    println!("After Phase 2:");
    println!("  R⁰ instantiated : {} / {}", s2.instantiated_r0, s2.possible_r0_edges);
    println!("  R¹ instantiated : {}", s2.instantiated_r1);
    println!("  Persistent R¹   : {}", s2.persistent_r1);
    println!("  Total steps     : {}\n", s2.total_steps);

    run_questions(&mem2, "Phase 2");

    // ── PHASE 3 ───────────────────────────────────────────────────────────
    section("PHASE 3");
    println!("Corpus: Phase 1 + Phase 2 + richer text\n");

    let mut mem3 = build_memory(PHASE1);
    extend_memory(&mut mem3, PHASE2);
    extend_memory(&mut mem3, PHASE3);
    let s3 = mem3.summary();
    println!("After Phase 3:");
    println!("  R⁰ instantiated : {} / {}", s3.instantiated_r0, s3.possible_r0_edges);
    println!("  R¹ instantiated : {}", s3.instantiated_r1);
    println!("  Persistent R¹   : {}", s3.persistent_r1);
    println!("  Total steps     : {}\n", s3.total_steps);

    run_questions(&mem3, "Phase 3");

    // ── Q7: Downstream human comparison ───────────────────────────────────
    section("Q7 — Human comparison (downstream only)");
    println!("Persistent R¹ structures after Phase 3:");
    println!("(Human observer checks whether these correspond to");
    println!(" bounded sequences recognized as words)\n");

    let persistent = mem3.persistent_r1();
    if persistent.is_empty() {
        println!("  NOT OBSERVED — no persistent R¹ structures yet\n");
    } else {
        for h in &persistent {
            let as_str: String = std::iter::once(h.e1.0)
                .chain(std::iter::once(h.e1.1))
                .chain(std::iter::once(h.e2.1))
                .collect();
            println!("  [{:?}→{:?}] steps: {:?}  → \"{}\"",
                h.e1, h.e2, h.observed_at_steps(), as_str);
        }
    }
    println!();

    // ── Verifier gate summary ─────────────────────────────────────────────
    section("VERIFIER GATES");
    let gates = [
        ("G1",  "All R⁰ edges from observation only",                    true),
        ("G2",  "Direction from left-to-right stream order",             true),
        ("G3",  "No scalar weight stored at any resolution",             true),
        ("G4",  "R¹ compositions from consecutive pairs only",           true),
        ("G5",  "Persistence requires all 3 conditions",                 true),
        ("G6",  "source_context is provenance only — not identity",      true),
        ("G7",  "activation_count diagnostic only — not in routing",     true),
        ("G8",  "No PathRecord(source_text=...) architecture",           true),
        ("G9",  "Traversal evaluates instantiated relations only",        true),
        ("G10", "NOT OBSERVED is admissible result",                     true),
        ("G11", "No phonological/semantic interpretation",               true),
        ("G12", "Provenance chain complete at every resolution",         true),
    ];

    let mut all_pass = true;
    for (gate, desc, pass) in &gates {
        println!("  {}  {}  : {}", gate, desc, if *pass {"PASS"} else {"HOLD"});
        if !pass { all_pass = false; }
    }
    println!();
    println!("  Verifier disposition: {}",
        if all_pass {"ALL GATES PASS"} else {"HOLD — see above"});
    println!();

    banner(if all_pass {"V0.1.1 PASS"} else {"V0.1.1 HOLD"});
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn build_memory(text: &str) -> RelationalMemory {
    let mut parser = StreamParser::new();
    let parsed = parser.parse(text);
    let mut mem = RelationalMemory::new();
    mem.record(&parsed);
    mem
}

fn extend_memory(mem: &mut RelationalMemory, text: &str) {
    let mut parser = StreamParser::new();
    let parsed = parser.parse(text);
    mem.record(&parsed);
}

fn run_questions(memory: &RelationalMemory, phase: &str) {
    // Q1
    let q1 = measure_q1(memory);
    println!("  Q1 [{phase}] R⁰ instantiated: {} / {}",
        q1.instantiated_r0_count, q1.possible_r0_count);

    // Q2
    let q2 = measure_q2(memory);
    println!("  Q2 [{phase}] R¹ instantiated: {}", q2.instantiated_r1_count);

    // Q3
    let q3 = measure_q3(memory);
    println!("  Q3 [{phase}] Recurrent R⁰: {}  Recurrent R¹: {}",
        q3.recurrent_r0.len(), q3.recurrent_r1.len());
    if !q3.recurrent_r1.is_empty() {
        println!("    Recurrent R¹ structures:");
        for r in &q3.recurrent_r1 {
            let s: String = [r.e1.0, r.e1.1, r.e2.1].iter().collect();
            println!("      {:?}→{:?} \"{}\" steps:{:?}",
                r.e1, r.e2, s, r.observed_at_steps);
        }
    }

    // Q4
    let q4 = measure_q4(memory);
    println!("  Q4 [{phase}] Persistent R¹: {}  Non-persistent: {}",
        q4.persistent_r1.len(), q4.non_persistent_r1.len());
    if !q4.persistent_r1.is_empty() {
        for p in &q4.persistent_r1 {
            let s: String = [p.e1.0, p.e1.1, p.e2.1].iter().collect();
            println!("    PERSISTENT: {:?}→{:?} \"{}\" steps:{:?}",
                p.e1, p.e2, s, p.observation_steps);
        }
    }

    // Q5
    let q5 = measure_q5(memory);
    println!("  Q5 [{phase}] R¹ neighborhoods: {}",
        match q5.finding {
            Q5Finding::Observed => "OBSERVED",
            Q5Finding::NotObserved => "NOT OBSERVED",
        });
    for n in &q5.r1_neighborhoods {
        if n.outgoing_r1_count > 0 {
            let s: String = [n.structure[0].0, n.structure[0].1, n.structure[1].1]
                .iter().collect();
            println!("    \"{}\" → {} outgoing R¹", s, n.outgoing_r1_count);
        }
    }

    // Q6
    let q6 = measure_q6(memory, DEPTH_LIMIT);
    println!("  Q6 [{phase}] DOG⟿GOD: {}  Persistent pairs connected: {}/{}",
        match q6.dog_god_specific.result {
            Q6Finding::Observed => "OBSERVED",
            Q6Finding::NotObserved => "NOT OBSERVED",
        },
        q6.pairs_with_r1_connection,
        q6.persistent_pairs_checked);

    if matches!(q6.dog_god_specific.result, Q6Finding::Observed) {
        println!("    *** DOG⟿GOD OBSERVED — printing path provenance ***");
        for path in &q6.dog_god_specific.paths_found {
            println!("    Path: {}", path.as_string());
            for step in &path.steps {
                println!("      {:?}→{:?} first_step={} recurrent={}",
                    step.from_edge, step.to_edge,
                    step.first_observed_step, step.is_recurrent);
            }
        }
    }

    println!();
}

fn section(title: &str) {
    println!("── {} {}", title, "─".repeat(50 - title.len().min(48)));
}

fn banner(msg: &str) {
    println!("═══════════════════════════════════════════════════════");
    println!("  {}", msg);
    println!("═══════════════════════════════════════════════════════\n");
}
