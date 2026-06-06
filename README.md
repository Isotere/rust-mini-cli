# rust-mini-cli

[![Rust CI](https://github.com/Isotere/rust-mini-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/Isotere/rust-mini-cli/actions/workflows/rust.yml)

Учебный CLI-проект на Rust. Площадка для отработки идиоматичного Rust (ownership,
модули, тесты, CI) с прицелом на законченный артефакт — **анализатор логов** из
командной строки.

> **Статус:** Phase 0 (Rust Ramp), ранняя стадия. Сейчас в репозитории учебный
> модуль-конвертер температур; основная функциональность (log analyzer) в работе.

## Цель

Не «ещё одна тула», а **shipped-артефакт как доказательство владения Rust**:
от пустого `cargo new` до бинаря с тестами, CI и документацией. Метрика —
законченный работающий инструмент, а не количество прочитанных глав.

Конечная задумка — `loga` (рабочее имя): принимать поток/файл логов, фильтровать,
агрегировать и выдавать сводку (по уровню, по времени, top-N сообщений).

## MVP scope

Приоритизация по MoSCoW.

### Must (ядро MVP)

- [ ] Чтение логов из файла и из `stdin` (pipe).
- [ ] Фильтр по уровню (`error` / `warn` / `info` / ...).
- [ ] Подсчёт строк по уровням — сводка.
- [ ] Коды возврата и сообщения об ошибках (нет файла, битая строка).
- [ ] Unit-тесты на парсинг и фильтрацию; зелёный CI.

### Should (повышает ценность)

- [ ] Фильтр по диапазону времени.
- [ ] Top-N самых частых сообщений.
- [ ] Аргументы через `clap`.
- [ ] Цветной вывод (с авто-отключением не в TTY).

### Nice (если останется время фазы)

- [ ] Несколько форматов логов (plain / JSON lines).
- [ ] Вывод сводки в JSON (для пайплайнов).
- [ ] Бенчмарки на больших файлах.
- [ ] Релизные бинари через GitHub Releases.

## Сборка и запуск

```bash
cargo build              # debug
cargo build --release    # optimized
cargo run                # запуск
cargo test               # тесты
```

Качество (то же, что гоняет CI):

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
```

## Структура

```
src/
├── main.rs              # entry point
├── lib.rs              # корень библиотеки (модули)
└── farenheit/
    ├── mod.rs
    └── converter.rs    # учебный модуль: конвертер температур F↔C + тесты
```

## CI

GitHub Actions (`.github/workflows/rust.yml`) на каждый push / PR в `master`:

- **lint** — `cargo fmt --check` + `cargo clippy -D warnings` (ubuntu).
- **test** — `cargo build` + `cargo test` на матрице ubuntu + macOS.

Кэш сборки — `Swatinem/rust-cache`; toolchain — `dtolnay/rust-toolchain@stable`.

## Лицензия

TBD.
