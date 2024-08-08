<div align="center">
<h1>jiffy</h1>

<a href="https://github.com/azzamsa/jiffy/actions/workflows/ci.yml">
<img src="https://github.com/azzamsa/jiffy/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://crates.io/crates/jiffy">
<img src="https://img.shields.io/crates/v/jiffy.svg">
</a>
<a href="https://docs.rs/jiffy/">
<img src="https://docs.rs/jiffy/badge.svg">
</a>

<p></p>

</div>

---

Representation for jiff objects in human languages.

## Usage

```rust
use jiff::ToSpan;

let dt = jiff::Zoned::now().checked_add(35.days()).unwrap();
let ht = jiffy::HumanTime::from(dt);
let english = format!("{}", ht);
assert_eq!("in a month", english);
```

## More Examples

To learn more, see other [examples](examples/).

## Acknowledgement

`jiffy` is a direct port of [chrono-humanize](https://gitlab.com/imp/chrono-humanize-rs).
