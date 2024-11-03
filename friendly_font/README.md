# Friendly Font

> A s$§t font that is f(%k&!"g censoring itself!

The "friendly Font" is an self censoring intelligent typeface.
It automatically removes unfriendly words and replaces them with random characters.
You could ask, what the f(%k?! Which d\*?khead comes up with that s$§t!
... well the nice thing about that solution is, that what you see is not reality.
The user sees the censored version, but underlying the real uncensored text is still available. That means you do not have to change the source text in any way.
Isn't that cool?

Well, try it out yourself!

## How to generate

You can use any font as a base.

Install `rust python python-fonttools`

Update `friendly_font_swearWords.txt`

Run `cargo run` to generate opentype substitude feature code

Run `python3 ../addfeatures.py -o friendly_font.otf Roboto-Regular.ttf substitute.fea` to apply it to a font

Use `friendly_font.otf` and type some bad words

## How does it work

In the [source](./src/main.rs) we read all words to censor.

We keep the first and last character so you can kinda see which word was written.

Replacing multiple characters with multiple other characters sadly does not work (or we just did not manage to get it working).
So we have to define a substitution rule for every character we want to change.

With an example of "shit":
We replace the "h" with a random character from our replacement list: "$".
`sub s h' i t by dollar;` so "shit" to "s$it".
Now we have to match our new word to replace the "i".
`sub s dollar i' t by section;` so "s$it" to "s$§t".
