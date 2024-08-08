extern crate jiffy;

#[cfg(test)]
mod duration {
    use jiff::ToSpan;
    use jiffy::Humanize;

    #[test]
    fn now() {
        let english = jiff::Span::default().humanize();
        assert_eq!("now", english);
    }

    #[test]
    fn plus_5s() {
        let english = 5.seconds().humanize();
        assert_eq!("now", english);
    }

    #[test]
    fn minus_5s() {
        let english = (-5).seconds().humanize();
        assert_eq!("now", english);
    }

    #[test]
    fn plus_15s() {
        let english = 15.seconds().humanize();
        assert_eq!("in 15 seconds", english);
    }

    #[test]
    fn minus_15s() {
        let english = (-15).seconds().humanize();
        assert_eq!("15 seconds ago", english);
    }

    #[test]
    fn plus_95s() {
        let english = 95.seconds().humanize();
        assert_eq!("in 2 minutes", english);
    }

    #[test]
    fn minus_95s() {
        let english = (-95).seconds().humanize();
        assert_eq!("2 minutes ago", english);
    }

    #[test]
    fn plus_125s() {
        let english = 125.seconds().humanize();
        assert_eq!("in 2 minutes", english);
    }

    #[test]
    fn minus_125s() {
        let english = (-125).seconds().humanize();
        assert_eq!("2 minutes ago", english);
    }

    #[test]
    fn plus_31m() {
        let english = 31.minutes().humanize();
        assert_eq!("in 31 minutes", english);
    }

    #[test]
    fn minus_31m() {
        let english = (-31).minutes().humanize();
        assert_eq!("31 minutes ago", english);
    }

    #[test]
    fn plus_45m() {
        let english = 45.minutes().humanize();
        assert_eq!("in 45 minutes", english);
    }

    #[test]
    fn minus_45m() {
        let english = (-45).minutes().humanize();
        assert_eq!("45 minutes ago", english);
    }

    #[test]
    fn plus_46m() {
        let english = 46.minutes().humanize();
        assert_eq!("in an hour", english);
    }

    #[test]
    fn minus_46m() {
        let english = (-46).minutes().humanize();
        assert_eq!("an hour ago", english);
    }

    #[test]
    fn plus_1h() {
        let english = 1.hours().humanize();
        assert_eq!("in an hour", english);
    }

    #[test]
    fn minus_1h() {
        let english = (-1).hours().humanize();
        assert_eq!("an hour ago", english);
    }

    #[test]
    fn plus_12h() {
        let english = 12.hours().humanize();
        assert_eq!("in 12 hours", english);
    }

    #[test]
    fn minus_12h() {
        let english = (-12).hours().humanize();
        assert_eq!("12 hours ago", english);
    }

