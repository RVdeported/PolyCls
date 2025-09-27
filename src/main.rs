mod imports;
mod schemas;

use crate::schemas::*;

use crate::schemas::GeneralSchema;
use std::path::PathBuf;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>>
{
  let p = PathBuf::from("samples/kz_customs_2.csv");
  let v = GeneralSchema::from_csv_file_path(&p, SegmentT::Kz)?;

  println!("{:#?}", v);

  Ok(())
}
