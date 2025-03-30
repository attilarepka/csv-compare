use anyhow::{Result, anyhow};
use clap::{ArgAction, Parser};
use colored::Colorize;
use csv::ReaderBuilder;
use inquire::Confirm;
use similar::{ChangeTag, TextDiff};

#[derive(Parser, Debug)]
#[command(author, version, about = None, long_about = None)]
struct Args {
    /// Orig CSV file
    #[arg(index = 1)]
    orig: String,
    /// Diff CSV file
    #[arg(index = 2)]
    diff: String,
    /// Orig index of column to compare
    #[arg(long)]
    orig_index: usize,
    /// Diff index of column to compare (optional, defaults to orig_index)
    #[arg(long, required = false)]
    diff_index: Option<usize>,
    /// Search prefix of selected rows
    #[arg(long, short, required = false)]
    with_prefix: Option<String>,
    /// Whether CSV's have headers
    #[arg(long, action = ArgAction::SetTrue)]
    with_headers: bool,
}

const DELIM: &str = "/";

fn filter_prefix(orig: &str) -> Result<String> {
    Ok(orig.split_once(DELIM).unwrap_or(("", "")).1.to_string())
}

fn parse_csv(
    path: &str,
    index: usize,
    with_prefix: Option<&str>,
    with_headers: bool,
) -> Result<Vec<String>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(with_headers)
        .from_path(path)?;
    let mut res = Vec::new();

    for record in reader.records() {
        if let Some(field) = record?.get(index - 1) {
            match with_prefix {
                Some(with_prefix) => {
                    if field.starts_with(with_prefix) {
                        res.push(filter_prefix(field)?);
                    }
                }
                None => {
                    res.push(field.to_string());
                    continue;
                }
            }
        }
    }
    Ok(res)
}

fn prompt_csv(orig: &[String], diff: &[String]) -> Result<()> {
    let ans = Confirm::new("Is this correct?")
        .with_default(false)
        .with_help_message(
            format!(
                "\norig has {} records, first record: {}\ndiff has {} records, first record: {}\n",
                orig.len(),
                orig.first().unwrap_or(&"N/A".to_string()),
                diff.len(),
                diff.first().unwrap_or(&"N/A".to_string())
            )
            .as_str(),
        )
        .prompt();

    match ans {
        Ok(true) => Ok(()),
        Ok(false) => Err(anyhow!(
            "User Interruption: The process has been interrupted. Exiting..."
        )),
        Err(err) => Err(err)?,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let orig_lines = parse_csv(
        args.orig.as_str(),
        args.orig_index,
        args.with_prefix.as_deref(),
        args.with_headers,
    )?;

    let diff_lines = parse_csv(
        args.diff.as_str(),
        args.diff_index.unwrap_or(args.orig_index),
        args.with_prefix.as_deref(),
        args.with_headers,
    )?;

    prompt_csv(&orig_lines, &diff_lines)?;

    println!("diff a/{} b/{}", args.orig.bold(), args.diff.bold());
    println!("---a/{}", args.orig.bold());
    println!("+++b/{}", args.diff.bold());

    let orig_slices: Vec<&str> = orig_lines.iter().map(|s| s.as_str()).collect();
    let diff_slices: Vec<&str> = diff_lines.iter().map(|s| s.as_str()).collect();

    let diff = TextDiff::from_slices(&orig_slices, &diff_slices);
    for hunk in diff.unified_diff().iter_hunks() {
        println!("{}", hunk.header().to_string().cyan());
        for change in hunk.iter_changes() {
            match change.tag() {
                ChangeTag::Delete => println!("{}{}", "-".red(), change.value().red()),
                ChangeTag::Insert => println!("{}{}", "+".green(), change.value().green()),
                ChangeTag::Equal => println!(" {}", change.value()),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::FileWriteStr;

    #[test]
    fn test_parse_csv() {
        let file = assert_fs::NamedTempFile::new("input.csv").unwrap();
        file.write_str("1,2,some some/some/path,4\n1,2,some some/some/other/path,4")
            .unwrap();
        let output = parse_csv(file.path().to_str().unwrap(), 3, None, false).unwrap();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "some some/some/path");
        assert_eq!(output[1], "some some/some/other/path");

        let output = parse_csv(file.path().to_str().unwrap(), 3, None, true).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "some some/some/other/path");

        let file = assert_fs::NamedTempFile::new("input.csv").unwrap();
        file.write_str("1,2,some some/other/path,4\n1,2,ayy some/ayy/other/path,4")
            .unwrap();

        let output = parse_csv(file.path().to_str().unwrap(), 3, Some("some"), false).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "other/path");
    }

    #[test]
    fn test_filter_prefix() {
        assert_eq!(filter_prefix("a a/1/1/1").unwrap(), "1/1/1");
    }
}
