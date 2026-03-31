use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "language_level")]
pub enum LanguageLevel {
    Begginer,
    Intermediate,
    Fluent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "language")]
pub enum LanguageEnum {
    English,
    Urdu,
    Spanish,
    Chinese,
    Korean,
    French,
    Russian,
    Germany,
    Arabic,
    Hindi,
    Persian,
    Turkish,
    Bengali,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "country", rename_all = "lowercase")]
pub enum Country {
    US,
    CA,
    GB,
    AU,
    DE,
    FR,
    IN,
    JP,
    CN,
    BR,
    ZA,
    NG,
    KE,
    EG,
    MX,
    PK,
    RU,
    IT,
    ES,
    NL,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "job_type")]
pub enum JobType {
    Fixed,
    Hourly,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "proposal_status")]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Type, Deserialize, Serialize, PartialEq, Eq)]
#[sqlx(type_name = "user_role")]
pub enum Role {
    Freelancer,
    Client,
}
