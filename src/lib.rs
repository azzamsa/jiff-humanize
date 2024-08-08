//! Representation for jiff objects in human languages
//!
//! # Quick Start
//!
//! `HumanTime` objects are created from jiff objects, such as `jiff::DateTime`
//! and `jiff::Span`
//!
//! # Examples
//!
//! Convert current time taken as `now` to `HumanTime`
//!
//! ```
//! let dt = jiff::Zoned::now();
//! let ht = jiffy::HumanTime::from(dt);
//!
//! assert_eq!("now", format!("{}", ht));
//! ```
//!
//!
//! ```
//! use jiff::ToSpan;
//!
//! let dt = jiff::Zoned::now().checked_sub(58.minutes()).unwrap();
//! let ht = jiffy::HumanTime::from(dt);
//!
//! assert_eq!("an hour ago", format!("{}", ht));
//! ```
//!
//! For full control over the text representation use `HumanTime::to_text_en()`
//!
//! ```
//! use jiff::ToSpan;
//! use jiffy::{Accuracy, HumanTime, Tense};
//!
//! # fn main() {
//! let dt = 45.days();
//! let ht = HumanTime::from(dt);
//!
//! assert_eq!("a month", ht.to_text_en(Accuracy::Rough, Tense::Present).unwrap());
//! assert_eq!("1 month and 15 days", ht.to_text_en(Accuracy::Precise, Tense::Present).unwrap());
//! // FIXME
//! // assert_eq!("1 month, 2 weeks and 1 day", ht.to_text_en(Accuracy::Precise, Tense::Present).unwrap());
//! # }
//! ```

#![warn(clippy::use_self)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

pub use crate::humantime::{Accuracy, HumanTime, Tense};

mod error;
mod humantime;

pub use error::Error;

/// Present the object in human friendly text form
pub trait Humanize {
    /// Emits `String` that represents current object in human friendly form
    fn humanize(&self) -> String;
}
