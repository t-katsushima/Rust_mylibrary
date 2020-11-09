// https://doc.rust-lang.org/std/option/enum.Option.html

// None なら42, Some(v) なら v.len
x.map_or(42, |v| v.len())