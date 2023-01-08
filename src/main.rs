use std::{fmt, fs};

use cipher::{Key, KeySizeUser, Unsigned};
use const_format::concatcp;
use des::Des;
use des_usage::helper::path::{InputPath, OutputPath};
use des_usage::{crydec, signature};

const KEY_SIZE_USIZE: usize = <Des as KeySizeUser>::KeySize::USIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkMode {
    Encryption,
    Decryption,
    Signature,
}

fn main() -> anyhow::Result<()> {
    'program_loop: loop {
        let work_mode = inquire::Select::new(
            "Оберіть режим роботи:",
            vec![
                WorkMode::Encryption,
                WorkMode::Decryption,
                WorkMode::Signature,
            ],
        )
        .with_help_message("↑↓ щоби рухатися, enter щоб обрати, вводьте щоБ відфільтрувати")
        .prompt();

        let work_mode = match work_mode {
            Ok(work_mode) => work_mode,
            Err(e) => match e {
                inquire::InquireError::OperationCanceled
                | inquire::InquireError::OperationInterrupted => break 'program_loop,
                _ => return Err(e.into()),
            },
        };

        let res = match work_mode {
            WorkMode::Encryption => encryption_mode()?,
            WorkMode::Decryption => decryption_mode()?,
            WorkMode::Signature => signature_mode()?,
        };

        if res.is_none() {
            break 'program_loop;
        }
    }

    Ok(())
}

fn encryption_mode() -> anyhow::Result<Option<()>> {
    let Some(key) = prompt_key_input()? else { return Ok(None) };
    println!("Ключ: {key:?}");

    let Some((input_data_path, output_data_path)) = prompt_paths_input("Введіть шлях до файлу, який зашифрувати:")? else { return Ok(None) };

    let data = fs::read(input_data_path)?;

    println!("Шифруємо файл введення...");
    let encrypted = crydec::encrypt(&data, &key);

    println!("Файл введення зашифровано");
    fs::write(output_data_path, encrypted)?;
    println!("Файл виведення записано");

    Ok(Some(()))
}

fn decryption_mode() -> anyhow::Result<Option<()>> {
    let Some(key) = prompt_key_input()? else { return Ok(None) };
    println!("Ключ: {key:?}");

    let Some((input_data_path, output_data_path)) = prompt_paths_input("Введіть шлях до файлу, який розшифрувати:")? else { return Ok(None) };

    let data = fs::read(input_data_path)?;

    println!("Шифруємо файл введення...");
    let decrypted = crydec::decrypt(&data, &key);

    println!("Файл введення зашифровано");
    fs::write(output_data_path, decrypted)?;
    println!("Файл виведення записано");

    Ok(Some(()))
}

fn signature_mode() -> anyhow::Result<Option<()>> {
    let Some(key) = prompt_key_input()? else { return Ok(None) };
    println!("Ключ: {key:?}");

    let path = skip_interruption!(inquire::CustomType::<InputPath>::new(
        "Введіть шлях до файлу, який підписати:"
    )
    .prompt());

    let data = fs::read(path)?;
    println!("Підписуємо файл введення...");

    let signature = signature::sign(&data, &key);
    println!("Підпис: {signature:?}");

    Ok(Some(()))
}

fn prompt_key_input() -> anyhow::Result<Option<Key<Des>>> {
    let key = skip_interruption!(inquire::Text::new(concatcp!(
        "Введіть ключ шифрування довжиною ",
        KEY_SIZE_USIZE,
        ":"
    ))
    .with_validator(inquire::length!(
        KEY_SIZE_USIZE,
        concatcp!("Довжина ключа має бути ", KEY_SIZE_USIZE)
    ))
    .prompt());
    let key = Key::<Des>::from_exact_iter(key.into_bytes().into_iter());
    Ok(Some(unsafe { key.unwrap_unchecked() }))
}

fn prompt_paths_input(
    inp_file_prompt: &'static str,
) -> anyhow::Result<Option<(InputPath, OutputPath)>> {
    let input_data_path =
        skip_interruption!(inquire::CustomType::<InputPath>::new(inp_file_prompt).prompt());
    let output_data_path = skip_interruption!(inquire::CustomType::<OutputPath>::new(
        "Введіть шлях до файлу, у який записати:"
    )
    .prompt());

    Ok(Some((input_data_path, output_data_path)))
}

#[macro_export(local_inner_macros)]
macro_rules! skip_interruption {
    ($e: expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => match e {
                inquire::InquireError::OperationCanceled => return Ok(None),
                inquire::InquireError::OperationInterrupted => return Ok(None),
                e => return Err(e.into()),
            },
        }
    };
}

impl fmt::Display for WorkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encryption => write!(f, "Шифрування"),
            Self::Decryption => write!(f, "Розшифрування"),
            Self::Signature => write!(f, "Підписання"),
        }
    }
}
