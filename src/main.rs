use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use gcode::{parse, GCode, Mnemonic};
use serde::Deserialize;

#[derive(Debug)]
#[allow(dead_code)]
struct GCodeEmote {
    gcode: GCode,
    emote: Option<String>,
}

#[derive(Debug)]
struct Emote {
    mnemonic: Mnemonic,
    number: u32,
    emote: String,
}

#[derive(Deserialize, Debug)]
struct EmoteJson {
    letter: char,
    number: u32,
    emote: String,
}

fn load_template_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Emote>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let template: Vec<EmoteJson> = serde_json::from_reader(reader)?;
    let mut emotes: Vec<Emote> = Vec::new();

    for t in template {
        let emote = Emote {
            mnemonic: Mnemonic::for_letter(t.letter).unwrap(),
            number: t.number,
            emote: t.emote,
        };

        emotes.push(emote);
    }

    Ok(emotes)
}

fn main() {
    let template = load_template_from_file("emotes.json").unwrap();
    println!("{:#?}", template);

    let src: String = read_to_string("/home/paul/Desktop/CCR6SE_biological.gcode")
        .unwrap()
        .parse()
        .unwrap();
    let gcodes: Vec<_> = parse(&src).collect();
    let mut emotes: Vec<GCodeEmote> = Vec::new();

    for gcode in gcodes {
        let gcode_emote: GCodeEmote;
        if let Some(emote_code) = template
            .iter()
            .find(|x| x.mnemonic == gcode.mnemonic() && x.number == gcode.major_number())
        {
            gcode_emote = GCodeEmote {
                gcode,
                emote: Some(emote_code.emote.to_owned()),
            };
        } else {
            gcode_emote = GCodeEmote { gcode, emote: None };
        }
        emotes.push(gcode_emote);
    }

    for emote in emotes {
        println!("{:?}", emote);
    }
}
