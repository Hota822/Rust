extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let tarou = Person { name: "太郎".to_string(), age: 18 };
    let json = serde_json::to_string(&tarou)?;
    println!("{}", json);

    Ok(())
}