# Relational Evolution Frame Declaration — V1
**Date:** 2026-09-20
**Authority:** V7 kernel, operators.rs V7, Metatron Dynamics, Inc.
**Status:** FROZEN — submitted to Verifier before any V0.3.1 code
**Supersedes:** Nothing — this is a new declaration
**Governed by:** V0.1.1, V0.2.0, V0.3.0 PASS/FREEZE findings

---

## 1. Purpose

This declaration establishes the mathematical and computational
frame for relational evolution in the language system before any
further experimental code is written.

It answers three questions left open after V0.3.0:

Q1 — What is the correct unit of observation for the operators?
Q2 — Can Δ/Σ handle the joint (D,S) field without a new operator?
Q3 — What constitutes association quality in this architecture?

It puts V0.3.1 on HOLD until these answers are Verifier-confirmed.

---

## 2. The problem with the current frame

V0.1.1 through V0.3.0 operated on a single relational field —
the discrimination field D — built from the character stream.

The operators received:

```
X_t = {R⁰/R¹/R² structures from character stream at step t}
```

This is sufficient to establish that relational memory accumulates,
that associations are history-dependent, and that R² distinguishes
immediate from persistent scope.

It is insufficient for language because:

```
Language is a dual-domain phenomenon.

D_t — what is discriminated (character/word stream)
S_t — what significance that discrimination carries

The relation between D_t and S_t is what makes language
carry more than co-occurrence statistics.

Neither Δ nor Σ operates across fields.
Both operate within a field.
The cross-field correspondence requires R.
```

This is the gap the Relational Evolution Frame addresses.

---

## 3. The declared unit of observation

For every observation O_t, the full experiential state is:

```
X_t = (D_t, S_t, P_t)

D_t — discriminated environmental relational structure
      Built from: character/word stream via R⁰/R¹/R²
      Contains: all relational structures instantiated
                from the input at step t
      Same machinery as V0.1.1 through V0.3.0

S_t — significance relational structure
      Built from: sparse participation of declared
                  significance loci S = {s₁...s₆}
      Contains: only loci connected to current relational
                trajectory through established experience
      Constraint: |S_t| << 6 for most observations
                  Participation requires relational path:
                  x → r₁ → r₂ → sᵢ
                  Not computed by comparison — derived
                  by traversal of existing relations

P_t — provenance
      The environmental relational trajectory:
      E_{t-1} → E_t
      Complete history of how the current state arose
```

Association quality is not a scalar attached to an edge.
Association quality is the environmental relational trajectory
in which the co-occurrence was observed:

```
Q(A,B) = {(D_{t}, S_{t}, P_{t}) : R(A,B) observed at t}
```

Every instantiation of R(A,B) carries its full X_t context.
This replaces count=8 with eight distinct environmental states.

---

## 4. The two primary experiential fields

```
Field 1 — Discrimination (D)
  Domain: character loci (V0.1.1 declaration, 60 loci)
           extended at V0.4.0 to word-level U-tokenizer loci
  Operators: Δ within D, Σ within D
  Produces: R⁰/R¹/R² discriminative relational structure
  Already built: V0.1.1 through V0.3.0

Field 2 — Significance (S)
  Domain: six declared significance loci S = {s₁...s₆}
  Operators: Δ within S, Σ within S
  Produces: sparse significance relational structure
  Not yet built: declared here, built at V0.5.0
```

Relevance is not a stored field. It is derived:

```
Relevance = R(X_t, H)

where H = persistent relational history
      R = relational correspondence operator
      X_t = current experiential state

Relevance is the correspondence between the current
experiential state and persistent history.
It is computed by traversal at query time.
It is never stored as a separate memory system.
```

---

## 5. The declared significance loci

Six loci declared as origins of relational differentiation.
These are not emotion categories. They are declared relational
coordinates around which experience accumulates structure.

```
s₁ — Threat/safety
     Species-typical association: sudden change, injury,
     predator-associated structures, boundary violation
     
s₂ — Loss/grief
     Species-typical association: absence, separation,
     death, cessation, non-return
     
s₃ — Care/nurture
     Species-typical association: dependency, protection,
     attachment, provision, proximity maintenance
     
s₄ — Seeking/curiosity
     Species-typical association: novel structure, resource,
     opportunity, approach motivation
     
s₅ — Play/affiliation
     Species-typical association: non-threat social
     engagement, reciprocal interaction, bonding
     
s₆ — Disgust/rejection
     Species-typical association: contamination,
     boundary violation, expulsion motivation
```

**Critical constraint:**
The program does not know these labels.
It knows s₁ through s₆ as declared loci.
Labels exist in experimental provenance only.
The program cannot be told "this is grief."
Significance structure must be acquired through experience.

