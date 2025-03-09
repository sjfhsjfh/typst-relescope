#import "../src/lib.typ": pick, scope

= Rust

#let items = scope(read("../src/relescope-rs/src/lib.rs"), lang: "rust")

#let data = pick(read("../src/relescope-rs/src/lib.rs"), bytes("ItemInfo"))

Here's what's returned from a pick:

#raw(data.src, lang: "rust")

The `pick` function

#raw(items.pick.src, lang: "rust")

The items in the `lib.rs` file: #items.keys()

= Python

#let items = scope(read("./test.py"), lang: "python")

#raw(items.test.src, lang: "python")

#let data = pick(read("./test.py"), bytes("Test"), lang: "python")

#raw(data.src, lang: "python")
