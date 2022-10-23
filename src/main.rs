mod version;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
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
    filename: String,

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
    let contents = fs::read_to_string(&cli.filename)?;

    let replaced = match cli.line {
        Some(line_number) => {
            replace_by_line(&contents, cli.segment, cli.number - 1, line_number - 1)
        }
        None => replace(&contents, cli.segment, cli.number - 1),
    }?;

    fs::write(&cli.filename, replaced)?;
    Ok(())
}

fn replace(input: &String, segment: Segment, index: u16) -> Result<String> {
    let version_match = VERSION_REGEX
        .find_iter(&input)
        .nth(index.into())
        .ok_or_else(|| Error::NoSemverFound)?;

    let version = version_match.as_str().parse::<Version>()?;
    let bumped = version.bump(segment);
    println!("{} -> {}", version.to_string(), bumped.to_string());

    let mut clone = input.to_string();
    clone.replace_range(version_match.range(), &bumped.to_string());
    Ok(clone)
}

fn replace_by_line(
    input: &String,
    segment: Segment,
    index: u16,
    line_index: u16,
) -> Result<String> {
    Ok(input
        .split("\n")
        .enumerate()
        .map(|(current_line_index, line)| {
            if current_line_index == line_index.into() {
                replace(&line.to_string(), segment, index)
            } else {
                Ok(line.to_string())
            }
        })
        .collect::<Result<Vec<_>>>()?
        .join("\n"))
}

#[cfg(test)]
mod replace {
    use super::replace;
    use super::Segment;

    #[test]
    fn index_is_0() {
        let input = "1.2.3".to_string();

        assert_eq!(replace(&input, Segment::Major, 0).unwrap(), "2.0.0");
        assert_eq!(replace(&input, Segment::Minor, 0).unwrap(), "1.3.0");
        assert_eq!(replace(&input, Segment::Minor, 0).unwrap(), "1.3.0");
    }

    #[test]
    fn index_is_1() {
        let input = "pkg1 = 3.0.0\npkg2 = 1.2.3".to_string();

        assert_eq!(
            replace(&input, Segment::Major, 1).unwrap(),
            "pkg1 = 3.0.0\npkg2 = 2.0.0"
        );

        assert_eq!(
            replace(&input, Segment::Minor, 1).unwrap(),
            "pkg1 = 3.0.0\npkg2 = 1.3.0"
        );

        assert_eq!(
            replace(&input, Segment::Minor, 1).unwrap(),
            "pkg1 = 3.0.0\npkg2 = 1.3.0"
        );
    }
}

#[cfg(test)]
mod replace_by_line {
    use super::Segment;
    use crate::replace_by_line;

    #[test]
    fn test() {
        let input = "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 1.2.3\n".to_string();

        assert_eq!(
            replace_by_line(&input, Segment::Major, 0, 2).unwrap(),
            "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 2.0.0\n",
        );

        assert_eq!(
            replace_by_line(&input, Segment::Minor, 0, 2).unwrap(),
            "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 1.3.0\n",
        );

        assert_eq!(
            replace_by_line(&input, Segment::Patch, 0, 2).unwrap(),
            "pkg1 = 3.0.0\npkg2 = 5.4.3\npkg3 = 1.2.4\n",
        );
    }
}
