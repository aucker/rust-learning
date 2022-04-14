use clap::Parse;
#[derive(Parser)]
struct Cli {
    // pattern to look for
    pattern: String,
    // path of the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)?;
    for line in content.lines() {
        if line.contain(&args.pattern) {
            println!("{}", line);
        }
    }

}