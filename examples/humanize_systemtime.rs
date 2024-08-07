use std::time::Duration as StdDuration;
use std::time::SystemTime;

use jiff_humanize::{HumanTime, Humanize};

fn humanize_systemtime(src_time: Option<SystemTime>) -> Option<String> {
    src_time.map(|time| time.humanize())
}

fn main() {
    let time = SystemTime::now() - StdDuration::from_secs(46);

    let humantime = HumanTime::from(time);
    println!("precise : '{:#}'", humantime);
    println!("rough   : '{}'", humantime);

    if let Some(text) = humanize_systemtime(Some(time)) {
        println!("{:#}", text);
    }
}
