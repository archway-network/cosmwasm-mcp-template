use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// (Optionally) define any custom query response types in the `AllQueryResponse` struct
/// @see: src/main.rs (CwMcp::list_query_responses)
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct AllQueryResponse {
//     pub custom_response: cw20_wrap::msg::CustomResponseExample,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidatedQuery {
    pub query_msg: String,
    pub query_request: String,
}
