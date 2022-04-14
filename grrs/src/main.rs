use clap::Parser;

#[derive(Parser)]
struct Cli {
    // pattern to look for
    pattern: String,
    // path of the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
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

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // line 24-31 implement the find function
}
