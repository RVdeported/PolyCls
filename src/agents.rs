use crate::conf::LlmConfig;
use crate::database::VecItem;
use crate::schemas::TypeT;
use rig::client::builder::DynClientBuilder;
use rig::completion::Prompt;
use rig::embeddings::Embedding;
// use rig_postgres::PostgresVectorStore;
// use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn embed_item(
  a_itm: &VecItem,
  a_conf: &LlmConfig,
) -> Result<Embedding, Box<dyn Error>>
{
  let itm = serde_json::to_string(a_itm)?;
  tracing::debug!("Embbedding {}", itm);
  let res = embed_str(&itm, a_conf).await?;

  Ok(res)
}

pub async fn embed_str(
  a_str: &String,
  a_conf: &LlmConfig,
) -> Result<Embedding, Box<dyn Error>>
{
  a_conf.set_env();
  let cli = DynClientBuilder::new()
    .embeddings(a_conf.protocol.to_str(), &a_conf.model_name)?;

  let res = cli.embed_text(a_str.as_str()).await?;

  return Ok(res);
}

pub async fn cls(
  a_itm: String,
  a_conf_model: &LlmConfig,
) -> Result<TypeT, Box<dyn Error>>
{
  a_conf_model.set_env();

  let res = DynClientBuilder::new()
    .agent(a_conf_model.protocol.to_str(), a_conf_model.model_name.as_str())?
    .preamble("\
    You are a construction products classifier. Your task is to read the \
    incoming product description with respective examples and classify the \
    description into one of the types: XPS, EPS, PSON, PSV, PIR, \
    GlassWool, StoneWool and Other.
    The categories can be described as follows:
    -- XPS (extruded polystyrene foam, extruded polystyrene foam, \
    extruded polystyrene foam) is a synthetic thermal insulation material
    -- EPS (expanded polystyrene, expanded polystyrene) is a lightweight, \
    rigid insulating material made from solid polystyrene particles.
    -- PSON (general purpose polystyrene, general purpose polystyrene) - \
    raw materials, in particular for XPS
    -- PSV (foaming polystyrene) - raw materials, in particular for EPS \
    -- PIR (Polyisocyanurate), a thermosetting polymer material with closed \
    cells with a sufficiently high degree of rigidity
    -- GlassWool is a fibrous mineral thermal insulation material, a \
    type of mineral wool
    -- StoneWool is basalt wool obtained from the melt by spraying on \
    rolls, thermal insulation material
    -- Other - category for not relevant and other materials
    Note that we are primerely interested in construction materials. \
    Thus, various decorating materials have to be classified as Other. \
    You will be given few examples of similar descriptions as well as evaluated \
    category. Use those for reference. \
    Answer ONLY with the selected category with no other descriptions or symbols.
    ")
    .max_tokens(100)
    .build()
    .prompt(a_itm)
    .await?;

  let c = remove_block(&res, "think".to_string());
  let out = TypeT::from_str(&c);

  Ok(out)
}

pub fn remove_block(a_in: &String, a_block: String) -> String
{
  let open = format!("<{}>", a_block);
  let cls = format!("</{}>", a_block);

  let mut out: String = "".to_string();
  let st = a_in.find(open.as_str());
  match st {
    Some(t) => {
      out.push_str(&a_in[0..t]);
    }
    _ => {
      return a_in.clone();
    }
  }

  let ed = a_in.find(cls.as_str());
  match ed {
    Some(t) => {
      let id = t + cls.len();
      out.push_str(&a_in[id..]);
    }
    _ => {
      return a_in.clone();
    }
  }

  return out;
}
