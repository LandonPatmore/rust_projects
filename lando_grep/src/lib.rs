use std::error::Error;
use std::{env, fs};

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  // &[String] is a slice of strings which is what a &Vec turns into
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let file_path = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file path"),
    };

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config {
      query,
      file_path,
      ignore_case,
    })
  }
}

// () just means that we return Unit on an Ok and dyn means that
// this error can be any type of trait Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // config.file_name is moved thus ownership is moved
  let contents =
    fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

  let results = search(config.ignore_case, &config.query, &contents);

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

// 'a is a lifetime annotation. It means that the references in the returned Vec<&str>
// are guaranteed to be valid for as long as the 'contents' reference is valid.
// We're returning slices of the original input instead of cloning the lines,
// so we tie the lifetimes to ensure the borrowed data stays valid
pub fn search<'a>(
  ignore_case: bool,
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| match ignore_case {
      true => line.to_lowercase().contains(&query.to_lowercase()),
      false => line.contains(query),
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(false, query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search(true, query, contents));
  }
}
