# Icon font

A font which allows you to type icons. Just like :Wifi:

## How to generate

In this example and source files we will be using `Roboto+SegMDL2.ttf` a mix of "Roboto" and ["Segoe MDL2 Assets icons"](https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-ui-symbol-font).
But you can generate your own font with icons and latin glyphs. See https://github.com/thedemons/merge_color_emoji_font

Install `rust python python-fonttools`

Run `cargo run` to generate opentype substitude feature code

Run `python3 ../addfeatures.py -o icon_font.ttf Roboto+SegMDL2.ttf substitute.fea` to apply it to a font

Use `icon_font.ttf` and type some icon names `:VPN:`

## How does it work

It works pretty similar to [our emoji font](../emoji_font/README.md).

In the [source](./src/main.rs) a hard coded list of unicode points and icon descriptions was inserted (from https://learn.microsoft.com/en-us/windows/apps/design/style/segoe-ui-symbol-font#pua-e700-e900).
Those unicode points are in the private use area.

We iterate over every icon and check the font for the glyph id. This glyph id can be used to get the glyph name of this unicode point in the font.
We have to do this as opentype font features do not allow substituting glyphs by their unicode point but only their glyph name which can be different in any font.

If you know how to substite a char by its unicode point or know a crate which allows us to get the glyph name by unicode point, please reach out via an issue. But as you can see in `substitute.fea` the mapping is not done greatly: "MapPin" should be `E707` but has the glyph name `uniE1C4`.

Now we can use the calt substite font feature to replace `:IconDescription:` with their glyph name. (`:` was chosen as it is a common delimiter for emojis and prevents accidential replacements. Its glyph name can also vary and does not have to be "colon").
Every character of the description also needs to be checked for the correct glyph name.

So with an example of "Wifi":
We start our font feature with `feature calt {` and check the glpyh if of `E701`: 1138.
This glyph id has the name "uniE701" in our font.
All the description characters do not need any special glpyh name.
So we get `sub colon' W' i' f' i' colon' by uniE701;`.
(all glyphs with `'` will be replaced)
Now we can end the feature `} calt;`.
After appling this feature to the font, the font engine does the rest of the work :D
