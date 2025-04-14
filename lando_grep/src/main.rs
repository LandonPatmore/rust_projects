use lando_grep::Config;
use std::{env, process};

fn main() {
  let config = Config::build(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem building config: {}", err);
    process::exit(1);
  });

  if let Err(e) = lando_grep::run(config) {
    eprintln!("Application error: {e}");
    process::exit(1);
  }
}
