use std::fs::read_to_string;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // pattern to look for
    pattern: String,
    // path of the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// #[derive(Debug)]
// struct CustomError(String);

use anyhow::{Context, Result};
fn main() -> Result<()> {
    // println!("Hello, world!");
    // 1. get the arguments
    // let pattern = std::env::args().nth(1).expect("no pattern");
    // let path = std::env::args().nth(1).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };
    // this should work, but to support `--pattern="foo"` or `--pattern "foo"` or implement `--help`
    // we need to parse CLI arguments

    // let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");
    //
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    // line 24-31 implement the find function
    // the read_to_string() function return a Result, this is a enum
    // let result = std::fs::read_to_string("test.txt");
    // match result {
    //     Ok(content) => { println!("File content: {}", content)},
    //     Err(error) => { println!("oops, {}", error)},
    // }

    // let result = std::fs::read_to_string("test.txt");
    // let content = match result {
    //     Ok(content) => {content},
    //     Err(error) => {println!("oops, something happens: {}", error)},
    // };
    // println!("file content: {}", content);

    // however, above function is what unwrap() function do
    // let content = std::fs::read_to_string("test.txt").unwrap();

    // if we don't need panic, we can return that error
    // let result = std::fs::read_to_string("test.txt");
    // let content = match result {
    //     Ok(content) => {content},
    //     Err(error) => {return Err(error.into());},
    // };
    // println!("file content: {}", content);
    // Ok(())

    // however, like unwrap(), we can use ? to append to a value of type Result
    // let content = std::fs::read_to_string("tesqt.txt")?;
    // println!("file content: {}", content);
    // Ok(())

    // let path = "tesqt.txt";
    // let content = std::fs::read_to_string(path)
    //     .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    // println!("file content: {}", content);
    // Ok(())

    // we can use the costum error with crate anyhow
    let path = "test.txtt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file; {}", path))?;
    println!("file content; {}", content);
    Ok(())

}



