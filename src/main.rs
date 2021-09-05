use aggregates::{Affiliate, Trade, get_aggregate};
use database::EventRow;
use error::{Error, Result};

mod database;
mod aggregates;
mod error;

#[derive(Debug, Default)]

struct EventEntry<T> {
  value: T,
  start_date: u64,
}

#[derive(Debug, Default)]
struct History<T> {
  event_list: Vec<EventEntry<T>>
}

impl<T> History<T> {
  fn now(&self) -> Option<&T> {
    self.at(100)
  }

  fn at(&self, time: u64) -> Option<&T>{
    self.event_list.iter()
      .filter(|entry| entry.start_date <= time)
      .map(|entry| &entry.value).next_back()
  }

  fn insert_at(&mut self, start_date: u64, value: T) {
    self.event_list.retain(|entry| entry.start_date < start_date);
    self.event_list.push(EventEntry {
      value,
      start_date
    });
  }
}

fn run() -> Result<()> {
  let database = database::Database::connect()?;

  let trade: Trade = get_aggregate(1, &database)?;
  let mut affiliate: Affiliate = get_aggregate(2, &database)?;

  let affiliate = affiliate.set_commission_plan_at(10, 0, &database);
  
  Ok(())
}

fn main() {
  match run() {
    Ok(_) => println!("everything is fine!"),
    Err(e) => println!("everything is bad! {}", e)
  }
}