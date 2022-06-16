#[allow(unused_imports)]

use near_sdk::{
  borsh,
  borsh::{ BorshDeserialize, BorshSerialize},
  log,
  serde::{ Deserialize, Serialize,},
  env,
  near_bindgen,
};

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Idea {
    id: u8,
    user: String,
    title: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Ideas {
    ideas: Vec<Idea>
}

#[near_bindgen]
impl Ideas {
    pub fn add_idea(&mut self, title: String) -> Idea {
      let account_id = env::signer_account_id();
      let user = String::from(account_id);
      let len_ = self.ideas.len();
      let mut idea_id = 0;
      if len_ > 0 {
        idea_id = self.ideas[len_ - 1].id + 1;
      }
      let idea = Idea {
        id: idea_id as u8,
        user,
        title,
      };
      self.ideas.push(idea.clone());
      return idea;
    }
    
    pub fn get_idea(&self, index: usize) -> Option<&Idea> {
      // let len = self.ideas.len();
      // if len > 0 {
      //   return self.ideas.get(index)
      // } else {
      //   return None
      // }
      self.ideas.get(index)
    }

  }

// #[cfg(test)]
// mod tests {
//   use super
//   #[test]
//   fn add_idea() {

//   }
// }
