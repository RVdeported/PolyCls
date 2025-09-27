use tokio_postgres::{Client, NoTls, Error};
use chrono::NaiveDate;

use crate::schemas::{GeneralSchema, SegmentT};

/// Parse a date string into a NaiveDate, with multiple format support
fn parse_date(date_str: &str) -> Option<NaiveDate> {
    // Try common date formats
    let formats = ["%Y-%m-%d", "%d.%m.%Y", "%d/%m/%Y", "%Y/%m/%d", "%d-%m-%Y"];
    
    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Some(date);
        }
    }
    
    // If none of the formats work, try parsing as a timestamp
    if let Ok(timestamp) = date_str.parse::<i64>() {
        if let Some(date) = chrono::DateTime::from_timestamp(timestamp, 0) {
            return Some(date.naive_local().date());
        }
    }

    // If all attempts fail, return None
    None
}

/// Initialize the GeneralSchema table in PostgreSQL
pub async fn init_general_schema_table(client: &Client) -> Result<(), Error> {
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

/// Insert a GeneralSchema object into the PostgreSQL table
pub async fn insert_general_schema(client: &Client, schema: &GeneralSchema) -> Result<u64, Error> {
    let effective_date = match &schema.effective_date {
        Some(date_str) if !date_str.is_empty() => {
            parse_date(date_str)
        },
        _ => None,
    };

    let fill_date = match &schema.fill_date {
        Some(date_str) if !date_str.is_empty() => {
            parse_date(date_str)
        },
        _ => None,
    };

    let gtd_registration_date = match &schema.gtd_registration_date {
        Some(date_str) if !date_str.is_empty() => {
            parse_date(date_str)
        },
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
                &schema.eval,
            ],
        )
        .await?;

    // The number of affected rows
    Ok(rows.len() as u64)
}

/// Connect to the PostgreSQL database
pub async fn connect_to_database(host: &str, user: &str, password: &str) -> Result<Client, Error> {
    let connection_string = format!("host={} user={} password={}", host, user, password);
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    // Spawn the connection handling in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}