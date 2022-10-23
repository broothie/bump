use crate::Segment;
use anyhow::Result;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn bump(&self, segment: Segment) -> Self {
        match segment {
            Segment::Patch => Self::new(self.major, self.minor, self.patch + 1),
            Segment::Minor => Self::new(self.major, self.minor + 1, 0),
            Segment::Major => Self::new(self.major + 1, 0, 0),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid number of segments: {0}")]
    InvalidSegmentCount(usize),

    #[error("unable to parse segment")]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.split(".").collect::<Vec<_>>();
        if segments.len() != 3 {
            return Err(Error::InvalidSegmentCount(segments.len()).into());
        }

        Ok(Self::new(
            segments[0].parse()?,
            segments[1].parse()?,
            segments[2].parse()?,
        ))
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod version {
    use super::Version;
    use crate::Segment;

    #[test]
    fn from_str() {
        assert_eq!("1.2.3".parse::<Version>().unwrap(), Version::new(1, 2, 3));
    }

    #[test]
    fn bump() {
        let version = Version::new(1, 2, 3);

        assert_eq!(version.bump(Segment::Major), Version::new(2, 0, 0));
        assert_eq!(version.bump(Segment::Minor), Version::new(1, 3, 0));
        assert_eq!(version.bump(Segment::Patch), Version::new(1, 2, 4));
    }

    #[test]
    fn to_string() {
        assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
    }
}
