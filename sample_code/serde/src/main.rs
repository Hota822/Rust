use serde::{ Serialize, Deserialize };

fn main() {
    let text = r#"{"name": "hoge", "age": 18, "phones": ["+44 1234567", "+44 0987654"]}"#;
    let structure = serde_json::from_str::<Person>(text);
    println!("{:?}", structure);

    let person = Person {
        name: "fuga".to_string(),
        age: 14,
        phones: vec!["+44 135790".to_string(),],
    };
    let json = serde_json::to_string(&person);
    println!("{}", json.unwrap());
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}