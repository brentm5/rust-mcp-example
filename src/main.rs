use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  debug: bool,
}


fn main() {
  let args = Args::parse();

  if args.debug {
    println!("Debug mode is on");
  } else {
    println!("Debug mode is off");
  }
}
