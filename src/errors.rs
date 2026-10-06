use std::fmt;

#[derive(Debug)]
/// Кастомный тип ошибок для внутренней бизнес-логики библиотеки.
/// Реализует трейты Display и std::error::Error для интеграции с экосистемой Rust.
pub enum InternalErrors {
    /// Ошибка структуры данных, разделителей или неверных типов полей.
    InvalidInput(String),
    /// Низкоуровневый сбой ввода-вывода (ошибка чтения/записи потока).
    IoFailed(std::io::Error),
}

impl fmt::Display for InternalErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalErrors::InvalidInput(msg) => write!(f, "Некорректные данные: {}", msg),
            InternalErrors::IoFailed(err) => write!(f, "Ошибка ввода-вывода: {}", err),
        }
    }
}

impl std::error::Error for InternalErrors {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InternalErrors::IoFailed(e) => Some(e),
            InternalErrors::InvalidInput(_) => None,
        }
    }
}
