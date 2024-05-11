use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Text {
    #[serde(unpack)]
    data: TextData
}
