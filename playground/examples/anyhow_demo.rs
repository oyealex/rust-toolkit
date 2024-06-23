use std::fs::File;
use std::io::Read;

use anyhow::{Context, Result as AnyResult};

// 读取文件内容的函数，使用 Result<T> 处理错误
fn read_file_contents(path: &str) -> AnyResult<String> {
    // 尝试打开文件，并在出错时添加上下文信息
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path))?;

    let mut contents = String::new();
    // 尝试读取文件内容，并在出错时添加上下文信息
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read file: {}", path))?;

    Ok(contents)
}

fn main() {
    // 指定文件路径
    let path = "example.txt";

    // 调用 read_file_contents 函数并处理可能的错误
    match read_file_contents(path) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Error: {:?}", e),
    }
}
