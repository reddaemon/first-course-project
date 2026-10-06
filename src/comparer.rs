use std::io::Read;

use crate::{
    InternalErrors,
    model::{self, Formats},
};

impl model::Formats {
    /// Симметрично сравнивает данные из двух независимых источников с любыми форматами.
    /// Возвращает список текстовых расхождений, не превышающий заданный лимит.
    pub fn compare_sources<R1: Read, R2: Read>(
        reader1: R1,
        format1: &Formats,
        reader2: R2,
        format2: &Formats,
        max_errors: usize,
    ) -> Result<Vec<String>, InternalErrors> {
        let list1 = format1.parse(reader1)?;
        let list2 = format2.parse(reader2)?;

        let mut report = Vec::new();
        let mut error_count = 0;

        if list1.len() != list2.len() {
            report.push(format!(
                "Файлы отличаются по количеству транзакций: {} != {}",
                list1.len(),
                list2.len()
            ));
        }

        let min_len = std::cmp::min(list1.len(), list2.len());

        for i in 0..min_len {
            if error_count >= max_errors {
                report.push(format!(
                    "... Достигнут лимит вывода ошибок (выведено {}) ...",
                    max_errors
                ));
                break;
            }

            let differences = list1[i].find_differences(&list2[i]);
            if !differences.is_empty() {
                error_count += 1;
                report.push(format!("Транзакции отличаются на позиции {}:", i));
                for diff in differences {
                    report.push(format!("  {}", diff));
                }
            }
        }

        Ok(report)
    }
}
