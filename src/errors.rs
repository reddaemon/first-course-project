use std::fmt;

#[derive(Debug)]
/// Кастомный тип ошибок для внутренней бизнес-логики библиотеки.
/// Реализует трейты Display и std::error::Error для интеграции с экосистемой Rust.
pub enum InternalErrors {
    InvalidInput(String),
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
