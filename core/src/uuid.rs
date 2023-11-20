use serde::Deserialize;
use uuid::Uuid;

use crate::enums::app_message::AppMessage;

#[derive(Clone, Deserialize)]
pub struct UniqueIdentifier {
    uuid: Uuid,
}

impl UniqueIdentifier {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    #[allow(dead_code)]
    pub fn string(&self) -> String {
        self.uuid.to_string()
    }

    pub fn from_string(uuid: String) -> UniqueIdentifier {
        UniqueIdentifier {
            uuid: Uuid::parse_str(uuid.as_str())
                .map_err(|_e| AppMessage::InvalidUUID)
                .unwrap(),
        }
    }
}
