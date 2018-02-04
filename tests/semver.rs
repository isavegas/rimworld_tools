extern crate rimworld_mod_manager;

mod semver_test {
    use rimworld_mod_manager::semver::Version;
    #[test]
    fn eq_test() {
        let a = Version::new(0, 0, 1);
        let b = Version::from_str("0.0.1").unwrap();
        assert!(a == b, "Versions should implement Eq");
    }

    #[test]
    fn revision_ord_test() {
        let a = Version::new(0, 0, 1);
        let b = Version::from_str("0.0.2").unwrap();
        assert!(a < b, "Versions should implement Ord. Revision");
    }

    #[test]
    fn minor_ord_test() {
        let a = Version::new(0, 0, 1);
        let b = Version::from_str("0.2.1").unwrap();
        assert!(a < b, "Versions should implement Ord. Minor");
    }

    #[test]
    fn major_ord_test() {
        let a = Version::new(0, 2, 1);
        let b = Version::from_str("12.2.1").unwrap();
        assert!(a < b, "Versions should implement Ord. Major");
    }

    #[test]
    fn major_over_minor_ord_test() {
        let a = Version::new(0, 2, 1);
        let b = Version::from_str("0.1.2").unwrap();
        assert!(a > b, "Major should take priority over minor");
    }

    #[test]
    fn minor_over_revision_ord_test() {
        let a = Version::new(2, 1, 0);
        let b = Version::from_str("1.2.0").unwrap();
        assert!(a > b, "Minor should take priority over revision");
    }

    #[test]
    fn parse_empty() {
        assert!(Version::from_str("").is_err());
    }

    #[test]
    fn parse_alpha() {
        assert!(Version::from_str("a.0.3").is_err());
    }

    #[test]
    fn parse_symbols() {
        assert!(Version::from_str("@2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("#2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("+2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("-2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("^2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("%2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("(2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str(")2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("$2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("!2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("~2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("`2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("/2.0.3").is_err(), "Symbols should not be allowed");
        assert!(Version::from_str("\\2.0.3").is_err(), "Symbols should not be allowed");
    }
    
    #[test]
    fn parse_unicode() {

    }

    #[test]
    fn parse_emoji() {
        assert!(Version::from_str("🤷2.0.3").is_err(), "Emoji should not be allowed");
        assert!(Version::from_str("😃2.0.3").is_err(), "Emoji should not be allowed");
        assert!(Version::from_str("🤔2.0.3").is_err(), "Emoji should not be allowed");
    }

    #[test]
    fn parse_missing_major() {
        assert!(Version::from_str(".0.3").is_err(), "Should have a major");
    }

    #[test]
    fn parse_missing_minor() {
        assert!(Version::from_str("0..3").is_err(), "Should have a minor");
    }

    #[test]
    fn parse_missing_revision() {
        assert!(Version::from_str("0.0.").is_err(), "Should have a revision");
    }

    #[test]
    fn parse_too_many_parts() {
        assert!(Version::from_str("0.0.0.0").is_err(), "Should have no more than 3 parts");
    }

    #[test]
    fn parse_too_few_parts() {
        assert!(Version::from_str("0.0").is_err(), "Should have no less than 3 parts");
    }

    #[test]
    fn parse_trailing_whitespace() {
        assert!(Version::from_str("0.0.0 ").is_err(), "Trailing spaces should be invalid");
        assert!(Version::from_str("0.0.0\n").is_err(), "Trailing newlines should be invalid");
        assert!(Version::from_str("0.0.0\t").is_err(), "Trailing tabs should be invalid");
    }

    #[test]
    fn parse_leading_whitespace() {
        assert!(Version::from_str(" 0.0.0").is_err(), "Leading spaces should be invalid");
        assert!(Version::from_str("\n0.0.0").is_err(), "Leading newlines should be invalid");
        assert!(Version::from_str("\t0.0.0").is_err(), "Leading tabs should be invalid");
    }

    #[test]
    fn to_string() {
        assert!(Version::new(0, 1, 0).to_string() == String::from("0.1.0"), "to_string should output valid semver")
    }
}