use std::process::exit;

use clap::Parser;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
   /// Maze's height
   #[arg(long, default_value_t = 7)]
   pub height: i32,

   /// Maze's width
   #[arg(long, default_value_t = 7)]
   pub width: i32,
}

pub fn get_options() -> Args {
   let args = Args::parse();

   if args.height < 3 || args.width < 3 {
    println!("Height and width must but at least 3.");
    exit(0);
   }

   return args;
}