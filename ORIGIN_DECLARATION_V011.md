# Origin Declaration — abr-relational-memory-traversal V0.1.1
**Date:** 2026-09-20  
**Authority:** V7 relational kernel, operators.rs V7  
**Supersedes:** V0.1 declaration (character-only, case-folded, 26-locus domain)  
**Status:** FROZEN — no code generated before this document is complete

---

## 1. Primary Domain

```
D = {
  a b c d e f g h i j k l m
  n o p q r s t u v w x y z
  A B C D E F G H I J K L M
  N O P Q R S T U V W X Y Z
  ' ' (space,      U+0020)
  '.' (period,     U+002E)
  ',' (comma,      U+002C)
  '!' (exclamation,U+0021)
  '?' (question,   U+003F)
  '\'' (apostrophe,U+0027)
  '-' (hyphen,     U+002D)
  '\n' (newline,   U+000A)
}

|D| = 60 declared loci
Possible directed edges: 60 × 60 = 3,600
```

Every character in this set is a declared locus.
No character is silently treated as a boundary event.
No character is silently excluded from the stream.
A space is a locus. A period is a locus. A capital letter is a locus.
Their relational neighborhoods will reflect their observable positions in text.

Characters outside D encountered in the input stream are declared
UnresolvedCharacter events. They are recorded with their stream position.
They are not passed to any operator. They are not silently dropped.

Case folding: NOT applied. 'D' and 'd' are distinct loci.
This is a declared decision, not a default.
What it preserves: capitalization as an observable relational property.
What it discards: nothing — both cases are retained.

---

## 2. Observable

x[v] ∈ {0, 1} — binary presence.

A locus is active (1) when it is the current character in the
observation stream. All other loci are inactive (0) at that step.

No scalar magnitude. No learned embedding. No frequency weight.

---

## 3. Observation Unit and Progression

The observation unit is one character at one declared stream position.

The observation stream X = x₁, x₂, x₃, ..., xₙ is the raw character
sequence of the declared input text, presented left to right.

This ordering is the declared relational direction.
Left-to-right is not a convention imposed on the mathematics.
It is the observable property of the declared input that establishes
which character is prior and which is current.

Sequential observation requirement (V7): observations must be presented
in the order they appear in the declared input. The system has no
mechanism to receive observations out of order. Cold start is admissible
on observation step 1 only.

---

## 4. R⁰ — Primary Relational Structure

A directed relation (u → v) at relational resolution R⁰ is declared
when character u appears at stream position i and character v appears
at stream position i+1.

This is the only admissible basis for declaring an R⁰ relation.
Combinatorial possibility is not a basis for declaration.
The fact that 3,600 directed pairs exist over D does not license
construction or evaluation of all 3,600.

No declaration, no evaluation.

Every instantiated R⁰ relation retains complete provenance:
- stream position at which it was observed
- the full input text segment containing it (for human readability)
- the observation step index

An instantiated relation exists because it was observed.
Its history is preserved without reducing it to a weight.

---

## 5. R¹ — Relations Among Relations

A directed relation at relational resolution R¹ is declared when
two R⁰ relations appear consecutively in the observation stream
and the target locus of the first equals the source locus of the second.

Given:
  e₁ = (u → v) observed at stream position i
  e₂ = (v → w) observed at stream position i+1

The R¹ structure e₁ → e₂ is licensed by the same observation
that licensed e₁ and e₂.

This is the general recursive principle:
  a relation may itself participate as a locus in further relation.

R¹ structures are not word records. They are not PathRecords with
source_text assigned as identity. The source text is retained
as provenance only — it does not participate in:
  - relational identity
  - equality decisions
  - traversal routing
  - persistence determination
  - lookup or association

R¹ identity is determined solely by the edge sequence.
(u→v)→(v→w) and (w→v)→(v→u) are distinct R¹ structures
because their edge sequences differ.
The locus set {u,v,w} being identical does not make them equivalent.

---

## 6. Recurrence — Observed Without Frequency as Weight

Recurrence is the condition that the same relational structure
appears in more than one independent observation.

For R⁰: edge (u→v) is recurrent if it appears in observations
at stream positions i and j where i ≠ j.

For R¹: structure e₁→e₂ is recurrent if the same edge pair
appears at stream positions (i, i+1) and (j, j+1) where i ≠ j.

Recurrence is recorded as a list of observation step indices —
not as a count stored separately from the list.
The count is derivable from the list length.
The list is the information. The count is a diagnostic only.
The count does not determine traversal.

---

## 7. Persistence — Under Existing Kernel Declaration

Persistence is not declared by frequency threshold.
No θ_min is introduced in V0.1.1.

A relational structure is persistent when:
  - it has been observed in more than one independent observation (recurrent)
  - AND its R⁰ edges are each independently instantiated
  - AND the R¹ composition of those edges was licensed by at least
    one observation (not merely inferred from separate edge existence)

The third condition is critical. The R¹ structure (d→o)→(o→g)
is persistent only if there exists at least one observation where
d→o and o→g appeared consecutively — not merely observations where
d→o appeared and separate observations where o→g appeared.

This mirrors the V7 sequential observation requirement:
a model-generated trajectory is not an observable sequence.
A combination of separately observed edges is not an observed path.

---

## 8. Derived Structure as Locus — Admissibility Condition

A persistent R¹ structure may become available as a locus
at relational resolution R² only when:

  Condition 1: it has been observed in multiple independent
    observations as a complete sequential structure
    (all constituent edges in the declared order, in one observation)

  Condition 2: it has a relational neighborhood —
    other R¹ structures that precede or follow it in the stream —
    that is itself observable (not inferred)

  Condition 3: its identity is determined by its edge sequence,
    not by source text, not by human label, not by position alone

