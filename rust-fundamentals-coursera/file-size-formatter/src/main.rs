use std::env;
use std::str::FromStr;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl Sizes {
    fn from(size: f64, unit: &str) -> Self {
        let bytes = match unit.to_lowercase().as_str() {
            "kb" => (size * 1000.0) as u64,
            "mb" => (size * 1_000_000.0) as u64,
            "gb" => (size * 1_000_000_000.0) as u64,
            _ => size as u64, // Para bytes directamente
        };

        // Usamos format_size para convertir y generar las cadenas en diferentes unidades
        let bytes_str = format_size(bytes);
        let kilobytes_str = format_size(bytes / 1000);
        let megabytes_str = format_size(bytes / 1_000_000);
        let gigabytes_str = format_size(bytes / 1_000_000_000);

        Sizes {
            bytes: bytes_str,
            kilobytes: kilobytes_str,
            megabytes: megabytes_str,
            gigabytes: gigabytes_str,
        }
    }
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} kilobytes", kb),
        FileSize::Megabytes(mb) => format!("{:.2} megabytes", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} gigabytes", gb),
    }
}

fn parse_input(input: &str) -> Result<(f64, String), String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Invalid input format. Expected: <number> <unit>".to_string());
    }

    let size = f64::from_str(parts[0]).map_err(|_| "Invalid number format".to_string())?;
    let unit = parts[1].to_string();

    Ok((size, unit))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    match parse_input(input) {
        Ok((size, unit)) => {
            let sizes = Sizes::from(size, &unit);
            println!("{:?}", sizes);
        }
        Err(e) => println!("Error: {}", e),
    }
}
