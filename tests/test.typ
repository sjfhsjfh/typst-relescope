#import "../src/lib.typ": pick

#let data = pick(read("../src/relescope-rs/src/lib.rs"), bytes("PickResult"))

#raw(data.src, lang: "rust")
