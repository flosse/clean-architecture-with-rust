use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub const fn new(n: u64) -> Self {
        Self(n)
    }
    pub const fn to_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for Id {
    fn from(from: u64) -> Self {
        Self(from)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_id() {
        let id = Id::new(55);
        assert_eq!(format!("{}", id), "55");
    }
}
