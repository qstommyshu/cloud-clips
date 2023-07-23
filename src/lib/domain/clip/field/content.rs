use super::ClipError;
use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, Deserialize, Serialize)]
//FIXME:rust won't create a struct unless the data is valid? https://www.bilibili.com/video/BV1tm4y1Z7HX?p=180&spm_id_from=pageDriver&vd_source=412d07365b85aaf0da45845a65078eae
//every content creation will be auto validated?
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        // there is content
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    // move self to String
    pub fn into_inner(self) -> String {
        self.0
    }
    // still keep a copy of self
    pub fn as_str(&self) -> &self {
        self.0.as_str()
    }
}
