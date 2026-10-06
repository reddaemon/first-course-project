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
    use super::*;
    use std::io::Cursor;

    fn create_sample_data() -> Vec<Operation> {
        vec![
            Operation {
                date: "2026-10-05".to_string(),
                category: "Продукты".to_string(),
                operation_type: "Расход".to_string(),
                summ: 1500,
            },
            Operation {
                date: "2026-10-06".to_string(),
                category: "Зарплата".to_string(),
                operation_type: "Доход".to_string(),
                summ: 50000,
            },
        ]
    }

    #[test]
    fn test_parse_csv_success() {
        let csv_data = "date,category,operation_type,summ\n\
                        2026-10-05,Продукты,Расход,1500\n\
                        2026-10-06,Зарплата,Доход,50000\n";

        // Используем абстрактный ввод через Cursor поверх среза байт
        let reader = Cursor::new(csv_data);
        let result = Formats::Csv.parse(reader).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].category, "Продукты");
        assert_eq!(result[1].summ, 50000);
    }

    #[test]
    fn test_parse_text_success() {
        let text_data = "2026-10-05;Продукты;Расход;1500\n\
                         2026-10-06;Зарплата;Доход;50000\n";

        let reader = Cursor::new(text_data);
        let result = Formats::Text.parse(reader).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].date, "2026-10-05");
        assert_eq!(result[1].operation_type, "Доход");
    }

    #[test]
    fn test_parse_bin_success() {
        let format = Formats::Bin;
        let original_data = create_sample_data();
        let mut buffer = Cursor::new(Vec::new());

        format.write(&mut buffer, &original_data).unwrap();

        buffer.set_position(0);

        let parsed_data = format.parse(&mut buffer).unwrap();
        assert_eq!(parsed_data.len(), 2);
        assert_eq!(parsed_data[0].category, "Продукты");
        assert_eq!(parsed_data[1].summ, 50000);
    }

    #[test]
    fn test_parse_csv_error() {
        let bad_csv = "date,category,operation_type,summ\n2026-10-05,Продукты,Расход,НЕ_ЧИСЛО\n";
        let reader = Cursor::new(bad_csv);
        let result = Formats::Csv.parse(reader).unwrap();

        assert_eq!(result[0].summ, 0);
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
    fn test_format_conversion() {
        let text_source = "2026-10-05;Продукты;Расход;1500\n";
        let reader = Cursor::new(text_source);
        let mut writer = Cursor::new(Vec::new());

        let count = Formats::convert(&Formats::Text, &Formats::Csv, reader, &mut writer).unwrap();
        assert_eq!(count, 1);

        writer.set_position(0);
        let csv_string = String::from_utf8(writer.into_inner()).unwrap();
        assert!(csv_string.starts_with("date,category"));
        assert!(csv_string.contains("2026-10-05,Продукты,Расход,1500"));
    }

    #[test]
    fn test_comparer_identical() {
        let list1 = create_sample_data();
        let list2 = create_sample_data();

        // 1. Создаем два буфера в памяти для бинарного формата
        let mut buffer1 = Cursor::new(Vec::new());
        let mut buffer2 = Cursor::new(Vec::new());

        // 2. Записываем обе коллекции
        Formats::Bin.write(&mut buffer1, &list1).unwrap();
        Formats::Bin.write(&mut buffer2, &list2).unwrap();

        // 3. Обязательно перематываем курсоры в начало перед чтением!
        buffer1.set_position(0);
        buffer2.set_position(0);

        // 4. Вызываем ассоциированную функцию, передавая туда Cursor (которые реализуют Read)
        let report =
            Formats::compare_sources(&mut buffer1, &Formats::Bin, &mut buffer2, &Formats::Bin, 5)
                .unwrap();

        // Данные совпадают, отчет должен быть пустым
        assert!(report.is_empty());
    }

    #[test]
    fn test_comparer_different() {
        let list1 = create_sample_data();
        let mut list2 = create_sample_data();

        list2[1].summ = 9999;

        let mut buffer1 = Cursor::new(Vec::new());
        let mut buffer2 = Cursor::new(Vec::new());

        Formats::Bin.write(&mut buffer1, &list1).unwrap();
        Formats::Bin.write(&mut buffer2, &list2).unwrap();

        buffer1.set_position(0);
        buffer2.set_position(0);

        let report =
            Formats::compare_sources(&mut buffer1, &Formats::Bin, &mut buffer2, &Formats::Bin, 5)
                .unwrap();

        assert!(!report.is_empty());
        let has_error_text = report
            .iter()
            .any(|line| line.contains("amount: 50000 != 9999"));
        assert!(
            has_error_text,
            "Отчет должен содержать информацию о разнице сумм"
        );
    }
}
