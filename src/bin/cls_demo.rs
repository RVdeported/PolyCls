use poly_cls::cls::*;
use poly_cls::conf::*;
use poly_cls::database::*;
use poly_cls::schemas::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    // .with_target(false)
    .init();

  // Load configuration
  let config: TendConfig =
    load_config(&"configs/test.ini".to_string())?;
  let llm_config = load_llm_config(&"configs/test.ini".to_string())?;

  // Connect to database using config details
  let client = connect_to_database(
    &config.postgres.host,
    &config.postgres.user,
    &config.postgres.psswd,
  )
  .await?;

  client
    .execute("DROP TABLE IF EXISTS general_schema;", &[])
    .await?;

  // Initialize the GeneralSchema table
  init_general_schema_table(&client).await?;

  // Read sample data from CSV file
  let ps = [
    (PathBuf::from("samples/kz_customs_2.csv"), SegmentT::Kz),
    (PathBuf::from("samples/eas_customs_2.csv"), SegmentT::Eas),
    (PathBuf::from("samples/rus_customs_2.csv"), SegmentT::Rus),
  ];

  let mut done: u8 = 0;
  for (path, seg) in ps {
    let v = GeneralSchema::from_csv_file_path(&path, seg)?;

    // Insert all records
    for schema in &v {
      let rows_affected =
        insert_general_schema(&client, schema).await?;
      println!("Inserted {} row(s)", rows_affected);
      done += 1;
    }
  }

  let res =
    eval(&client, &llm_config[1], &llm_config[0], &config.main).await;

  for r in res {
    println!("{:?}", r);
  }

  Ok(())
}
