use crate::error::Error;
use crate::Segment;
use anyhow::Result;

pub struct Version {
    patch: u16,
    minor: u16,
    major: u16,
}

impl Version {
    pub fn from_string(string: &str) -> Result<Self> {
        let segments = string.split(".").collect::<Vec<&str>>();
        if segments.len() != 3 {
            return Err(Error::InvalidSegmentCount(segments.len()).into());
        }

        Ok(Self {
            major: segments[0].parse()?,
            minor: segments[1].parse()?,
            patch: segments[2].parse()?,
        })
    }

    pub fn bump(&self, segment: &Segment) -> Self {
        match segment {
            Segment::Patch => Self {
                patch: self.patch + 1,
                minor: self.minor,
                major: self.major,
            },
            Segment::Minor => Self {
                patch: 0,
                minor: self.minor + 1,
                major: self.major,
            },
            Segment::Major => Self {
                patch: 0,
                minor: 0,
                major: self.major + 1,
            },
        }
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
