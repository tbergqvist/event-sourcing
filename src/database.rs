use mysql::prelude::Queryable;
use mysql::{Pool};
use crate::error::Result;

pub struct Database {
    connection_pool: Pool
}

pub struct EventRow {
  pub id: i64,
	pub json_data: String,
}

impl Database {
	pub fn connect() -> Result<Database> {
    let url = mysql::Opts::from_url("mysql://root:asdf@localhost:3306/event_sourcing").expect("Floppy config");
    let connection_pool = mysql::Pool::new(url)?;
    Ok(Database{connection_pool})
	}

  pub fn get_events_for_aggregate(&self, aggregate_id: u64) -> Result<Vec<EventRow>> {
    let mut conn = self.connection_pool.get_conn()?;
    let statement = conn.prep("SELECT id, json_data from event_log where aggregate_id = ?")?;
    let events = conn.exec_map(&statement, (aggregate_id,), |(id, json_data)| {
      EventRow { id, json_data }
    })?;
    Ok(events)
  }

  pub fn insert_events(&self, expected_version: u64, events: Vec<EventRow>) -> Result<()> {
    //TODO
    Ok(())
  }
}