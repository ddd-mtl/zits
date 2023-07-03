use std::path::PathBuf;
use structopt::StructOpt;

const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug, StructOpt, Clone)]
#[structopt(about = DESCRIPTION, after_help = "This command helps generate typescript bindings for zome code. Compatible with holochain-client-js v0.12.0")]
struct Args {
    /// Activate debug mode
    #[structopt(long, help = "Dry-run, prints to stdout", long = "debug")]
    debug: bool,

    ///
    #[structopt(
    long = "no-hc-imports",
    help = "Do not add generate imports for holochain types",
    )]
    no_hc_imports: bool,

    ///
    #[structopt(
    long = "no-proxy",
    help = "Do not generate a ZomeProxy subclass",
    )]
    no_proxy: bool,

    /// zome-name
    #[structopt(
    long = "--default-zome-name",
    help = "Set the DEFAULT_ZOME_NAME for the generated proxy. Default is output's filename.",
    )]
    zome_name: Option<String>,

    /// Other Typescript files to import
    #[structopt(
    short = "d",
    long = "dependency",
    help = "Verbatim Typescript added to import section of generated files. Use for adding imports to your own typescript types",
    )]
    external_imports: Vec<String>,

    /// Input file
    #[structopt(
    short = "i",
    long = "input",
    help = "Required; rust file(s) to read type information from",
    required = true
    )]
    input: Vec<PathBuf>,

    /// Output file (this is the "<name>.d.ts" that gets generated)
    #[structopt(
    parse(from_os_str),
    short = "o",
    long = "output",
    help = "Required; file to write generated types to"
    )]
    output: PathBuf,
}

fn main() {
    let args: Args = Args::from_args();
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("Running zits v{}", VERSION);
    zits::generate_typescript_bindings(
        args.input,
        args.external_imports,
        args.output,
        args.debug,
        !args.no_hc_imports,
        !args.no_proxy, args.zome_name);
}
