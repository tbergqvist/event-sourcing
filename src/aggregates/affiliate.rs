use crate::{History, database::{Database, EventRow}, error::{Error, Result}};

use super::Aggregate;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
struct CommissionPlan {
  value: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommissionPlanChanged {
  value: u8,
  start_date: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AffiliateEvent {
  CommissionPlanChanged(CommissionPlanChanged)
}

#[derive(Debug, Default)]
pub struct Affiliate {
  id: u64,
  version: u64,
  commission_plan: History<CommissionPlan>,
}

impl Affiliate {
  pub fn set_commission_plan_at(&mut self, new_plan: u8, start_date: u64, database: &Database) -> Result<&mut Self> {
    let change = AffiliateEvent::CommissionPlanChanged(CommissionPlanChanged { value: new_plan, start_date });
    
    database.insert_events(self.version + 1, vec![EventRow { id: 0, json_data: serde_json::to_string(&change)? }])?;
    self.update(change);
    self.version += 1;
    Ok(self)
  }
}

impl Aggregate for Affiliate {
  type EventType = AffiliateEvent;

  fn update(&mut self, event: AffiliateEvent) {
    match event {
      AffiliateEvent::CommissionPlanChanged(commission_plan) => {
        self.commission_plan.insert_at(commission_plan.start_date, CommissionPlan{ value: commission_plan.value });
      }
    }
  }
}