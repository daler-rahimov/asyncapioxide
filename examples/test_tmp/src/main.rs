// use asyncapi::{AsyncAPI, Channel, ReferenceOr};
// use indexmap::IndexMap;
use asyncapioxide::asyncapi_doc;
use schemars;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
struct User {
    username: String,
    password: String,
}

#[asyncapi_doc(
    on ="todo:create",
    response_model = "User",
    request_model = "User",
    emits = [
        (event = "todo:created", model = "Username", description = "some description")
    ]
)]
fn main() {
    println!("Hello, world!");
}

// fn main() {
//     // servers
//     let mut servers = IndexMap::new();
//     servers.insert(
//         "production".to_string(),
//         ReferenceOr::Item(asyncapi::Server {
//             url: "api.example.com:{port}".to_string(),
//             protocol: "socketio".to_string(),
//             ..Default::default()
//         }),
//     );

//     // channels
//     let mut channels = IndexMap::new();

//     static DEF_CHANNEL_STR: &str = r#"
//     subscribe:
//       message:
//         payload:
//           type: string
//           pattern: ^hello .+$
//     publish:
//       message:
//         payload:
//           type: string
//           pattern: ^hello .+$
//     x-handlers:
//       disconnect: disconnect "#;
//     let parsed_channels: Channel = serde_yaml::from_str(DEF_CHANNEL_STR).unwrap();
//     channels.insert("/".to_string(), parsed_channels);

//     // components

//     let api = AsyncAPI {
//         asyncapi: "2.5.0".to_string(),
//         info: asyncapi::Info {
//             title: "Example API".to_string(),
//             version: "1.0.0".to_string(),
//             ..Default::default()
//         },
//         servers: servers,
//         channels: channels,
//         ..Default::default()
//     };

//     println!("{}", serde_yaml::to_string(&api).unwrap());
// }
