use structopt::StructOpt;

pub mod vm;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short, long)]
    pub debug: bool,
}

fn main() {
    let opt = Options::from_args();
    println!("{:?}", opt);
}
