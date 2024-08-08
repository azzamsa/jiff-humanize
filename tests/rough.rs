#[cfg(test)]
mod duration {
    use jiff::ToSpan;
    use jiffy::HumanTime;

    #[test]
    fn now() {
        let ht = HumanTime::now();
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn plus_1s() {
        let ht = HumanTime::from(1.seconds());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn minus_1s() {
        let ht = HumanTime::from((-1).seconds());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn plus_5s() {
        let ht = HumanTime::from(5.seconds());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn minus_5s() {
        let ht = HumanTime::from((-5).seconds());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn plus_10s() {
        let ht = HumanTime::from(10.seconds());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn minus_10s() {
        let ht = HumanTime::from((-10).seconds());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn plus_15s() {
        let ht = HumanTime::from(15.seconds());
        let english = format!("{}", ht);
        assert_eq!("in 15 seconds", english);
    }

    #[test]
    fn minus_15s() {
        let ht = HumanTime::from((-15).seconds());
        let english = format!("{}", ht);
        assert_eq!("15 seconds ago", english);
    }

    #[test]
    fn plus_95s() {
        let ht = HumanTime::from(95.seconds());
        let english = format!("{}", ht);
        assert_eq!("in 2 minutes", english);
    }

    #[test]
    fn minus_95s() {
        let ht = HumanTime::from((-95).seconds());
        let english = format!("{}", ht);
        assert_eq!("2 minutes ago", english);
    }

    #[test]
    fn plus_125s() {
        let ht = HumanTime::from(125.seconds());
        let english = format!("{}", ht);
        assert_eq!("in 2 minutes", english);
    }

    #[test]
    fn minus_125s() {
        let ht = HumanTime::from((-125).seconds());
        let english = format!("{}", ht);
        assert_eq!("2 minutes ago", english);
    }

    #[test]
    fn plus_31m() {
        let ht = HumanTime::from(31.minutes());
        let english = format!("{}", ht);
        assert_eq!("in 31 minutes", english);
    }

    #[test]
    fn minus_31m() {
        let ht = HumanTime::from((-31).minutes());
        let english = format!("{}", ht);
        assert_eq!("31 minutes ago", english);
    }

    #[test]
    fn plus_45m() {
        let ht = HumanTime::from(45.minutes());
        let english = format!("{}", ht);
        assert_eq!("in 45 minutes", english);
    }

    #[test]
    fn minus_45m() {
        let ht = HumanTime::from((-45).minutes());
        let english = format!("{}", ht);
        assert_eq!("45 minutes ago", english);
    }

    #[test]
    fn plus_46m() {
        let ht = HumanTime::from(46.minutes());
        let english = format!("{}", ht);
        assert_eq!("in an hour", english);
    }

    #[test]
    fn minus_46m() {
        let ht = HumanTime::from((-46).minutes());
        let english = format!("{}", ht);
        assert_eq!("an hour ago", english);
    }

    #[test]
    fn plus_1h() {
        let ht = HumanTime::from(1.hours());
        let english = format!("{}", ht);
        assert_eq!("in an hour", english);
    }

    #[test]
    fn minus_1h() {
        let ht = HumanTime::from((-1).hours());
        let english = format!("{}", ht);
        assert_eq!("an hour ago", english);
    }

    #[test]
    fn plus_12h() {
        let ht = HumanTime::from(12.hours());
        let english = format!("{}", ht);
        assert_eq!("in 12 hours", english);
    }

    #[test]
    fn minus_12h() {
        let ht = HumanTime::from((-12).hours());
        let english = format!("{}", ht);
        assert_eq!("12 hours ago", english);
    }

    #[test]
    fn plus_23h() {
        let ht = HumanTime::from(23.hours());
        let english = format!("{}", ht);
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_23h() {
        let ht = HumanTime::from((-23).hours());
        let english = format!("{}", ht);
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_26h() {
        let ht = HumanTime::from(26.hours());
        let english = format!("{}", ht);
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_26h() {
        let ht = HumanTime::from((-26).hours());
        let english = format!("{}", ht);
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_1d() {
        let ht = HumanTime::from(1.days());
        let english = format!("{}", ht);
        assert_eq!("in a day", english);
    }

    #[test]
    fn minus_1d() {
        let ht = HumanTime::from((-1).days());
        let english = format!("{}", ht);
        assert_eq!("a day ago", english);
    }

    #[test]
    fn plus_2d() {
        let ht = HumanTime::from(2.days());
        let english = format!("{}", ht);
        assert_eq!("in 2 days", english);
    }

    #[test]
    fn minus_2d() {
        let ht = HumanTime::from((-2).days());
        let english = format!("{}", ht);
        assert_eq!("2 days ago", english);
    }

    #[test]
    fn plus_6d_13h() -> anyhow::Result<()> {
        let ht = HumanTime::from(6.days().checked_add(13.hours())?);
        let english = format!("{}", ht);
        assert_eq!("in a week", english);
        Ok(())
    }

    #[test]
    fn minus_6d_13h() -> anyhow::Result<()> {
        let ht = HumanTime::from((-6).days().checked_add((-13).hours())?);
        let english = format!("{}", ht);
        assert_eq!("a week ago", english);
        Ok(())
    }

    #[test]
    fn plus_7d() {
        let ht = HumanTime::from(7.days());
        let english = format!("{}", ht);
        assert_eq!("in a week", english);
    }

    #[test]
    fn minus_7d() {
        let ht = HumanTime::from((-7).days());
        let english = format!("{}", ht);
        assert_eq!("a week ago", english);
    }

    #[test]
    fn plus_10d() {
        let ht = HumanTime::from(10.days());
        let english = format!("{}", ht);
        assert_eq!("in a week", english);
    }

    #[test]
    fn minus_10d() {
        let ht = HumanTime::from((-10).days());
        let english = format!("{}", ht);
        assert_eq!("a week ago", english);
    }

    #[test]
    fn plus_11d() {
        let ht = HumanTime::from(11.days());
        let english = format!("{}", ht);
        assert_eq!("in 2 weeks", english);
    }

    #[test]
    fn minus_11d() {
        let ht = HumanTime::from((-11).days());
        let english = format!("{}", ht);
        assert_eq!("2 weeks ago", english);
    }

    #[test]
    fn plus_4w() {
        let ht = HumanTime::from(4.weeks());
        let english = format!("{}", ht);
        assert_eq!("in 4 weeks", english);
    }

    #[test]
    fn minus_4w() {
        let ht = HumanTime::from((-4).weeks());
        let english = format!("{}", ht);
        assert_eq!("4 weeks ago", english);
    }

    #[test]
    fn plus_30d() {
        let ht = HumanTime::from(30.days());
        let english = format!("{}", ht);
        assert_eq!("in a month", english);
    }

    #[test]
    fn minus_30d() {
        let ht = HumanTime::from((-30).days());
        let english = format!("{}", ht);
        assert_eq!("a month ago", english);
    }

    #[test]
    fn plus_45d() {
        let ht = HumanTime::from(45.days());
        let english = format!("{}", ht);
        assert_eq!("in a month", english);
    }

    #[test]
    fn minus_45d() {
        let ht = HumanTime::from((-45).days());
        let english = format!("{}", ht);
        assert_eq!("a month ago", english);
    }

    #[test]
    fn plus_46d() {
        let ht = HumanTime::from(46.days());
        let english = format!("{}", ht);
        assert_eq!("in 2 months", english);
    }

    #[test]
    fn minus_46d() {
        let ht = HumanTime::from((-46).days());
        let english = format!("{}", ht);
        assert_eq!("2 months ago", english);
    }

    #[test]
    fn plus_24w() {
        let ht = HumanTime::from(24.weeks());
        let english = format!("{}", ht);
        assert_eq!("in 5 months", english);
    }

    #[test]
    fn minus_24w() {
        let ht = HumanTime::from((-24).weeks());
        let english = format!("{}", ht);
        assert_eq!("5 months ago", english);
    }

    #[test]
    fn plus_26w() {
        let ht = HumanTime::from(26.weeks());
        let english = format!("{}", ht);
        assert_eq!("in 6 months", english);
    }

    #[test]
    fn minus_26w() {
        let ht = HumanTime::from((-26).weeks());
        let english = format!("{}", ht);
        assert_eq!("6 months ago", english);
    }

    #[test]
    fn plus_50w() {
        let ht = HumanTime::from(50.weeks());
        let english = format!("{}", ht);
        assert_eq!("in a year", english);
    }

    #[test]
    fn minus_50w() {
        let ht = HumanTime::from((-50).weeks());
        let english = format!("{}", ht);
        assert_eq!("a year ago", english);
    }

    #[test]
    fn plus_100w() {
        let ht = HumanTime::from(100.weeks());
        let english = format!("{}", ht);
        assert_eq!("in 2 years", english);
    }

    #[test]
    fn minus_100w() {
        let ht = HumanTime::from((-100).weeks());
        let english = format!("{}", ht);
        assert_eq!("2 years ago", english);
    }

    #[test]
    fn plus_120w() {
        let ht = HumanTime::from(120.weeks());
        let english = format!("{}", ht);
        assert_eq!("in 2 years", english);
    }

    #[test]
    fn minus_120w() {
        let ht = HumanTime::from((-120).weeks());
        let english = format!("{}", ht);
        assert_eq!("2 years ago", english);
    }

    #[test]
    fn plus_200w() {
        let ht = HumanTime::from(200.weeks());
        let english = format!("{}", ht);
        assert_eq!("in 3 years", english);
    }

    #[test]
    fn minus_200w() {
        let ht = HumanTime::from((-200).weeks());
        let english = format!("{}", ht);
        assert_eq!("3 years ago", english);
    }
}

#[cfg(test)]
mod utc {
    use jiffy::HumanTime;

    #[test]
    fn now() {
        let ht = HumanTime::from(jiff::Zoned::now());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }
}

#[cfg(test)]
mod local {
    use jiff::ToSpan;
    use jiffy::HumanTime;

    #[test]
    fn now() {
        let ht = HumanTime::from(jiff::Zoned::now());
        let english = format!("{}", ht);
        assert_eq!("now", english);
    }

    #[test]
    fn minus_35d() -> anyhow::Result<()> {
        let past = jiff::Zoned::now().checked_sub(35.days())?;
        let ht = HumanTime::from(past);
        let english = format!("{}", ht);
        assert_eq!("a month ago", english);
        Ok(())
    }

    #[test]
    fn plus_35d() -> anyhow::Result<()> {
        let future = jiff::Zoned::now().checked_add(35.days())?;
        let ht = HumanTime::from(future);
        let english = format!("{}", ht);
        assert_eq!("in a month", english);
        Ok(())
    }
}
