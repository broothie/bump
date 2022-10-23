mod version;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs,
    io::{self, BufRead},
};
use version::Version;

lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Segment {
    Patch,
    Minor,
    Major,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("no semver pattern found")]
    NoSemverFound,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    filename: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Segment::Patch, help = "SemVer segment to bump")]
    segment: Segment,

    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "If the input contains multiple SemVer patterns, use this to target by occurrence (1-indexed)",
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    number: u16,

    #[arg(
        short,
        long,
        help = "If the input contains multiple SemVer patterns, use this to target by line number (1-indexed)",
        value_parser = clap::value_parser!(u16).range(1..),
    )]
    line: Option<u16>,
}

fn main() -> Result<()> {
    let cli = CLI::parse();

    let contents = if let Some(filename) = cli.filename.clone() {
        fs::read_to_string(&filename)?
    } else {
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.map_err(Into::into))
            .collect::<Result<Vec<_>>>()?
            .join("\n")
    };

    let (output, version, bumped) = match cli.line {
        Some(line_number) => {
            replace_by_line(&contents, cli.segment, cli.number - 1, line_number - 1)
        }
        None => replace(&contents, cli.segment, cli.number - 1),
    }?;

    if let Some(filename) = cli.filename {
        fs::write(&filename, output)?;
        println!("{} -> {}", version.to_string(), bumped.to_string());
    } else {
        println!("{}", output);
    }

    Ok(())
}

fn replace(
    input: &String,
    segment: Segment,
    semver_index: u16,
) -> Result<(String, Version, Version)> {
    let version_match = VERSION_REGEX
        .find_iter(&input)
        .nth(semver_index.into())
        .ok_or_else(|| Error::NoSemverFound)?;

    let version = version_match.as_str().parse::<Version>()?;
    let bumped = version.bump(segment);

    let mut replaced = input.to_string();
    replaced.replace_range(version_match.range(), &bumped.to_string());

    Ok((replaced, version, bumped))
}

fn replace_by_line(
    input: &String,
    segment: Segment,
    semver_index: u16,
    line_index: u16,
) -> Result<(String, Version, Version)> {
    let mut result = None;

    let output = input
        .split("\n")
        .enumerate()
        .map(|(current_line_index, line)| {
            if current_line_index == line_index.into() {
                let (output, version, bumped) = replace(&line.to_string(), segment, semver_index)?;
                result = Some((version, bumped));

                Ok(output)
            } else {
                Ok(line.to_string())
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let (version, bumped) = result.ok_or(Error::NoSemverFound)?;
    Ok((output.join("\n"), version, bumped))
}

#[cfg(test)]
mod replace {
    use super::replace;
    use super::Segment;
    use crate::Version;

    #[test]
    fn index_is_0() {
        let input = Version::new(1, 2, 3);

        assert_eq!(
            replace(&input.to_string(), Segment::Major, 0).unwrap(),
            ("2.0.0".to_string(), input, Version::new(2, 0, 0)),
        );

        assert_eq!(
            replace(&input.to_string(), Segment::Minor, 0).unwrap(),
            ("1.3.0".to_string(), input, Version::new(1, 3, 0)),
        );

        assert_eq!(
            replace(&input.to_string(), Segment::Patch, 0).unwrap(),
            ("1.2.4".to_string(), input, Version::new(1, 2, 4))
        );
    }

    #[test]
    fn index_is_1() {
        let input = "pkg1 = 3.0.0\npkg2 = 1.2.3".to_string();

        assert_eq!(
            replace(&input, Segment::Major, 1).unwrap(),
            (
                "pkg1 = 3.0.0\npkg2 = 2.0.0".to_string(),
                Version::new(1, 2, 3),
                Version::new(2, 0, 0),
            ),
        );

        assert_eq!(
            replace(&input, Segment::Minor, 1).unwrap(),
            (
                "pkg1 = 3.0.0\npkg2 = 1.3.0".to_string(),
                Version::new(1, 2, 3),
                Version::new(1, 3, 0),
            ),
        );

        assert_eq!(
            replace(&input, Segment::Patch, 1).unwrap(),
            (
                "pkg1 = 3.0.0\npkg2 = 1.2.4".to_string(),
                Version::new(1, 2, 3),
                Version::new(1, 2, 4),
            ),
        );
    }
}

#[cfg(test)]
mod replace_by_line {
    use super::Segment;
    use crate::replace_by_line;
    use crate::Version;

    #[test]
    fn test() {
        let input = "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 1.2.3\n".to_string();

        assert_eq!(
            replace_by_line(&input, Segment::Major, 0, 2).unwrap(),
            (
                "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 2.0.0\n".to_string(),
                Version::new(1, 2, 3),
                Version::new(2, 0, 0),
            ),
        );

        assert_eq!(
            replace_by_line(&input, Segment::Minor, 0, 2).unwrap(),
            (
                "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 1.3.0\n".to_string(),
                Version::new(1, 2, 3),
                Version::new(1, 3, 0),
            ),
        );

        assert_eq!(
            replace_by_line(&input, Segment::Patch, 0, 2).unwrap(),
            (
                "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 1.2.4\n".to_string(),
                Version::new(1, 2, 3),
                Version::new(1, 2, 4),
            ),
        );
    }
}
