use std::io::{IsTerminal, Read, Write};

use crate::{
    InternalErrors,
    model::{self},
};

impl model::Formats {
    /// Высокоуровневый метод для потоковой конвертации данных из одного формата в другой.
    /// Принимает абстрактные источники Read и приёмники Write.
    /// Возвращает ошибку, если чтение или запись завершились сбоем.
    pub fn convert<R: Read, W: Write>(
        from_format: &model::Formats,
        to_format: &model::Formats,
        reader: R,
        writer: W,
    ) -> Result<usize, InternalErrors> {
        let data = from_format.parse(reader)?;

        let record_count = data.len();

        eprintln!("\n--- RESULT: ---");

        if matches!(to_format, model::Formats::Bin) && std::io::stdout().is_terminal() {
            return Err(InternalErrors::InvalidInput(
                "Консоль Windows не поддерживает вывод сырых бинарных данных на экран. Перенаправьте вывод в файл с помощью '>'".to_string()
            ));
        }

        to_format
            .write(writer, &data)
            .map_err(InternalErrors::IoFailed)?;

        Ok(record_count)
    }
}
