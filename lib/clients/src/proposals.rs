use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utils::enums::{JobType, ProposalStatus};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    id: i32,
    freelancer_id: i32,
    cover_letter: String,
    job_id: i32,
    bid_amount: f64,
    job_type: JobType,
    status: ProposalStatus,
}
