use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;
use std::env;
use std::path::PathBuf;

#[derive(Deserialize)]
pub enum Protocol {
  #[serde(rename = "ollama")]
  Ollama,
  #[serde(rename = "openai")]
  OpenAI,
  #[serde(rename = "deepseek")]
  Deepseek,
  #[serde(rename = "anthropic")]
  Anthropic,
}

impl Protocol {
  pub fn to_str(self: &Self) -> &str {
    match self {
      Protocol::Ollama => return "ollama",
      Protocol::OpenAI => return "openai",
      Protocol::Anthropic => return "anthropic",
      Protocol::Deepseek => return "deepseek",
    }
  }
}

pub fn load_config(a_path: &String) -> Result<TendConfig, ConfigError> {
  let config = Config::builder()
    .add_source(File::new(a_path.as_str(), FileFormat::Ini))
    .build()?;
  config.try_deserialize()
}

pub fn load_llm_config(a_path: &String) -> Result<Vec<LlmConfig>, ConfigError> {
  let config = Config::builder()
    .add_source(File::with_name(a_path.as_str()))
    .build()?;

  let mut out: Vec<LlmConfig> = Vec::new();

  for i in 1..255 {
    let item = format!("llm_{}", i);
    let o = config.get::<LlmConfig>(item.as_str());

    match o {
      Ok(o) => out.push(o),
      Err(_) => {
        break;
      }
    }
  }

  Ok(out)
}

//===========================================================================//
// General config                                                            //
//===========================================================================//

#[derive(Deserialize)]
pub struct TendConfig {
  pub main: MainConfig,
  pub tender_plan: Option<TenderPlanConfig>,
  pub postgres: PostgresConfig,
}

#[derive(Deserialize)]
pub struct MainConfig {
  pub start_date: String,
  pub files_path: PathBuf,
  pub out_files_path: PathBuf,
  pub example_org_path: PathBuf,
  pub example_tex_path: PathBuf,
}

//===========================================================================//
// TenderPlan api config                                                     //
//===========================================================================//
#[derive(Deserialize)]
pub struct TenderPlanConfig {
  pub api_key: String,
  pub query: String,
}

//===========================================================================//
// postgres config                                                           //
//===========================================================================//
#[derive(Deserialize)]
pub struct PostgresConfig {
  pub host: String,
  pub user: String,
  pub psswd: String,
}

//===========================================================================//
// llm config                                                                //
//===========================================================================//
#[derive(Deserialize)]
pub struct LlmConfig {
  pub host: String,
  pub protocol: Protocol,
  pub api_key: String,
  pub model_name: String,
}

impl LlmConfig {
  pub fn set_env(self: &Self) {
    match self.protocol {
      Protocol::Ollama => unsafe {
        env::set_var("OLLAMA_API_BASE_URL", self.host.clone());
      },
      Protocol::OpenAI => unsafe {
        env::set_var("OPENAI_API_KEY", self.api_key.clone());
        env::set_var("OPENAI_BASE_URL", self.host.clone());
      },
      Protocol::Anthropic => unsafe {
        env::set_var("ANTHROPIC_API_KEY", self.api_key.clone());
        env::set_var("ANTHROPIC_BASE_URL", self.host.clone());
      },
      Protocol::Deepseek => unsafe {
        env::set_var("DEEPSEEK_API_KEY", self.api_key.clone());
        env::set_var("DEEPSEEK_BASE_URL", self.host.clone());
      },
    }
  }
}
