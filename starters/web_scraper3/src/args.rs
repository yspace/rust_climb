use argh::FromArgs;

#[derive(FromArgs)]
/// Reach new heights.
struct GoUp {
    /// whether or not to jump
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high to go
    #[argh(option)]
    height: usize,

    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,
}

pub  mod v1{
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Debug, Parser)]
    struct Args {
        #[clap(long)]
        save_to: PathBuf,
    }

    #[test]
    fn requires_save_location() {
        // omitting the required argument generates an error
        assert!(Args::try_parse_from(&["app"]).is_err());

        // passing in the required argument allows Args to be constructed
        let _ = Args::parse_from(&["app", "--save-to=somewhere.db"]);
    }
}
