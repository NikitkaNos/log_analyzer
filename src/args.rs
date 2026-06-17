//#[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_args_parse_with_two_arguments() {
//         let temp_file = tempfile::NamedTempFile::new().unwrap();
//         let path = temp_file.path().to_str().unwrap();

//         let args = vec![
//             "log_analyzer".to_string(),
//             path.to_string(),
//             "log.txt".to_string(),
//             "ERROR".to_string(),
//             "--quiet".to_string(),
//         ];

//         let result = Args::parse_from(args);

//         assert!(result.is_ok());
//         let args = result.unwrap();
//         assert!(args.quiet);
//         assert_eq!(args.file_path, std::path::PathBuf::from("log.txt"));
//         assert_eq!(args.pattern, "ERROR");
//     }
// }
use std::path::PathBuf;

//use crate::args;
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub file_path: PathBuf,
    pub pattern: String,
    pub quiet: bool,
}
impl Args {
    pub fn parse_from(mut args: Vec<String>) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(format!(
                "Ожидается 2 аргумента, получено: {}\n\
                 Использование: {} <лог-файл> <паттерн> [--quiet]",
                args.len() - 1,
                args[0]
            ));
        }
        let file_path = PathBuf::from(args.remove(1));
        let pattern = args.remove(1);

        let quiet = args.iter().any(|arg| arg == "--quiet" || arg == "-q");

        //Проверяем существование файла
        if !file_path.exists() {
            return Err(format!("Файл не найден: {}", file_path.display()));
        }

        Ok(Args {
            file_path,
            pattern,
            quiet,
        })
    }
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        Self::parse_from(args)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_with_two_arguments() {
        // ВАЖНО: Используем реальный существующий файл
        // Создаём временный файл для теста
        let _temp_file = std::fs::File::create("test_temp.log").unwrap();
        let path = "test_temp.log".to_string();

        let args = vec!["log_analyzer".to_string(), path, "ERROR".to_string()];

        let result = Args::parse_from(args);

        // Удаляем временный файл после теста
        std::fs::remove_file("test_temp.log").unwrap();

        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.file_path, std::path::PathBuf::from("test_temp.log"));
        assert_eq!(args.pattern, "ERROR");
        assert!(!args.quiet);
    }

    #[test]
    fn test_args_with_quiet_flag() {
        // Создаём временный файл
        let _temp_file = std::fs::File::create("test_temp_quiet.log").unwrap();
        let path = "test_temp_quiet.log".to_string();

        let args = vec![
            "log_analyzer".to_string(),
            path,
            "ERROR".to_string(),
            "--quiet".to_string(),
        ];

        let result = Args::parse_from(args);

        // Удаляем временный файл
        std::fs::remove_file("test_temp_quiet.log").unwrap();

        assert!(result.is_ok());
        let args = result.unwrap();
        assert!(args.quiet);
    }

    #[test]
    fn test_args_with_short_quiet_flag() {
        // Создаём временный файл
        let _temp_file = std::fs::File::create("test_temp_short.log").unwrap();
        let path = "test_temp_short.log".to_string();

        let args = vec![
            "log_analyzer".to_string(),
            path,
            "ERROR".to_string(),
            "-q".to_string(), // Короткий флаг
        ];

        let result = Args::parse_from(args);

        // Удаляем временный файл
        std::fs::remove_file("test_temp_short.log").unwrap();

        assert!(result.is_ok());
        let args = result.unwrap();
        assert!(args.quiet);
    }

    #[test]
    fn test_args_error_when_file_not_exists() {
        let args = vec![
            "log_analyzer".to_string(),
            "nonexistent_file.txt".to_string(),
            "ERROR".to_string(),
        ];

        let result = Args::parse_from(args);
        assert!(result.is_err());
        // Проверяем, что ошибка содержит правильное сообщение
        let err = result.unwrap_err();
        assert!(err.contains("Файл не найден"));
    }
}
