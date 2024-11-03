# Emoji font

A font which allows you to type emojis. Just like :smiley:

## How to generate

In this example and source files we will be using `seguiemj.ttf` (or `TwitterColorEmoji+RobotoBasev2.ttf`).
But you can generate your own font with icons and latin glyphs. See https://github.com/thedemons/merge_color_emoji_font

Install `rust python python-fonttools`

Define the path/name as a environment variable `export FONT_PATH=seguiemj.ttf`

Run `cargo run` to generate opentype substitude feature code

Run `python3 ../addfeatures.py -o emoji_font.ttf $FONT_PATH substitute.fea` to apply it to a font

Use `emoji_font.ttf` and type some emoji names `:rocket:`

## Motivation

Just for fun. Because why not.
Other emoji fonts use font features too. I.e. material icons or font awesome. But those require you do use their font for places where you want emoji and not use it where you do not want emoji. No way to opt in or out.
We use the ":emoji:" notation, you can opt into an emoji if you like.

It allows using emoji in software that does not support them nativly.
E.e. with a database using utf8mb3
It is kinda wastefull in terms of storage. utf8mb4 would take 4byte for an emoji. Using our font and utf8mb3 can take so much more: 30 x 3byte for `:stuck_out_tongue_winking_eye:`.

## How does it work

It works pretty similar to [our icon font](../icon_font/README.md).

In the [source](./src/main.rs) we get all emojis defined in the unicode spec from the `emojis` crate.

We iterate over every emoji with a shortcode and check the font for the glyph id. This glyph id can be used to get the glyph name of this unicode point in the font.
We have to do this as opentype font features do not allow substituting glyphs by their unicode point but only their glyph name which can be different in any font.

If you know how to substite a char by its unicode point or know a crate which allows us to get the glyph name by unicode point, please reach out via an issue. But as you can see in `substitute.fea` the mapping is not done greatly: "\_" should be `U+005F` but has the glyph name `underscore`.

Now we can use the calt substite font feature to replace `:EmojiShortcode:` with their glyph name. (`:` was chosen as it is a common delimiter for emojis and prevents accidential replacements. Its glyph name can also vary and does not have to be "colon").
Every character of the description also needs to be checked for the correct glyph name.

So with an example of "rocket":
We start our font feature with `feature calt {` and check the glpyh if of `U+1F680`: 3491.
This glyph id has the name "u1F680" in our font.
All the description characters do not need any special glpyh name.
So we get `sub colon' r' o' c' k' e' t' colon' by u1F680;`.
(all glyphs with `'` will be replaced)
Now we can end the feature `} calt;`.
After appling this feature to the font, the font engine does the rest of the work :D
