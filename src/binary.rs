use std::io::{self, Read};

use crate::{
    InternalErrors,
    model::{self, Operation},
};

impl model::Formats {
    /// парсер бинарного формата
    pub fn parse_binary<R: Read>(&self, mut reader: R) -> Result<Vec<Operation>, InternalErrors> {
        let mut header_bytes = [0u8; 4];

        reader.read_exact(&mut header_bytes).map_err(|e| {
            InternalErrors::InvalidInput(format!("Не удалось прочитать заголовок файла: {}", e))
        })?;

        if header_bytes == [b'B', b'I', b'N', b'F'] {
            let total_records = self.read_u32(reader.by_ref()).map_err(|e| {
                InternalErrors::InvalidInput(format!(
                    "не удалось прочитать количество записей: {}",
                    e
                ))
            })? as usize;

            let mut results = Vec::with_capacity(total_records);
            for _ in 0..total_records {
                let date_len = self.read_u32(reader.by_ref()).map_err(|e| {
                    InternalErrors::InvalidInput(format!("Ошибка чтения длины даты: {}", e))
                })? as usize;

                let date = self.read_string(&mut reader, date_len).map_err(|e| {
                    eprintln!("Файл оборвался при чтении NAME длиной {}", date_len);
                    InternalErrors::IoFailed(e)
                })?;

                let category_len = match self.read_u32(&mut reader) {
                    Ok(len) => len as usize,
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(InternalErrors::IoFailed(e)),
                };
                let category = self.read_string(&mut reader, category_len).map_err(|e| {
                    eprintln!("Файл оборвался при чтении CATEGORY длиной {}", category_len);
                    InternalErrors::IoFailed(e)
                })?;

                let operation_type_len = match self.read_u32(&mut reader) {
                    Ok(len) => len as usize,
                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                    Err(e) => return Err(InternalErrors::IoFailed(e)),
                };
                let operation_type =
                    self.read_string(&mut reader, operation_type_len)
                        .map_err(|e| {
                            eprintln!(
                                "Файл оборвался при чтении OPERATION_TYPE длиной {}",
                                operation_type_len
                            );
                            InternalErrors::IoFailed(e)
                        })?;
                let summ = self.read_u32(&mut reader).map_err(|e| {
                    eprintln!("Файл оборвался при чтении SUMM для объекта '{}'", date);
                    InternalErrors::IoFailed(e)
                })?;
                results.push(Operation {
                    date,
                    category,
                    operation_type,
                    summ,
                });
            }
            Ok(results)
        } else if header_bytes[0].is_ascii_digit() || header_bytes == [b'd', b'a', b't', b'a'] {
            let full_stream = header_bytes.as_slice().chain(reader);

            self.parse_text(full_stream)
        } else {
            Err(InternalErrors::InvalidInput(
                "Неверная сигнатура бинарного файла".to_string(),
            ))
        }
    }

    fn read_u32<R: Read>(&self, mut reader: R) -> io::Result<u32> {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn read_string<R: Read>(&self, mut reader: R, len: usize) -> io::Result<String> {
        let mut bytes = vec![0u8; len];
        reader.read_exact(&mut bytes)?;

        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
