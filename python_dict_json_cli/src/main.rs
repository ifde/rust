use std::{
    env,
    io::{self, Read},
    process::{Command, Stdio},
};

fn main() {
    let input = read_input().unwrap_or_else(|error| exit_with_error(&error));
    let json = python_dict_to_json(&input).unwrap_or_else(|error| exit_with_error(&error));

    println!("{json}");
}

fn read_input() -> Result<String, String> {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.is_empty() {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .map_err(|error| format!("Не удалось прочитать stdin: {error}"))?;
        Ok(input)
    } else {
        Ok(arguments.join(" "))
    }
}

fn python_dict_to_json(input: &str) -> Result<String, String> {
    let script = r#"
import ast
import json
import sys

try:
    value = ast.literal_eval(sys.stdin.read())
except (SyntaxError, ValueError) as error:
    print(f"Некорректный Python-словарь: {error}", file=sys.stderr)
    sys.exit(1)

if not isinstance(value, dict):
    print("Ожидался Python-словарь, например {'name': 'Ada'}", file=sys.stderr)
    sys.exit(1)

print(json.dumps(value, ensure_ascii=False, indent=2))
"#;

    let mut process = Command::new("python3")
        .arg("-c")
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("Не удалось запустить python3: {error}"))?;

    use std::io::Write;
    process
        .stdin
        .take()
        .ok_or("Не удалось открыть stdin процесса python3")?
        .write_all(input.as_bytes())
        .map_err(|error| format!("Не удалось передать входные данные: {error}"))?;

    let output = process
        .wait_with_output()
        .map_err(|error| format!("Не удалось получить результат python3: {error}"))?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .map(|json| json.trim_end().to_owned())
            .map_err(|error| format!("python3 вернул не-UTF-8 текст: {error}"))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

fn exit_with_error(error: &str) -> ! {
    eprintln!("Ошибка: {error}");
    std::process::exit(1);
}
