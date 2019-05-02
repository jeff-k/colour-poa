extern crate bio;

use bio::alignment::pairwise::{Semiring, MIN_SCORE};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

use std::fmt;

#[derive(Copy, Clone, Debug, Eq)]
pub struct Colours {
    pub c1: i32,
    pub c2: i32,
}

impl Ord for Colours {
    fn cmp(&self, other: &Colours) -> Ordering {
        let sm = Ord::max(self.c1, self.c2);
        let om = Ord::max(other.c1, other.c2);

        if sm > om {
            Ordering::Greater
        } else if om > sm {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Colours {
    fn partial_cmp(&self, other: &Colours) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Colours {
    fn eq(&self, other: &Colours) -> bool {
        self.c1 == other.c1 && self.c2 == other.c2
    }
}

impl Semiring for Colours {
    fn mul(self, rhs: Colours) -> Colours {
        Colours {
            c1: self.c1 + rhs.c1,
            c2: self.c2 + rhs.c2,
        }
    }
    fn add(self, rhs: Colours) -> Colours {
        let mc2 = Ord::max(self.c2, rhs.c2);
        let mc1 = Ord::max(self.c1, rhs.c1);
        if mc1 > mc2 {
            Colours { c1: mc1, c2: 0i32 }
        } else if mc2 > mc1 {
            Colours { c1: 0i32, c2: mc2 }
        } else {
            Colours { c1: mc1, c2: mc2 }
        }
    }

    fn zero() -> Colours {
        Colours {
            c1: MIN_SCORE,
            c2: MIN_SCORE,
        }
    }
    fn one() -> Colours {
        Colours { c1: 0i32, c2: 0i32 }
    }
}

pub struct ColourMatchParams {
    pub match_score: Colours,
    pub mismatch_score: Colours,
}

impl fmt::Display for Colours {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.c1, self.c2)
    }
}

impl Colours {
    pub fn label(self) -> String {
        let c = match (self.c1 > 0, self.c2 > 0) {
            (true, false) => "red",
            (true, true) => "red:blue",
            (false, true) => "blue",
            (false, false) => "black",
        };
        format!("({},{})\" color=\"{}", self.c1, self.c2, c)
    }
}
