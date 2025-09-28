use poly_cls::agents::*;
use poly_cls::conf::*;
use poly_cls::database::*;
use poly_cls::schemas::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  // Load configuration
  let config: TendConfig =
    load_config(&"configs/test.ini".to_string())?;

  let llm_configs = load_llm_config(&"configs/test.ini".to_string())?;

  // Connect to database using config details
  let client = connect_to_database(
    &config.postgres.host,
    &config.postgres.user,
    &config.postgres.psswd,
  )
  .await?;

  let file_path = PathBuf::from("samples/eas_customs_2.csv");

  init_vector_storage_data(&client).await?;

  let rows =
    GeneralSchema::from_csv_file_path(&file_path, SegmentT::Eas)?;

  for row in rows {
    insert_vec_store(&client, &row, &llm_configs[1]).await?;
  }

  let search = find_vec_store(
    &client,
    &"пенополистирол".to_string(),
    &llm_configs[1],
    2,
  )
  .await?;

  println!("{:#?}", search);

  Ok(())
}
