Enum variant utils
==================

Handy utilities for working with Rust enums.


* VariantDef - automatically define a const struct to
  make metadata about your enum variants available in a consistent way
* IsVariant - quickly check if something is an instance of a specific variant
* VariantName - quickly get (just) the variant name
* VariantPropsToJsonArray - get the variants properties as a `Vec<serde_json::Value>`
* StepCount - enums that models steps of a process - get `Step` that displays current
  position, e.g. `"3 / 12"`

More tools and detailed docs to follow soon.
