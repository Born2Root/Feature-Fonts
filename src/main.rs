use std::fs;

use rand::{thread_rng, Rng};

fn main() -> std::io::Result<()> {
    let contents =
        fs::read_to_string("./swearWords.txt").expect("Should have been able to read the file");

    let mut words = contents.split('\n').collect::<Vec<&str>>();
    // replace longes word first, otherwise the long word replacement won't match
    words.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut output = String::from("feature calt {");

    let mut rng = thread_rng();
    let replacements = vec![
        "x",           // x
        "asterisk",    // *
        "percent",     // %
        "numbersign",  // #
        "at",          // @
        "exclam",      // !
        "ampersand",   // &
        "section",     // §
        "dollar",      // $
        "asciicircum", // ^
        "question",    // ?
        "parenleft",   // (
        "0",           // 0
    ];

    for word in words {
        let first = word.chars().next().unwrap();
        let last = word.chars().last().unwrap();

        output = format!("{output}\n    # replace {word}");

        let mut used_replacements = Vec::new();

        // leave first and last letter as is
        for (index, char) in word.get(1..word.len() - 1).unwrap().chars().enumerate() {
            // get random replace letter
            let i = rng.gen_range(0..replacements.len() - 1);
            let replace = replacements.get(i).unwrap();

            let mut otherchars = String::new();
            for x in word.get(index + 2..word.len() - 1).unwrap().chars() {
                otherchars = format!("{otherchars} {x}");
            }

            let mut previous_replace = String::new();
            for prev in &used_replacements {
                previous_replace = format!("{previous_replace} {prev}");
            }

            output = format!("{output}\n    sub {first}{previous_replace} {char}'{otherchars} {last} by {replace};");

            used_replacements.push(replace);
        }

        output = format!("{output}\n");
    }

    output = format!("{output}\n}} calt;");

    fs::write("substitute.fea", output)?;

    Ok(())
}
