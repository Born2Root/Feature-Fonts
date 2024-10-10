# Feature-Fonts
On this repository we release some of our font experiments using OpenType Font-Features.

## Friendly Font 😀
> A s$§t font that is f(%k&!"g censoring himself!

The "friendly Font" is an self censoring intelligent typeface.
It automatically removes unfriendly words and replaces them with random characters.
You could ask, what the f(%k?! Which d*?khead comes up with that s$§t!
... well the nice thing about that solution is, that what you see is not reality.
The user sees the censored version, but underlying the real uncensored text is still available. That means you do not have to change the source text in any way.
Isn't that cool? 

Well, try it out yourself!

[DEMO](https://Born2Root.github.io/Friendly-Font)

### How to adapt the Font to your swear-words:
The typeface can be easily adapted via Python.

- Install `rust python python-fonttools`
- Update `friendly_font_swearWords.txt`
- Run `cargo run` to generate opentype substitude feature code
- Run `python3 addfeatures.py -o friendly_font.otf friendly_font.otf substitute.fea` to apply it to a font
- Reload the `friendly_font_demo.html` to see and test the altered font.

## Emoji Font
Stay tuned, it will be 🥳

