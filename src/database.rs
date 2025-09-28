use chrono::NaiveDate;
use csv::Writer;
use rig::Embed;
use std::fs::File;
use std::path::Path;
use tokio_postgres::{Client, Error, NoTls, types::ToSql};

use crate::{
  agents::{embed_item, embed_str},
  conf::LlmConfig,
  schemas::{GeneralSchema, SegmentT, TypeT},
};
use serde::{Deserialize, Serialize};

/// Parse a date string into a NaiveDate, with multiple format support
pub fn parse_date(date_str: &str) -> Option<NaiveDate>
{
  // Try common date formats, including US format (month/day/year)
  let formats = [
    "%Y-%m-%d", // 2024-05-01
    "%m/%d/%Y", // 05/01/2024 - US format
    "%Y/%m/%d", // 2024/05/01
    "%d.%m.%Y", // 01.05.2024
    "%d-%m-%Y", // 01-05-2024
  ];

  for format in &formats {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
      return Some(date);
    }
  }

  // If none of the formats work, try parsing as a timestamp
  if let Ok(timestamp) = date_str.parse::<i64>() {
    if let Some(date) = chrono::DateTime::from_timestamp(timestamp, 0)
    {
      return Some(date.naive_local().date());
    }
  }

  // If all attempts fail, return None
  None
}

#[derive(Debug)]
pub struct ToEval
{
  pub id: String,
  pub descr: String,
  pub eval: TypeT,
}

impl ToEval
{
  pub fn new(a_id: String, a_descr: String) -> Self
  {
    return Self {
      id: a_id,
      descr: a_descr,
      eval: TypeT::NoEval,
    };
  }
}

pub async fn get_rows_for_eval(
  a_cli: &Client,
) -> Result<Vec<ToEval>, Box<dyn std::error::Error>>
{
  let res = a_cli
    .query(
      "SELECT id, product_description_1, product_description_2 \
    FROM general_schema WHERE eval='NoEval'",
      &[],
    )
    .await?;

  let mut out = Vec::new();

  for s in res {
    let id: i32 = s.get("id");
    let s1: String = s.get(1);
    let s2: Option<String> = s.get(2);
    let s3 = format!("{} {}", s1, s2.unwrap_or("".to_string()));

    out.push(ToEval::new(id.to_string(), s3));
  }

  return Ok(out);
}

