# 1️⃣ Сборка приложения
FROM rust:1.77 as builder

WORKDIR /usr/src/order_manager_back

# Копируем Cargo-файлы сначала (чтобы использовать кэш слоёв)
COPY Cargo.toml Cargo.lock ./

# Скачиваем зависимости
RUN cargo fetch

# Копируем исходники
COPY src ./src

# Собираем релизный бинарник
RUN cargo build --release

# 2️⃣ Минимальный runtime
FROM fedora:39
RUN dnf install -y ca-certificates && dnf clean all

# Копируем бинарник из builder
COPY --from=builder /usr/src/order_manager_back/target/release/order_manager_back /usr/local/bin/order_manager_back

# Указываем команду по умолчанию
CMD ["order_manager_back"]
