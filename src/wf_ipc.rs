//! Wayfire JSON IPC protocols

// todo: This JSON-RPC syntax is a bit weird, maybe use a crate for it?
// The syntax seems to be JSON-RPC 1.0, but the `params` field is now called `data`
// see: `jsonrpsee` crate
use serde::{Deserialize, Serialize};

/*
    reference format:
def get_msg_template(method: str):
    # Create generic message template
    message = {}
    message["method"] = method
    message["data"] = {}
    return message
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct WayfireMessage {
    pub method: String,
    pub data: serde_json::Value,
}

impl WayfireMessage {
    pub fn into_json(self) -> serde_json::Value {
        serde_json::json!({
            "method": self.method,
            "data": self.data,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WayfireCommand {
    #[serde(rename = "window-rules/events/watch")]
    Watch,
    #[serde(rename = "window-rules/output-info")]
    OutputInfo {
        id: i32,
    },
    ListViews,
    ConfigureView {
        id: i32,
        geometry: Geometry,
    },
}

impl WayfireCommand {
    pub fn into_message(self) -> WayfireMessage {
        let method = match self {
            Self::Watch => "window-rules/events/watch",
            Self::OutputInfo { .. } => "window-rules/output-info",
            Self::ListViews => "list-views",
            Self::ConfigureView { .. } => "configure-view",
        };
        // data is: just serialize the struct, unless there's no data, then it's an empty object
        // todo: prolly make this a macro or something
        let data = match self {
            Self::Watch => serde_json::json!({}),
            Self::OutputInfo { id } => serde_json::json!({ "id": id }),
            Self::ListViews => serde_json::json!({}),
            Self::ConfigureView { id, geometry } => {
                serde_json::json!({ "id": id, "geometry": geometry })
            }
        };
        WayfireMessage {
            method: method.to_string(),
            data,
        }
    }
}
