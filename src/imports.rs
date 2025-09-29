use crate::conf::LlmConfig;
use crate::database::{
  insert_general_schema, insert_vec_store, parse_date,
};
use crate::schemas::*;
use chrono::NaiveDate;
use csv::Reader;
use futures_batch::ChunksTimeoutStreamExt;
use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use tokio_postgres::Client;

pub async fn read_file(
  a_path: &PathBuf,
  a_seg: SegmentT,
  a_cli: &Client,
  a_conf_embed: &LlmConfig,
) -> Result<usize, Box<dyn Error>>
{
  let date_res = a_cli.query("SELECT effective_date FROM general_schema WHERE seg = $1 ORDER BY effective_date DESC LIMIT 1", 
    &[&format!("{:?}", a_seg)]).await?;

  let date: NaiveDate;

  if date_res.len() == 0 {
    date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
  } else {
    date = date_res[0].get(0);
  }

  let v = GeneralSchema::from_csv_file_path(a_path, a_seg)?;

  let filtered: Vec<GeneralSchema> = v
    .into_iter()
    .filter(|x| {
      parse_date(x.effective_date.clone().unwrap().as_str()).unwrap()
        > date
    })
    .collect();

  //------------------------------------//
  // Store new values                   //
  //------------------------------------//
  for val in filtered.iter() {
    insert_general_schema(a_cli, val).await?;
  }

  //------------------------------------//
  // Store new vector items             //
  //------------------------------------//
  let only_known: Vec<GeneralSchema> = filtered
    .into_iter()
    .filter(|x| x.eval.clone().unwrap() != "NoEval")
    .collect();

  for vecs in only_known.chunks(500) {
    let mut tasks = Vec::new();
    for vec in vecs {
      tasks.push(insert_vec_store(a_cli, vec, a_conf_embed));
    }

    for r in futures::future::join_all(tasks).await {
      r?;
    }
  }

  Ok(only_known.len())
}
