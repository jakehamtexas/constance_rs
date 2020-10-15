use crate::{constancerc::dto::language::Language, types::OutputOptions};

use super::write_configuration::WriteConfiguration;
use super::{
    super::table_to_constants::table_constant::TableConstant, casing_engine,
    file_buffer_engine::dotnet::Dotnet, file_buffer_engine::rust::Rust,
    file_buffer_engine::typescript::Typescript, file_buffer_engine::FileBufferEngine,
    file_buffer_engine::FileBufferEngineType,
};
pub fn get_write_configurations(
    table_constants: &[TableConstant],
    output_options: &OutputOptions,
) -> Vec<WriteConfiguration> {
    let file_buffer_engine_types = &output_options
        .language_targets
        .clone()
        .map(|vec| {
            vec.iter()
                .map(|language_raw| Language::from_string(language_raw))
                .collect()
        })
        .unwrap_or_else(|| vec![Language::default()])
        .into_iter()
        .map(|language| match language {
            Language::Typescript => FileBufferEngineType::Typesript(Typescript {}),
            Language::Dotnet => FileBufferEngineType::Dotnet(Dotnet {}),
            Language::Rust => FileBufferEngineType::Rust(Rust {}),
        })
        .collect::<Vec<_>>();

    table_constants
        .iter()
        .flat_map(|c| match c {
            TableConstant::SimpleEnum(e) => file_buffer_engine_types
                .iter()
                .map(|engine_type| {
                    let filename = get_filename(&engine_type, c);
                    let buffer = match &engine_type {
                        FileBufferEngineType::Typesript(ts) => ts.simple_enum(e),
                        FileBufferEngineType::Dotnet(dotnet) => dotnet.simple_enum(e),
                        FileBufferEngineType::Rust(rs) => rs.simple_enum(e),
                    };
                    WriteConfiguration { filename, buffer }
                })
                .collect::<Vec<_>>(),
            TableConstant::SimpleEnumWithDescription(e) => file_buffer_engine_types
                .iter()
                .map(|engine_type| {
                    let filename = get_filename(&engine_type, c);
                    let buffer = match &engine_type {
                        FileBufferEngineType::Typesript(ts) => ts.simple_enum_with_description(e),
                        FileBufferEngineType::Dotnet(dotnet) => {
                            dotnet.simple_enum_with_description(e)
                        }
                        FileBufferEngineType::Rust(rs) => rs.simple_enum_with_description(e),
                    };
                    WriteConfiguration { filename, buffer }
                })
                .collect::<Vec<_>>(),
            TableConstant::StringEnum(e) => file_buffer_engine_types
                .iter()
                .map(|engine_type| {
                    let filename = get_filename(&engine_type, c);
                    let buffer = match &engine_type {
                        FileBufferEngineType::Typesript(ts) => ts.string_enum(e),
                        FileBufferEngineType::Dotnet(dotnet) => dotnet.string_enum(e),
                        FileBufferEngineType::Rust(rs) => rs.string_enum(e),
                    };
                    WriteConfiguration { filename, buffer }
                })
                .collect::<Vec<_>>(),
            TableConstant::StringEnumWithDescription(e) => file_buffer_engine_types
                .iter()
                .map(|engine_type| {
                    let filename = get_filename(&engine_type, c);
                    let buffer = match &engine_type {
                        FileBufferEngineType::Typesript(ts) => ts.string_enum_with_description(e),
                        FileBufferEngineType::Dotnet(dotnet) => {
                            dotnet.string_enum_with_description(e)
                        }
                        FileBufferEngineType::Rust(rs) => rs.string_enum_with_description(e),
                    };
                    WriteConfiguration { filename, buffer }
                })
                .collect::<Vec<_>>(),
            TableConstant::ObjectLike(e) => file_buffer_engine_types
                .iter()
                .map(|engine_type| {
                    let filename = get_filename(&engine_type, c);
                    let buffer = match &engine_type {
                        FileBufferEngineType::Typesript(ts) => ts.object_like(e),
                        FileBufferEngineType::Dotnet(dotnet) => dotnet.object_like(e),
                        FileBufferEngineType::Rust(rs) => rs.object_like(e),
                    };
                    WriteConfiguration { filename, buffer }
                })
                .collect::<Vec<_>>(),
            TableConstant::ObjectLikeWithDescription(e) => file_buffer_engine_types
                .iter()
                .map(|engine_type| {
                    let filename = get_filename(&engine_type, c);
                    let buffer = match &engine_type {
                        FileBufferEngineType::Typesript(ts) => ts.object_like_with_description(e),
                        FileBufferEngineType::Dotnet(dotnet) => {
                            dotnet.object_like_with_description(e)
                        }
                        FileBufferEngineType::Rust(rs) => rs.object_like_with_description(e),
                    };
                    WriteConfiguration { filename, buffer }
                })
                .collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>()
}

fn get_filename(engine_type: &FileBufferEngineType, constant: &TableConstant) -> String {
    let identifier = match constant {
        TableConstant::SimpleEnum(e) => e.identifier.clone(),
        TableConstant::SimpleEnumWithDescription(e) => e.identifier.clone(),

        TableConstant::StringEnum(e) => e.identifier.clone(),
        TableConstant::StringEnumWithDescription(e) => e.identifier.clone(),
        _ => panic!("Unimplemented!"),
    };

    let name = [identifier.database_name, identifier.object_name].join("_");

    match engine_type {
        FileBufferEngineType::Typesript(_) => {
            format!("{}{}", casing_engine::pascal_case(&name), ".ts")
        }
        FileBufferEngineType::Dotnet(_) => {
            format!("{}{}", casing_engine::pascal_case(&name), ".cs")
        }
        FileBufferEngineType::Rust(_) => format!("{}{}", casing_engine::snake_case(&name), ".rs"),
    }
}
