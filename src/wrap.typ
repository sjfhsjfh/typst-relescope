#let bin = plugin("../bin/plugin.wasm")


#let supported-langs = ("rust",)

/// Pick a target from a source
#let pick(
  src,
  target,
  lang: "rust",
) = {
  assert(lower(lang) in supported-langs, message: "Unsupported language: " + lang)
  json(bin.pick(bytes(src), bytes(target), bytes(lower(lang))))
}
