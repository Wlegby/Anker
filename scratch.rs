use serde::Deserialize;

fn main() {
    let val = serde_json::Value::Null;
    let r: Result<(), _> = serde_json::from_value(val);
    println!("r: {:?}", r);
}
