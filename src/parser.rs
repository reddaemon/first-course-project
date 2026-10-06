use std::fmt::{Display, Formatter};
use std::io::Read;
use std::str::FromStr;

use crate::{
    InternalErrors,
    model::{self, Operation, OperationType},
};

impl model::Formats {
    /// Выполняет парсинг данных из абстрактного источника чтения в вектор транзакций.
    /// Автоматически определяет и обрабатывает выбранный формат данных.
    /// Возвращает ошибку при нарушении структуры или сбоях ввода-вывода.
    pub fn parse<R: Read>(&self, reader: R) -> Result<Vec<Operation>, InternalErrors> {
        match self {
            model::Formats::Csv | model::Formats::Text => self.parse_text(reader),
            model::Formats::Bin => self.parse_binary(reader),
        }
    }
}

impl FromStr for model::Formats {
    type Err = InternalErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(model::Formats::Text),
            "csv" => Ok(model::Formats::Csv),
            "binary" | "bin" => Ok(model::Formats::Bin),
            _ => Err(InternalErrors::InvalidInput(format!(
                "Неизвестный формат данных: '{}'. Поддерживаются: text, csv, binary",
                s
            ))),
        }
    }
}

impl FromStr for OperationType {
    type Err = InternalErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Приводим к нижнему регистру, чтобы разбор был регистронезависимым
        match s.trim().to_lowercase().as_str() {
            "income" | "доход" | "вход" => Ok(OperationType::Income),
            "expense" | "расход" | "выход" => Ok(OperationType::Expense),
            _ => Err(InternalErrors::InvalidInput(format!(
                "Неизвестный тип операции: '{}'. Ожидалось 'Доход' или 'Расход'",
                s
            ))),
        }
    }
}

impl Display for OperationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Income => write!(f, "Доход"),
            OperationType::Expense => write!(f, "Расход"),
        }
    }
}
