use Stage::*;

use gtk::prelude::*;
use gtk::{Builder, Button, Label, Separator};

const RACERS: usize = 5;

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
}

#[derive(Clone)]
struct Player {
    name: String,
    score: usize,
    rank: usize
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

pub struct Tournament {
    stage: Stage,
    names: Vec<Player>,
}
impl Tournament {
    pub fn new() -> Self {
        Tournament {
            stage: GROUP,
            names: vec![
                Player::new("Justus"),
                Player::new("Faith"),
                Player::new("Hope"),
                Player::new("Mommy"),
                Player::new("Daddy"),
            ],
        }
    }
    fn rank_players(&mut self) -> Vec<Player> {
        self.names.sort_by(|a, b| a.score.cmp(&b.score));
        let mut i = 0; 
        let mut last_score = RACERS + 1;
        let mut last_rank = 0;
        self.names.iter_mut().map(|a| {
            i += 1;
            if a.score == last_score {
                a.rank = last_rank;
            } else {
                a.rank = i;
                last_score = a.score;
                last_rank = a.rank;
            }
            a.clone()
        }).collect()
    }
    fn cull_player(&mut self) -> bool {
        let original_length = self.names.len();
        self.names = self.rank_players().iter_mut().filter(|a| {a.rank != RACERS}).map(|a| a.clone()).collect();
        original_length != self.names.len()
    }
}

pub struct Display {
    tournament: Tournament,
    refresh: Button,
    ranks: Vec<Label>,
    separators: Vec<Separator>,
    win_button_1: Button,
    win_button_2: Button,
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
        let mut separators = vec![];
        for i in 1..RACERS {
            separators.push(builder.get_object(&format!("separator_{}", i)).unwrap());
        }
        Display {
            tournament,
            refresh: builder.get_object("refresh").unwrap(),
            ranks,
            separators,
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
    pub fn display_ranks(&mut self) {
        self.tournament.rank_players();
        for i in 0..RACERS {
            self.ranks[i].set_text(&format!("#{}: {}", self.tournament.names[i].rank, self.tournament.names[i].name));
        }
    }
    pub fn connect_refresh(&self) {
        self.refresh.connect_clicked(|_| {
            //Refresh
        });
    }
    pub fn connect_buttons_clicked(&self) {
        self.win_button_1.connect_clicked(|_| {

        });
        self.win_button_2.connect_clicked(|_| {

        });
    }
}
