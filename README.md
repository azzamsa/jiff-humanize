## Jiff Humanize

## Quick Start

```rust
use jiff::{Local, Duration};
use jiffy::HumanTime;

let dt = jiff::Zoned::now() + Duration::days(35);
let ht = HumanTime::from(dt);
let english = format!("{}", ht);
assert_eq!("in a month", english);
```
