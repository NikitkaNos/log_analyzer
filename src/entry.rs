// src/entry.rs
use std::fmt;

/// Запись лога технологического журнала 1С
///
/// Хранит структурированную информацию из одной строки лога.
///
/// # Пример
/// ```
/// use log_analyzer::entry::LogEntry;
///
/// let entry = LogEntry::new(
///     "2025-04-13 10:15:23.456".to_string(),
///     "EXCEPTION".to_string(),
///     Some(1250),
///     "полная строка лога".to_string(),
/// );
///
/// assert_eq!(entry.datetime, "2025-04-13 10:15:23.456");
/// assert_eq!(entry.event_level, "EXCEPTION");
/// assert_eq!(entry.duration_ms, Some(1250));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    /// Дата и время события (из первых 23 символов строки)
    pub datetime: String,
    /// Уровень события: EXCEPTION, WARNING, INFO и т.д.
    pub event_level: String,
    /// Длительность операции в миллисекундах (если есть)
    pub duration_ms: Option<u64>,
    /// Полная исходная строка (для отладки и вывода)
    pub raw_line: String,
}

impl LogEntry {
    /// Создаёт новую запись лога
    pub fn new(
        datetime: String,
        event_level: String,
        duration_ms: Option<u64>,
        raw_line: String,
    ) -> Self {
        Self {
            datetime,
            event_level,
            duration_ms,
            raw_line,
        }
    }

    /// Проверяет, есть ли у записи длительность
    ///
    /// # Пример
    /// ```
    /// use log_analyzer::entry::LogEntry;
    ///
    /// let entry = LogEntry::new(
    ///     "2025-04-13".to_string(),
    ///     "EXCEPTION".to_string(),
    ///     Some(1250),
    ///     "".to_string(),
    /// );
    /// assert!(entry.has_duration());
    ///
    /// let entry = LogEntry::new(
    ///     "2025-04-13".to_string(),
    ///     "INFO".to_string(),
    ///     None,
    ///     "".to_string(),
    /// );
    /// assert!(!entry.has_duration());
    /// ```
    pub fn has_duration(&self) -> bool {
        self.duration_ms.is_some()
    }

    /// Возвращает длительность или 0, если её нет
    ///
    /// # Пример
    /// ```
    /// use log_analyzer::entry::LogEntry;
    ///
    /// let entry = LogEntry::new(
    ///     "2025-04-13".to_string(),
    ///     "EXCEPTION".to_string(),
    ///     Some(1250),
    ///     "".to_string(),
    /// );
    /// assert_eq!(entry.duration_or_zero(), 1250);
    ///
    /// let entry = LogEntry::new(
    ///     "2025-04-13".to_string(),
    ///     "INFO".to_string(),
    ///     None,
    ///     "".to_string(),
    /// );
    /// assert_eq!(entry.duration_or_zero(), 0);
    /// ```
    pub fn duration_or_zero(&self) -> u64 {
        self.duration_ms.unwrap_or(0)
    }

    /// Возвращает длительность как f64 (для вычислений)
    ///
    /// # Пример
    /// ```
    /// use log_analyzer::entry::LogEntry;
    ///
    /// let entry = LogEntry::new(
    ///     "2025-04-13".to_string(),
    ///     "EXCEPTION".to_string(),
    ///     Some(1250),
    ///     "".to_string(),
    /// );
    /// assert_eq!(entry.duration_as_f64(), 1250.0);
    ///
    /// let entry = LogEntry::new(
    ///     "2025-04-13".to_string(),
    ///     "INFO".to_string(),
    ///     None,
    ///     "".to_string(),
    /// );
    /// assert_eq!(entry.duration_as_f64(), 0.0);
    /// ```
    pub fn duration_as_f64(&self) -> f64 {
        self.duration_ms.map(|d| d as f64).unwrap_or(0.0)
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.duration_ms {
            Some(dur) => write!(
                f,
                "[{}] {} - {}ms: {}",
                self.datetime, self.event_level, dur, self.raw_line
            ),
            None => write!(
                f,
                "[{}] {}: {}",
                self.datetime, self.event_level, self.raw_line
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(
            "2025-04-13 10:15:23.456".to_string(),
            "EXCEPTION".to_string(),
            Some(1250),
            "полная строка лога".to_string(),
        );

        assert_eq!(entry.datetime, "2025-04-13 10:15:23.456");
        assert_eq!(entry.event_level, "EXCEPTION");
        assert_eq!(entry.duration_ms, Some(1250));
        assert_eq!(entry.raw_line, "полная строка лога");
    }

    #[test]
    fn test_has_duration() {
        let with_duration = LogEntry::new(
            "2025-04-13".to_string(),
            "EXCEPTION".to_string(),
            Some(1250),
            "".to_string(),
        );
        assert!(with_duration.has_duration());

        let without_duration = LogEntry::new(
            "2025-04-13".to_string(),
            "INFO".to_string(),
            None,
            "".to_string(),
        );
        assert!(!without_duration.has_duration());
    }

    #[test]
    fn test_duration_or_zero() {
        let with_duration = LogEntry::new(
            "2025-04-13".to_string(),
            "EXCEPTION".to_string(),
            Some(1250),
            "".to_string(),
        );
        assert_eq!(with_duration.duration_or_zero(), 1250);

        let without_duration = LogEntry::new(
            "2025-04-13".to_string(),
            "INFO".to_string(),
            None,
            "".to_string(),
        );
        assert_eq!(without_duration.duration_or_zero(), 0);
    }

    #[test]
    fn test_display() {
        let entry = LogEntry::new(
            "2025-04-13 10:15:23.456".to_string(),
            "EXCEPTION".to_string(),
            Some(1250),
            "ошибка".to_string(),
        );
        assert_eq!(
            format!("{}", entry),
            "[2025-04-13 10:15:23.456] EXCEPTION - 1250ms: ошибка"
        );
    }
}