**Relations among loci:**
Six loci produce at most 30 directed first-order cross-locus
relations before provenance. Relations among those relations
produce R¹ and R² significance structures. Enormous
representational richness from a minimal basis.

Example significance configurations (not encoded — observed):
```
s₃ → s₂   (care context, then loss — grief over loved one)
s₁ → s₂   (threat context, then loss — fear of losing)
s₃ → s₅   (care context, then play — affectionate engagement)
s₄ → s₂   (seeking context, then loss — longing)
```

---

## 6. The three declared memory scopes

Three scopes, not two. V0.3.0 declared immediate and persistent.
This declaration adds initial disposition.

```
Scope 1 — Initial disposition (S₀)
  Contains: species-typical significance associations
  Updated by: never — S₀ does not change through experience
  Source: declared by Origin from experimentally justified
          species-typical relational regularities
  Role: provides relational possibilities available before
        individual experience begins
  Not called memory — it is initial relational disposition

Scope 2 — Immediate experience (C)
  Contains: D_C, S_C — current conversation
  Updated by: each observation within the conversation
  Reset by: X*X*X boundary token (V0.3.0 declaration)
  Same EdgeHistory structure from V0.1.1

Scope 3 — Persistent experience (L)
  Contains: D_L, S_L — accumulated across all training
  Updated by: each observation, never reset
  Same EdgeHistory structure from V0.1.1
```

**Critical constraint:**
S₀ is never modified by experience.
Only S_C and S_L accumulate through observation.
The distinction between what is available by disposition
and what is established by experience must be preserved
in both code and provenance records.

---

## 7. The operator architecture — resolved

**Q2 from Section 1 is now answered:**

```
Δ — within-field evolution
    Δ(D_t) — discriminative relational evolution
    Δ(S_t) — significance relational evolution
    Cannot operate across fields

Σ — within-field accumulation
    Σ(Δ(D_t)) — accumulated discriminative state
    Σ(Δ(S_t)) — accumulated significance state
    Cannot operate across fields

R — cross-field correspondence
    R(D_t, S_t) — binding: D ↔ S at this observation
    R(X_t, H)   — relevance: current state vs history
    R is the operator that language requires
    R is not new to V7 — newly declared as necessary here
```

The complete relational evolution pipeline:

```
Observation O_t
    ↓
Δ(D_t)          — discriminative relational evolution within D
Δ(S_t)          — significance relational evolution within S
    ↓
R(D_t, S_t)     — binding: D ↔ S at this observation
                  Not permanent lexical association
                  Bound at observation, with provenance
    ↓
Σ over all three — accumulated experiential state X_t
    ↓
R(X_t, H)       — relevance: correspondence between
                  present state and persistent history
                  Computed by traversal, never stored
    ↓
Expression      — relational continuation structurally
                  continuous with present state
                  Not selection among candidates
                  Not highest score
                  Structural continuity
```

**Why R is necessary and not reducible to Δ/Σ:**

Language presents two simultaneously active relational fields
that are not reducible to each other and not sequential.
D_t and S_t arise from the same observation but carry
different relational information. Their correspondence is
what carries linguistic significance.

`"My father died"` and `"The mosquito died"` produce similar
D_t structures around `died`. Their S_t structures differ
because the surrounding relational context connects to
different significance loci through accumulated experience.

R(D_t, S_t) captures that difference.
Δ and Σ within D alone cannot.

---

## 8. D ↔ S binding — what it is and what it is not

```
IS:
  A relational correspondence established at observation time
  between the discriminative structure D_t and the significance
  structure S_t active during that observation
  Preserved with full environmental provenance
  Allows the same D structure to participate in different
  S configurations across different observations

IS NOT:
  A permanent lexical property
  A semantic label
  A scoring of D against S
  death → grief (permanent)
  Instead: R(D_death, S_loss | E_t₁) — one observation
           R(D_death, S_threat | E_t₂) — another observation
           R(D_death, S_seeking | E_t₃) — another
  The binding is experiential, not definitional
```

---

## 9. What LTD now means in this frame

V0.3.0 attempted to observe LTD as edge count change.
That was insufficient. LTD in the correct frame is:

```
LTD = changing participation of a relational configuration
      in present traversal, while historical provenance
      remains intact

Not: association.strength -= decay
Not: edge count decreasing

But: ΔX_t = X_{t+1} - X_t

What changed in the full experiential state (D_t, S_t, P_t)
as a consequence of sustained absence of activation?

Specifically: does a configuration that was previously reached
by traversal from the current relational frontier cease to be
reached — not because it was deleted, but because the
relational path to it no longer participates in the currently
active trajectory?
```

This is structurally different from decay. The historical
record is never deleted. What changes is whether the
configuration participates in the present relational frontier.

V0.3.1, when it runs, measures this — not edge count change.

---

