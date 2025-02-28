#import "../src/lib.typ": pick

#let data = pick(read("../src/relescope-rs/src/lib.rs"), bytes("PickResult"))

#raw(data.src, lang: "rust")


#let data = pick(read("./test.py"), bytes("Test"), lang: "python")

#raw(data.src, lang: "python")
