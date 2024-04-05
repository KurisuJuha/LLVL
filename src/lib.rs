mod interpreter;
mod llvl;
mod memory;

use std::{
    collections::HashMap,
    io::{stdin, Cursor},
};

use interpreter::Interpreter;
use memory::Memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn run(code: String) -> String {
    let mut table = HashMap::new();
    table.insert("すきすきだいすき", b'>');
    table.insert("すきすき大好き", b'<');
    table.insert("すき好きだいすき", b'+');
    table.insert("すき好き大好き", b'-');
    table.insert("きんぴら大好き", b'.');
    table.insert("好きすき大好き", b',');
    table.insert("好き好きだいすき", b'[');
    table.insert("好き好き大好き", b']');

    let mut input = stdin();
    let mut buffer = Cursor::new(Vec::new());

    let mut interpreter = Interpreter::new(Memory::new());
    interpreter
        .run(
            &llvl::to_bf_code(code.as_str(), &table),
            &mut input,
            &mut buffer,
        )
        .unwrap();

    let inner = buffer.into_inner();
    String::from_utf8(inner).unwrap()
}
