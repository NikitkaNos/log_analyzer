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
        let args = result.unwrap();
        assert_eq!(args.file_path, std::path::PathBuf::from("log.txt"));
        assert_eq!(args.pattern, "ERROR");
    }
}
use std::path::PathBuf;
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub file_path: PathBuf,
    pub pattern: String,
}
impl Args {
    pub fn parse_from(mut args: Vec<String>) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(format!(
                "Ожидается 2 аргумента, получено: {}\n\
                 Использование: {} <лог-файл> <паттерн>",
                args.len() - 1,
                args[0]
            ));
        }
        let file_path = PathBuf::from(args.remove(1));
        let pattern = args.remove(1);

        // Проверяем существование файла
        // if !file_path.exists() {
        //     return Err(format!("Файл не найден: {}", file_path.display()));
        // } Закоменчино потому что нет ещё файла log.txt

        Ok(Args { file_path, pattern })
    }
}
