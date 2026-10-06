use std::env;
use std::fs::File;
use std::process;

use first_course_project::Formats;

fn print_usage() {
    println!("Использование CLI-приложения:");
    println!(" чтение --read <формат> <путь_к_файлу>");
    println!(" запись --write <формат> <путь_к_файлу>");
    println!("доступнные форматы: text, csv, binary");
    println!("convert <из_формата> <входной_файл> <входной_формат> <выходной_файл>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        print_usage();
        process::exit(1)
    }

    let action = &args[1];
    let format_str = &args[2];
    let file_path = &args[3];

    let format: Formats = format_str.parse().unwrap_or_else(|e| {
        eprintln!("Ошибка: {}", e);
        process::exit(1);
    });

    match action.as_str() {
        "read" => {
            let file = File::open(file_path).unwrap_or_else(|e| {
                eprintln!("Opening file error '{}': {}", file_path, e);
                process::exit(1);
            });
            match format.parse(file) {
                Ok(items) => {
                    println!("Succesfully readed: {}", items.len());
                    println!("{:#?}", items);
                }
                Err(e) => {
                    eprintln!("Parsing error: {}", e);
                    process::exit(1);
                }
            }
        }

        "write" => {
            println!(
                "Вставьте данные в выбранном формате и нажмите Ctrl+D (или Ctrl+Z на Windows) для отправки (EOF):"
            );

            let input_stream = std::io::stdin();

            let reader = input_stream.lock();

            let dataset = format.parse(reader).unwrap_or_else(|e| {
                eprintln!("parse error input data: {}", e);
                std::process::exit(1);
            });

            if dataset.is_empty() {
                println!("data is empty");
                std::process::exit(0);
            }

            let file = File::create(file_path).unwrap_or_else(|e| {
                eprintln!("Ошибка создания файла '{}': {}", file_path, e);
                process::exit(1);
            });

            match format.write(file, &dataset) {
                Ok(_) => println!(
                    "write successfully {} objects to file '{}'",
                    dataset.len(),
                    file_path
                ),
                Err(e) => {
                    eprintln!("Write error {}", e);
                    process::exit(1);
                }
            }
        }
        "convert" => {
            let local_args: Vec<String> = std::env::args().skip(1).collect();

            let is_valid_stream_mode =
                local_args.len() == 3 && local_args[2].parse::<Formats>().is_ok();
            let is_valid_file_mode = local_args.len() == 5;

            if !is_valid_stream_mode && !is_valid_file_mode {
                eprintln!("Ошибка: Неверный синтаксис команды convert.");
                eprintln!(
                    "Использование:\n  Потоковая: cargo run -- convert <из> <в>\n  Файловая:  cargo run -- convert <из> <вход_файл> <в> <выход_файл>"
                );
                process::exit(1);
            }

            let (from_format_str, to_format_str) = if local_args.len() == 3 {
                (local_args[1].as_str(), local_args[2].as_str())
            } else {
                (local_args[1].as_str(), local_args[3].as_str())
            };

            let from_format: Formats = from_format_str.parse().unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

            let to_format: Formats = to_format_str.parse().unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });

            let result = if local_args.len() == 5 {
                let input_path = &local_args[2];
                let output_path = &local_args[4];

                let input_file = File::open(input_path).unwrap_or_else(|e| {
                    eprintln!("Ошибка открытия входного файла: {}: {} ", e, input_path);
                    process::exit(1);
                });

                let output_file = File::create(output_path).unwrap_or_else(|e| {
                    eprintln!("Ошибка создания выходного файла: {}", e);
                    process::exit(1);
                });

                Formats::convert(&from_format, &to_format, input_file, output_file)
            } else {
                eprintln!(
                    "Вставьте данные в выбранном формате и нажмите Ctrl+D (или Ctrl+Z в Windows) для конвертации:"
                );

                Formats::convert(
                    &from_format,
                    &to_format,
                    std::io::stdin(),
                    std::io::stdout(),
                )
            };

            match result {
                Ok(count) => {
                    if local_args.len() == 5 {
                        println!("Конвертация успешна! Перенесено записей: {}", count);
                    }
                }
                Err(e) => {
                    eprintln!("Критическая ошибка библиотеки при конвертации: {}", e);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("unknown action error '{}'", action);
            print_usage();
            process::exit(1);
        }
    }
}