/// Initialize the GeneralSchema table in PostgreSQL
pub async fn init_general_schema_table(
  client: &Client,
) -> Result<(), Error>
{
  client
    .execute(
      "CREATE TABLE IF NOT EXISTS general_schema (
                id SERIAL PRIMARY KEY,
                seg TEXT NOT NULL,
                declaration_number TEXT,
                regime TEXT,
                sheet_number TEXT,
                position TEXT,
                effective_date DATE,
                fill_date DATE,
                batch TEXT,
                sender_inn TEXT,
                sender_kpp TEXT,
                sender_category TEXT,
                sender_region TEXT,
                sender_name TEXT,
                sender_address TEXT,
                receiver_inn TEXT,
                receiver_kpp TEXT,
                receiver_category TEXT,
                receiver_region TEXT,
                receiver_name TEXT,
                receiver_address TEXT,
                contractor_inn TEXT,
                contractor_kpp TEXT,
                contractor_category TEXT,
                contractor_region TEXT,
                contractor_name TEXT,
                contractor_address TEXT,
                regime_code TEXT,
                customs_regime TEXT,
                border_customs_code TEXT,
                border_customs_name TEXT,
                border_customs_city TEXT,
                border_customs_address TEXT,
                internal_customs_code TEXT,
                internal_customs_name TEXT,
                internal_customs_address TEXT,
                delivery_terms_code TEXT,
                delivery_terms_alpha_code TEXT,
                delivery_terms TEXT,
                payment_form_code TEXT,
                payment_form TEXT,
                stat_accounting_sign_code TEXT,
                stat_accounting_sign TEXT,
                origin_country_code TEXT,
                origin_country TEXT,
                destination_country_code TEXT,
                destination_country TEXT,
                trade_country_code TEXT,
                trade_country TEXT,
                origin_country_code_2 TEXT,
                origin_country_2 TEXT,
                tnved_code TEXT,
                tnved_description TEXT,
                transport_border_code TEXT,
                transport_border TEXT,
                transport_internal_code TEXT,
                transport_internal TEXT,
                additional_unit_code_1 TEXT,
                additional_unit_1 TEXT,
                additional_unit_code_2 TEXT,
                additional_unit_2 TEXT,
                cost_usd TEXT,
                product_description_1 TEXT,
                product_description_2 TEXT,
                manufacturer TEXT,
                brand TEXT,
                brand_more TEXT,
                net_weight TEXT,
                gross_weight TEXT,
                quantity_physical TEXT,
                quantity_additional TEXT,
                cargo_type TEXT,
                number_of_packages TEXT,
                delivery_point TEXT,
                container_sign TEXT,
                preferences TEXT,
                customs_value TEXT,
                invoice_value TEXT,
                stat_value TEXT,
                stat_value_usd TEXT,
                total_invoice_value TEXT,
                customs_correct_sign TEXT,
                customs_correct_desc TEXT,
                currency_code TEXT,
                currency_description TEXT,
                exchange_rate TEXT,
                gtd_registration_place TEXT,
                gtd_registration_date DATE,
                loading_place TEXT,
                processing_code TEXT,
                processing_description TEXT,
                declaration_type TEXT,
                price_per_kg TEXT,
                price_per_unit TEXT,
                price_per_additional_unit TEXT,
                info_type TEXT,
                decision_code_ts TEXT,
                recall_decision_code TEXT,
                customs_value_ts TEXT,
                previous_customs_value TEXT,
                total_customs_payments TEXT,
                customs_region TEXT,
                customs_procedure TEXT,
                additional_bdecl1 TEXT,
                customs_op TEXT,
                incoterm TEXT,
                category TEXT,
                eval TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
      &[],
    )
    .await?;

  Ok(())
}

pub async fn init_vector_storage_data(
  a_cli: &Client,
) -> Result<(), Error>
{
  a_cli.batch_execute("\
    CREATE EXTENSION IF NOT EXISTS vector;\
    DROP TABLE IF EXISTS poly_docs;
    CREATE TABLE IF NOT EXISTS poly_docs (\
      id uuid DEFAULT gen_random_uuid(), -- we can have repeated entries
      document TEXT NOT NULL,
      embedding vector(1024)
    ); \
    CREATE INDEX IF NOT EXISTS document_embeddings_idx ON poly_docs 
    USING hnsw(embedding vector_cosine_ops);
    
    ").await?;

  Ok(())
}

#[derive(
  Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Default,
)]
pub struct VecItem
{
  pub description: String,
  pub eval: String,
}

pub async fn find_vec_store(
  a_cli: &Client,
  a_query: &String,
  a_llm_conf: &LlmConfig,
  a_top_n: usize,
) -> Result<Vec<String>, Box<dyn std::error::Error>>
{
  let emb = embed_str(a_query, a_llm_conf).await?;

  let res = a_cli.query(format!(
    "SELECT document FROM poly_docs ORDER BY embedding <=> '{:?}' LIMIT {}",
    emb.vec, a_top_n
  ).as_str(), &[]).await?;

  let out: Vec<String> = res.into_iter().map(|x| x.get(0)).collect();

  return Ok(out);
}

