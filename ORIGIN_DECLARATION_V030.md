# Origin Declaration — abr-relational-memory-traversal V0.3.0
**Date:** 2026-09-20
**Authority:** V7 relational kernel, operators.rs V7
**Supersedes:** V0.2.0 declaration for experimental scope only
**V0.1.1 and V0.2.0:** FROZEN — no modifications
**Status:** FROZEN — no code generated before this document is complete

---

## 1. Governing question

> Does relational resolution itself distinguish current conversational
> experience from persistent historical experience, without a scoring
> function, weighting, or scalar compression?

This question has two parts:

**Part A — Structural distinction:**
When a system has both long-term training history (H_L) and
current conversation history (H_C), and the same structure C
appears in both, does the relational structure at R¹ and R²
resolution distinguish them — without introducing ρ_conv,
ρ_long, η, λ, or any combination function?

**Part B — Relational evolution (LTD observation):**
When an association established in H_L is no longer reinforced
in subsequent observations, what relational change corresponds
to the loss of its current participation? This is measured from
the observation sequence — not declared as a decay formula.

---

## 2. What is NOT introduced in V0.3.0

The following are explicitly excluded. They may become necessary
later — but only if the observations demand them. They are not
introduced because the biological analogy suggests them.

```
ρ_conv        — scalar conversation coupling
ρ_long        — scalar long-term coupling
η             — LTP rate parameter
λ             — LTD rate parameter
α             — conversation/long-term weight
f(ρ_C, ρ_L)  — any combining function
activation_count() used for routing
frequency used for selection
any scalar encoding of relational history
```

The V0.2.0 finding stands: relational history itself carries the
information. Whether scalar compression is required is determined
by observation in this experiment — not assumed in advance.

---

## 3. Declared boundary token

```
Conversation boundary token: X*X*X
```

**Role:** declared boundary event in the character stream.
Not a locus. Not passed to any operator. Not a relational
structure. When observed in the stream, it triggers:
  — conversation memory scope: reset to empty
  — long-term memory scope: unaffected

**Declared by Origin:** 2026-09-20

**Parser behavior:**
The sequence X*X*X is recognized before character-level processing.
When encountered, the parser emits a BoundaryEvent::ConversationEnd
and does not produce Locus entries for the characters X, *, X, *, X.

**Why this token:**
X*X*X will not appear in natural English text. It is visually
distinct. It is unambiguous in the character stream. It is not
producible by accident in normal conversation.

---

## 4. Two declared memory scopes

Both scopes use the existing EdgeHistory structure from V0.1.1.
No new scalar fields. No new data types beyond the scope container.

```rust
struct ScopedRelationalMemory {
    long_term: HashMap<EdgeKey, EdgeHistory>,
        // Accumulates across all observations and conversations.
        // Never reset by X*X*X.
        // Persists indefinitely.
        // Source of persistent historical experience H_L.

    conversation: HashMap<EdgeKey, EdgeHistory>,
        // Accumulates within the current conversation only.
        // Reset to empty when X*X*X is observed.
        // Source of immediate conversational experience H_C.
        // Before reset: the experiment observes what (if anything)
        // transfers to long_term. This is measured, not assumed.
}
```

Both scopes are queried when C is presented.
Their outputs are recorded separately.
No combining function is applied in V0.3.0.

---

## 5. Declared training structure

**Phase 1 — Establish H_L (long-term history):**

```
the dog runs fast
a dog runs every day
the big dog runs outside
dog runs to the door
see the dog run
dogs run and run
X*X*X
the dog runs in the morning
dog runs every single day
a fast dog runs far
X*X*X
```

The X*X*X tokens within training mark conversation boundaries
during the training phase itself. Each boundary resets the
conversation scope. Long-term scope accumulates across all of them.

After Phase 1:
- long_term contains: strong `dog → runs` history across multiple
  conversation boundaries
- conversation contains: the final training segment only

**Phase 2 — Introduce H_C (conversation conflict):**

Without resetting long_term, begin a new conversation:

```
my dog eats blueberries every morning
the dog eats quickly at the bowl
dogs eat together
a hungry dog eats well
```

No X*X*X yet — this is the active conversation.

After Phase 2:
- long_term: unchanged from Phase 1 (dog → runs dominant)
- conversation: dog → eats established

**Phase 3 — Present C:**

```
C = "dog"
```

Present to the ScopedRelationalMemory that now contains both
long_term (dog → runs) and conversation (dog → eats).

Record all relational structures available from C in each scope
separately. Do not combine. Do not select.

---

## 6. Five declared measurements

**Q1 — Scope separation verification**

Confirm mechanically that long_term and conversation are distinct
stores. Confirm X*X*X reset conversation but not long_term.
Record the R⁰ and R¹ counts in each scope after Phase 1 and
after Phase 2.

**Q2 — Association acquisition in each scope**

After Phase 2:
- In long_term: is `dog → runs` history present with multiple
  observations across conversation boundaries?
- In conversation: is `dog → eats` history present?
Record with full provenance.

**Q3 — R¹ structures available from C in each scope**

