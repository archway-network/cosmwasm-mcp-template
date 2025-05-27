use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ValidatedExecute {
    pub execute_msg: String,
    pub cosmos_msg: String,
}
