mod emote;
mod option;

use clap::Parser;

use emote::read_gcode_file;
use option::Opt;

fn main() {
    let opt = Opt::parse();
    let gcode_path = &opt.gcode_path;
    let emotes = read_gcode_file(gcode_path);

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