pub async fn insert_vec_store(
  a_cli: &Client,
  a_schema: &GeneralSchema,
  a_llm_conf: &LlmConfig,
) -> Result<(), Box<dyn std::error::Error>>
{
  let item = VecItem {
    description: format!(
      "{} {}",
      a_schema.product_description_1.clone().unwrap(),
      a_schema
        .product_description_2
        .clone()
        .unwrap_or("".to_string())
    ),
    eval: format!("{:?}", a_schema.eval.clone().unwrap()),
  };

  let doc = embed_item(&item, a_llm_conf).await?;
  a_cli
    .execute(
      format!(
        "INSERT INTO poly_docs (document, embedding) VALUES ('{}', '{:?}')",
        doc.document, doc.vec
      ).as_str(),
      &[],
    )
    .await?;

  Ok(())
}
/// Insert a GeneralSchema object into the PostgreSQL table
pub async fn insert_general_schema(
  client: &Client,
  schema: &GeneralSchema,
) -> Result<u64, Error>
{
  // Validate and convert the eval field before insertion
  let validated_eval = match &schema.eval {
    Some(eval_str) => {
      // Check if the eval string matches any of the TypeT enum variants (case-insensitive)
      let eval_lower = eval_str.to_lowercase();
      match eval_lower.as_str() {
        "xps" => Some("Xps".to_string()),
        "eps" => Some("Eps".to_string()),
        "pson" => Some("Pson".to_string()),
        "psv" => Some("Psv".to_string()),
        "pir" => Some("Pir".to_string()),
        "glasswool" | "glass wool" => Some("GlassWool".to_string()),
        "stonewool" | "stone wool" => Some("StoneWool".to_string()),
        "other" => Some("Other".to_string()),
        "noeval" => Some("NoEval".to_string()),
        _ => Some("NoEval".to_string()), // Default to NoEval if no match
      }
    }
    None => Some("NoEval".to_string()), // Default to NoEval if eval is None
  };

  let effective_date = match &schema.effective_date {
    Some(date_str) if !date_str.is_empty() => parse_date(date_str),
    _ => None,
  };

  let fill_date = match &schema.fill_date {
    Some(date_str) if !date_str.is_empty() => parse_date(date_str),
    _ => None,
  };

  let gtd_registration_date = match &schema.gtd_registration_date {
    Some(date_str) if !date_str.is_empty() => parse_date(date_str),
    _ => None,
  };

  let (seg_str, seg_value) = match schema.seg {
    SegmentT::Kz => ("Kz", 1),
    SegmentT::Rus => ("Rus", 2),
    SegmentT::Eas => ("Eas", 3),
  };

  let rows = client
        .query(
            "INSERT INTO general_schema (
                seg, declaration_number, regime, sheet_number, position, 
                effective_date, fill_date, batch, sender_inn, sender_kpp, 
                sender_category, sender_region, sender_name, sender_address, 
                receiver_inn, receiver_kpp, receiver_category, receiver_region, 
                receiver_name, receiver_address, contractor_inn, contractor_kpp, 
                contractor_category, contractor_region, contractor_name, contractor_address, 
                regime_code, customs_regime, border_customs_code, border_customs_name, 
                border_customs_city, border_customs_address, internal_customs_code, 
                internal_customs_name, internal_customs_address, delivery_terms_code, 
                delivery_terms_alpha_code, delivery_terms, payment_form_code, payment_form, 
                stat_accounting_sign_code, stat_accounting_sign, origin_country_code, 
                origin_country, destination_country_code, destination_country, 
                trade_country_code, trade_country, origin_country_code_2, origin_country_2, 
                tnved_code, tnved_description, transport_border_code, transport_border, 
                transport_internal_code, transport_internal, additional_unit_code_1, 
                additional_unit_1, additional_unit_code_2, additional_unit_2, 
                cost_usd, product_description_1, product_description_2, manufacturer, 
                brand, brand_more, net_weight, gross_weight, quantity_physical, 
                quantity_additional, cargo_type, number_of_packages, delivery_point, 
                container_sign, preferences, customs_value, invoice_value, 
                stat_value, stat_value_usd, total_invoice_value, customs_correct_sign, 
                customs_correct_desc, currency_code, currency_description, 
                exchange_rate, gtd_registration_place, gtd_registration_date, 
                loading_place, processing_code, processing_description, declaration_type, 
                price_per_kg, price_per_unit, price_per_additional_unit, info_type, 
                decision_code_ts, recall_decision_code, customs_value_ts, 
                previous_customs_value, total_customs_payments, customs_region, 
                customs_procedure, additional_bdecl1, customs_op, incoterm, 
                category, eval
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, 
                $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, 
                $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, 
                $42, $43, $44, $45, $46, $47, $48, $49, $50, $51, $52, $53, $54, 
                $55, $56, $57, $58, $59, $60, $61, $62, $63, $64, $65, $66, $67, 
                $68, $69, $70, $71, $72, $73, $74, $75, $76, $77, $78, $79, $80, 
                $81, $82, $83, $84, $85, $86, $87, $88, $89, $90, $91, $92, $93, 
                $94, $95, $96, $97, $98, $99, $100, $101, $102, $103, $104, $105, 
                $106, $107
            ) RETURNING id",
            &[
                &seg_str,
                &schema.declaration_number,
                &schema.regime,
                &schema.sheet_number,
                &schema.position,
                &effective_date,
                &fill_date,
                &schema.batch,
                &schema.sender_inn,
                &schema.sender_kpp,
                &schema.sender_category,
                &schema.sender_region,
                &schema.sender_name,
                &schema.sender_address,
                &schema.receiver_inn,
                &schema.receiver_kpp,
                &schema.receiver_category,
                &schema.receiver_region,
                &schema.receiver_name,
                &schema.receiver_address,
                &schema.contractor_inn,
                &schema.contractor_kpp,
                &schema.contractor_category,
                &schema.contractor_region,
                &schema.contractor_name,
                &schema.contractor_address,
                &schema.regime_code,
                &schema.customs_regime,
                &schema.border_customs_code,
                &schema.border_customs_name,
                &schema.border_customs_city,
                &schema.border_customs_address,
                &schema.internal_customs_code,
                &schema.internal_customs_name,
                &schema.internal_customs_address,
                &schema.delivery_terms_code,
                &schema.delivery_terms_alpha_code,
                &schema.delivery_terms,
                &schema.payment_form_code,
                &schema.payment_form,
                &schema.stat_accounting_sign_code,
                &schema.stat_accounting_sign,
                &schema.origin_country_code,
                &schema.origin_country,
                &schema.destination_country_code,
                &schema.destination_country,
                &schema.trade_country_code,
                &schema.trade_country,
                &schema.origin_country_code_2,
                &schema.origin_country_2,
                &schema.tnved_code,
                &schema.tnved_description,
                &schema.transport_border_code,
                &schema.transport_border,
                &schema.transport_internal_code,
                &schema.transport_internal,
                &schema.additional_unit_code_1,
                &schema.additional_unit_1,
                &schema.additional_unit_code_2,
                &schema.additional_unit_2,
                &schema.cost_usd,
                &schema.product_description_1,
                &schema.product_description_2,
                &schema.manufacturer,
                &schema.brand,
                &schema.brand_more,
                &schema.net_weight,
                &schema.gross_weight,
                &schema.quantity_physical,
                &schema.quantity_additional,
                &schema.cargo_type,
                &schema.number_of_packages,
                &schema.delivery_point,
                &schema.container_sign,
                &schema.preferences,
                &schema.customs_value,
                &schema.invoice_value,
                &schema.stat_value,
                &schema.stat_value_usd,
                &schema.total_invoice_value,
                &schema.customs_correct_sign,
                &schema.customs_correct_desc,
                &schema.currency_code,
                &schema.currency_description,
                &schema.exchange_rate,
                &schema.gtd_registration_place,
                &gtd_registration_date,
                &schema.loading_place,
                &schema.processing_code,
                &schema.processing_description,
                &schema.declaration_type,
                &schema.price_per_kg,
                &schema.price_per_unit,
                &schema.price_per_additional_unit,
                &schema.info_type,
                &schema.decision_code_ts,
                &schema.recall_decision_code,
                &schema.customs_value_ts,
                &schema.previous_customs_value,
                &schema.total_customs_payments,
                &schema.customs_region,
                &schema.customs_procedure,
                &schema.additional_bdecl1,
                &schema.customs_op,
                &schema.incoterm,
                &schema.category,
                &validated_eval,
            ],
        )
        .await?;

  // The number of affected rows
  Ok(rows.len() as u64)
}

