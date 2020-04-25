use Stage::*;

use gtk::prelude::*;
use gtk::{Builder, Button, Label};

const RACERS: usize = 5;

#[derive(Clone, PartialEq)]
enum Stage {
    GROUP,
    SEMI,
    BRONZE,
    GOLD,
}
impl Stage {
    fn to_string(&self) -> String {
        match self {
            GROUP => "Group Stage".to_string(),
            SEMI => "Semifinal Stage".to_string(),
            BRONZE => "Bronze Medal Match".to_string(),
            GOLD => "Gold Medal Match".to_string(),
        }
    }
    fn get_ccs(&self) -> String {
        match self {
            GROUP => "100 ccs".to_string(),
            SEMI => "150 ccs".to_string(),
            BRONZE => "150 ccs".to_string(),
            GOLD => "200 ccs".to_string(),
        }
    }
    fn get_number_of_races(&self) -> usize {
        match self {
            GROUP => 1,
            SEMI => 4,
            BRONZE => 4,
            GOLD => 4,
        }
    }
    fn next_stage(&mut self) -> bool {
        match self {
            GROUP => {
                *self = SEMI;
                true
            },
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
struct Player {
    name: String,
    score: usize,
    rank: usize,
}
impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            score: 0,
            rank: 0,
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
        Tournament {
            stage: GROUP,
            names,
            race: 1,
            races,
            over: false,
        }
    }
    fn rank_players(&mut self) -> Vec<Player> {
        self.names.sort_by(|a, b| a.score.cmp(&b.score));
        self.names.reverse();
        let mut i = 0;
        let mut last_score = RACERS + 1;
        let mut last_rank = 0;
        self.names
            .iter_mut()
            .map(|a| {
                i += 1;
                if a.score == last_score {
                    a.rank = last_rank;
                } else {
                    a.rank = i;
                    last_score = a.score;
                    last_rank = a.rank;
                }
                a.clone()
            })
            .collect()
    }
    fn add_semi_races(&mut self) {
        self.rank_players();
        let mut race = self.race + 1;
        for j in 0..2 {
            self.races.push(Race::new(&self.names[j].name, &self.names[3 - j].name, race, 4));
            race += 1;
        }
    }
    fn add_tiebreaker_races(&mut self) {

    }
    fn add_bronze_races(&mut self) {

    }
    fn add_gold_races(&mut self) {

    }
    fn clear_player_scores(&mut self) {
        for name in self.names.iter_mut() {
            name.score = 0;
        }
    }
    pub fn next_stage(&mut self) -> bool {
        if self.stage == GROUP {
            if !self.cull_player() {
                self.add_tiebreaker_races();
                return false;
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
                    self.clear_player_scores();
                    self.add_bronze_races();
                    for i in 0..2 {
                        self.names[i].score = 10;
                    }
                },
                GOLD => {
                    self.clear_player_scores();
                    self.add_gold_races();
                },
                GROUP => unreachable!()
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
        self.names = self
            .rank_players()
            .iter_mut()
            .filter(|a| a.rank != RACERS)
            .map(|a| a.clone())
            .collect();
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
    builder: Builder,
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
            builder,
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
        self.tournament.rank_players();
        let mut start_range = 0;
        let mut end_range = self.tournament.names.len();
        if self.tournament.stage == BRONZE {
            start_range = 2;
            for i in 0..2 {
                self.ranks[i].set_text(&format!("#{}: {}", self.tournament.names[i].rank, self.tournament.names[i].name));
            }
        } else if self.tournament.stage == GOLD {
            end_range = 2;
        }
        for i in start_range..end_range {
            self.ranks[i].set_text(&format!(
                "#{}: {} ({} wins)",
                self.tournament.names[i].rank, self.tournament.names[i].name,
                self.tournament.names[i].score
            ));
        }
    }
}
