use poly_cls::cls::*;
use poly_cls::conf::*;
use poly_cls::database::*;
use poly_cls::imports::*;
use poly_cls::schemas::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
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

  // Initialize the GeneralSchema table
  init_general_schema_table(&client, true).await?;
  init_vector_storage_data(&client, true).await?;

  // Read sample data from CSV file
  let ps = [
    // (PathBuf::from("samples/kz_customs_1.csv"), SegmentT::Kz),
    // (PathBuf::from("samples/eas_customs_1.csv"), SegmentT::Eas),
    (PathBuf::from("samples/rus_customs_4.csv"), SegmentT::Rus),
  ];

  let mut done: usize = 0;
  for (path, seg) in ps {
    done += read_file(&path, seg, &client, &llm_config[1]).await?;
  }
  tracing::info!("Red total of {} rows", done);

  // Randomly choose pos for eval
  for t in enum_iterator::all::<TypeT>() {
    if t.clone() as u8 == TypeT::NoEval as u8 {
      continue;
    }
    client.batch_execute(format!("
      UPDATE general_schema SET eval='NoEval' WHERE id IN (
        SELECT id FROM general_schema WHERE eval='{:?}' ORDER BY RANDOM() LIMIT 4
      );", t).as_str()).await?;
  }

  let res =
    eval(&client, &llm_config[1], &llm_config[0], &config.main).await;

  let mut ok: usize = 0;
  for r in res.iter() {
    let q = client
      .query(
        format!(
          "
      SELECT eval_sec FROM general_schema WHERE id={}",
          r.id
        )
        .as_str(),
        &[],
      )
      .await?;
    assert!(q.len() == 1);
    let st: String = q[0].get(0);
    let real_t = TypeT::from_str(&st);

    if real_t.clone() as u8 == r.eval.clone() as u8 {
      ok += 1;
    }

    println!("{}:\n{:?} - {:?}", r.descr, r.eval, real_t);
  }

  println!(
    "Total acc: {}\\{}, {}%",
    ok,
    res.len(),
    ok * 100 / res.len()
  );

  Ok(())
}
