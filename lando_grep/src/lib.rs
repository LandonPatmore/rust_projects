use std::error::Error;
use std::{env, fs};

pub struct Config {
  pub query: String,
  pub file_name: String,
  pub ignore_case: bool,
}

impl Config {
  // &[String] is a slice of strings which is what a &Vec turns into
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let file_name = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config {
      query,
      file_name,
      ignore_case,
    })
  }
}

// () just means that we return Unit on an Ok and dyn means that
// this error can be any type of trait Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // config.file_name is moved thus ownership is moved
  let contents =
    fs::read_to_string(config.file_name).expect("Something went wrong reading the file");

  let results = match config.ignore_case {
    true => search_case_insensitive(&config.query, &contents),
    false => search(&config.query, &contents),
  };

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
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

// Same here as above
pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
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

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
