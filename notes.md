# Заметки по Rust 

Установка: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 

`rustc` = компилятор Rust: превращает файл .rs в программу.  
`cargo` (запускает rustc и дополнительно умеет запускать тесты и скачивать зависимости). 

## 1. Модули и пространство имён

`std` — стандартная библиотека Rust. Это модуль, то есть пространство имён для связанного кода.

Запись `std::fs::read_to_string` означает: в `std` найти модуль `fs`, а в нём — функцию `read_to_string`.

`fs` — не класс. Это модуль для работы с файловой системой.

```rust
let text = std::fs::read_to_string("data.txt");
```

## 2. Строки: `&str` и `String`

`"Привет"` — строковый литерал типа `&str`: неизменяемая ссылка на текст, записанный в программе.

`String` — отдельная строка, которой владеет программа. Её создаёт `.to_string()`.

```rust
let literal: &str = "Привет";
let text: String = "Привет".to_string();
```

## 3. `Result`: успех или ошибка

`Result<T, E>` — перечисление (`enum`) для операции, которая может завершиться успешно или с ошибкой:

```rust
Ok(T)  // успех, внутри значение типа T
Err(E) // ошибка, внутри значение типа E
```

`Ok(...)` не преобразует строку. Он создаёт успешный вариант `Result` и кладёт переданное значение внутрь.

```rust
let text: String = "Привет".to_string();
let result: Result<String, std::io::Error> = Ok(text);
```

Здесь `text` — строка, а `result` — успешный `Result` со строкой внутри. По смыслу это `Result::Ok(text)`, но обычно пишут коротко: `Ok(text)`.

Rust понимает, что `Ok` относится к `Result`, по ожидаемому типу, например по типу, который должна вернуть функция.

`Result` нужен, когда операция может как дать значение, так и не получиться. По варианту `Ok` или `Err` код выбирает, что делать дальше.

```rust
let result = std::fs::read_to_string("data.txt");

match result {
    Ok(text) => println!("Файл: {text}"),
    Err(error) => println!("Не удалось прочитать файл: {error}"),
}
```

Без `Result` функция не смогла бы честно сообщить одновременно и текст файла, и причину, по которой чтение не удалось.

`match result` проверяет, какой вариант `Result` лежит в переменной: `Ok` или `Err`. В `Ok(text)` имя `text` получает строку изнутри успешного результата; в `Err(error)` имя `error` получает ошибку.

## 4. Ошибки и их распространение

`std::io::Error` — структура (`struct`) с информацией об ошибке ввода-вывода, например «файл не найден». Это не класс. 

```rust
fn read_data() -> Result<String, std::io::Error> {
    let text: String = std::fs::read_to_string("data.txt")?;
    Ok(text)
}
``` 

Эквивалентно: 

```rust
let text: String = match std::fs::read_to_string("data.txt") {
    Ok(value) => value,
    Err(error) => return Err(error), /// конвертируем std::io::Error error обратно в тип Result<>
};
``` 

s

## 5. Преобразование строки в число

```rust
let guess: u32 = "42".parse().expect("Не число!");
```

`parse()` пытается превратить строку в число. `u32` задаёт нужный тип числа. `expect()` останавливает программу с указанным сообщением, если преобразование не удалось. 

## 6. Работа с Rustlings for HSE

Решай упражнения по порядку, проверяй их командой из README репозитория и сохраняй изменения коммитами. В этой папке `origin` указывает на аккаунт `ifde`.

```bash
git add .
git commit -m "Solve variables exercises"
git push
```

Проверка GitHub Actions запускается только для pull request в ветку `master`. Поэтому создай отдельную ветку, отправляй изменения в неё и затем открой pull request в `master`.

```bash
git switch -c solutions
git push -u origin solutions
```

`rustlings-for-hse` — отдельный Git-репозиторий внутри папки с материалами. Внешний репозиторий специально игнорирует его через `.gitignore`. Чтобы увидеть его изменения в VS Code, открой именно папку `rustlings-for-hse` как отдельную рабочую папку или выбери команду `Git: Open Repository`.  

`cargo install rustlings` - Cargo скачает исходный код Rustlings, скомпилирует его с помощью rustc и установит готовую программу rustlings
`rustlings check-all` - проверка всех упражнений 
