use std::{collections::HashMap, env, fs::read, io::Write};

use anyhow::Result;

use crate::interpreter::run;

mod interpreter;
mod memory;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut table = HashMap::new();
    table.insert("すきすきだいすき", b'>');
    table.insert("すきすき大好き", b'<');
    table.insert("すき好きだいすき", b'+');
    table.insert("すき好き大好き", b'-');
    table.insert("きんぴら大好き", b'.');
    table.insert("好きすき大好き", b',');
    table.insert("好き好きだいすき", b'[');
    table.insert("好き好き大好き", b']');

    if args.len() > 2 {
        match args[1].as_str() {
            "run" => main_run(&args, &table)?,
            "parse" => main_parse(&args, &table)?,
            _ => {}
        };

        Ok(())
    } else {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut code = String::new();

        loop {
            write!(stdout, "> ")?;
            stdout.flush()?;
            stdin.read_line(&mut code)?;
            run(&to_bf_code(&code, &table))?;
        }
    }
}

fn main_run(args: &[String], table: &HashMap<&str, u8>) -> Result<()> {
    let path = &args[2];
    let code = String::from_utf8(read(path)?)?;

    run(&to_bf_code(&code, table))?;
    Ok(())
}

fn main_parse(args: &[String], table: &HashMap<&str, u8>) -> Result<()> {
    let code = &args[2];
    println!("{}", to_llvl_code(code, table));
    Ok(())
}

fn to_llvl_code(code: &str, table: &HashMap<&str, u8>) -> String {
    let code: Vec<u8> = code.as_bytes().to_vec();
    let reverse_table = table.iter().fold(HashMap::new(), |mut acc, (key, value)| {
        acc.insert(*value, key);
        acc
    });

    let mut result = String::new();

    for c in code.into_iter() {
        if let Some(s) = reverse_table.get(&c) {
            result.push_str(s);
        }
    }

    result
}

fn to_bf_code(code: &str, table: &HashMap<&str, u8>) -> Vec<u8> {
    let mut code: String = code.chars().collect();
    let mut result = Vec::new();

    while !code.is_empty() {
        let mut exists = false;

        for key in table.keys() {
            if code.starts_with(key) {
                code = code.split_off(key.len());
                result.push(*table.get(key).unwrap());

                exists = true;
            }
        }

        if !exists {
            code = code.split_off(1);
        }
    }

    result
}
