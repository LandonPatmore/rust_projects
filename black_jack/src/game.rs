use crate::player::Player;
use crate::table::Table;

pub struct Game<'a> {
  table: Table<'a>,
  current_turn: Player<'a>,
}