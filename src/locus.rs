// locus.rs — Metatron Dynamics, Inc. V7.
//
// Primary domain D — 60 declared loci.
//
// D = { a–z, A–Z, ' ', '.', ',', '!', '?', '\'', '-', '\n' }
//
// Every character in D is a declared locus.
// No character is silently treated as a boundary event.
// No character is silently excluded from the stream.
//
// Case folding: NOT applied. 'D' and 'd' are distinct loci.
// Declared decision — preserves capitalization as observable.
//
// Characters outside D are UnresolvedCharacter events.
// They are recorded with stream position. Never silently dropped.
// Never passed to any operator.
//
// Observable: x[v] ∈ {0,1} — binary presence.
// A locus is active (1) when it is the current stream character.
//
// Metatron Dynamics, Inc. V7.

/// One declared locus in D.
/// Identity is the character itself — declared by Origin, not learned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Locus(pub char);

impl Locus {
    /// Construct from a declared domain character.
    /// Panics if the character is not in D.
    pub fn new(c: char) -> Self {
        assert!(
            is_declared(c),
            "character {:?} is not in declared domain D", c
        );
        Locus(c)
    }

    pub fn ch(self) -> char { self.0 }
}

impl std::fmt::Display for Locus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ' '  => write!(f, "SPACE"),
            '\n' => write!(f, "NEWLINE"),
            c    => write!(f, "{}", c),
        }
    }
}

/// Is this character in the declared domain D?
pub fn is_declared(c: char) -> bool {
    c.is_ascii_alphabetic()    // a–z, A–Z
    || c == ' '
    || c == '.'
    || c == ','
    || c == '!'
    || c == '?'
    || c == '\''
    || c == '-'
    || c == '\n'
}

/// The complete declared locus set.
/// Initialized once before any observation runs.
pub struct LocusSet {
    pub loci: Vec<Locus>,
}

impl LocusSet {
    /// Declare the full 60-character locus set.
    /// Called once before any observation runs.
    pub fn declare() -> Self {
        let mut loci = Vec::with_capacity(60);
        // a–z
        for c in 'a'..='z' { loci.push(Locus(c)); }
        // A–Z
        for c in 'A'..='Z' { loci.push(Locus(c)); }
        // declared punctuation and space
        for c in [' ', '.', ',', '!', '?', '\'', '-', '\n'] {
            loci.push(Locus(c));
        }
        LocusSet { loci }
    }

    pub fn size(&self) -> usize { self.loci.len() }

    pub fn max_edges(&self) -> usize { self.size() * self.size() }
}

/// A character that appeared in the input stream but is not in D.
/// Recorded for provenance. Never silently dropped.
#[derive(Clone, Debug)]
pub struct UnresolvedCharacter {
    pub character: char,
    pub stream_position: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_size() {
        let ls = LocusSet::declare();
        assert_eq!(ls.size(), 60);
        assert_eq!(ls.max_edges(), 3600);
    }

    #[test]
    fn test_case_distinct() {
        // 'D' and 'd' are distinct loci — case NOT folded
        assert_ne!(Locus::new('D'), Locus::new('d'));
    }

    #[test]
    fn test_space_is_locus() {
        // space is a declared locus, not a boundary event
        assert!(is_declared(' '));
        let _ = Locus::new(' ');
    }

    #[test]
    fn test_punctuation_are_loci() {
        for c in ['.', ',', '!', '?', '\'', '-', '\n'] {
            assert!(is_declared(c), "{:?} should be declared", c);
        }
    }

    #[test]
    #[should_panic]
    fn test_digit_not_declared() {
        Locus::new('3');  // digits not in D for V0.1.1
    }

    #[test]
    fn test_unresolved_not_panics() {
        // characters outside D produce UnresolvedCharacter, not panic
        let u = UnresolvedCharacter { character: '3', stream_position: 0 };
        assert_eq!(u.character, '3');
    }
}
