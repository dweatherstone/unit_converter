mod cli {

    use assert_cmd::Command;
    use clap::Parser;
    use predicates::{prelude::*, str::contains};

    use unitconvert::cli::Cli;

    #[test]
    fn missing_argument_should_fail() {
        let mut cmd = Command::cargo_bin("unitconvert").unwrap();
        cmd.args(["--from", "m", "--to", "ft"]); // missing value
        cmd.assert()
            .failure()
            .stderr(contains("required arguments").and(contains("<VALUE>")));
    }

    #[test]
    fn valid_conversion_should_work() {
        let mut cmd = Command::cargo_bin("unitconvert").unwrap();
        cmd.args(["--from", "m", "--to", "ft", "2"]);
        cmd.assert().success().stdout(contains("2 m = 6.56168 ft"));
    }

    #[test]
    fn parse_valid_args() {
        let args = Cli::try_parse_from(["unitconvert", "--from", "m", "--to", "ft", "2"])
            .expect("Should parse");
        assert_eq!(args.from, "m");
        assert_eq!(args.to, "ft");
        assert_eq!(args.value, 2.0);
    }

    #[test]
    fn parse_missing_value_fails() {
        let result = Cli::try_parse_from(["unitconvert", "--from", "m", "--to", "ft"]);
        assert!(result.is_err());
    }
}
