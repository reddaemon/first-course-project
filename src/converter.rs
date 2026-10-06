use std::io::{Read, Write};

use crate::{
    InternalErrors,
    model::{self},
};

impl model::Formats {
    /// Высокоуровневый метод для потоковой конвертации данных из одного формата в другой.
    /// Принимает абстрактные источники Read и приёмники Write.
    pub fn convert<R: Read, W: Write>(
        from_format: &model::Formats,
        to_format: &model::Formats,
        reader: R,
        writer: W,
    ) -> Result<usize, InternalErrors> {
        let data = from_format.parse(reader)?;

        let record_count = data.len();

        to_format.write(writer, &data).map_err(|e| {
            InternalErrors::InvalidInput(format!("write error due to convertation: {}", e))
        })?;

        Ok(record_count)
    }
}