## 10. Association quality — final declaration

```
Association quality is not:
  A scalar (0.87)
  A frequency count
  A sentiment score
  A category label

Association quality is:
  The significance-contextualized environmental relational
  trajectory in which the co-occurrence was observed

  Q(A,B) = {X_t : R(A,B) observed at t}
           = {(D_t, S_t, P_t) : R(A,B) observed at t}

  Every observation of R(A,B) is a distinct element of Q(A,B)
  Their commonalities and differences constitute the quality
  of the association — without collapsing them into a number
```

---

## 11. Interpretation boundaries

Do not claim:
- The system understands meaning
- The system feels grief, threat, or care
- The significance loci represent emotions
- Biological synaptic behavior has been reproduced
- S₀ is equivalent to genetic encoding

Do claim (if observed):
- The system has accumulated relational structure in D and S
- The binding R(D_t, S_t) was established at observation t
  with complete provenance
- The same D structure participates in different S
  configurations across different observations
- Expression follows from relational continuity, not scoring

---

## 12. What is NOT yet built and must not be built prematurely

```
NOT YET — wait for Verifier confirmation of this frame:
  Significance loci in code
  S₀ initial disposition
  R(D_t, S_t) binding operator
  R(X_t, H) relevance traversal
  Joint (D,S) experiential state

NEXT IN SEQUENCE:
  V0.3.0 correction commit  (6 items — code cleanup)
  Verifier review of this declaration
  V0.3.1 clean LTD experiment (D only, no S yet)
            using correct ΔX_t frame
  V0.4.0 U-tokenizer connection (word-level D loci)
  V0.5.0 significance loci S₁...S₆ introduced
  V0.6.0 R(D_t, S_t) binding declared and tested
  V0.7.0 R(X_t, H) relevance traversal
  V0.8.0 joint expression from (D,S) relational continuity
```

---

## 13. Declared progression — updated

```
V0.1.1  PASS/FREEZE  Recursive relational training from alphabet
V0.2.0  PASS/FREEZE  Association training, history-dependent
                     relational continuation
V0.3.0  PASS/FREEZE  Scoped memory (X*X*X), R² scope distinction
                     LTD: NOT YET OBSERVED (corpus not clean)
V0.3.1  NEXT         Clean LTD experiment — ΔX_t frame
                     D only, no S yet, correct gap measurement
V0.4.0  FUTURE       U-tokenizer — word-level D loci
V0.5.0  FUTURE       Significance loci s₁...s₆ in S domain
                     S₀ initial disposition declared
V0.6.0  FUTURE       R(D_t, S_t) binding — D ↔ S at observation
V0.7.0  FUTURE       R(X_t, H) relevance traversal
V0.8.0  FUTURE       Joint expression from (D,S) continuity
V0.9.0  FUTURE       Multi-turn conversation layer
```

---

## 14. Open questions for Verifier

The following are not yet declared — Verifier ruling needed:

**OQ-REF-1 — S₀ sourcing**
What is the admissible source for initial disposition S₀?
Options: (a) declared by Origin from published experimental
literature on species-typical significance regularities;
(b) derived from corpus statistics on significance-adjacent
language before the system experiences individual training;
(c) both, with explicit provenance for each locus.

**OQ-REF-2 — R(D_t, S_t) admission rule**
What makes an observation eligible for D ↔ S binding?
Must S_t be non-empty? Or is an empty S_t an admissible
observation with binding R(D_t, ∅)?

**OQ-REF-3 — Significance locus activation threshold**
How far along a relational path can a significance locus
be from the current observation and still be considered
participating? Is there a declared depth limit, or is
participation determined purely by traversal reachability?

**OQ-REF-4 — ΔX_t composition**
When computing ΔX_t = X_{t+1} - X_t, do D and S
differences combine or remain separate? The current
declaration keeps them separate. Is that correct, or
does R(D_t, S_t) produce a combined field over which
Δ then operates?

These open questions do not block V0.3.0 correction or
V0.3.1 (D only). They must be resolved before V0.5.0.

---

Metatron Dynamics, Inc. V7.

---

## 15. Declared non-participation as active input

This section was added after V0.3.0 PASS/FREEZE based on the
following architectural observation:

> In biological neural systems, the forward relational evolution
> of a cell reads non-activation as an active input — not as
> silence, not as a gap in the record, but as a positive
> relational signal that produces measurable change in the
> cell's relational state.

V0.3.0 confirmed that the current architecture does not have
this property:

```
sustained_non_recurrence ⇏ change_in_established_association
```

This is not a failure. It is a precise measurement of rigidity
in the current representation. The system responds to activation
but not to non-participation. This section declares what is
needed to introduce relational flexibility.

### 15.1 Declared sets at each observation step

For each observation O_t, three sets are now declared:

