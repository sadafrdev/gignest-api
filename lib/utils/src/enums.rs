use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "language_level")]
pub enum LanguageLevel {
    BEGINNER,
    INTERMEDIATE,
    FLUENT,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "language")]
pub enum LanguageEnum {
    ENGLISH,
    URDU,
    SPANISH,
    CHINESE,
    KOREAN,
    FRENCH,
    RUSSIAN,
    GERMAN,
    ARABIC,
    HINDI,
    PERSIAN,
    TURKISH,
    BENGALI,
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
