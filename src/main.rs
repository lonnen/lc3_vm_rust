use structopt::StructOpt;

pub mod vm;

mod flags;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short, long)]
    pub debug: bool,
}

fn main() {
    let opt = Options::from_args();
    println!("{:?}", opt);
}
