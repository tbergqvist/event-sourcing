use crate::{History, error::{Error, Result}};

use super::Aggregate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum TradeEvent {
  ProfitChanged(f64, u64)
}

#[derive(Debug, Default)]
pub struct Trade {
  id: u64,
  profit: History<f64>,
  trader_id: History<u32>,
}

impl Aggregate for Trade {
  type EventType = TradeEvent;

  fn update(&mut self, event: TradeEvent) {
    match event {
      TradeEvent::ProfitChanged(profit, start_date) => {
        self.profit.insert_at(start_date, profit)
      }
    }
  }
}