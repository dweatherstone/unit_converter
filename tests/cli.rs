#[cfg(test)]
mod cli {

    use assert_cmd::Command;
    use clap::Parser;
    use predicates::{prelude::*, str::contains};

    use unitconvert::cli::{Cli, Commands};

    #[test]
    fn missing_argument_should_fail() {
        let mut cmd = Command::cargo_bin("unitconvert").unwrap();
        cmd.args(["convert", "--from", "m", "--to", "ft"]); // missing value
        cmd.assert()
            .failure()
            .stderr(contains("required arguments").and(contains("<VALUE>")));
    }

    #[test]
    fn valid_conversion_should_work() {
        let mut cmd = Command::cargo_bin("unitconvert").unwrap();
        cmd.args(["convert", "--from", "m", "--to", "ft", "2"]);
        cmd.assert()
            .success()
            .stdout(contains("2 m = 6.561679790026246 ft"));
    }

    #[test]
    fn parse_valid_args() {
        let cli = Cli::try_parse_from(["unitconvert", "convert", "--from", "m", "--to", "ft", "2"])
            .expect("Should parse");

        match cli.command {
            Commands::Convert(args) => {
                assert_eq!(args.from, "m");
                assert_eq!(args.to, "ft");
                assert_eq!(args.value, 2.0);
            }
            _ => panic!("Expected Convert subcommand"),
        }
    }

    #[test]
    fn parse_missing_value_fails() {
        let result = Cli::try_parse_from(["unitconvert", "--from", "m", "--to", "ft"]);
        assert!(result.is_err());
    }

    #[test]
    fn cli_expression_works() {
        let mut cmd = Command::cargo_bin("unitconvert").unwrap();
        cmd.args(["expression", "--expr", "10C -> F"])
            .assert()
            .success()
            .stdout(contains("10 °C = "));
    }
}
