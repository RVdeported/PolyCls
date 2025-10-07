use poly_cls::cls;
use poly_cls::conf::*;
use poly_cls::database::*;
use poly_cls::imports::*;
use poly_cls::schemas::*;
use std::path::PathBuf;
use std::str::FromStr;

#[tokio::main]
async fn main()
{
  let args: Vec<String> = std::env::args().collect();

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    // .with_target(false)
    .init();

  // Load configuration
  let config: TendConfig =
    load_config(&"configs/test.ini".to_string())
      .expect("Wrong llm config");
  let llm_config = load_llm_config(&"configs/test.ini".to_string())
    .expect("Wrong llm config");

  // Connect to database using config details
  let client = connect_to_database(
    &config.postgres.host,
    &config.postgres.user,
    &config.postgres.psswd,
  )
  .await
  .expect("Postgres error");

  // Initialize the GeneralSchema table
  init_general_schema_table(&client, false)
    .await
    .expect("Could not init schema general");
  init_vector_storage_data(&client, false)
    .await
    .expect("Could not init vector storage");

  for pack in args[1..].chunks_exact(2) {
    let path = pack[0].clone();
    let seg = pack[1].clone();

    let p = PathBuf::from(&path);
    let seg_t = SegmentT::from_str(seg.as_str());
    match seg_t {
      Err(_) => {
        tracing::error!("Could not recognize segment {}", seg);
        continue;
      }
      _ => {}
    }

    let res =
      read_file(&p, seg_t.expect(""), false, &client, &llm_config[1])
        .await;

    if res.is_ok() {
      tracing::info!("Ok read {} {}", path, seg);
    } else {
      tracing::error!(
        "Could not read {} {} {:?}",
        path,
        seg,
        res.err().unwrap()
      );
    }
  }

  let res =
    cls::eval(&client, &llm_config[1], &llm_config[0], &config.main)
      .await;

  println!("Totally evaled {} entries", res.len());
}
