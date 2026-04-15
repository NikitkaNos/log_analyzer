#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_with_two_arguments() {
        // Симулируем аргументы: ["program_name", "log.txt", "ERROR"]
        let args = vec![
            "log_analyzer".to_string(),
            "log.txt".to_string(),
            "ERROR".to_string(),
        ];

        let result = Args::parse_from(args);

        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.file_path, std::path::PathBuf::from("log.txt"));
        assert_eq!(args.pattern, "ERROR");
    }
}
use std::path::PathBuf;
#[derive(Debug, CLone, PartialEq)]
pub struct Args {
    pub file_path: PathBuf,
    pub pattern: String,
}