/// Export all GeneralSchema records from the database to a CSV file
pub async fn export_general_schema_to_csv<P: AsRef<Path>>(
  client: &Client,
  file_path: P,
) -> Result<(), Box<dyn std::error::Error>>
{
  // Query all records from the database
  let rows = client
        .query("SELECT id, seg, declaration_number, regime, sheet_number, position, effective_date, fill_date, batch, sender_inn, sender_kpp, sender_category, sender_region, sender_name, sender_address, receiver_inn, receiver_kpp, receiver_category, receiver_region, receiver_name, receiver_address, contractor_inn, contractor_kpp, contractor_category, contractor_region, contractor_name, contractor_address, regime_code, customs_regime, border_customs_code, border_customs_name, border_customs_city, border_customs_address, internal_customs_code, internal_customs_name, internal_customs_address, delivery_terms_code, delivery_terms_alpha_code, delivery_terms, payment_form_code, payment_form, stat_accounting_sign_code, stat_accounting_sign, origin_country_code, origin_country, destination_country_code, destination_country, trade_country_code, trade_country, origin_country_code_2, origin_country_2, tnved_code, tnved_description, transport_border_code, transport_border, transport_internal_code, transport_internal, additional_unit_code_1, additional_unit_1, additional_unit_code_2, additional_unit_2, cost_usd, product_description_1, product_description_2, manufacturer, brand, brand_more, net_weight, gross_weight, quantity_physical, quantity_additional, cargo_type, number_of_packages, delivery_point, container_sign, preferences, customs_value, invoice_value, stat_value, stat_value_usd, total_invoice_value, customs_correct_sign, customs_correct_desc, currency_code, currency_description, exchange_rate, gtd_registration_place, gtd_registration_date, loading_place, processing_code, processing_description, declaration_type, price_per_kg, price_per_unit, price_per_additional_unit, info_type, decision_code_ts, recall_decision_code, customs_value_ts, previous_customs_value, total_customs_payments, customs_region, customs_procedure, additional_bdecl1, customs_op, incoterm, category, eval, created_at FROM general_schema", &[])
        .await?;

  // Create a CSV writer
  let file = File::create(file_path)?;
  let mut wtr = Writer::from_writer(file);

  // Write the header
  wtr.write_record(&[
    "id",
    "seg",
    "declaration_number",
    "regime",
    "sheet_number",
    "position",
    "effective_date",
    "fill_date",
    "batch",
    "sender_inn",
    "sender_kpp",
    "sender_category",
    "sender_region",
    "sender_name",
    "sender_address",
    "receiver_inn",
    "receiver_kpp",
    "receiver_category",
    "receiver_region",
    "receiver_name",
    "receiver_address",
    "contractor_inn",
    "contractor_kpp",
    "contractor_category",
    "contractor_region",
    "contractor_name",
    "contractor_address",
    "regime_code",
    "customs_regime",
    "border_customs_code",
    "border_customs_name",
    "border_customs_city",
    "border_customs_address",
    "internal_customs_code",
    "internal_customs_name",
    "internal_customs_address",
    "delivery_terms_code",
    "delivery_terms_alpha_code",
    "delivery_terms",
    "payment_form_code",
    "payment_form",
    "stat_accounting_sign_code",
    "stat_accounting_sign",
    "origin_country_code",
    "origin_country",
    "destination_country_code",
    "destination_country",
    "trade_country_code",
    "trade_country",
    "origin_country_code_2",
    "origin_country_2",
    "tnved_code",
    "tnved_description",
    "transport_border_code",
    "transport_border",
    "transport_internal_code",
    "transport_internal",
    "additional_unit_code_1",
    "additional_unit_1",
    "additional_unit_code_2",
    "additional_unit_2",
    "cost_usd",
    "product_description_1",
    "product_description_2",
    "manufacturer",
    "brand",
    "brand_more",
    "net_weight",
    "gross_weight",
    "quantity_physical",
    "quantity_additional",
    "cargo_type",
    "number_of_packages",
    "delivery_point",
    "container_sign",
    "preferences",
    "customs_value",
    "invoice_value",
    "stat_value",
    "stat_value_usd",
    "total_invoice_value",
    "customs_correct_sign",
    "customs_correct_desc",
    "currency_code",
    "currency_description",
    "exchange_rate",
    "gtd_registration_place",
    "gtd_registration_date",
    "loading_place",
    "processing_code",
    "processing_description",
    "declaration_type",
    "price_per_kg",
    "price_per_unit",
    "price_per_additional_unit",
    "info_type",
    "decision_code_ts",
    "recall_decision_code",
    "customs_value_ts",
    "previous_customs_value",
    "total_customs_payments",
    "customs_region",
    "customs_procedure",
    "additional_bdecl1",
    "customs_op",
    "incoterm",
    "category",
    "eval",
    "created_at",
  ])?;

  // Write each row
  for row in rows {
    let record = [
      row.get::<_, i32>(0).to_string(), // id
      row.get::<_, String>(1),          // seg
      option_to_string(row.get::<_, Option<String>>(2)), // declaration_number
      option_to_string(row.get::<_, Option<String>>(3)), // regime
      option_to_string(row.get::<_, Option<String>>(4)), // sheet_number
      option_to_string(row.get::<_, Option<String>>(5)), // position
      option_date_to_string(
        row.get::<_, Option<chrono::NaiveDate>>(6),
      ), // effective_date
      option_date_to_string(
        row.get::<_, Option<chrono::NaiveDate>>(7),
      ), // fill_date
      option_to_string(row.get::<_, Option<String>>(8)), // batch
      option_to_string(row.get::<_, Option<String>>(9)), // sender_inn
      option_to_string(row.get::<_, Option<String>>(10)), // sender_kpp
      option_to_string(row.get::<_, Option<String>>(11)), // sender_category
      option_to_string(row.get::<_, Option<String>>(12)), // sender_region
      option_to_string(row.get::<_, Option<String>>(13)), // sender_name
      option_to_string(row.get::<_, Option<String>>(14)), // sender_address
      option_to_string(row.get::<_, Option<String>>(15)), // receiver_inn
      option_to_string(row.get::<_, Option<String>>(16)), // receiver_kpp
      option_to_string(row.get::<_, Option<String>>(17)), // receiver_category
      option_to_string(row.get::<_, Option<String>>(18)), // receiver_region
      option_to_string(row.get::<_, Option<String>>(19)), // receiver_name
      option_to_string(row.get::<_, Option<String>>(20)), // receiver_address
      option_to_string(row.get::<_, Option<String>>(21)), // contractor_inn
      option_to_string(row.get::<_, Option<String>>(22)), // contractor_kpp
      option_to_string(row.get::<_, Option<String>>(23)), // contractor_category
      option_to_string(row.get::<_, Option<String>>(24)), // contractor_region
      option_to_string(row.get::<_, Option<String>>(25)), // contractor_name
      option_to_string(row.get::<_, Option<String>>(26)), // contractor_address
      option_to_string(row.get::<_, Option<String>>(27)), // regime_code
      option_to_string(row.get::<_, Option<String>>(28)), // customs_regime
      option_to_string(row.get::<_, Option<String>>(29)), // border_customs_code
      option_to_string(row.get::<_, Option<String>>(30)), // border_customs_name
      option_to_string(row.get::<_, Option<String>>(31)), // border_customs_city
      option_to_string(row.get::<_, Option<String>>(32)), // border_customs_address
      option_to_string(row.get::<_, Option<String>>(33)), // internal_customs_code
      option_to_string(row.get::<_, Option<String>>(34)), // internal_customs_name
      option_to_string(row.get::<_, Option<String>>(35)), // internal_customs_address
      option_to_string(row.get::<_, Option<String>>(36)), // delivery_terms_code
      option_to_string(row.get::<_, Option<String>>(37)), // delivery_terms_alpha_code
      option_to_string(row.get::<_, Option<String>>(38)), // delivery_terms
      option_to_string(row.get::<_, Option<String>>(39)), // payment_form_code
      option_to_string(row.get::<_, Option<String>>(40)), // payment_form
      option_to_string(row.get::<_, Option<String>>(41)), // stat_accounting_sign_code
      option_to_string(row.get::<_, Option<String>>(42)), // stat_accounting_sign
      option_to_string(row.get::<_, Option<String>>(43)), // origin_country_code
      option_to_string(row.get::<_, Option<String>>(44)), // origin_country
      option_to_string(row.get::<_, Option<String>>(45)), // destination_country_code
      option_to_string(row.get::<_, Option<String>>(46)), // destination_country
      option_to_string(row.get::<_, Option<String>>(47)), // trade_country_code
      option_to_string(row.get::<_, Option<String>>(48)), // trade_country
      option_to_string(row.get::<_, Option<String>>(49)), // origin_country_code_2
      option_to_string(row.get::<_, Option<String>>(50)), // origin_country_2
      option_to_string(row.get::<_, Option<String>>(51)), // tnved_code
      option_to_string(row.get::<_, Option<String>>(52)), // tnved_description
      option_to_string(row.get::<_, Option<String>>(53)), // transport_border_code
      option_to_string(row.get::<_, Option<String>>(54)), // transport_border
      option_to_string(row.get::<_, Option<String>>(55)), // transport_internal_code
      option_to_string(row.get::<_, Option<String>>(56)), // transport_internal
      option_to_string(row.get::<_, Option<String>>(57)), // additional_unit_code_1
      option_to_string(row.get::<_, Option<String>>(58)), // additional_unit_1
      option_to_string(row.get::<_, Option<String>>(59)), // additional_unit_code_2
      option_to_string(row.get::<_, Option<String>>(60)), // additional_unit_2
      option_to_string(row.get::<_, Option<String>>(61)), // cost_usd
      option_to_string(row.get::<_, Option<String>>(62)), // product_description_1
      option_to_string(row.get::<_, Option<String>>(63)), // product_description_2
      option_to_string(row.get::<_, Option<String>>(64)), // manufacturer
      option_to_string(row.get::<_, Option<String>>(65)), // brand
      option_to_string(row.get::<_, Option<String>>(66)), // brand_more
      option_to_string(row.get::<_, Option<String>>(67)), // net_weight
      option_to_string(row.get::<_, Option<String>>(68)), // gross_weight
      option_to_string(row.get::<_, Option<String>>(69)), // quantity_physical
      option_to_string(row.get::<_, Option<String>>(70)), // quantity_additional
      option_to_string(row.get::<_, Option<String>>(71)), // cargo_type
      option_to_string(row.get::<_, Option<String>>(72)), // number_of_packages
      option_to_string(row.get::<_, Option<String>>(73)), // delivery_point
      option_to_string(row.get::<_, Option<String>>(74)), // container_sign
      option_to_string(row.get::<_, Option<String>>(75)), // preferences
      option_to_string(row.get::<_, Option<String>>(76)), // customs_value
      option_to_string(row.get::<_, Option<String>>(77)), // invoice_value
      option_to_string(row.get::<_, Option<String>>(78)), // stat_value
      option_to_string(row.get::<_, Option<String>>(79)), // stat_value_usd
      option_to_string(row.get::<_, Option<String>>(80)), // total_invoice_value
      option_to_string(row.get::<_, Option<String>>(81)), // customs_correct_sign
      option_to_string(row.get::<_, Option<String>>(82)), // customs_correct_desc
      option_to_string(row.get::<_, Option<String>>(83)), // currency_code
      option_to_string(row.get::<_, Option<String>>(84)), // currency_description
      option_to_string(row.get::<_, Option<String>>(85)), // exchange_rate
      option_to_string(row.get::<_, Option<String>>(86)), // gtd_registration_place
      option_date_to_string(
        row.get::<_, Option<chrono::NaiveDate>>(87),
      ), // gtd_registration_date
      option_to_string(row.get::<_, Option<String>>(88)), // loading_place
      option_to_string(row.get::<_, Option<String>>(89)), // processing_code
      option_to_string(row.get::<_, Option<String>>(90)), // processing_description
      option_to_string(row.get::<_, Option<String>>(91)), // declaration_type
      option_to_string(row.get::<_, Option<String>>(92)), // price_per_kg
      option_to_string(row.get::<_, Option<String>>(93)), // price_per_unit
      option_to_string(row.get::<_, Option<String>>(94)), // price_per_additional_unit
      option_to_string(row.get::<_, Option<String>>(95)), // info_type
      option_to_string(row.get::<_, Option<String>>(96)), // decision_code_ts
      option_to_string(row.get::<_, Option<String>>(97)), // recall_decision_code
      option_to_string(row.get::<_, Option<String>>(98)), // customs_value_ts
      option_to_string(row.get::<_, Option<String>>(99)), // previous_customs_value
      option_to_string(row.get::<_, Option<String>>(100)), // total_customs_payments
      option_to_string(row.get::<_, Option<String>>(101)), // customs_region
      option_to_string(row.get::<_, Option<String>>(102)), // customs_procedure
      option_to_string(row.get::<_, Option<String>>(103)), // additional_bdecl1
      option_to_string(row.get::<_, Option<String>>(104)), // customs_op
      option_to_string(row.get::<_, Option<String>>(105)), // incoterm
      option_to_string(row.get::<_, Option<String>>(106)), // category
      option_to_string(row.get::<_, Option<String>>(107)), // eval
      option_datetime_to_string(
        row.get::<_, Option<chrono::NaiveDateTime>>(108),
      ), // created_at
    ];

    wtr.write_record(&record)?;
  }

  // Flush the writer to ensure all data is written
  wtr.flush()?;

  Ok(())
}

