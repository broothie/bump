use crate::Segment;
use anyhow::Result;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    pub fn bump(&self, segment: &Segment) -> Self {
        match segment {
            Segment::Patch => Self {
                major: self.major,
                minor: self.minor,
                patch: self.patch + 1,
            },
            Segment::Minor => Self {
                major: self.major,
                minor: self.minor + 1,
                patch: 0,
            },
            Segment::Major => Self {
                major: self.major + 1,
                minor: 0,
                patch: 0,
            },
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

        Ok(Self {
            major: segments[0].parse()?,
            minor: segments[1].parse()?,
            patch: segments[2].parse()?,
        })
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

    fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    #[test]
    fn from_str() {
        assert_eq!("1.2.3".parse::<Version>().unwrap(), new(1, 2, 3));
    }

    #[test]
    fn bump() {
        let version = new(1, 2, 3);

        assert_eq!(version.bump(&Segment::Major), new(2, 0, 0));
        assert_eq!(version.bump(&Segment::Minor), new(1, 3, 0));
        assert_eq!(version.bump(&Segment::Patch), new(1, 2, 4));
    }

    #[test]
    fn to_string() {
        assert_eq!(new(1, 2, 3).to_string(), "1.2.3");
    }
}
