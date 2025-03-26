use anyhow::{Result, anyhow};
use clap::{ArgAction, Parser};
use colored::Colorize;
use csv::ReaderBuilder;
use inquire::Confirm;

#[derive(Parser, Debug)]
#[command(author, version, about = "csv-compare", long_about = None)]
struct Args {
    /// Input source
    #[arg(long, short)]
    src: String,
    /// Destination source
    #[arg(long, short)]
    dst: String,
    /// Source index of column
    #[arg(long)]
    src_index: usize,
    /// Destination index of column
    #[arg(long)]
    dst_index: usize,
    /// Search prefix of selected rows
    #[arg(long, short, required = false)]
    with_prefix: Option<String>,
    /// Whether CSV's have headers
    #[arg(long, action= ArgAction::SetFalse)]
    with_headers: bool,
}

const DELIM: &str = "/";

fn filter_prefix(src: &str) -> Result<String> {
    Ok(src.split_once(DELIM).unwrap_or(("", "")).1.to_string())
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
            if field.starts_with(with_prefix.unwrap_or("")) {
                res.push(filter_prefix(field)?);
            }
        } else {
            Err(anyhow!(
                "Index Not Found: The expected index '{}' was not found.",
                index
            ))?;
        }
    }

    Ok(res)
}

fn prompt_csv(src: &[String], dst: &[String]) -> Result<()> {
    let ans = Confirm::new("Is this correct?")
        .with_default(false)
        .with_help_message(
            format!(
                "\nsrc has {} records, first record: {}\ndst has {} records, first record: {}\n",
                src.len(),
                src.first().unwrap(),
                dst.len(),
                dst.first().unwrap(),
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

fn filter_diff<'a>(src: &'a [String], dst: &[String]) -> Result<Vec<&'a String>> {
    Ok(src
        .iter()
        .filter(|e| !dst.iter().any(|dst| dst.contains(*e)))
        .collect())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let src_paths = parse_csv(
        args.src.as_str(),
        args.src_index,
        args.with_prefix.as_deref(),
        args.with_headers,
    )?;

    let dst_paths = parse_csv(
        args.dst.as_str(),
        args.dst_index,
        args.with_prefix.as_deref(),
        args.with_headers,
    )?;
    prompt_csv(&src_paths, &dst_paths)?;

    let src_diff = filter_diff(&src_paths, &dst_paths)?;
    for src_elem in src_diff {
        println!("{} {}", "+".green(), src_elem.green());
    }

    let dst_diff = filter_diff(&dst_paths, &src_paths)?;
    for dst_elem in dst_diff {
        println!("{} {}", "-".red(), dst_elem.red());
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
        assert_eq!(output[0], "some/path");
        assert_eq!(output[1], "some/other/path");

        let output = parse_csv(file.path().to_str().unwrap(), 3, None, true).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "some/other/path");

        assert!(parse_csv(file.path().to_str().unwrap(), 5, None, false).is_err());

        let file = assert_fs::NamedTempFile::new("input.csv").unwrap();
        file.write_str("1,2,some some/other/path,4\n1,2,ayy some/ayy/other/path,4")
            .unwrap();

        let output = parse_csv(file.path().to_str().unwrap(), 3, Some("some"), false).unwrap();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "other/path");
    }

    #[test]
    fn test_filter_diff() {
        let src = vec!["1", "1", "2"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let dst = vec!["1", "1", "3"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        assert_eq!(filter_diff(&src, &dst).unwrap(), vec![&"2".to_string()]);
        assert_eq!(filter_diff(&dst, &src).unwrap(), vec![&"3".to_string()]);
    }

    #[test]
    fn test_filter_prefix() {
        assert_eq!(filter_prefix("a a/1/1/1").unwrap(), "1/1/1");
    }
}
