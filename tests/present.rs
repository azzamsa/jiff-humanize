macro_rules! duration_test {
    ($($name:ident: $duration:expr, $rough:expr, $precise:expr,)+) => {
        $(#[test]
        fn $name() {
            let ht = HumanTime::from($duration);
            let rough = ht.to_text_en(Accuracy::Rough, Tense::Present);
            let precise = ht.to_text_en(Accuracy::Precise, Tense::Present);
            assert_eq!($rough, rough);
            assert_eq!($precise, precise);
        })+
    }
}

#[cfg(test)]
mod duration {
    use jiff::ToSpan;
    use jiffy::{Accuracy, HumanTime, Tense};

    // test_name: Duration expression, "Rough text", "Precise text"
    duration_test! {
        now: jiff::Span::default(), "now", "0 seconds",
        plus_1s: 1.seconds(), "now", "1 second",
        minus_1s: (-1).seconds(), "now", "1 second",
        plus_5s: 5.seconds(), "now", "5 seconds",
        minus_5s: (-5).seconds(), "now", "5 seconds",
        plus_15s: 15.seconds(), "15 seconds", "15 seconds",
        minus_15s: (-15).seconds(), "15 seconds", "15 seconds",
        plus_95s: 95.seconds(), "2 minutes", "1 minute and 35 seconds",
        minus_95s: (-95).seconds(), "2 minutes", "1 minute and 35 seconds",
        plus_125s: 125.seconds(), "2 minutes", "2 minutes and 5 seconds",
        minus_125s: (-125).seconds(), "2 minutes", "2 minutes and 5 seconds",
        plus_31m: 31.minutes(), "31 minutes", "31 minutes",
        minus_31m: (-31).minutes(), "31 minutes", "31 minutes",
        plus_45m: 45.minutes(), "45 minutes", "45 minutes",
        minus_45m: (-45).minutes(), "45 minutes", "45 minutes",
        plus_46m: 46.minutes(), "an hour", "46 minutes",
        minus_46m: (-46).minutes(), "an hour", "46 minutes",
        plus_1h: 1.hours(), "an hour", "1 hour",
        minus_1h: (-1).hours(), "an hour", "1 hour",
        plus_12h: 12.hours(), "12 hours", "12 hours",
        minus_12h: (-12).hours(), "12 hours", "12 hours",
        plus_23h: 23.hours(), "a day", "23 hours",
        minus_23h: (-23).hours(), "a day", "23 hours",
        plus_26h: 26.hours(), "a day", "1 day and 2 hours",
        minus_26h: (-26).hours(), "a day", "1 day and 2 hours",
        plus_1d: 1.days(), "a day", "1 day",
        minus_1d: (-1).days(), "a day", "1 day",
        plus_2d: 2.days(), "2 days", "2 days",
        minus_2d: (-2).days(), "2 days", "2 days",
        plus_6d_13h: 6.days().checked_add(13.hours()).unwrap(), "a week", "6 days and 13 hours",
        minus_6d_13h: (-6).days().checked_add((-13).hours()).unwrap(), "a week", "6 days and 13 hours",
        plus_7d: 7.days(), "a week", "1 week",
        minus_7d: (-7).days(), "a week", "1 week",
        plus_10d: 10.days(), "a week", "1 week and 3 days",
        minus_10d: (-10).days(), "a week", "1 week and 3 days",
        plus_11d: 11.days(), "2 weeks", "1 week and 4 days",
        minus_11d: (-11).days(), "2 weeks", "1 week and 4 days",
        plus_4w: 4.weeks(), "4 weeks", "4 weeks",
        minus_4w: (-4).weeks(), "4 weeks", "4 weeks",
        plus_30d: 30.days(), "a month", "1 month",
        minus_30d: (-30).days(), "a month", "1 month",
        plus_45d: 45.days(), "a month", "1 month, 2 weeks and 1 day",
        minus_45d: (-45).days(), "a month", "1 month, 2 weeks and 1 day",
        plus_46d: 46.days(), "2 months", "1 month, 2 weeks and 2 days",
        minus_46d: (-46).days(), "2 months", "1 month, 2 weeks and 2 days",
        plus_24w: 24.weeks(), "5 months", "5 months, 2 weeks and 4 days",
        minus_24w: (-24).weeks(), "5 months", "5 months, 2 weeks and 4 days",
        plus_26w: 26.weeks(), "6 months", "6 months and 2 days",
        minus_26w: (-26).weeks(), "6 months", "6 months and 2 days",
        plus_50w: 50.weeks(), "a year", "11 months, 2 weeks and 6 days",
        minus_50w: (-50).weeks(), "a year", "11 months, 2 weeks and 6 days",
        plus_100w: 100.weeks(), "2 years", "1 year, 11 months and 5 days",
        minus_100w: (-100).weeks(), "2 years", "1 year, 11 months and 5 days",
        plus_101w: 101.weeks(), "2 years", "1 year, 11 months, 1 week and 5 days",
        minus_101w: (-101).weeks(), "2 years", "1 year, 11 months, 1 week and 5 days",
        plus_120w: 120.weeks(), "2 years", "2 years, 3 months, 2 weeks and 6 days",
        minus_120w: (-120).weeks(), "2 years", "2 years, 3 months, 2 weeks and 6 days",
        plus_200w: 200.weeks(), "3 years", "3 years, 10 months and 5 days",
        minus_200w: (-200).weeks(), "3 years", "3 years, 10 months and 5 days",
    }
}

#[cfg(test)]
mod utc {
    use jiffy::{Accuracy, HumanTime, Tense};

    #[test]
    fn now() {
        let ht = HumanTime::from(jiff::Zoned::now());
        let rough = ht.to_text_en(Accuracy::Rough, Tense::Present);
        assert_eq!("now", rough);
    }
}

#[cfg(test)]
mod local {
    use jiff::ToSpan;
    use jiffy::{Accuracy, HumanTime, Tense};

    #[test]
    fn now() {
        let ht = HumanTime::from(jiff::Zoned::now());
        let rough = ht.to_text_en(Accuracy::Rough, Tense::Present);
        assert_eq!("now", rough);
    }

    #[test]
    fn minus_35d() {
        let past = jiff::Zoned::now().checked_sub(35.days()).unwrap();
        let ht = HumanTime::from(past);
        let rough = ht.to_text_en(Accuracy::Rough, Tense::Present);
        assert_eq!("a month", rough);
    }

    #[test]
    fn plus_35d() {
        let future = jiff::Zoned::now().checked_add(35.days()).unwrap();
        let ht = HumanTime::from(future);
        let rough = ht.to_text_en(Accuracy::Rough, Tense::Present);
        assert_eq!("a month", rough);
    }
}
