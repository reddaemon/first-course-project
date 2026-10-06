use std::io::{BufRead, BufReader, Read};

use crate::{
    InternalErrors,
    model::{self, Operation},
};

impl model::Formats {
    /// парсер текстовых форматов text и csv
    /// Возвращает ошибку, если в строке не 4 поля или неверный тип данных.
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

            if line.trim().is_empty() {
                continue;
            }
            let columns: Vec<&str> = line.trim().split(delimiter).collect();
            if columns.len() == 4 {
                let date = columns[0].to_string();
                let category = columns[1].to_string();
                let operation_type = columns[2].parse()?;

                let summ = columns[3].trim().parse::<u32>().map_err(|_| {
                    InternalErrors::InvalidInput(format!(
                        "Некорректное числовое значение суммы: '{}'",
                        columns[3]
                    ))
                })?;

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
                    "Неверная структура данных. Ожидалось 4 поля (разделенных '{}'), найдено {}: '{}'",
                    delimiter,
                    columns.len(),
                    line.trim()
                )));
            }
        }
        Ok(results)
    }
}
