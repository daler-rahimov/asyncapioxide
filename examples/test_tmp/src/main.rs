use asyncapioxide::gen_doc;
use schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
struct User {
    username: String,
    password: String,
}

#[gen_doc(
    on ="todo:create",
    response_model = "User",
    request_model = "User",
    emits = [
        (event = "todo:created", model = "Username", description = "some description")
    ]
)]
fn main() {
    todo!()
}
