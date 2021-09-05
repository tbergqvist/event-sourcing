pub mod trade;
pub mod affiliate;

pub use trade::Trade;
pub use affiliate::Affiliate;

use crate::{database, error::{Result}};
use serde::{de::DeserializeOwned};

pub trait Aggregate {
  type EventType: DeserializeOwned;
  fn update(&mut self, event: Self::EventType);
}

pub fn get_aggregate<T: Default + Aggregate>(
  aggregate_id: u64,
  database: &database::Database
) -> Result<T> {
  let events: Result<Vec<T::EventType>> = database.get_events_for_aggregate(aggregate_id)?
    .into_iter()
    .map(|row| serde_json::from_str(&row.json_data).map_err(|e|e.into()))
    .collect();

  let t = events?.into_iter()
    .fold(T::default(), |mut aggregate, event | {
      aggregate.update(event); 
      aggregate
    }
  );

  Ok(t)
}
