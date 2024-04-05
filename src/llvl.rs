use std::collections::HashMap;

pub fn to_llvl_code(code: &str, table: &HashMap<&str, u8>) -> String {
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

pub fn to_bf_code(code: &str, table: &HashMap<&str, u8>) -> Vec<u8> {
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