Present C = "dog". Record separately:
- R¹ structures reachable from terminal edge (o,g) in long_term
- R¹ structures reachable from terminal edge (o,g) in conversation
Do not combine. Record all. NOT OBSERVED admissible.

**Q4 — R² observation (the key experimental question)**

For each R¹ structure reachable from C:
- What R¹ structures follow it in long_term?
- What R¹ structures follow it in conversation?
- Do these R²-level neighborhoods differ between scopes?

This is the structural distinction question. If R² shows that
conversation-scope R¹ structures have different relational
neighborhoods than long_term-scope R¹ structures, the distinction
is structurally present without any scalar encoding.

NOT OBSERVED is admissible and would indicate that R² resolution
is insufficient — requiring either larger corpora, deeper
resolution, or (only then) scalar measures.

**Q5 — LTD observation (Phase 3 extension)**

After Phase 3, continue training long_term with observations that
contain `dog` but NOT `dog runs`:

```
the dog eats
the dog sleeps
the dog plays
the dog barks
...
```

At checkpoints (every 10 observations), record the complete
observation sequence for `(g,SPACE)→(SPACE,r)` in long_term.

The question: does the sequential gap in observation of
`(g,SPACE)→(SPACE,r)` correspond to any observable change in
the R¹ or R² structures surrounding it?

We are NOT measuring scalar decay. We are measuring whether
the relational structure of the memory — at R¹ and R² resolution
— changes as a function of the observation gap.

If no structural change is observed: the observations do not yet
demand a decay formula. The gap is recorded but the relational
structure is unchanged.

If structural change is observed: record precisely what changed
and what observation gap produced it. This is the basis for
declaring the forgetting gradient — derived from observation,
not from the biological analogy.

---

## 7. Verifier gates — V0.3.0

```
G1   X*X*X recognized as boundary event — not as loci
G2   X*X*X resets conversation scope only — long_term unaffected
G3   Both scopes use identical EdgeHistory structure from V0.1.1
G4   No ρ_conv, ρ_long, η, λ, or combining function in codebase
G5   Q3 records R¹ from each scope separately — no combination
G6   Q4 records R² from each scope separately — no combination
G7   Q5 measures observation gap structurally — no decay formula
G8   NOT OBSERVED is admissible for Q3, Q4, Q5
G9   All 26 V0.1.1 tests pass before V0.3.0 runs
G10  No modifications to V0.1.1 or V0.2.0 lib files
G11  V0.3.0 is a new executor: src/bin/v030_main.rs only
G12  Complete provenance at every resolution in both scopes
G13  No selection mechanism introduced at any point
G14  JSON run record produced covering all five measurements
G15  Scalar quantities (counts, gaps) recorded as diagnostics
     only — never used for routing or selection
```

---

## 8. What a finding establishes

**If Q4 shows R² structural distinction between scopes:**

> The same current observation C occupies different relational
> conditions in long_term and conversation scope at R² resolution.
> The distinction is structurally present without scalar encoding.
> Relational resolution distinguishes immediate from persistent
> experience.

This is the mathematical basis for context-sensitive relational
expression without attention or weighting.

**If Q4 returns NOT OBSERVED:**

> R² resolution is insufficient to distinguish the scopes given
> the declared corpora. Either: (a) expand corpora, (b) go to R³,
> or (c) the observations demand a scalar measure — declare it then.

**If Q5 shows structural change from observation gap:**

> The observation sequence itself produces relational change
> corresponding to the loss of current participation of a prior
> association. The forgetting gradient is derivable from observation.

**If Q5 returns NOT OBSERVED:**

> The observation gap does not yet produce structural change.
> Scalar decay is not yet demanded by observation.
> Continue accumulating observation gaps and re-measure.

---

## 9. Interpretation boundaries

Do not claim:
- The system understands context
- The system has working memory
- The system learned to forget
- LTD has been implemented
- Biological synaptic behavior has been reproduced

Do claim (if observed):
- The relational structure available from C differs between
  long_term and conversation scope at R¹ / R² resolution
- The observation sequence shows structural change
  corresponding to the observation gap for a given association

---

## 10. What V0.3.0 does NOT yet build

- No conversation generation
- No response production
- No multi-turn conversation loop
- No connection to U-tokenizer (V0.4.0)
- No word-level loci (V0.4.0)
- No full 170,000-word corpus (V0.4.0)

V0.3.0 establishes the structural basis for context-sensitive
relational memory. Everything above it waits on this result.

---

## 11. Declared progression

```
V0.1.1  PASS/FREEZE  Recursive relational training from alphabet
V0.2.0  PASS/FREEZE  Association training and history-dependent
                     relational continuation
V0.3.0  THIS        Scoped relational memory — immediate vs
                     persistent — and LTD structural observation
V0.4.0  NEXT        U-tokenizer connection — word-level loci
V0.5.0  FUTURE      Full English corpus — 170,000-word domain
V0.6.0  FUTURE      Relational expression rule — what governs
                     which continuation is expressed
V0.7.0  FUTURE      Conversation layer — multi-turn relational
                     state across X*X*X boundaries
```

Metatron Dynamics, Inc. V7.