When these conditions are met, the persistent R¹ structure
may participate as a locus in R² relations, subject to the same
provenance requirements as R⁰ loci.

The system does not know these structures correspond to words.
That is a downstream human comparison, made after the run.

---

## 9. Traversal Rules

Traversal operates over already-instantiated relational structure.
It does not evaluate uninstantiated pairs.
It does not create relations during traversal.

At R⁰: traversal follows instantiated (char → char) edges forward.
At R¹: traversal follows instantiated (edge → edge) compositions.
At R²: traversal follows instantiated (R¹-structure → R¹-structure)
  compositions, if any have been derived.

Cycle detection: a cycle is detected when traversal would revisit
a locus already in the current path. Cycles are recorded as findings.
Traversal does not follow cycles beyond detection.
Cycles are not errors.

activation_count() is available as a diagnostic measurement.
It does not appear in any traversal routing decision.
Frequency does not control routing.

Depth limit: declared by Origin before each traversal run.

---

## 10. Provenance Requirements

Every instantiated structure at every resolution carries:
  - the stream positions at which it was observed
  - the observation step indices
  - a human-readable excerpt of the source text at those positions
    (for inspection only — not for relational identity)

Every derived structure at R¹ and above carries:
  - provenance of every constituent R⁰ edge
  - provenance of the specific observation(s) that licensed the
    R¹ composition (not merely the constituent edges separately)

No structure exists in the system without a complete provenance chain
traceable to declared observations of the input stream.

---

## 11. Declared Input Corpus — V0.1.1

Frozen by Origin before execution. Not modifiable after first run.

Phase 1 — minimal:
  "dog dot cat cot"

Phase 2 — extended (the same corpus that produced the V0.1 run):
  "the dog runs the dog lives the dog eats god creates
   god lives god gives living things eat animals live
   the cat runs the cat lives dogs and cats god and dog
   a dog a god"

Phase 3 — richer (new in V0.1.1):
  "The dog saw the cat. The cat saw the dog.
   A good dog. A good cat. Good things.
   God is good. The dog is good. Is the cat good?
   dogs and cats, cats and dogs.
   don't stop. can't stop. won't stop."

Phase 3 is declared here but run only after Phase 1 and Phase 2
measurements are recorded and frozen.

---

## 12. Seven Experimental Questions — V0.1.1

Q1: What R⁰ structures are instantiated by the declared stream?
    Record every (char→char) edge with full provenance.

Q2: What R¹ structures arise from consecutive R⁰ pairs?
    Record every (edge→edge) composition with full provenance.

Q3: Which R⁰ and R¹ structures recur across independent observations?
    Record the observation indices for every recurrence.

Q4: Which recurrent structures satisfy the persistence conditions?
    Record which ones do and which ones do not, and why.

Q5: Do any persistent R¹ structures develop relational neighborhoods
    at the R¹ level — i.e., do R¹ structures precede or follow
    other R¹ structures consistently enough to license R² observation?
    Record findings. NOT OBSERVED is an admissible result.

Q6: Does traversal from any R¹ structure reach another R¹ structure
    through accumulated relational history?
    Record paths and full provenance. NOT OBSERVED is admissible.

Q7 (downstream comparison only, after run is complete):
    Do any derived persistent structures correspond to bounded
    sequences that humans recognize as words in the input text?
    This is a human comparison. It is not an input to the system.

---

## 13. Verifier Gates — V0.1.1

G1:  Every R⁰ edge declared from stream observation only.
     No edge exists without at least one ObservationRecord.

G2:  Direction of every R⁰ edge traceable to left-to-right
     stream order of the declared input.

G3:  No scalar weight stored on any relation at any resolution.
     History is preserved as observation record lists.

G4:  R¹ compositions licensed only by consecutive R⁰ pairs
     in a single observation — not by separately observed edges.

G5:  Persistent R¹ structures identified only when Conditions
     1, 2, and 3 from Section 8 are all met.

G6:  Source text retained for provenance only — not used in
     relational identity, equality, traversal, or routing.

G7:  activation_count() available as diagnostic only —
     not consumed by any traversal routing decision.

G8:  No PathRecord(source_text="dog") architecture.
     Derived structures identified by edge sequence only.

G9:  Traversal does not evaluate uninstantiated relations.

G10: NOT OBSERVED is an admissible result for Q5, Q6, Q7.
     The experiment is a question, not a training objective.

G11: No phonological, phonotactic, or semantic interpretation
     of results. Orthographic relational structure only.

G12: Complete provenance chain available for every structure
     at every resolution, traceable to declared stream observations.

---

## 14. What This Declaration Does Not Contain

- No association formula (no ρ_assoc, χ, Z)
- No frequency threshold (no θ_min)
- No word identity as a mathematical primitive
- No semantic labels
- No prediction objective
- No loss function
- No correct answer supplied to the system
- No DOG→GOD as a training target
- No phonological claims
- No linguistic hierarchy (character/word/phrase/sentence)
  encoded as system primitives

These are excluded not because they are wrong in general,
but because none of them follows from the declared observable,
the relational operators, persistence, or provenance.

No declaration, no evaluation.

---

## 15. Architectural Principle

The same relational process operates at every resolution.

There are no separate algorithms for:
  character_learning()
  word_learning()
  phrase_learning()

There is one process:
  observation → relation → recurrence → persistence → derived structure → further relation

The machine does not know what humans call the structures it derives.
That comparison is made by humans after the run.

Metatron Dynamics, Inc. V7.
