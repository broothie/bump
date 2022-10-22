mod error;
mod version;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use error::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};
use version::Version;

lazy_static! {
    static ref VERSION_REGEX: Regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Segment {
    Patch,
    Minor,
    Major,
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
        help = "If the input contains multiple SemVer patterns, use this to target by occurrence (1-indexed)"
    )]
    number: usize,

    #[arg(
        short,
        long,
        help = "If the input contains multiple SemVer patterns, use this to target by line number (1-indexed)"
    )]
    line: Option<usize>,
}

fn main() -> Result<()> {
    let cli = CLI::parse();
    let file = File::open(&cli.filename)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut found = false;
    let mut semver_count = 0;
    for (current_line_index, line_text) in reader.lines().enumerate() {
        let mut line_text = line_text?;

        if found {
            lines.push(line_text);
            continue;
        }

        let version_match = match cli.line {
            Some(target_line_number) => {
                if target_line_number == current_line_index + 1 {
                    let version_match = VERSION_REGEX
                        .find_iter(&line_text)
                        .nth(cli.number - 1)
                        .ok_or_else(|| Error::NoSemverFound(line_text.clone()))?;

                    Some(version_match)
                } else {
                    None
                }
            }
            _ => {
                let matches = VERSION_REGEX.find_iter(&line_text).collect::<Vec<_>>();

                if semver_count + matches.len() >= cli.number {
                    matches
                        .get(cli.number - 1 - semver_count)
                        .map(|m| m.to_owned())
                } else {
                    semver_count += matches.len();
                    None
                }
            }
        };

        if let Some(version_match) = version_match {
            let version = Version::from_string(version_match.as_str())?;
            let bumped = version.bump(&cli.segment);

            let mut clone = line_text.clone();
            clone.replace_range(version_match.range(), &bumped.to_string());
            line_text = clone;

            found = true;
            println!("{} -> {}", version.to_string(), bumped.to_string());
        }

        lines.push(line_text);
    }

    if !found {
        return Err(Error::NoSemverFound(cli.filename).into());
    }

    fs::write(cli.filename, lines.join("\n"))?;
    Ok(())
}
