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
use std::path::PathBuf;

use crate::conf::{TendConfig, load_config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
  let conf = load_llm_config(&"./configs/test.ini".to_string())?;

  let test = VecItem {
    description: "A sample description".to_string(),
    eval: "EVAL".to_string(),
  };

  let res = embed_item(&test, &conf[1]).await?;

  println!("{:#?}", res);
  Ok(())
}
