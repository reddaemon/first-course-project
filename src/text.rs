use std::io::{BufRead, BufReader, Read};

use crate::{
    InternalErrors,
    model::{self, Operation},
};

impl model::Formats {
    /// парсер текстовых форматов text и csv
    pub fn parse_text<R: Read>(&self, reader: R) -> Result<Vec<Operation>, InternalErrors> {
        let reader = BufReader::new(reader);
        let mut results = Vec::new();

        let delimiter = match self {
            model::Formats::Csv => ',',
            model::Formats::Text | model::Formats::Bin => ';',
        };

        let mut lines = reader.lines();

        if matches!(self, model::Formats::Csv) {
            lines.next();
        }

        for line in lines {
            let line = line.map_err(|e| {
                InternalErrors::InvalidInput(format!("Ошибка чтения строки из файла: {}", e))
            })?;
            let columns: Vec<&str> = line.trim().split(delimiter).collect();
            if columns.len() >= 2 {
                let date = columns[0].to_string();
                let category = columns[1].to_string();
                let operation_type = columns[2].to_string();

                let summ: u32 = columns[3].parse().unwrap_or(0);

                results.push(Operation {
                    date,
                    category,
                    operation_type,
                    summ,
                });
            } else {
                // Если разделитель не найден вообще (в векторе всего 1 элемент),
                // это гарантированный признак того, что передан не тот формат файла!
                return Err(InternalErrors::InvalidInput(format!(
                    "Строка не содержит ожидаемый разделитель '{}': '{}'",
                    delimiter,
                    line.trim()
                )));
            }
        }
        Ok(results)
    }
}
