use structopt::StructOpt;
use colored::* ;

#[derive(StructOpt)]
struct Options{
    #[structopt(default_value="Meow!")]
    /// What does the cat say?
    message: String , // [1]

    #[structopt(short = "d" , long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile:ꀀOption<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    let eye = if options.dead { "x" } else { "o" }; // [1]

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog");
    }else{
//        println!("{}", message);//
         println!("{}",message.bright_yellow().underline()
         .on_purple());
    }


    println!(" \\");
    println!("ꂠ\\");
    println!("ꂠꂠꀀ/\\_/\\");
    println!("ꂠꂠ( {eye} {eye} )", eye=eye.red().bold()); // [2]
    println!("ꂠꂠ=( I )=");
}
