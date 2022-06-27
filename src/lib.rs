#[allow(unused_imports)]

use near_sdk::{
  borsh,
  borsh::{ BorshDeserialize, BorshSerialize},
  log,
  serde::{ Deserialize, Serialize,},
  env,
  near_bindgen,
};

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate="near_sdk::serde")]
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
      let len = self.ideas.len();
      let mut idea_id = 0;
      if len > 0 {
        idea_id = self.ideas[len - 1].id + 1;
      }
      let idea = Idea {
        id: idea_id as u8,
        user,
        title,
      };
      self.ideas.push(idea.clone());
      return idea;
    }
    
    pub fn get_idea(&self, index: isize) -> Option<&Idea> {
      if index < 0 {
        env::panic_str("It can't be negative");
      }
      self.ideas.get(index as usize)
    }

    pub fn remove_idea(&mut self, index: usize) -> Idea {
      self.ideas.remove(index)
    }

  }

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
  use super::*;
  use near_sdk::{
    AccountId,
    env,
    testing_env,
    test_utils::VMContextBuilder
  };

  fn setup() -> Ideas {
    let builder = VMContextBuilder::new();
    testing_env!(builder.build());
    let contract = Ideas::default();
    return contract;
  }
  
  #[test]
  fn get_idea() {
    let contract = setup();
    
    let idea = contract.get_idea(0);
    assert!(idea.is_none());
  }

  #[test]
  #[should_panic(expected="It can't be negative")]
  fn get_wrong_idea() {
    let contract = setup();

    let _idea = contract.get_idea(-1);

  }

}
