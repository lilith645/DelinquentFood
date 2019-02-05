use super::{Character, Choice, CommonCharacterChoices, match_common_choices};

static NUMBERS: &'static [&'static str] = &["Hello", "Hello", "Hello", "Hello", "Hello"];

static LANGUAGE: &str = "Rust";


pub struct ConvoStats{
    had_intro: bool,
}

pub struct Sheep {
    stats: ConvoStats
}

pub enum SheepChoices {
    EatGrass,
    SomethingCool,
}

pub fn match_sheep_choices(choice: SheepChoices) -> i32 {
    match choice {
        SheepChoices::EatGrass => 1,
        SheepChoices::SomethingCool => 2,
    }
}

impl Sheep {
    pub fn new() -> Sheep{
        Sheep{
            stats: ConvoStats::new()
        }
    }
}

fn generate_default_options() -> Vec<Choice> {
    let mut i = Vec::new();


    i.push(Choice {
        text: BASE_OPTIONS[0].to_string(),
        choice_number: match_sheep_choices(SheepChoices::EatGrass),
    });

    i.push(Choice {
        text: BASE_OPTIONS[1].to_string(),
        choice_number: match_sheep_choices(SheepChoices::SomethingCool)
    });

    i.push(Choice {
        text: BASE_OPTIONS[2].to_string(),
        choice_number: 0
    });


    i
}

impl Character for Sheep {
    fn get_relevant_text(&self, choice: i32, characters: Vec<Box<Character>>) -> &str {
        if choice < 1 {
            if match_common_choices(CommonCharacterChoices::CONVOSTART) == choice {
                return HELLO_TEXT
            }
        }

        if choice == match_sheep_choices(SheepChoices::EatGrass) {
            return EAT_GRASS_TXT
        }

        if choice == match_sheep_choices(SheepChoices::SomethingCool) {
            return SOMETHING_COOL_TXT
        }

        DUNNO_TEXT
    }

    fn get_relevant_options(&self, choice: i32, characters: Vec<Box<Character>>) -> Vec<Choice> {
        if choice < 1 {
            if match_common_choices(CommonCharacterChoices::CONVOSTART) == choice {
                return generate_default_options()
            }
        }

        if choice == match_sheep_choices(SheepChoices::EatGrass) {
            let mut i = Vec::new();

            i.push(Choice {
                text: EAT_GRASS_OPTIONS[0].to_string(),
                choice_number: 0
            });

            return i

        }

        if choice == match_sheep_choices(SheepChoices::SomethingCool) {

            let mut i = Vec::new();

            for j in 0..SOMETHING_COOL_OPTIONS.len() {
                i.push(Choice {
                    text: SOMETHING_COOL_OPTIONS[j].to_string(),
                    choice_number: 0
                });
            }

            return i
        }

        generate_default_options()
    }
}

impl ConvoStats {
    pub fn new() -> ConvoStats {
        ConvoStats {
            had_intro: false,
        }
    }
}

static DUNNO_TEXT: &str = "Sorry hunny, I don't really know!";
static HELLO_TEXT: &str = "Hello, I am Ms Sheep. A nurse";
static BASE_OPTIONS: &'static [&'static str] = &["Do you eat grass?", "Do something cool", "Bye"];

static EAT_GRASS_TXT: &str = "Why yes young sir, I do eat grass!";
static EAT_GRASS_OPTIONS: &'static [&'static str] = &["Kind of obvious then, I guess."];

static SOMETHING_COOL_TXT: &str = "Watch me pop a gnarly grind on this here corpse. Fuck, I accidental put my foot in his stomach. Oh well! This will be out little secret";
static SOMETHING_COOL_OPTIONS: &'static [&'static str] = &["Neato", "Cool", "Gnarly Man", "You're a chick!"];