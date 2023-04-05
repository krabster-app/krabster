use super::database::DatabaseConfiguration;
use super::providers::ProvidersConfiguration;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Configuration {
    database: DatabaseConfiguration,
    providers: ProvidersConfiguration,
}
