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
}
