use asyncapioxide::gen_doc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[gen_doc]
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
struct Username(String);

fn main() {
    println!("Hello, world!");
}
