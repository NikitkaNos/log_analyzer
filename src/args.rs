#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_with_two_arguments() {
        let args = vec![
            "log_analyzer".to_string(),
            "log.txt".to_string(),
            "ERROR".to_string(),
        ];

        let result = Args::parse_from(args);

        assert!(result.is_ok());
    }
}
