use crate::agents::cls;
use crate::conf::{LlmConfig, MainConfig};
use crate::database::{
  ToEval, find_vec_store, get_rows_for_eval, update_eval_status,
};
use crate::schemas::*;
use futures::future::join_all;
use tokio_postgres::Client;

pub async fn eval(
  a_cli: &Client,
  a_conf_emb: &LlmConfig,
  a_conf_model: &LlmConfig,
  a_conf_main: &MainConfig,
) -> Vec<ToEval>
{
  let mut to_eval = get_rows_for_eval(a_cli)
    .await
    .expect("Could not retrieve data from db");

  let mut tasks = Vec::new();
  for itm in to_eval.iter() {
    let sim = find_vec_store(
      a_cli,
      &itm.descr,
      a_conf_emb,
      a_conf_main.top_n,
    )
    .await
    .expect("Could not make vector search");

    let smpls = sim.join("\n");
    let prompt =
      format!("DESCRIPTION: {}\nSAMPLES:\n{}", &itm.descr, smpls);

    tasks.push(prompt);
  }

  let mut results = Vec::new();
  for ch in tasks.chunks(50) {
    let mut tts = Vec::new();
    for s in ch {
      tts.push(cls(s.clone(), a_conf_model));
    }

    for t in join_all(tts).await {
      results.push(t);
    }
  }

  for (eval, res_) in to_eval.iter_mut().zip(results) {
    let out = res_;

    match out {
      Ok(res) => {
        match res {
          TypeT::NoEval => {
            tracing::warn!("Could not evaluate item {}", &eval.id);
          }
          _ => {}
        }
        eval.eval = res;
        update_eval_status(a_cli, &eval)
          .await
          .expect("Could not update the entries");
      }
      Err(e) => {
        tracing::error!(
          " Could not eval id: {} err: {:?}",
          eval.id,
          e
        );
      }
    }
  }

  return to_eval;
}
