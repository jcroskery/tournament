use Stage::*;

use gtk::prelude::*;
use gtk::{Builder, Button, Label};
use rand::prelude::*;

const RACERS: usize = 5;

#[derive(Clone, PartialEq)]
enum Stage {
    GROUP,
    TIEBREAKER,
    SEMI,
    BRONZE,
    GOLD,
}
impl Stage {
    fn to_string(&self) -> String {
        match self {
            GROUP => "Group Stage".to_string(),
            TIEBREAKER => "Tiebreaker Stage".to_string(),
            SEMI => "Semifinal Stage".to_string(),
            BRONZE => "Bronze Medal Match".to_string(),
            GOLD => "Gold Medal Match".to_string(),
        }
    }
    fn get_ccs(&self) -> String {
        match self {
            GROUP => "100 ccs".to_string(),
            TIEBREAKER => "100 ccs".to_string(),
            SEMI => "150 ccs".to_string(),
            BRONZE => "150 ccs".to_string(),
            GOLD => "200 ccs".to_string(),
        }
    }
    fn next_stage(&mut self) -> bool {
        match self {
            GROUP => {
                *self = SEMI;
                true
            },
            TIEBREAKER => {
                *self = SEMI;
                true
            }
            SEMI => {
                *self = BRONZE;
                true
            },
            BRONZE => {
                *self = GOLD;
                true
            },
            GOLD => {
                false
            }
        }
    }
}

#[derive(Clone)]
pub struct Player {
    name: String,
    score: usize,
    rank: usize,
}
impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            score: 0,
            rank: 1,
        }
    }
}

#[derive(Clone)]
struct Race {
    racer_1: String,
    racer_2: String,
    race: usize,
    length: usize,
}
impl Race {
    fn new(racer_1: &str, racer_2: &str, race: usize, length: usize) -> Self {
        Race {
            racer_1: racer_1.to_string(),
            racer_2: racer_2.to_string(),
            race,
            length,
        }
    }
}

pub struct Tournament {
    stage: Stage,
    names: Vec<Player>,
    race: usize,
    races: Vec<Race>,
    pub over: bool,
    dead_player: Option<Player>,
}
impl Tournament {
    pub fn new() -> Self {
        let names = vec![
            Player::new("Justus"),
            Player::new("Faith"),
            Player::new("Hope"),
            Player::new("Mommy"),
            Player::new("Daddy"),
        ];
        let mut races = vec![];
        let mut race = 1;
        for i in 0..RACERS {
            for j in (i + 1)..RACERS {
                races.push(Race::new(&names[i].name, &names[j].name, race, 1));
                race += 1;
            }
        }
        races.shuffle(&mut rand::thread_rng());
        Tournament {
            stage: GROUP,
            names,
            race: 1,
            races,
            over: false,
            dead_player: None,
        }
    }
    pub fn rank_players(&mut self) -> Vec<Player> {
        self.names.sort_by(|a, b| a.score.cmp(&b.score));
        self.names.reverse();
        let mut i = 0;
        let mut last_score = None;
        let mut last_rank = 0;
        self.names
            .iter_mut()
            .map(|a| {
                i += 1;
                if last_score != None && a.score == last_score.unwrap() {
                    a.rank = last_rank;
                } else {
                    a.rank = i;
                    last_score = Some(a.score);
                    last_rank = a.rank;
                }
                a.clone()
            })
            .collect()
    }
    fn add_semi_races(&mut self) {
        let mut race = self.race + 1;
        for j in 0..2 {
            self.races.push(Race::new(&self.names[j].name, &self.names[3 - j].name, race, 4));
            race += 1;
        }
    }
    fn add_tiebreaker_races(&mut self) {
        self.rank_players();
        let mut lowest_rank = 1;
        let mut new_players = vec![];
        for player in self.names.iter_mut().rev() {
            if player.rank > lowest_rank {
                lowest_rank = player.rank;
                new_players.push(player);
            } else if player.rank != lowest_rank {
                player.score += 10;
            } else {
                new_players.push(player);
            }
        }
        let mut race = self.race + 1;
        for i in 0..new_players.len() {
            for j in (i + 1)..new_players.len() {
                self.races.push(Race::new(&new_players[i].name, &new_players[j].name, race, 1));
                race += 1;
            }
        }
    }
    fn add_bronze_races(&mut self) {
        self.rank_players();
        self.races.push(Race::new(&self.names[2].name, &self.names[3].name, self.race + 1, 4));
    }
    fn add_gold_races(&mut self) {
        self.rank_players();
        self.races.push(Race::new(&self.names[0].name, &self.names[1].name, self.race + 1, 4));
    }
    fn clear_player_scores(&mut self) {
        for name in self.names.iter_mut() {
            name.score = 0;
        }
    }
    pub fn next_stage(&mut self) -> bool {
        if self.stage == GROUP || self.stage == TIEBREAKER {
            if !self.cull_player() {
                self.stage = TIEBREAKER;
                self.add_tiebreaker_races();
                return true;
            } 
        }
        let is_there_a_next_stage = self.stage.next_stage();
        if is_there_a_next_stage {
            match self.stage {
                SEMI => {
                    self.clear_player_scores();
                    self.add_semi_races();
                },
                BRONZE => {
                    self.add_bronze_races();
                    for i in 0..2 {
                        self.names[i].score = 10;
                    }
                },
                GOLD => {
                    self.add_gold_races();
                },
                GROUP | TIEBREAKER => unreachable!()
            }
        }
        is_there_a_next_stage
    }
    pub fn next_race(&mut self) -> bool {
        self.race += 1;
        self.races.remove(0);
        self.races.len() != 0
    }
    pub fn record_winner(&mut self, winner: bool) {
        let name = if winner {
            &self.races[0].racer_1
        } else {
            &self.races[0].racer_2
        };
        for player in self.names.iter_mut() {
            if &player.name == name {
                player.score += 1;
            }
        }
    }
    fn cull_player(&mut self) -> bool {
        let original_length = self.names.len();
        for name in self.rank_players().iter_mut() {
            if name.rank == RACERS {
                self.dead_player = self.names.pop();
            }
        }
        original_length != self.names.len()
    }
}

