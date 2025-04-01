use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Пути к proto-файлам
    let proto_file = PathBuf::from("proto/content_service.proto").canonicalize().unwrap();
    let proto_dir = PathBuf::from("proto")
        .canonicalize()
        .expect("Failed to resolve proto directory");

    // Конфигурация prost
    let mut prost_config = prost_build::Config::new();

    // 1. Настройка для google.protobuf.Timestamp
    prost_config.extern_path(
        ".google.protobuf.Timestamp",
        "::prost_types::Timestamp", // Для protobuf-кодирования
    );

    // 2. Добавляем serde-атрибуты для всех сообщений
    prost_config.type_attribute(
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    );

    // 3. Особые правила для Timestamp (используем pbjson-types для JSON)
    prost_config.type_attribute(
        "google.protobuf.Timestamp",
        r#"
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(from = "pbjson_types::Timestamp")]
        #[serde(into = "pbjson_types::Timestamp")]
        "#,
    );

    // Генерация структур protobuf
    prost_config.compile_protos(&[proto_file.clone()], &[proto_dir.to_str().unwrap()])?;

    // Конфигурация tonic для gRPC
    tonic_build::configure()
        .build_server(true)  // Генерировать сервер
        .build_client(true)  // Генерировать клиент
        .compile_protos_with_config(
            prost_config,
            &[proto_file],
            &[proto_dir.to_str().unwrap()],
        )?;

    Ok(())
}
