mod emote;
mod option;

use anyhow::Result;
use clap::Parser;

use emote::{read_gcode_file, GCodeEmote};
use option::Opt;

fn main() {
    let opt = Opt::parse();
    let gcode_path = &opt.gcode_path;
    let custom_emote_path = &opt.custom_emote_path;
    let emotes: Result<Vec<GCodeEmote>>;
    if let Some(path) = custom_emote_path {
        emotes = read_gcode_file(gcode_path, &Some(path));
    } else {
        emotes = read_gcode_file(gcode_path, &None);
    }

    if let Ok(emotes) = emotes {
        for entry in emotes {
            if let Some(emote) = entry.emote {
                println!(
                    "{}{}: {}",
                    entry.gcode.mnemonic(),
                    entry.gcode.major_number(),
                    emote
                );
            } else {
                println!(
                    "{}{}: unknown",
                    entry.gcode.mnemonic(),
                    entry.gcode.major_number()
                );
            }
        }
    } else {
        println!("Error loading emotes");
    }
}
