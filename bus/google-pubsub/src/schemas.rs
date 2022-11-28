use apache_avro::AvroSchema;
use apache_avro::Schema;
use apache_avro::Writer;
use serde::Deserialize;
use serde::Serialize;
use std::env::current_dir;
use std::fs;

#[derive(AvroSchema, Deserialize, Serialize)]
pub struct Demo {
    pub event: String,
    pub value: i64,
}

fn load_topic_schema(topic: &str) -> Result<Schema, String> {
    let root = current_dir().expect("No dir found");
    let file_full_path = root.join(format!("/avro/{}.json", topic));
    let contents =
        fs::read_to_string(file_full_path).expect("Should have been able to read the file");
    let schema = Schema::parse_str(contents.as_str()).expect("Bad schema file");
    Ok(schema)
}

pub fn serialize_using_avro<T: Serialize>(topic: &str, data: T) -> Result<Vec<u8>, String> {
    let schema = load_topic_schema(topic)?;
    let mut writer = Writer::new(&schema, Vec::new());
    writer.append_ser(data).unwrap();
    let encoded = writer.into_inner().expect("Failed to encode");
    Ok(encoded)
}