// Helper function to convert Option<String> to String for CSV output
fn option_to_string(opt: Option<String>) -> String
{
  match opt {
    Some(s) => s,
    None => String::new(),
  }
}

// Helper function to convert Option<NaiveDate> to String for CSV output
fn option_date_to_string(opt: Option<chrono::NaiveDate>) -> String
{
  match opt {
    Some(d) => d.format("%Y-%m-%d").to_string(),
    None => String::new(),
  }
}

// Helper function to convert Option<NaiveDateTime> to String for CSV output
fn option_datetime_to_string(
  opt: Option<chrono::NaiveDateTime>,
) -> String
{
  match opt {
    Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
    None => String::new(),
  }
}

/// Connect to the PostgreSQL database
/// Update the eval status for a specific entry in the general_schema table
pub async fn update_eval_status(
  a_cli: &Client,
  to_eval: &ToEval,
) -> Result<(), Box<dyn std::error::Error>>
{
  a_cli
    .execute(
      format!(
        "UPDATE general_schema SET eval = '{:?}' WHERE id = {}",
        to_eval.eval, to_eval.id
      )
      .as_str(),
      &[],
    )
    .await?;

  Ok(())
}

pub async fn connect_to_database(
  host: &str,
  user: &str,
  password: &str,
) -> Result<Client, Error>
{
  let connection_string =
    format!("host={} user={} password={}", host, user, password);
  let (client, connection) =
    tokio_postgres::connect(&connection_string, NoTls).await?;

  // Spawn the connection handling in the background
  tokio::spawn(async move {
    if let Err(e) = connection.await {
      eprintln!("Connection error: {}", e);
    }
  });

  Ok(client)
}
