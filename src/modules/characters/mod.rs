pub mod sheep;

pub enum CommonCharacterChoices {
    CONVOSTART,
}

pub fn match_common_choices(choice: CommonCharacterChoices) -> i32 {
    match choice {
        CommonCharacterChoices::CONVOSTART => 0
    }
}

pub struct Choice {
    pub text: String,
    pub choice_number: i32,
}

pub trait Character {
    fn get_relevant_text(&self, choice: i32, characters: Vec<Box<Character>>) -> &str;
    fn get_relevant_options(&self, choice: i32, characters: Vec<Box<Character>>) -> Vec<Choice>;
}