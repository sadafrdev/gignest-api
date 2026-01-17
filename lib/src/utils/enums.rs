use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "language_level")]
pub enum LanguageLevel {
    BEGINNER,
    INTERMEDIATE,
    FLUENT,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "language")]
pub enum Language {
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
