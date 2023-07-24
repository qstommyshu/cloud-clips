use serde::{ Deserialize, Serialize };
use derive_more::Constructor;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64); //u64 because number of hits can't be negative

impl Hits {
    // handled by derive_more::Constructor trait, falls into tuple struct category
    // pub fn new(data: u64) -> Self {
    //     Self(data)
    // }
    pub fn into_inner(self) -> u64 {
        self.0
    }
}
