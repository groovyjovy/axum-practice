use serde::{Serialize, Deserialize};
use serde_with::{serde_as, NoneAsEmptyString};

#[serde_as]
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GetColors {
    #[serde_as(as = "NoneAsEmptyString")]
    color_1_r: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    color_1_g: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    color_1_b: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    color_2_r: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    color_2_g: Option<u8>,
    #[serde_as(as = "NoneAsEmptyString")]
    color_2_b: Option<u8>,
}

impl GetColors {
    pub fn params(&self) -> ([u8; 3], [u8; 3]) {
        (
            [
                self.color_1_r.unwrap_or(0),
                self.color_1_g.unwrap_or(0),
                self.color_1_b.unwrap_or(0),
            ],
            [
                self.color_2_r.unwrap_or(0),
                self.color_2_g.unwrap_or(0),
                self.color_2_b.unwrap_or(0),
            ],
        )
    }
}

impl Default for GetColors {
    fn default() -> Self {
        Self {
            color_1_r: None,
            color_1_g: None,
            color_1_b: None,
            color_2_r: None,
            color_2_g: None,
            color_2_b: None,
        }
    }
}

#[derive(serde::Serialize)]
pub struct Response {
    pub delta_e: f32,
}
