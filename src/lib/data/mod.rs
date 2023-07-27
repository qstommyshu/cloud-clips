use derive_more::{ Display, From };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use std::str::FromStr;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    // TODO: explain what does this nil() does?
    // https://www.bilibili.com/video/BV1tm4y1Z7HX/?p=186&spm_id_from=pageDriver&vd_source=412d07365b85aaf0da45845a65078eae
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    // uuid::Error is a enum of
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parser_str(id)?))
    }
}
