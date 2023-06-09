#![allow(dead_code)]

// use serde_derive::Deserialize;
// use serde_derive::Serialize;

use crate::board;
// use crate::piece;
// use crate::bit_constant;
// use std::borrow::Borrow;
// use std::borrow::Borrow;
// use crate::common;
use crate::coord;
use crate::piece;
// use crate::coord::CoordPair;
// use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

#[derive(Debug)] //, Serialize, Deserialize
pub struct Move {
    before: Option<Weak<Move>>,
    after: RefCell<Option<Vec<Rc<Move>>>>,

    pub coordpair: coord::CoordPair,
    remark: RefCell<Option<String>>,

    to_piece: RefCell<piece::Piece>,
}

impl Move {
    pub fn root() -> Rc<Self> {
        Rc::new(Move {
            before: None,
            after: RefCell::new(None),

            coordpair: coord::CoordPair::new(),
            remark: RefCell::new(None),

            to_piece: RefCell::new(piece::Piece::None),
        })
    }

    pub fn is_root(&self) -> bool {
        self.before.is_none()
    }

    pub fn before(&self) -> Option<Rc<Self>> {
        match &self.before {
            None => None,
            Some(before) => Some(before.upgrade().unwrap()),
        }
    }

    pub fn after_len(&self) -> usize {
        self.after.borrow().as_ref().unwrap_or(&vec![]).len()
    }

    pub fn after(&self) -> Vec<Rc<Self>> {
        self.after.borrow().clone().unwrap_or(vec![])
    }

    pub fn remark(&self) -> String {
        self.remark.borrow().clone().unwrap_or(String::new())
    }

    pub fn set_remark(&self, remark: String) {
        if !remark.is_empty() {
            *self.remark.borrow_mut() = Some(remark);
        }
    }

    pub fn append(self: &Rc<Self>, coordpair: coord::CoordPair, remark: String) -> Rc<Self> {
        let amove = Rc::new(Self {
            before: Some(Rc::downgrade(self)),
            after: RefCell::new(None),

            coordpair,
            remark: RefCell::new(if remark.is_empty() {
                None
            } else {
                Some(remark)
            }),

            to_piece: RefCell::new(piece::Piece::None),
        });

        self.after
            .borrow_mut()
            .get_or_insert(Vec::new())
            .push(amove.clone());

        amove
    }

    pub fn get_to_piece(&self) -> piece::Piece {
        *self.to_piece.borrow()
    }

    pub fn set_to_piece(&self, piece: piece::Piece) {
        *self.to_piece.borrow_mut() = piece;
    }

    pub fn before_moves(self: &Rc<Self>) -> Vec<Rc<Self>> {
        let mut before_moves = Vec::new();
        let mut amove = self.before().unwrap();
        while !amove.is_root() {
            before_moves.insert(0, amove.clone());
            amove = amove.before().unwrap();
        }
        // before_moves.reverse();

        before_moves
    }

    pub fn to_string(
        self: &Rc<Self>,
        record_type: coord::RecordType,
        board: &board::Board,
    ) -> String {
        let coordpair_string = if self.is_root() {
            String::new()
        } else {
            if record_type == coord::RecordType::PgnZh {
                if self.is_root() {
                    String::new()
                } else {
                    board
                        .to_move_before(self)
                        .get_zhstr_from_coordpair(&self.coordpair)
                }
            } else {
                self.coordpair.to_string(record_type)
            }
        };

        let mut remark = self.remark();
        if !remark.is_empty() {
            remark = format!("{{{}}}", remark);
        }

        let num = self.after_len();
        let after_num = if num > 0 {
            format!("({})", num)
        } else {
            String::new()
        };

        format!("{}{}{}\n", coordpair_string, remark, after_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amove() {
        let root_move = Move::root();

        let from_coord = coord::Coord::from(0, 0).unwrap();
        let to_coord = coord::Coord::from(0, 2).unwrap();
        let coordpair = coord::CoordPair::from(from_coord, to_coord);
        let remark = String::from("Hello, move.");
        let amove = root_move.append(coordpair, remark);
        let board = board::Board::new();

        assert_eq!(
            "(0,0)(0,2){Hello, move.}\n",
            amove.to_string(coord::RecordType::Txt, &board)
        );
    }
}
