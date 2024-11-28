TARGET := "./target/debug/bsbo-01-22-team1-var13"

# Список этапов.
default:
    @just --list

# Сборка проекта (вспомогательный этап).
build:
    @cargo build --quiet
    
# Трассировка вызовов функций через `tracing`.
call-trace: build
    #!/usr/bin/env nu
    {{TARGET}} e>| ignore

# Тестовые данные и human-readable информация.
test-output: build
    #!/usr/bin/env nu
    {{TARGET}} | ignore
