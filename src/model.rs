/// Представляет единичную запись финансовой транзакции в системе.
/// Хранит текстовые данные о дате, категории, типе операции и сумму.
#[derive(Debug, Clone, PartialEq)]
pub struct Operation {
    pub date: String,
    pub category: String,
    pub operation_type: OperationType,
    pub summ: u32,
}

/// Тип финансовой операции.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Income,
    Expense,
}

/// Поддерживаемые форматы файлов для импорта, экспорта и конвертации данных.
pub enum Formats {
    Csv,
    Text,
    Bin,
}

impl Operation {
    /// Построчно сравнивает текущую транзакцию с другой записью.
    /// Возвращает текстовое описание полей, в которых обнаружены расхождения.
    pub fn find_differences(&self, other: &Operation) -> Vec<String> {
        let mut diffs = Vec::new();

        if self.date != other.date {
            diffs.push(format!("date: '{}' != '{}'", self.date, other.date));
        }
        if self.category != other.category {
            diffs.push(format!(
                "category: '{}' != '{}'",
                self.category, other.category
            ));
        }
        if self.operation_type != other.operation_type {
            diffs.push(format!(
                "operation_type: '{}' != '{}'",
                self.operation_type, other.operation_type
            ));
        }
        if self.summ != other.summ {
            diffs.push(format!("amount: {} != {}", self.summ, other.summ));
        }

        diffs
    }
}
