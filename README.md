# rust-mini-cli

[![Rust CI](https://github.com/Isotere/rust-mini-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/Isotere/rust-mini-cli/actions/workflows/rust.yml)

Учебный CLI на Rust — **анализатор логов**: читает строки из `stdin`, парсит,
фильтрует по уровню/подстроке и печатает сводку по уровням.

Цель проекта — не «ещё одна тула», а **shipped-артефакт как доказательство
владения Rust**: от пустого `cargo new` до бинаря с тестами, CI и документацией.
Метрика — законченный работающий инструмент, а не число прочитанных глав.

> **Статус:** `v0.1.0` — MVP работает (stdin → filter → summary). Phase 0 (Rust Ramp).

## Возможности

- Чтение лог-строк из `stdin` (pipe).
- Устойчивый парсинг строки формата `timestamp:LEVEL:message` на регулярном
  выражении — корректно разбирает ISO-таймстемпы с двоеточиями
  (`2026-06-07T12:00:00`), битые/пустые строки молча пропускаются.
- Фильтр по минимальному уровню (`--min-level`) и по подстроке сообщения
  (`--contains`).
- Сводка: общее число записей и счётчик по уровням (детерминированный порядок
  по severity — на `BTreeMap`).
- Unit- и интеграционные тесты; зелёный CI (fmt + clippy + test).

## Уровни

Распознаются (регистронезависимо, с алиасами), порядок по severity:

```
UNKNOWN < DEBUG < INFO < WARNING < ERROR < FATAL
```

Алиасы: `WARN` → `WARNING`, `ERR` → `ERROR`. Нераспознанный уровень → `UNKNOWN`.

## Формат строки лога

```
<timestamp>:<LEVEL>:<message>
```

Разделитель — двоеточие или пробел; лишние двоеточия остаются в `message`.
Примеры валидных строк:

```
2026-06-07 12:00:00:INFO:server started
2026-06-07T12:00:01:ERROR:db timeout: connection refused
ts WARN low disk space
```

## Установка и запуск

```bash
cargo build --release          # сборка
cargo run -- [ФЛАГИ]           # запуск (читает stdin)
```

Флаги:

| Флаг | Что делает |
|------|------------|
| `--min-level <LEVEL>` | оставить записи этого уровня и выше |
| `--contains <SUBSTR>` | оставить записи, в сообщении которых есть подстрока |
| `--show` | печатать каждую прошедшую строку (не только сводку) |

Прошедшие строки идут в **stdout**, сводка — в **stderr** (чтобы не смешивать
данные и отчёт в пайпах).

### Примеры

```bash
# Только ERROR и выше, с выводом строк:
printf '2026-06-07 12:00:00:INFO:ok\n2026-06-07 12:00:01:ERROR:boom\n' \
  | cargo run -q -- --min-level ERROR --show
# stdout: 2026-06-07 12:00:01 ERROR boom
# stderr: --- summary ---
#         total: 1
#         ERROR: 1

# Сводка по реальному файлу:
cat app.log | cargo run -q -- --min-level WARNING

# Поиск по подстроке:
cat app.log | cargo run -q -- --contains timeout --show
```

## Архитектура

Cargo workspace (`crates/*`). Ядро — крейт `logentry` (lib + bin в одном пакете):

```
crates/logentry/src/
├── main.rs        # тонкий bin: разбор аргументов (clap) → run() → печать сводки
├── lib.rs         # корень библиотеки
├── app.rs         # run<R: BufRead, W: Write>() — ядро пайплайна (тестируемо, без прямого stdin/stdout)
├── parser/        # parse(&str) -> Result<LogEntry, ParseError> (regex + кастомный error)
│   ├── core.rs
│   └── error.rs
├── entry.rs       # LogEntry, LogLevel (Ord по severity, Display)
├── filter.rs      # предикаты: by_level / min_level / message_contains
└── aggregator.rs  # Summary: total + by_level (BTreeMap), count/iter
```

Принцип: вся логика в `run()`, которая принимает `reader`/`writer` параметрами
(dependency injection) — поэтому пайплайн тестируется на байтовом буфере без
реального stdin.

## Разработка

```bash
cargo test --workspace                              # тесты
cargo fmt --all --check                             # форматирование
cargo clippy --workspace --all-targets -- -D warnings   # линт
```

CI (GitHub Actions, `.github/workflows/rust.yml`) на каждый push / PR:
fmt-check + clippy (`-D warnings`) + build + test на ubuntu/macOS.

## Roadmap

- [x] **Must:** stdin, фильтр по уровню, сводка по уровням, осмысленные ошибки
  парсинга, тесты + зелёный CI.
- [ ] Чтение из файла-аргумента (сейчас только stdin).
- [ ] Валидация значения `--min-level` (неизвестный уровень сейчас трактуется как
  `UNKNOWN` = без фильтра).
- [ ] Фильтр по диапазону времени; top-N частых сообщений.
- [ ] Цветной вывод (с авто-отключением вне TTY); сводка в JSON.

## Лицензия

MIT (см. [LICENSE](LICENSE)).
