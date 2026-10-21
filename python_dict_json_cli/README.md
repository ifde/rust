# Python dict → JSON CLI

Минимальная CLI-программа на Rust. Принимает Python-словарь и выводит JSON.

Для разбора Python-синтаксиса использует установленный `python3` и безопасную функцию `ast.literal_eval`: код из входной строки не выполняется.

## Запуск

```bash
cargo run -- "{'name': 'Ada', 'age': 37, 'active': True}"
```

Результат:

```json
{
  "name": "Ada",
  "age": 37,
  "active": true
}
```

Можно передать значение через stdin:

```bash
echo "{'items': [1, 2, 3], 'value': None}" | cargo run
```
