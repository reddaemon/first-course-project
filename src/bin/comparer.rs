use std::env;
use std::fs::File;
use std::process;
use std::str::FromStr;

use first_course_project::Formats;

fn print_usage() {
    println!("Использование утилиты сравнения:");
    println!("Одинаковый формат: cargo run --bin comparer -- <формат> <путь1> <путь2>");
    println!("Разные форматы: cargo run --bin comparer -- <формат1> <путь1> <формат2> <путь2>");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let (format1_str, path1, format2_str, path2) = match args.len() {
        3 => (
            args[0].as_str(),
            args[1].as_str(),
            args[0].as_str(),
            args[2].as_str(),
        ),

        4 => (
            args[0].as_str(),
            args[1].as_str(),
            args[2].as_str(),
            args[3].as_str(),
        ),

        _ => {
            print_usage();
            process::exit(1);
        }
    };

    let format1 = Formats::from_str(format1_str).unwrap_or_else(|e| {
        eprintln!("Ошибка формата 1: {}", e);
        process::exit(1);
    });
    let format2 = Formats::from_str(format2_str).unwrap_or_else(|e| {
        eprintln!("Ошибка формата 2: {}", e);
        process::exit(1);
    });

    let file1 = File::open(path1).unwrap_or_else(|e| {
        eprintln!("Ошибка открытия файла '{}': {}", path1, e);
        process::exit(1);
    });
    let file2 = File::open(path2).unwrap_or_else(|e| {
        eprintln!("Ошибка открытия файла '{}': {}", path2, e);
        process::exit(1);
    });

    match Formats::compare_sources(file1, &format1, file2, &format2, 5) {
        Ok(report) => {
            if report.is_empty() {
                println!("Успех: Файлы абсолютно идентичны!");
                process::exit(0);
            } else {
                for line in report {
                    println!("{}", line);
                }
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Критическая ошибка сравнения: {}", e);
            process::exit(1);
        }
    }
}