pub struct Display {
    pub tournament: Tournament,
    pub refresh: Button,
    ranks: Vec<Label>,
    pub win_button_1: Button,
    pub win_button_2: Button,
    race: Label,
    vs: Label,
    ccs: Label,
    current_races: Label,
    stage: Label,
}

impl Display {
    pub fn new(builder: Builder, tournament: Tournament) -> Self {
        let mut ranks = vec![];
        for i in 0..RACERS {
            ranks.push(builder.get_object(&format!("rank_{}", i)).unwrap());
        }
        Display {
            tournament,
            refresh: builder.get_object("refresh").unwrap(),
            ranks,
            win_button_1: builder.get_object("win_button_1").unwrap(),
            win_button_2: builder.get_object("win_button_2").unwrap(),
            race: builder.get_object("race").unwrap(),
            vs: builder.get_object("vs").unwrap(),
            ccs: builder.get_object("ccs").unwrap(),
            current_races: builder.get_object("current_races").unwrap(),
            stage: builder.get_object("stage").unwrap(),
        }
    }
    pub fn display_race(&self) {
        let race = self.tournament.race;
        let racer_1 = self.tournament.races[0].racer_1.clone();
        let racer_2 = self.tournament.races[0].racer_2.clone();
        self.win_button_1.set_label(&format!("{} won", racer_1));
        self.win_button_2.set_label(&format!("{} won", racer_2));
        self.vs.set_text(&format!("{} vs {}", racer_1, racer_2));
        self.current_races.set_text(&format!(
            "{} Race(s)", self.tournament.races[0].length
        ));
        self.race.set_text(&format!("Match {}:", race));
    }
    pub fn display_stage(&self) {
        let stage = self.tournament.stage.clone();
        self.ccs.set_text(&stage.get_ccs());
        self.stage.set_text(&stage.to_string());
    }
    pub fn display_ranks(&mut self) {
        for i in 0..self.tournament.names.len() {
            let mut win = String::new();
            if self.tournament.stage == GROUP {
                win = format!("({} wins)", self.tournament.names[i].score);
            }
            self.ranks[i].set_text(&format!(
                "#{}: {} {}",
                self.tournament.names[i].rank, self.tournament.names[i].name, win
            ));
        }
        if let Some(player) = self.tournament.dead_player.take() {
            self.ranks[RACERS - 1].set_text(&format!("#{}: {}", RACERS, player.name));
        }
    }
}
