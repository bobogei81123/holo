pub struct Game {
    players: Players,
}

struct Players {
}

impl Game {
    pub fn new() -> Self {
        Self
    }

    pub fn step(&mut self) -> Instruction {
    }
}

pub enum Instruction {
    Message(Message),
    Question(Question),
}

pub enum Message {
    NightBegin,
    StageBegin(Box<Stage>),
    StageEnd(Box<Stage>),
    SeerResult,
}

pub enum Question {
    YesOrNo,
    SelectPlayer,
}

pub enum Respond {
    YesNo(bool),
    SelectPlayer(Option<()>),
}

trait Stage {
    fn step(&mut self, respond: Option<()>) -> Vec<Instruction> {
        vec![]
    }
}

pub struct Player;

pub enum Role {
    Villager,
    Seer(Seer),
    Witch,
    Idiot,
    Hunter,
    Werewolf,
}

pub enum Team {
    Villagers,
    Deities,
    Werewolves,
}

impl Role {
    pub fn team(&self) -> Team {
        match self {
            Role::Villager => Team::Villagers,
            Role::Seer(..) | Role::Witch | Role::Idiot | Role::Hunter => Team::Deities,
            Role::Werewolf => Team::Werewolves,
        }
    }
}

pub struct Seer {
}

pub struct Witch {
    can_revive: bool,
    can_poison: bool,
}
