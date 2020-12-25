use structopt::StructOpt;

mod flags;
mod input;
mod vm;

pub(crate) use self::input::Source;
pub(crate) use self::vm::VM;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short, long)]
    pub debug: bool,

    /// The path to the image file(s).
    pub files: Vec<std::path::PathBuf>,
}

fn main() {
    let opt = Options::from_args();
    let source = Source::infer(opt.files);
    let vm = VM::new(); // this will need arguments and whatnot soon
    // vm.rum(&source);
}
