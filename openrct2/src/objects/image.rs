#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    Raw,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PaletteType {
    Keep,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageFile {
    pub path: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub src_x: Option<i32>,
    pub src_y: Option<i32>,
    pub src_width: Option<i32>,
    pub src_height: Option<i32>,
    pub format: Option<Format>,
    pub palette: Option<PaletteType>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Image {
    String(String),
    ImageFile(ImageFile),
}
