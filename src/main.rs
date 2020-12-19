use structopt::StructOpt;

mod vm;
mod flags;
mod input;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short, long)]
    pub debug: bool,

    /// The path to the image file(s).
    pub files: Vec<std::path::PathBuf>,
}

fn main() {
    let opt = Options::from_args();
    println!("{:?}", opt);
}
