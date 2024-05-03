use autoincrement::{prelude::*, AutoIncrement};
use once_cell::sync::Lazy;
pub enum Status {
    InProgress,
    Closed,
    Resolved,
    Open
}

pub struct Epic {
  pub id: u32,
  pub name: String,
  pub description: String,
  pub status: Status,
  pub stories: Vec<u32>
}
#[derive(Incremental, PartialEq, Eq, Debug)]
struct EpicId(u32);
static mut EPIC_GENERATOR: once_cell::sync::Lazy<AutoIncrement<EpicId>> = Lazy::new(|| EpicId::init());
impl Epic {
    pub fn new(name: String, description: String) -> Self {
      let id = unsafe { EPIC_GENERATOR.pull().0 };
        Epic {
            id,
            name, 
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

pub struct Story {
  pub id: u32,
  pub name: String,
  pub description: String,
  pub status: Status
}
#[derive(Incremental, PartialEq, Eq, Debug)]
struct StoryId(u32);
static mut STORY_GENERATOR: once_cell::sync::Lazy<AutoIncrement<StoryId>> = Lazy::new(|| StoryId::init());
impl Story {
    pub fn new(name: String, description: String) -> Self {
      let id = unsafe { STORY_GENERATOR.pull().0 };
      Story {
          id,
          name, 
          description,
          status: Status::Open 
      }
    }
}

pub struct DBState {
  pub last_item_id: u32,
  pub epics: Vec<Epic>,
  pub stories: Vec<Story>,
}
