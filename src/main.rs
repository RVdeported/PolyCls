pub mod agents;
pub mod conf;
pub mod database;
pub mod imports;
pub mod schemas;

use crate::agents::embed_item;

use crate::conf::*;
use crate::database::*;
use crate::schemas::GeneralSchema;
use crate::schemas::*;
use csv::Reader;
use std::path::PathBuf;

use crate::conf::{TendConfig, load_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    // .with_target(false)
    .init();

  // Load configuration
  let config: TendConfig =
    load_config(&"configs/test.ini".to_string())
      .expect("Wrong llm config");

  // Connect to database using config details
  let client = connect_to_database(
    &config.postgres.host,
    &config.postgres.user,
    &config.postgres.psswd,
  )
  .await
  .expect("Postgres error");

  let path = PathBuf::from("samples/eas_dates.csv");

  let mut queries = String::new();
  queries.reserve(22790 * 120);

  let f = std::fs::File::open(path)?;
  let mut r = Reader::from_reader(f);

  for row_packed in r.records() {
    let row = row_packed?;
    let id = row.get(0).unwrap();
    let date_str = row.get(1).unwrap();
    let date = parse_date(date_str).unwrap();

    queries.push_str(
      format!(
        "UPDATE general_schema SET effective_date='{:?}' \
      WHERE declaration_number='{}' AND seg='Eas'; ",
        date, id
      )
      .as_str(),
    );
  }

  client.batch_execute(&queries.as_str()).await?;

  Ok(())
}
