use serde::Deserialize;
use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Freelancer,
    Client,
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
