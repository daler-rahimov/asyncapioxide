use asyncapioxide::gen_doc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// #[gen_doc]
#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
struct Username(String);

#[gen_doc(
    on ="todo:create",
    response_model = Username,
    request_model = Username,
    emits = (
        (event = "todo:created", model = "Username", description = "some description")
    )
)]
pub fn create() {
    todo!()
}

// #[gen_doc(
//     on ="todo:create",
//     response_model = Username,
//     request_model = Username,
//     emits = (
//         (event = "todo:created", model = "Username", description = "some description")
//     )
// )]

fn main() {}