    #[test]
    fn plus_23h() {
        let english = 23.hours().humanize();
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_23h() {
        let english = (-23).hours().humanize();
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_26h() {
        let english = 26.hours().humanize();
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_26h() {
        let english = (-26).hours().humanize();
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_1d() {
        let english = 1.days().humanize();
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_1d() {
        let english = (-1).days().humanize();
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_2d() {
        let english = 2.days().humanize();
        assert_eq!("in 2 days", english);
    }

    #[test]
    fn minus_2d() {
        let english = (-2).days().humanize();
        assert_eq!("2 days ago", english);
    }

    #[test]
    fn plus_6d_13h() -> anyhow::Result<()> {
        let english = 6.days().checked_add(13.hours())?.humanize();
        assert_eq!("in a week", english);
        Ok(())
    }

    #[test]
    fn minus_6d_13h() -> anyhow::Result<()> {
        let english = (-6).days().checked_add((-13).hours())?.humanize();
        assert_eq!("a week ago", english);
        Ok(())
    }

    #[test]
    fn plus_7d() {
        let english = 7.days().humanize();
        assert_eq!("in a week", english);
    }

    #[test]
    fn minus_7d() {
        let english = (-7).days().humanize();
        assert_eq!("a week ago", english);
    }

    #[test]
    fn plus_10d() {
        let english = 10.days().humanize();
        assert_eq!("in a week", english);
    }

    #[test]
    fn minus_10d() {
        let english = (-10).days().humanize();
        assert_eq!("a week ago", english);
    }

    #[test]
    fn plus_11d() {
        let english = 11.days().humanize();
        assert_eq!("in 2 weeks", english);
    }

    #[test]
    fn minus_11d() {
        let english = (-11).days().humanize();
        assert_eq!("2 weeks ago", english);
    }

    #[test]
    fn plus_4w() {
        let english = 4.weeks().humanize();
        assert_eq!("in 4 weeks", english);
    }

    #[test]
    fn minus_4w() {
        let english = (-4).weeks().humanize();
        assert_eq!("4 weeks ago", english);
    }

    #[test]
    fn plus_30d() {
        let english = 30.days().humanize();
        assert_eq!("in a month", english);
    }

    #[test]
    fn minus_30d() {
        let english = (-30).days().humanize();
        assert_eq!("a month ago", english);
    }

    #[test]
    fn plus_45d() {
        let english = 45.days().humanize();
        assert_eq!("in a month", english);
    }

    #[test]
    fn minus_45d() {
        let english = (-45).days().humanize();
        assert_eq!("a month ago", english);
    }

    #[test]
    fn plus_46d() {
        let english = 46.days().humanize();
        assert_eq!("in 2 months", english);
    }

    #[test]
    fn minus_46d() {
        let english = (-46).days().humanize();
        assert_eq!("2 months ago", english);
    }

    #[test]
    fn plus_24w() {
        let english = 24.weeks().humanize();
        assert_eq!("in 5 months", english);
    }

    #[test]
    fn minus_24w() {
        let english = (-24).weeks().humanize();
        assert_eq!("5 months ago", english);
    }

    #[test]
    fn plus_26w() {
        let english = 26.weeks().humanize();
        assert_eq!("in 6 months", english);
    }

    #[test]
    fn minus_26w() {
        let english = (-26).weeks().humanize();
        assert_eq!("6 months ago", english);
    }

    #[test]
    fn plus_50w() {
        let english = 50.weeks().humanize();
        assert_eq!("in a year", english);
    }

    #[test]
    fn minus_50w() {
        let english = (-50).weeks().humanize();
        assert_eq!("a year ago", english);
    }

    #[test]
    fn plus_100w() {
        let english = 100.weeks().humanize();
        assert_eq!("in 2 years", english);
    }

    #[test]
    fn minus_100w() {
        let english = (-100).weeks().humanize();
        assert_eq!("2 years ago", english);
    }

    #[test]
    fn plus_120w() {
        let english = 120.weeks().humanize();
        assert_eq!("in 2 years", english);
    }

    #[test]
    fn minus_120w() {
        let english = (-120).weeks().humanize();
        assert_eq!("2 years ago", english);
    }

    #[test]
    fn plus_200w() {
        let english = 200.weeks().humanize();
        assert_eq!("in 3 years", english);
    }

    #[test]
    fn minus_200w() {
        let english = (-200).weeks().humanize();
        assert_eq!("3 years ago", english);
    }
}

#[cfg(test)]
mod utc {
    use jiffy::Humanize;

    #[test]
    fn now() {
        let english = jiff::Zoned::now().humanize();
        assert_eq!("now", english);
    }
}

#[cfg(test)]
mod local {
    use jiff::ToSpan;
    use jiffy::Humanize;

    #[test]
    fn now() {
        let english = jiff::Zoned::now().humanize();
        assert_eq!("now", english);
    }

    #[test]
    fn minus_35d() -> anyhow::Result<()> {
        let past = jiff::Zoned::now().checked_sub(35.days())?;
        let english = past.humanize();
        assert_eq!("a month ago", english);
        Ok(())
    }

    #[test]
    fn plus_35d() -> anyhow::Result<()> {
        let future = jiff::Zoned::now().checked_add(35.days())?;
        let english = future.humanize();
        assert_eq!("in a month", english);
        Ok(())
    }
}
