use font_kit::font::Font;
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::Path,
};

fn main() -> std::io::Result<()> {
    let mut output = String::from("feature calt {");

    let font_path = &match env::var("FONT_PATH") {
        Ok(path) => path,
        Err(_) => {
            eprintln!("variable $FONT_PATH not defined");
            std::process::exit(1);
        }
    };

    let mut file = File::open(font_path)?;
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data)?;

    let font = Font::from_path(Path::new(font_path), 0).unwrap();

    let face = match ttf_parser::Face::parse(&font_data, 0) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    for emoji in emojis::iter().filter(|emoji| emoji.shortcode().is_some()) {
        let emoji_char = emoji.as_str().chars().next().unwrap();

        let glyph_id_emoji = font.glyph_for_char(emoji_char);

        if glyph_id_emoji.is_none() {
            println!(
                "no glyph_id for {}: {}",
                emoji.shortcode().unwrap(),
                emoji_char
            );
            continue;
        }
        let glyph_id_emoji = glyph_id_emoji.unwrap();

        let emoji_name;
        if let Some(name) = face.glyph_name(ttf_parser::GlyphId(glyph_id_emoji as u16)) {
            emoji_name = name.to_string();
        } else {
            println!("No glyph names found.");
            continue;
        }
        output.push_str(&format!("\n  # replace {}", emoji.name()));

        output = format!(
            "{output}\n  sub colon' {}colon' by {};\n",
            emoji
                .shortcode()
                .unwrap()
                .chars()
                .map(|c| {
                    let glyph_id = font.glyph_for_char(c).unwrap();

                    if let Some(name) = face.glyph_name(ttf_parser::GlyphId(glyph_id as u16)) {
                        return format!("{name}' ");
                    } else {
                        println!("No glyph names found.");
                    }
                    String::new()
                })
                .collect::<String>(),
            emoji_name
        );
    }

    output = format!("{output}\n}} calt;");

    fs::write("substitute.fea", output)?;

    Ok(())
}
