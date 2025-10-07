use chrono::NaiveDate;
use clap::Parser;
use poly_cls::cls;
use poly_cls::conf::*;
use poly_cls::database::*;
use poly_cls::imports::*;
use poly_cls::schemas::*;
use std::path::PathBuf;
use std::str::FromStr;

fn to_absolute_path(
  path: PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>>
{
  if path.is_absolute() {
    Ok(path.to_path_buf())
  } else {
    let current_dir = env::current_dir()?;
    Ok(current_dir.join(path))
  }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli
{
  #[arg(short, long)]
  start_d: Option<NaiveDate>,

  #[arg(short, long)]
  end_d: Option<NaiveDate>,

  #[arg(long)]
  seg: Option<SegmentT>,

  #[arg(short, long)]
  out_path: Option<PathBuf>,
}

#[tokio::main]
async fn main()
{
  let args = Cli::parse();

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

  let start_date = args
    .start_d
    .unwrap_or(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
  let end_date = args
    .end_d
    .unwrap_or(NaiveDate::from_ymd_opt(2100, 1, 1).unwrap());

  let mut query = format!(
    "SELECT * FROM general_schema WHERE effective_date BETWEEN '{:?}' AND '{:?}' ",
    start_date, end_date
  );

  if args.seg.is_some() {
    query = format!("{} AND seg='{:?}'", query, args.seg.unwrap());
  }

  let path = args.out_path.unwrap_or(PathBuf::from("out.csv"));
  let abs_path =
    to_absolute_path(path).expect("Could not get absolute path");

  query = format!(
    "COPY ({}) TO '{}' DELIMITER ',' CSV HEADER",
    query,
    abs_path.to_str().unwrap()
  );

  println!("{}", query);

  client
    .batch_execute(query.as_str())
    .await
    .expect("Could not execute postrgres");
}