```
A_t = {e : edge e activated at step t}
      The active set — what fired

V_t = {e : edge e present in relational memory AND
            reachable from the current relational frontier}
      The available set — what was reachable but may not have fired

N_t = V_t \ A_t
      The non-participating set — what was available but did not fire
      = declared non-participation
      = active relational input
      NOT a gap in the record
      NOT inferred from absence
      DERIVED from the declared relational state at each step
```

N_t is not passive. It is derived from the observable relational
state — V_t is computable from the current relational memory,
A_t is computable from the current observation. Their difference
is N_t. This derivation is traceable through M at every step.

### 15.2 Relational flexibility — declared architectural property

```
RELATIONAL FLEXIBILITY

Definition:
  The capacity of the relational system to modulate current
  relational participation in response to the complete
  observed input — including declared non-participation N_t —
  while preserving full historical provenance.

Formal distinction:

  Rigid system (current architecture, V0.1.1 through V0.3.0):
    ΔX_t = f(A_t) only
    N_t has no effect on relational state
    sustained non-recurrence ⇏ change in participation

  Flexible system (declared here, to be built):
    ΔX_t = f(A_t, N_t)
    N_t is an active input to the relational evolution operators
    sustained N_t membership → change in current participation

Distinction from forgetting:
  Forgetting implies loss of historical provenance.
  Relational flexibility implies change in current participation.
  The historical record is NEVER altered.
  What changes is which structures are currently active
  at the relational frontier.

Biological basis:
  Derived from the property by which forward relational
  evolution in biological cells reads non-activation as
  an active input, producing measurable change in
  relational state — neural flexibility.
  This is the architectural property being declared.
  Whether the computational instantiation warrants
  biological labels (LTD, synaptic depression) is
  determined by observation after measurement — not before.

Relationship to the V0.3.0 null result:
  V0.3.0 confirmed the current system is rigid.
  Relational flexibility is what gets declared and
  built next. The null result is the baseline.
```

### 15.3 LTD-like evolution — now statable precisely

With N_t declared as an active input, LTD-like evolution
is observable when:

```
e ∈ N_t for k consecutive observation steps
where k is a declared threshold (Origin declaration,
not optimized, not inferred — a declared integer)
```

The result is change in relational participation — specifically,
edge e recedes from the active relational frontier. Its
historical provenance is intact. It is historically observed,
and currently persistent in the record, but no longer
currently participating in traversal from the present
relational frontier.

This resolves the three-way distinction declared by the
Verifier after V0.2.0:

```
historically observed ≠ currently persistent ≠ currently participating
```

Relational flexibility is the property that governs the third
term — current participation — without disturbing the first two.

### 15.4 Why this is fundamental to simulating human experience

Every other approach to human-like language response encodes
the description of human experience — sentiment scores,
emotion classifiers, attention weights over semantic embeddings.
They describe the output of human experience and try to
reproduce it directly.

This architecture is different. It instantiates the mechanism
by which experience becomes structured — the same forward
relational evolution that biological systems use, expressed
in mathematics that can run.

Relational flexibility is not a simulation of how humans feel.
It is the property that makes a system's current state
responsive to the present moment rather than frozen in its
accumulated past. That responsiveness is what makes human
language feel alive:

  Not that humans have access to large databases.
  But that what they say next is genuinely shaped by
  what is happening now — including what is not being said,
  what is not being activated, what is receding from the
  present relational frontier.

The N_t declaration is small in code.
It is fundamental in architecture.

### 15.5 What this adds to the build sequence

```
V0.3.0  PASS/FREEZE  Rigidity confirmed — N_t not yet declared
                     sustained_non_recurrence ⇏ participation change

This declaration:
  N_t = V_t \ A_t as active input declared
  Relational flexibility declared as named property
  k (non-participation threshold) to be declared by Origin
    before V0.3.1 code is written

V0.3.1  NEXT (pending Verifier confirmation of this frame):
  First flexibility experiment
  Declare N_t at each observation step
  Observe whether current participation changes under
  sustained non-activation with k declared
  No scalar decay. No deletion. Provenance intact.
  NOT OBSERVED remains admissible.
```

### 15.6 Open question added — OQ-REF-5

```
OQ-REF-5 — Non-participation threshold k
  What is the admissible form of the declared threshold k?
  Options:
  (a) Absolute count: e ∈ N_t for k consecutive steps
  (b) Relative: k expressed as a ratio of V_t size
      (how many available structures were non-participating)
  (c) Relational: k derived from the R¹ depth at which
      e is reachable from the current frontier
  The most admissible form is one derivable from the
  declared relational state — not an arbitrary integer.
  Option (c) is the most consistent with the architecture.
  Origin declaration required before V0.3.1 code.
```

Metatron Dynamics, Inc. V7.
