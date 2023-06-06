#![allow(dead_code)]
// #![allow(unused_imports)]

use std::collections::HashMap;

use crate::bit_constant;

#[derive(Debug)]
pub struct Effect {
    pub to_index: usize,

    pub score: i32,
    pub frequency: i32,
}

#[derive(Debug)]
pub struct MoveEffect {
    pub from_index: usize,
    pub effects: Vec<Effect>,
}

#[derive(Debug)]
pub struct HashlockMoveEffect {
    pub hashlock: u64,
    pub move_effects: Vec<MoveEffect>,
}

#[derive(Debug)]
pub struct HistoryMoveEffect {
    pub history: HashMap<u64, HashlockMoveEffect>,
}

// 后期根据需要扩展
impl Effect {
    pub fn new(to_index: usize, score: i32, frequency: i32) -> Effect {
        Effect {
            to_index,
            score,
            frequency,
        }
    }

    pub fn to_string(&self) -> String {
        let coord = bit_constant::Coord::from_index(self.to_index).unwrap();
        let row = coord.row;
        let col = coord.col;
        let score = self.score;
        let fre = self.frequency;
        format!("({},{})-{score}-{fre} ", row, col)
    }
}

impl MoveEffect {
    pub fn new(from_index: usize) -> MoveEffect {
        MoveEffect {
            from_index,
            effects: Vec::new(),
        }
    }

    pub fn add(&mut self, to_index: usize, score: i32, frequency: i32) {
        self.effects.push(Effect::new(to_index, score, frequency));
    }

    pub fn to_string(&self) -> String {
        let coord = bit_constant::Coord::from_index(self.from_index).unwrap();
        let row = coord.row;
        let col = coord.col;
        let mut result = format!("[{},{}] => ", row, col);
        for effect in self.effects.iter() {
            result.push_str(&effect.to_string());
        }
        result.push_str(&format!("【{}】\n", self.effects.len()));

        result
    }
}

impl HashlockMoveEffect {
    pub fn new() -> HashlockMoveEffect {
        HashlockMoveEffect {
            hashlock: 0,
            move_effects: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for move_effect in self.move_effects.iter() {
            result.push_str(&move_effect.to_string());
        }
        result.push_str(&format!(
            "move_effect_len:【{}】\n",
            self.move_effects.len()
        ));

        result
    }
}

impl HistoryMoveEffect {
    pub fn get_move_effect(&self, mut hashkey: u64, hashlock: u64) -> Option<&Vec<MoveEffect>> {
        let mut index = 0;
        while self.history.contains_key(&hashkey) {
            let option_hashlock_move_effect = self.history.get(&hashkey);
            match option_hashlock_move_effect {
                None => return None,
                Some(hashlock_move_effect) => {
                    if hashlock_move_effect.hashlock == hashlock {
                        return Some(&hashlock_move_effect.move_effects);
                    }
                }
            };

            if index == bit_constant::COLORCOUNT {
                assert!(false, "重复碰撞最多允许2次!");
                return None;
            }

            hashkey ^= bit_constant::COLLIDEZOBRISTKEY[index];
            assert!(false, "hashlock is not same! index:{index}\n");

            index += 1;
        }

        None
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (hashkey, hashlock_move_effect) in self.history.iter() {
            result.push_str(&format!(
                "hashkey:{:016x}\nmove_effect:\n{}\n",
                hashkey,
                hashlock_move_effect.to_string()
            ));
        }
        result.push_str(&format!("history_len:【{}】\n", self.history.len()));

        result
    }
}