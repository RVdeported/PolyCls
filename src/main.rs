mod conf;
mod database;
mod imports;
mod schemas;

use crate::conf::*;
use crate::database::*;
use crate::schemas::GeneralSchema;
use crate::schemas::*;
use std::path::PathBuf;

use crate::conf::{TendConfig, load_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  // Load configuration
  let config: TendConfig =
    load_config(&"configs/test.ini".to_string())?;

  // Read sample data from CSV file
  let p = PathBuf::from("samples/kz_customs_2.csv");
  let v = GeneralSchema::from_csv_file_path(&p, SegmentT::Kz)?;

  // Connect to database using config details
  let client = connect_to_database(
    &config.postgres.host,
    &config.postgres.user,
    &config.postgres.psswd,
  )
  .await?;

  // Initialize the GeneralSchema table
  init_general_schema_table(&client).await?;

  // Insert all records
  for schema in &v {
    let rows_affected =
      insert_general_schema(&client, schema).await?;
    println!("Inserted {} row(s)", rows_affected);
  }

  println!("Successfully processed {} records", v.len());

  Ok(())
}
