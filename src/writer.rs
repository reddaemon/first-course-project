use std::io::{self, Write};

use crate::model::{self, Operation};

impl model::Formats {
    /// Записывает коллекцию транзакций в абстрактный приёмник.
    /// Упаковывает данные в соответствии со спецификацией выбранного формата.
    pub fn write<W: Write>(&self, mut writer: W, data_list: &[Operation]) -> io::Result<()> {
        match self {
            model::Formats::Text => {
                for item in data_list {
                    writeln!(
                        writer,
                        "{};{};{};{}",
                        item.date, item.category, item.operation_type, item.summ
                    )?;
                }
            }
            model::Formats::Csv => {
                writeln!(writer, "date,category,kind,amount")?;
                for item in data_list {
                    writeln!(
                        writer,
                        "{},{},{},{}",
                        item.date, item.category, item.operation_type, item.summ
                    )?;
                }
            }
            model::Formats::Bin => {
                writer.write_all(&[b'B', b'I', b'N', b'F'])?;

                let total_records = data_list.len() as u32;
                self.write_u32(&mut writer, total_records)?;

                for item in data_list {
                    let date_bytes = item.date.as_bytes();
                    let date_len = date_bytes.len() as u32;
                    self.write_u32(&mut writer, date_len)?;
                    self.write_string(&mut writer, &item.date)?;

                    let category_bytes = item.category.as_bytes();
                    let category_len = category_bytes.len() as u32;
                    self.write_u32(&mut writer, category_len)?;
                    self.write_string(&mut writer, &item.category)?;

                    let operation_type_bytes = item.operation_type.as_bytes();
                    let operation_type_len = operation_type_bytes.len() as u32;
                    self.write_u32(&mut writer, operation_type_len)?;
                    self.write_string(&mut writer, &item.operation_type)?;

                    self.write_u32(&mut writer, item.summ)?;
                }
            }
        }
        writer.flush()?;
        Ok(())
    }

    fn write_u32<W: Write>(&self, writer: &mut W, value: u32) -> io::Result<()> {
        writer.write_all(&value.to_le_bytes())
    }

    fn write_string<W: Write>(&self, writer: &mut W, text: &str) -> io::Result<()> {
        writer.write_all(text.as_bytes())
    }
}
