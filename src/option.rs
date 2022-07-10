use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[clap(version, about = "Fun emote status for 3D printers from gcodes")]
pub struct Opt {
    #[clap(long, required(true), help = "The path to the gcode file")]
    pub gcode_path: PathBuf,

    #[clap(
        long,
        env = "EMOTO_PRINT_CUSTOM_EMOTE_PATH",
        help = "Optionally pass a path to a custom emote JSON file"
    )]
    pub custom_emote_path: Option<PathBuf>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_opt() {
        assert!(Opt::try_parse_from(Some("")).is_err());
    }
}
