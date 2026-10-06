pub mod binary;
pub mod comparer;
pub mod converter;
pub mod errors;
pub mod model;
pub mod parser;
pub mod text;
pub mod writer;

pub use errors::InternalErrors;
pub use model::{Formats, Operation};

#[cfg(test)]
mod tests {
    use crate::model::OperationType;

    use super::*;
    use std::io::Cursor;

    fn create_sample_data() -> Vec<Operation> {
        vec![
            Operation {
                date: "2026-10-05".to_string(),
                category: "Продукты".to_string(),
                operation_type: model::OperationType::Expense,
                summ: 1500,
            },
            Operation {
                date: "2026-10-06".to_string(),
                category: "Зарплата".to_string(),
                operation_type: model::OperationType::Income,
                summ: 50000,
            },
        ]
    }

    #[test]
    fn test_parse_csv_success() -> Result<(), InternalErrors> {
        let csv_data = "date,category,operation_type,summ\n\
                        2026-10-05,Продукты,Расход,1500\n\
                        2026-10-06,Зарплата,Доход,50000\n";

        let reader = Cursor::new(csv_data.as_bytes());

        let result = Formats::Csv.parse(reader)?;

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].category, "Продукты");
        assert_eq!(result[1].summ, 50000);

        Ok(())
    }

    #[test]
    fn test_parse_text_success() -> Result<(), Box<dyn std::error::Error>> {
        let text_data = "2026-10-05;Продукты;Расход;1500\n\
                         2026-10-06;Зарплата;Доход;50000\n";

        let reader = Cursor::new(text_data);
        let result = Formats::Text.parse(reader)?;

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].date, "2026-10-05");
        assert_eq!(result[1].operation_type, OperationType::Income);
        Ok(())
    }

    #[test]
    fn test_parse_bin_success() -> Result<(), Box<dyn std::error::Error>> {
        let format = Formats::Bin;
        let original_data = create_sample_data();
        let mut buffer = Cursor::new(Vec::new());

        format.write(&mut buffer, &original_data)?;

        buffer.set_position(0);

        let parsed_data = format.parse(&mut buffer)?;

        assert_eq!(parsed_data, original_data,);
        Ok(())
    }

    #[test]
    fn test_parse_csv_error() {
        let bad_csv = "date,category,operation_type,summ\n2026-10-05,Продукты,Расход,НЕ_ЧИСЛО\n";
        let reader = Cursor::new(bad_csv);
        let result = Formats::Csv.parse(reader);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_text_error() {
        let bad_text = "Случайный текст без точек с запятой";
        let reader = Cursor::new(bad_text.as_bytes());

        let result = Formats::Text.parse(reader);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bin_error_signature() {
        let bad_bin_bytes = vec![b'W', b'R', b'O', b'G', 0, 0, 0, 0];
        let reader = Cursor::new(bad_bin_bytes);
        let result = Formats::Bin.parse(reader);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bin_truncated_error() {
        let mut truncated_bytes = vec![b'B', b'I', b'N', b'F'];
        truncated_bytes.extend_from_slice(&1u32.to_le_bytes());
        truncated_bytes.extend_from_slice(&10u32.to_le_bytes());
        truncated_bytes.extend_from_slice(b"2026-10-06");

        let reader = Cursor::new(truncated_bytes);
        let result = Formats::Bin.parse(reader);

        assert!(result.is_err(),);

        if let Err(InternalErrors::InvalidInput(err_msg)) = result {
            assert!(
                err_msg.contains("ожидалось поле 'длина категории'")
                    || err_msg.contains("записи #0")
            );
        } else {
            panic!("Ожидалась ошибка типа InternalErrors::InvalidInput");
        }
    }

    #[test]
    fn test_format_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let text_source = "2026-10-05;Продукты;Расход;1500\n";
        let reader = Cursor::new(text_source);
        let mut writer = Cursor::new(Vec::new());

        let count = Formats::convert(&Formats::Text, &Formats::Csv, reader, &mut writer)?;
        assert_eq!(count, 1);

        writer.set_position(0);
        let csv_string = String::from_utf8(writer.into_inner())?;
        assert!(csv_string.starts_with("date,category"));
        assert!(csv_string.contains("2026-10-05,Продукты,Расход,1500"));
        Ok(())
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn test_format_conversion_with_bin() -> Result<(), Box<dyn std::error::Error>> {
        let csv_source = "date,category,operation_type,summ\n2026-10-06,Продукты,Расход,1500\n";
        let reader = Cursor::new(csv_source.as_bytes());
        let mut bin_writer = Cursor::new(Vec::new());

        let count = Formats::convert(&Formats::Csv, &Formats::Bin, reader, &mut bin_writer)?;
        assert_eq!(count, 1, "Должна быть сконвертирована ровно 1 запись");

        bin_writer.set_position(0);
        let parsed_from_bin = Formats::Bin.parse(&mut bin_writer)?;

        assert_eq!(parsed_from_bin.len(), 1);
        assert_eq!(parsed_from_bin[0].category, "Продукты");
        assert_eq!(parsed_from_bin[0].operation_type, OperationType::Expense);
        assert_eq!(parsed_from_bin[0].summ, 1500);
        Ok(())
    }

    #[test]
    fn test_comparer_different_lengths() -> Result<(), Box<dyn std::error::Error>> {
        let list1 = create_sample_data();
        let mut list2 = create_sample_data();
        list2.pop();

        let mut buffer1 = Cursor::new(Vec::new());
        let mut buffer2 = Cursor::new(Vec::new());

        Formats::Bin.write(&mut buffer1, &list1)?;
        Formats::Bin.write(&mut buffer2, &list2)?;

        buffer1.set_position(0);
        buffer2.set_position(0);

        let report =
            Formats::compare_sources(&mut buffer1, &Formats::Bin, &mut buffer2, &Formats::Bin, 5)?;

        assert!(
            !report.is_empty(),
            "Отчет не должен быть пустым при разной длине списков"
        );

        let has_length_error = report
            .iter()
            .any(|line| line.contains("отличаются по количеству транзакций"));
        assert!(
            has_length_error,
            "Отчет должен явно содержать сообщение о разнице в количестве транзакций"
        );
        Ok(())
    }

    #[test]
    fn test_comparer_identical() -> Result<(), Box<dyn std::error::Error>> {
        let list1 = create_sample_data();
        let list2 = create_sample_data();

        let mut buffer1 = Cursor::new(Vec::new());
        let mut buffer2 = Cursor::new(Vec::new());

        Formats::Bin.write(&mut buffer1, &list1)?;
        Formats::Bin.write(&mut buffer2, &list2)?;

        buffer1.set_position(0);
        buffer2.set_position(0);

        let report =
            Formats::compare_sources(&mut buffer1, &Formats::Bin, &mut buffer2, &Formats::Bin, 5)?;

        assert!(report.is_empty());
        Ok(())
    }

    #[test]
    fn test_comparer_different() -> Result<(), Box<dyn std::error::Error>> {
        let list1 = create_sample_data();
        let mut list2 = create_sample_data();

        list2[1].summ = 9999;

        let mut buffer1 = Cursor::new(Vec::new());
        let mut buffer2 = Cursor::new(Vec::new());

        Formats::Bin.write(&mut buffer1, &list1)?;
        Formats::Bin.write(&mut buffer2, &list2)?;

        buffer1.set_position(0);
        buffer2.set_position(0);

        let report =
            Formats::compare_sources(&mut buffer1, &Formats::Bin, &mut buffer2, &Formats::Bin, 5)?;

        assert!(!report.is_empty());
        let has_error_text = report
            .iter()
            .any(|line| line.contains("amount: 50000 != 9999"));
        assert!(
            has_error_text,
            "Отчет должен содержать информацию о разнице сумм"
        );
        Ok(())
    }

    #[test]
    fn test_parse_invalid_operation_type_error() {
        let bad_data = "2026-10-06;Продукты;Инвестиции;1500\n";
        let reader = Cursor::new(bad_data.as_bytes());

        let result = Formats::Text.parse(reader);

        assert!(result.is_err());

        if let Err(InternalErrors::InvalidInput(err_msg)) = result {
            assert!(err_msg.contains("Неизвестный тип операции"));
        } else {
            panic!("Ожидалась ошибка валидации типа операции");
        }
    }
}
