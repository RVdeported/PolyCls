use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use strum_macros::EnumString;

#[derive(
  Debug, Clone, Serialize, Deserialize, enum_iterator::Sequence,
)]
pub enum TypeT
{
  Xps,
  Eps,
  Pson,
  Psv,
  Pir,
  GlassWool,
  StoneWool,
  Other,
  NoEval,
  NoNeed,
}

impl TypeT
{
  pub fn from_str(a_str: &String) -> Self
  {
    if a_str.to_lowercase() == "xps" {
      return TypeT::Xps;
    } else if a_str.to_lowercase() == "eps" {
      return TypeT::Eps;
    } else if a_str.to_lowercase() == "pson" {
      return TypeT::Pson;
    } else if a_str.to_lowercase() == "pir" {
      return TypeT::Pir;
    } else if a_str.to_lowercase() == "psv" {
      return TypeT::Psv;
    } else if a_str.to_lowercase() == "glasswool" {
      return TypeT::GlassWool;
    } else if a_str.to_lowercase() == "stonewool" {
      return TypeT::StoneWool;
    } else if a_str.to_lowercase() == "other" {
      return TypeT::Other;
    } else if a_str.to_lowercase() == "noneed" {
      return TypeT::NoNeed;
    } else {
      return TypeT::NoEval;
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumString)]
pub enum SegmentT
{
  Kz,
  Rus,
  Eas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSchema
{
  pub id: Option<String>,
  pub seg: SegmentT,
  pub declaration_number: Option<String>,
  pub regime: Option<String>,
  pub sheet_number: Option<String>,
  pub position: Option<String>,
  pub effective_date: Option<String>,
  pub fill_date: Option<String>,
  pub batch: Option<String>,
  pub sender_inn: Option<String>,
  pub sender_kpp: Option<String>,
  pub sender_category: Option<String>,
  pub sender_region: Option<String>,
  pub sender_name: Option<String>,
  pub sender_address: Option<String>,
  pub receiver_inn: Option<String>,
  pub receiver_kpp: Option<String>,
  pub receiver_category: Option<String>,
  pub receiver_region: Option<String>,
  pub receiver_name: Option<String>,
  pub receiver_address: Option<String>,
  pub contractor_inn: Option<String>,
  pub contractor_kpp: Option<String>,
  pub contractor_category: Option<String>,
  pub contractor_region: Option<String>,
  pub contractor_name: Option<String>,
  pub contractor_address: Option<String>,
  pub regime_code: Option<String>,
  pub customs_regime: Option<String>,
  pub border_customs_code: Option<String>,
  pub border_customs_name: Option<String>,
  pub border_customs_city: Option<String>,
  pub border_customs_address: Option<String>,
  pub internal_customs_code: Option<String>,
  pub internal_customs_name: Option<String>,
  pub internal_customs_address: Option<String>,
  pub delivery_terms_code: Option<String>,
  pub delivery_terms_alpha_code: Option<String>,
  pub delivery_terms: Option<String>,
  pub payment_form_code: Option<String>,
  pub payment_form: Option<String>,
  pub stat_accounting_sign_code: Option<String>,
  pub stat_accounting_sign: Option<String>,
  pub origin_country_code: Option<String>,
  pub origin_country: Option<String>,
  pub destination_country_code: Option<String>,
  pub destination_country: Option<String>,
  pub trade_country_code: Option<String>,
  pub trade_country: Option<String>,
  pub origin_country_code_2: Option<String>,
  pub origin_country_2: Option<String>,
  pub tnved_code: Option<String>,
  pub tnved_description: Option<String>,
  pub transport_border_code: Option<String>,
  pub transport_border: Option<String>,
  pub transport_internal_code: Option<String>,
  pub transport_internal: Option<String>,
  pub additional_unit_code_1: Option<String>,
  pub additional_unit_1: Option<String>,
  pub additional_unit_code_2: Option<String>,
  pub additional_unit_2: Option<String>,
  pub cost_usd: Option<String>,
  pub product_description_1: Option<String>,
  pub product_description_2: Option<String>,
  pub manufacturer: Option<String>,
  pub brand: Option<String>,
  pub brand_more: Option<String>,
  pub net_weight: Option<String>,
  pub gross_weight: Option<String>,
  pub quantity_physical: Option<String>,
  pub quantity_additional: Option<String>,
  pub cargo_type: Option<String>,
  pub number_of_packages: Option<String>,
  pub delivery_point: Option<String>,
  pub container_sign: Option<String>,
  pub preferences: Option<String>,
  pub customs_value: Option<String>,
  pub invoice_value: Option<String>,
  pub stat_value: Option<String>,
  pub stat_value_usd: Option<String>,
  pub total_invoice_value: Option<String>,
  pub customs_correct_sign: Option<String>,
  pub customs_correct_desc: Option<String>,
  pub currency_code: Option<String>,
  pub currency_description: Option<String>,
  pub exchange_rate: Option<String>,
  pub gtd_registration_place: Option<String>,
  pub gtd_registration_date: Option<String>,
  pub loading_place: Option<String>,
  pub processing_code: Option<String>,
  pub processing_description: Option<String>,
  pub declaration_type: Option<String>,
  pub price_per_kg: Option<String>,
  pub price_per_unit: Option<String>,
  pub price_per_additional_unit: Option<String>,
  pub info_type: Option<String>,
  pub decision_code_ts: Option<String>,
  pub recall_decision_code: Option<String>,
  pub customs_value_ts: Option<String>,
  pub previous_customs_value: Option<String>,
  pub total_customs_payments: Option<String>,
  pub customs_region: Option<String>,
  pub customs_procedure: Option<String>,
  pub additional_bdecl1: Option<String>,
  pub customs_op: Option<String>,
  pub incoterm: Option<String>,
  pub category: Option<String>,
  pub eval: Option<String>,
  pub eval_sec: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EasSchema
{
  #[serde(rename = "Регистрационный №")]
  pub registration_number: Option<String>,

  #[serde(rename = "код таможни")]
  pub customs_code: Option<String>,

  #[serde(rename = "название таможни")]
  pub customs_name: Option<String>,

  #[serde(rename = "адрес таможни")]
  pub customs_address: Option<String>,

  #[serde(rename = "Дата подачи статформы")]
  pub submission_date: Option<String>,

  #[serde(rename = "Отчетный месяц перемещения товаров")]
  pub movement_month: Option<String>,

  #[serde(rename = "04 ИМ/ЭК")]
  pub movement_type: Option<String>,

  #[serde(rename = "09 Код вида транспорта на границе")]
  pub border_transport_code: Option<String>,

  #[serde(rename = "011 ИНН отправителя")]
  pub sender_inn: Option<String>,

  #[serde(rename = "012 Наименование отправителя")]
  pub sender_name: Option<String>,

  #[serde(rename = "013 Адрес отправителя")]
  pub sender_address: Option<String>,

  #[serde(rename = "014 Регион отправителя")]
  pub sender_region: Option<String>,

  #[serde(rename = "015 Код страны отправителя")]
  pub sender_country_code: Option<String>,

  #[serde(rename = "021 ИНН получателя")]
  pub receiver_inn: Option<String>,

  #[serde(rename = "022 Наименование получателя")]
  pub receiver_name: Option<String>,

  #[serde(rename = "023 Адрес получателя")]
  pub receiver_address: Option<String>,

  #[serde(rename = "024 Регион получателя")]
  pub receiver_region: Option<String>,

  #[serde(rename = "025 Код страны получателя")]
  pub receiver_country_code: Option<String>,

  #[serde(rename = "05 Код торгующей страны")]
  pub trade_country_code: Option<String>,

  #[serde(rename = "08 Код страны отправления")]
  pub departure_country_code: Option<String>,

  #[serde(rename = "06 Код страны назначения")]
  pub destination_country_code: Option<String>,

  #[serde(rename = "11 Код товара по ТН ВЭД ТС")]
  pub tnved_code: Option<String>,

  #[serde(rename = "12 Наименование товара по ТН ВЭД ТС")]
  pub tnved_name: Option<String>,

  #[serde(rename = "ТИП")]
  pub category: Option<String>,

  #[serde(rename = "ТИП 2")]
  pub eval: Option<String>,

  #[serde(rename = "ТИП 3")]
  pub eval_sec: Option<String>,

  #[serde(rename = "16 Вес нетто кг")]
  pub net_weight: Option<String>,

  #[serde(rename = "13 Цена товара")]
  pub product_price: Option<String>,

  #[serde(rename = "17 Статистическая стоимость руб")]
  pub statistical_cost_rub: Option<String>,

  #[serde(rename = "14 Статистическая стоимость $")]
  pub statistical_cost_usd: Option<String>,

  #[serde(rename = "15 Код страны происхождения")]
  pub origin_country_code: Option<String>,

  #[serde(rename = "18 Количество товара в единице измерения")]
  pub quantity: Option<String>,

  #[serde(rename = "18_1 Краткое наименование единицы измерения")]
  pub unit_name: Option<String>,

  #[serde(rename = "gd1")]
  pub gd1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KzSchema
{
  #[serde(rename = "ND ГрА Регистрационный номер ДТ")]
  pub registration_number: Option<String>,

  #[serde(rename = "G32")]
  pub g32: Option<String>,

  #[serde(rename = "G07 вид гтд")]
  pub g07_type: Option<String>,

  #[serde(rename = "вид гтд")]
  pub document_type: Option<String>,

  #[serde(rename = "G072 Дата регистрации ДТ")]
  pub registration_date: Option<String>,

  #[serde(rename = "GD1 Дата изменения статуса ДТ")]
  pub status_change_date: Option<String>,

  #[serde(rename = "G071 ТО декларирования")]
  pub declaration_customs_code: Option<String>,

  #[serde(rename = "наименование ТО декларирования")]
  pub declaration_customs_name: Option<String>,

  #[serde(rename = "адрес ТО декларирования")]
  pub declaration_customs_address: Option<String>,

  #[serde(rename = "Бизнес-партнер")]
  pub business_partner: Option<String>,

  #[serde(rename = "Адрес Бизнес-партнера")]
  pub business_partner_address: Option<String>,

  #[serde(rename = "УВЭД РНН/ИИН/БИН")]
  pub uved_rnn_iin_bin: Option<String>,

  #[serde(rename = "УВЭД Наименование")]
  pub uved_name: Option<String>,

  #[serde(rename = "УВЭД Адрес")]
  pub uved_address: Option<String>,

  #[serde(rename = "БИН/ИИН контрактодержателя")]
  pub contractor_bin_iin: Option<String>,

  #[serde(rename = "контрактодержатель")]
  pub contractor: Option<String>,

  #[serde(rename = "адрес контрактодержателя")]
  pub contractor_address: Option<String>,

  #[serde(rename = "БИН/ИИН декларанта")]
  pub declarant_bin_iin: Option<String>,

  #[serde(rename = "декларант")]
  pub declarant: Option<String>,

  #[serde(rename = "G011 Тип перемещения")]
  pub movement_type: Option<String>,

  #[serde(rename = "G0121 Таможенная процедура")]
  pub customs_procedure: Option<String>,

  #[serde(rename = "Наименование таможенной процедуры")]
  pub customs_procedure_name: Option<String>,

  #[serde(rename = "G11 Торгующая страна")]
  pub trade_country: Option<String>,

  #[serde(rename = "G11 Наименование торгующей страны")]
  pub trade_country_name: Option<String>,

  #[serde(rename = "G15A Код страны отправления")]
  pub departure_country_code: Option<String>,

  #[serde(rename = "G15 Наименование страны отправления")]
  pub departure_country_name: Option<String>,

  #[serde(rename = "G16 Страна происхождения")]
  pub origin_country: Option<String>,

  #[serde(rename = "G16 Наименование страны происхождения")]
  pub origin_country_name: Option<String>,

  #[serde(rename = "G17A Код страны назначения")]
  pub destination_country_code: Option<String>,

  #[serde(rename = "G17 Наименование страны назначения")]
  pub destination_country_name: Option<String>,

  #[serde(rename = "G202 Условия поставки")]
  pub delivery_terms: Option<String>,

  #[serde(rename = "Условия поставки")]
  pub delivery_terms_name: Option<String>,

  #[serde(rename = "G2021 место поставки")]
  pub delivery_place: Option<String>,

  #[serde(rename = "G221 Код валюты контракта")]
  pub contract_currency_code: Option<String>,

  #[serde(rename = "валюта контракта")]
  pub contract_currency_name: Option<String>,

  #[serde(rename = "G23 Курс валюты НБ РК")]
  pub exchange_rate: Option<String>,

  #[serde(rename = "G33 Код товара по ТН ВЭД ТС")]
  pub tnved_code: Option<String>,

  #[serde(rename = "ТНВЭД ТС")]
  pub tnved_name: Option<String>,

  #[serde(rename = "G31_1 Наименование товаров 1")]
  pub product_name_1: Option<String>,

  #[serde(rename = "G31_1 Наименование товаров 2")]
  pub product_name_2: Option<String>,

  #[serde(rename = "ТИП")]
  pub category: Option<String>,

  #[serde(rename = "ТИП 2")]
  pub eval: Option<String>,

  #[serde(rename = "ТИП 3")]
  pub eval_sec: Option<String>,

  #[serde(rename = "G31_11 Производитель")]
  pub manufacturer: Option<String>,

  #[serde(rename = "G31_12 Товарный знак")]
  pub trademark: Option<String>,

  #[serde(rename = "G31_14 Марка товара")]
  pub product_brand: Option<String>,

  #[serde(rename = "Артикул товара")]
  pub product_article: Option<String>,

  #[serde(rename = "Сорт")]
  pub sort: Option<String>,

  #[serde(rename = "Наименование сортимента для 4403")]
  pub sortiment_name: Option<String>,

  #[serde(rename = "Описание")]
  pub description: Option<String>,

  #[serde(rename = "Описание1")]
  pub description_1: Option<String>,

  #[serde(rename = "Описание2")]
  pub description_2: Option<String>,

  #[serde(rename = "количество деи")]
  pub dei_quantity: Option<String>,

  #[serde(rename = "деи")]
  pub dei: Option<String>,

  #[serde(rename = "количество деи из оп товара 2")]
  pub dei_quantity_2: Option<String>,

  #[serde(rename = "деи из оп товара 2")]
  pub dei_2: Option<String>,

  #[serde(rename = "Гр41 Количество в ДЕИ")]
  pub quantity_dei: Option<String>,

  #[serde(rename = "Гр41 Код ДЕИ")]
  pub dei_code: Option<String>,

  #[serde(rename = "Гр41 Наименование ДЕИ")]
  pub dei_name: Option<String>,

  #[serde(rename = "Количество ДЕИ2")]
  pub quantity_dei_2: Option<String>,

  #[serde(rename = "Код ДЕИ2")]
  pub dei_code_2: Option<String>,

  #[serde(rename = "Описание ДЕИ2")]
  pub dei_description_2: Option<String>,

  #[serde(rename = "G35 Вес брутто (кг)")]
  pub gross_weight: Option<String>,

  #[serde(rename = "G38 Вес нетто (кг)")]
  pub net_weight: Option<String>,

  #[serde(rename = "G42 Фактурная стоимость")]
  pub invoice_cost: Option<String>,

  #[serde(rename = "G45 Таможенная стоимость")]
  pub customs_cost: Option<String>,

  #[serde(rename = "G46 Статистическая стоимость")]
  pub statistical_cost: Option<String>,

  #[serde(rename = "G25 Вид транспорта на границе")]
  pub border_transport: Option<String>,

  #[serde(rename = "транспорт на границе")]
  pub border_transport_name: Option<String>,

  #[serde(rename = "G26 Вид транспорта внутри страны")]
  pub internal_transport: Option<String>,

  #[serde(rename = "транспорт внутри страны")]
  pub internal_transport_name: Option<String>,

  #[serde(rename = "Статус ДТ")]
  pub dt_status: Option<String>,

  #[serde(rename = "Итого фактурная стоимость")]
  pub total_invoice_cost: Option<String>,

  #[serde(rename = "Код метода определения стоимости")]
  pub valuation_method_code: Option<String>,

  #[serde(rename = "Графа 43#2")]
  pub column_43_2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RusSchema
{
  #[serde(rename = "Декларация №")]
  pub declaration_number: Option<String>,

  #[serde(rename = "Режим")]
  pub regime: Option<String>,

  #[serde(rename = "Лист №")]
  pub sheet_number: Option<String>,

  #[serde(rename = "Позиция")]
  pub position: Option<String>,

  #[serde(rename = "Дата")]
  pub date: Option<String>,

  #[serde(rename = "Дата заполнения")]
  pub fill_date: Option<String>,

  #[serde(rename = "Порция")]
  pub batch: Option<String>,

  #[serde(rename = "Инн отправ.")]
  pub sender_inn: Option<String>,

  #[serde(rename = "КПП отправ.")]
  pub sender_kpp: Option<String>,

  #[serde(rename = "Категория отправителя")]
  pub sender_category: Option<String>,

  #[serde(rename = "Регион отправителя")]
  pub sender_region: Option<String>,

  #[serde(rename = "Название отправителя")]
  pub sender_name: Option<String>,

  #[serde(rename = "Адрес отправителя")]
  pub sender_address: Option<String>,

  #[serde(rename = "Инн получ.")]
  pub receiver_inn: Option<String>,

  #[serde(rename = "КПП получ.")]
  pub receiver_kpp: Option<String>,

  #[serde(rename = "Категория получателя")]
  pub receiver_category: Option<String>,

  #[serde(rename = "Регион получателя")]
  pub receiver_region: Option<String>,

  #[serde(rename = "Название получателя")]
  pub receiver_name: Option<String>,

  #[serde(rename = "Адрес получателя")]
  pub receiver_address: Option<String>,

  #[serde(rename = "Инн контр.")]
  pub contractor_inn: Option<String>,

  #[serde(rename = "КПП контр.")]
  pub contractor_kpp: Option<String>,

  #[serde(rename = "Категория контрактодержателя")]
  pub contractor_category: Option<String>,

  #[serde(rename = "Регион контрактодержателя")]
  pub contractor_region: Option<String>,

  #[serde(rename = "Название контрактодержателя")]
  pub contractor_name: Option<String>,

  #[serde(rename = "Адрес контрактодержателя")]
  pub contractor_address: Option<String>,

  #[serde(rename = "Код режим")]
  pub regime_code: Option<String>,

  #[serde(rename = "Тамож.режим")]
  pub customs_regime: Option<String>,

  #[serde(rename = "Код таможни на границе")]
  pub border_customs_code: Option<String>,

  #[serde(rename = "Название таможни на границе")]
  pub border_customs_name: Option<String>,

  #[serde(rename = "Город тамож.на границе")]
  pub border_customs_city: Option<String>,

  #[serde(rename = "Адрес таожни на границе")]
  pub border_customs_address: Option<String>,

  #[serde(rename = "Код таможни внутри страны")]
  pub internal_customs_code: Option<String>,

  #[serde(rename = "Таможня внутри страны")]
  pub internal_customs_name: Option<String>,

  #[serde(rename = "Адрес тамож.внутри страны")]
  pub internal_customs_address: Option<String>,

  #[serde(rename = "Код условия постаки")]
  pub delivery_terms_code: Option<String>,

  #[serde(rename = "Буквенный код усл.постав.")]
  pub delivery_terms_alpha_code: Option<String>,

  #[serde(rename = "Условия поставки")]
  pub delivery_terms: Option<String>,

  #[serde(rename = "Код формы рассчета")]
  pub payment_form_code: Option<String>,

  #[serde(rename = "Форма рассчета")]
  pub payment_form: Option<String>,

  #[serde(rename = "Код признака стат.учета")]
  pub stat_accounting_sign_code: Option<String>,

  #[serde(rename = "Признак стат.учета")]
  pub stat_accounting_sign: Option<String>,

  #[serde(rename = "Код страны отправ.")]
  pub departure_country_code: Option<String>,

  #[serde(rename = "Страна отправления")]
  pub departure_country_name: Option<String>,

  #[serde(rename = "Код страны получ.")]
  pub destination_country_code: Option<String>,

  #[serde(rename = "Страна получатель")]
  pub destination_country_name: Option<String>,

  #[serde(rename = "Код торг. страны")]
  pub trade_country_code: Option<String>,

  #[serde(rename = "Страна торг.")]
  pub trade_country_name: Option<String>,

  #[serde(rename = "Код страны происхождения")]
  pub origin_country_code: Option<String>,

  #[serde(rename = "Страна происхождения")]
  pub origin_country_name: Option<String>,

  #[serde(rename = "ТНВЭД")]
  pub tnved_code: Option<String>,

  #[serde(rename = "Описания кода ТНВЭД")]
  pub tnved_description: Option<String>,

  #[serde(rename = "Код транс.на границе")]
  pub border_transport_code: Option<String>,

  #[serde(rename = "Транспорт на границе")]
  pub border_transport_name: Option<String>,

  #[serde(rename = "Код трансп.внутри страны")]
  pub internal_transport_code: Option<String>,

  #[serde(rename = "Транспорт внутри страны")]
  pub internal_transport_name: Option<String>,

  #[serde(rename = "Код доп.ед.изм.физ.обьем")]
  pub additional_unit_code_1: Option<String>,

  #[serde(rename = "Доп.ед.измерения")]
  pub additional_unit_1: Option<String>,

  #[serde(rename = "Код доп.ед.изм")]
  pub additional_unit_code_2: Option<String>,

  #[serde(rename = "Доп ед.изм.физ.обьема")]
  pub additional_unit_2: Option<String>,

  #[serde(rename = "Стоимость USD")]
  pub cost_usd: Option<String>,

  #[serde(rename = "Описание товара")]
  pub product_description: Option<String>,

  #[serde(rename = "ТИП 2")]
  pub eval: Option<String>,

  #[serde(rename = "ТИП 3")]
  pub eval_sec: Option<String>,

  #[serde(rename = "Производитель")]
  pub manufacturer: Option<String>,

  #[serde(rename = "Бренд (торговая марка)")]
  pub brand: Option<String>,

  #[serde(rename = "Вес нетто")]
  pub net_weight: Option<String>,

  #[serde(rename = "Вес брутто")]
  pub gross_weight: Option<String>,

  #[serde(rename = "Кол-во в ед.физ.обьема")]
  pub quantity_physical: Option<String>,

  #[serde(rename = "Кол-во в доп.ед.")]
  pub quantity_additional: Option<String>,

  #[serde(rename = "Вид грузовых мест")]
  pub cargo_type: Option<String>,

  #[serde(rename = "Кол-во мест")]
  pub number_of_packages: Option<String>,

  #[serde(rename = "Пункт поставки")]
  pub delivery_point: Option<String>,

  #[serde(rename = "Признак Контейнер.")]
  pub container_sign: Option<String>,

  #[serde(rename = "Преференции")]
  pub preferences: Option<String>,

  #[serde(rename = "Тамож.стоимость")]
  pub customs_value: Option<String>,

  #[serde(rename = "Фактур.стоимость")]
  pub invoice_value: Option<String>,

  #[serde(rename = "Общая фактур.стоимость")]
  pub total_invoice_value: Option<String>,

  #[serde(rename = "Признак тамож.коррект")]
  pub customs_correct_sign: Option<String>,

  #[serde(rename = "Расшифровка признака")]
  pub customs_correct_desc: Option<String>,

  #[serde(rename = "Код валюты")]
  pub currency_code: Option<String>,

  #[serde(rename = "Описание валюты")]
  pub currency_description: Option<String>,

  #[serde(rename = "Курс")]
  pub exchange_rate: Option<String>,

  #[serde(rename = "Место оформления ГТД")]
  pub gtd_registration_place: Option<String>,

  #[serde(rename = "Дата оформления")]
  pub gtd_registration_date: Option<String>,

  #[serde(rename = "Место погрузки товара")]
  pub loading_place: Option<String>,

  #[serde(rename = "Код оформления")]
  pub processing_code: Option<String>,

  #[serde(rename = "Описание кода оформления")]
  pub processing_description: Option<String>,

  #[serde(rename = "Тип декларации")]
  pub declaration_type: Option<String>,

  #[serde(rename = "Цена за кг.")]
  pub price_per_kg: Option<String>,

  #[serde(rename = "Цена за физ.ед.")]
  pub price_per_unit: Option<String>,

  #[serde(rename = "Цена за доп.ед.изм.")]
  pub price_per_additional_unit: Option<String>,

  #[serde(rename = "Вид информации")]
  pub info_type: Option<String>,

  #[serde(rename = "Код принятия окончательного решения по ТС")]
  pub decision_code_ts: Option<String>,

  #[serde(rename = "Код решения об отзыве")]
  pub recall_decision_code: Option<String>,

  #[serde(rename = "Таможенная стоимость (ТС)")]
  pub customs_value_ts: Option<String>,

  #[serde(rename = "Предыдущее значение ТС")]
  pub previous_customs_value: Option<String>,

  #[serde(rename = "Общая сумма там.платежей")]
  pub total_customs_payments: Option<String>,

  #[serde(rename = "Регион таможни оформления")]
  pub customs_region: Option<String>,

  #[serde(rename = "Там.процедура")]
  pub customs_procedure: Option<String>,

  #[serde(rename = "Доп. BDECL1")]
  pub additional_bdecl1: Option<String>,
}

impl From<EasSchema> for GeneralSchema
{
  fn from(eas: EasSchema) -> Self
  {
    GeneralSchema {
      id: None,
      seg: SegmentT::Eas,
      declaration_number: eas.registration_number,
      regime: eas.movement_type,
      sheet_number: None,
      position: None,
      effective_date: eas.gd1,
      fill_date: eas.submission_date,
      batch: None,
      sender_inn: eas.sender_inn,
      sender_kpp: None,
      sender_category: None,
      sender_region: eas.sender_region,
      sender_name: eas.sender_name,
      sender_address: eas.sender_address,
      receiver_inn: eas.receiver_inn,
      receiver_kpp: None,
      receiver_category: None,
      receiver_region: eas.receiver_region,
      receiver_name: eas.receiver_name,
      receiver_address: eas.receiver_address,
      contractor_inn: None,
      contractor_kpp: None,
      contractor_category: None,
      contractor_region: None,
      contractor_name: None,
      contractor_address: None,
      regime_code: None,
      customs_regime: None,
      border_customs_code: eas.customs_code,
      border_customs_name: eas.customs_name,
      border_customs_city: None,
      border_customs_address: eas.customs_address,
      internal_customs_code: None,
      internal_customs_name: None,
      internal_customs_address: None,
      delivery_terms_code: None,
      delivery_terms_alpha_code: None,
      delivery_terms: None,
      payment_form_code: None,
      payment_form: None,
      stat_accounting_sign_code: None,
      stat_accounting_sign: None,
      origin_country_code: eas.origin_country_code,
      origin_country: None,
      destination_country_code: eas.destination_country_code,
      destination_country: None,
      trade_country_code: eas.trade_country_code,
      trade_country: None,
      origin_country_code_2: eas.departure_country_code,
      origin_country_2: None,
      tnved_code: eas.tnved_code,
      tnved_description: eas.tnved_name.clone(),
      transport_border_code: eas.border_transport_code,
      transport_border: None,
      transport_internal_code: None,
      transport_internal: None,
      additional_unit_code_1: None,
      additional_unit_1: None,
      additional_unit_code_2: None,
      additional_unit_2: None,
      cost_usd: eas.statistical_cost_usd.clone(),
      product_description_1: eas.tnved_name,
      product_description_2: None,
      manufacturer: None,
      brand: None,
      brand_more: None,
      net_weight: eas.net_weight,
      gross_weight: None,
      quantity_physical: eas.quantity,
      quantity_additional: None,
      cargo_type: None,
      number_of_packages: None,
      delivery_point: None,
      container_sign: None,
      preferences: None,
      customs_value: eas.statistical_cost_rub.clone(),
      invoice_value: eas.product_price,
      stat_value: eas.statistical_cost_rub,
      stat_value_usd: eas.statistical_cost_usd,
      total_invoice_value: None,
      customs_correct_sign: None,
      customs_correct_desc: None,
      currency_code: None,
      currency_description: None,
      exchange_rate: None,
      gtd_registration_place: None,
      gtd_registration_date: None,
      loading_place: None,
      processing_code: None,
      processing_description: None,
      declaration_type: None,
      price_per_kg: None,
      price_per_unit: None,
      price_per_additional_unit: None,
      info_type: None,
      decision_code_ts: None,
      recall_decision_code: None,
      customs_value_ts: None,
      previous_customs_value: None,
      total_customs_payments: None,
      customs_region: None,
      customs_procedure: None,
      additional_bdecl1: None,
      customs_op: None,
      incoterm: None,
      category: eas.category,
      eval: eas.eval,
      eval_sec: eas.eval_sec,
    }
  }
}

impl From<KzSchema> for GeneralSchema
{
  fn from(kz: KzSchema) -> Self
  {
    GeneralSchema {
      id: None,
      seg: SegmentT::Kz,
      declaration_number: kz.registration_number,
      regime: kz.movement_type,
      sheet_number: Some("0".to_string()),
      position: Some("1".to_string()),
      effective_date: kz.registration_date.clone(),
      fill_date: kz.status_change_date,
      batch: kz.g32,
      sender_inn: kz.uved_rnn_iin_bin,
      sender_kpp: None,
      sender_category: None,
      sender_region: None,
      sender_name: kz.uved_name,
      sender_address: kz.uved_address,
      receiver_inn: kz.contractor_bin_iin,
      receiver_kpp: None,
      receiver_category: None,
      receiver_region: None,
      receiver_name: kz.contractor,
      receiver_address: kz.contractor_address,
      contractor_inn: kz.declarant_bin_iin,
      contractor_kpp: None,
      contractor_category: None,
      contractor_region: None,
      contractor_name: kz.declarant,
      contractor_address: None,
      regime_code: kz.customs_procedure,
      customs_regime: kz.customs_procedure_name.clone(),
      border_customs_code: kz.declaration_customs_code,
      border_customs_name: kz.declaration_customs_name,
      border_customs_city: None,
      border_customs_address: kz.declaration_customs_address,
      internal_customs_code: None,
      internal_customs_name: None,
      internal_customs_address: None,
      delivery_terms_code: kz.delivery_terms,
      delivery_terms_alpha_code: None,
      delivery_terms: kz.delivery_terms_name,
      payment_form_code: None,
      payment_form: None,
      stat_accounting_sign_code: None,
      stat_accounting_sign: None,
      origin_country_code: kz.origin_country,
      origin_country: kz.origin_country_name,
      destination_country_code: kz.destination_country_code,
      destination_country: kz.destination_country_name,
      trade_country_code: kz.trade_country,
      trade_country: kz.trade_country_name,
      origin_country_code_2: kz.departure_country_code,
      origin_country_2: kz.departure_country_name,
      tnved_code: kz.tnved_code,
      tnved_description: kz.tnved_name,
      transport_border_code: kz.border_transport,
      transport_border: kz.border_transport_name,
      transport_internal_code: kz.internal_transport,
      transport_internal: kz.internal_transport_name,
      additional_unit_code_1: kz.dei_code,
      additional_unit_1: kz.dei_name,
      additional_unit_code_2: kz.dei_code_2,
      additional_unit_2: kz.dei_description_2,
      cost_usd: kz.invoice_cost.clone(),
      product_description_1: kz.product_name_1,
      product_description_2: kz.product_name_2,
      manufacturer: kz.manufacturer,
      brand: kz.product_brand,
      brand_more: kz.trademark,
      net_weight: kz.net_weight,
      gross_weight: kz.gross_weight,
      quantity_physical: kz.quantity_dei,
      quantity_additional: kz.quantity_dei_2,
      cargo_type: kz.dei,
      number_of_packages: kz.dei_quantity,
      delivery_point: kz.delivery_place,
      container_sign: None,
      preferences: None,
      customs_value: kz.customs_cost,
      invoice_value: kz.invoice_cost.clone(),
      stat_value: kz.statistical_cost,
      stat_value_usd: None,
      total_invoice_value: kz.total_invoice_cost,
      customs_correct_sign: kz.valuation_method_code,
      customs_correct_desc: None,
      currency_code: kz.contract_currency_code,
      currency_description: kz.contract_currency_name,
      exchange_rate: kz.exchange_rate,
      gtd_registration_place: None,
      gtd_registration_date: kz.registration_date,
      loading_place: None,
      processing_code: None,
      processing_description: None,
      declaration_type: None,
      price_per_kg: None,
      price_per_unit: None,
      price_per_additional_unit: None,
      info_type: None,
      decision_code_ts: None,
      recall_decision_code: None,
      customs_value_ts: None,
      previous_customs_value: None,
      total_customs_payments: None,
      customs_region: None,
      customs_procedure: kz.customs_procedure_name,
      additional_bdecl1: None,
      customs_op: None,
      incoterm: None,
      category: kz.category,
      eval: kz.eval,
      eval_sec: kz.eval_sec,
    }
  }
}

impl From<RusSchema> for GeneralSchema
{
  fn from(rus: RusSchema) -> Self
  {
    GeneralSchema {
      id: None,
      seg: SegmentT::Rus,
      declaration_number: rus.declaration_number,
      regime: rus.regime,
      sheet_number: rus.sheet_number,
      position: rus.position,
      effective_date: rus.date,
      fill_date: rus.fill_date,
      batch: rus.batch,
      sender_inn: rus.sender_inn,
      sender_kpp: rus.sender_kpp,
      sender_category: rus.sender_category,
      sender_region: rus.sender_region,
      sender_name: rus.sender_name,
      sender_address: rus.sender_address,
      receiver_inn: rus.receiver_inn,
      receiver_kpp: rus.receiver_kpp,
      receiver_category: rus.receiver_category,
      receiver_region: rus.receiver_region,
      receiver_name: rus.receiver_name,
      receiver_address: rus.receiver_address,
      contractor_inn: rus.contractor_inn,
      contractor_kpp: rus.contractor_kpp,
      contractor_category: rus.contractor_category,
      contractor_region: rus.contractor_region,
      contractor_name: rus.contractor_name,
      contractor_address: rus.contractor_address,
      regime_code: rus.regime_code,
      customs_regime: rus.customs_regime,
      border_customs_code: rus.border_customs_code,
      border_customs_name: rus.border_customs_name,
      border_customs_city: rus.border_customs_city,
      border_customs_address: rus.border_customs_address,
      internal_customs_code: rus.internal_customs_code,
      internal_customs_name: rus.internal_customs_name,
      internal_customs_address: rus.internal_customs_address,
      delivery_terms_code: rus.delivery_terms_code,
      delivery_terms_alpha_code: rus.delivery_terms_alpha_code,
      delivery_terms: rus.delivery_terms,
      payment_form_code: rus.payment_form_code,
      payment_form: rus.payment_form,
      stat_accounting_sign_code: rus.stat_accounting_sign_code,
      stat_accounting_sign: rus.stat_accounting_sign,
      origin_country_code: rus.origin_country_code,
      origin_country: rus.origin_country_name,
      destination_country_code: rus.destination_country_code,
      destination_country: rus.destination_country_name,
      trade_country_code: rus.trade_country_code,
      trade_country: rus.trade_country_name,
      origin_country_code_2: rus.departure_country_code,
      origin_country_2: rus.departure_country_name,
      tnved_code: rus.tnved_code,
      tnved_description: rus.tnved_description,
      transport_border_code: rus.border_transport_code,
      transport_border: rus.border_transport_name,
      transport_internal_code: rus.internal_transport_code,
      transport_internal: rus.internal_transport_name,
      additional_unit_code_1: rus.additional_unit_code_1,
      additional_unit_1: rus.additional_unit_1,
      additional_unit_code_2: rus.additional_unit_code_2,
      additional_unit_2: rus.additional_unit_2,
      cost_usd: rus.cost_usd.clone(),
      product_description_1: rus.product_description.clone(),
      product_description_2: None,
      manufacturer: rus.manufacturer,
      brand: rus.brand,
      brand_more: None,
      net_weight: rus.net_weight,
      gross_weight: rus.gross_weight,
      quantity_physical: rus.quantity_physical,
      quantity_additional: rus.quantity_additional,
      cargo_type: rus.cargo_type,
      number_of_packages: rus.number_of_packages,
      delivery_point: rus.delivery_point,
      container_sign: rus.container_sign,
      preferences: rus.preferences,
      customs_value: rus.customs_value,
      invoice_value: rus.invoice_value,
      stat_value: rus.total_invoice_value.clone(),
      stat_value_usd: rus.cost_usd.clone(),
      total_invoice_value: rus.total_invoice_value,
      customs_correct_sign: rus.customs_correct_sign,
      customs_correct_desc: rus.customs_correct_desc,
      currency_code: rus.currency_code,
      currency_description: rus.currency_description,
      exchange_rate: rus.exchange_rate,
      gtd_registration_place: rus.gtd_registration_place,
      gtd_registration_date: rus.gtd_registration_date,
      loading_place: rus.loading_place,
      processing_code: rus.processing_code,
      processing_description: rus.processing_description,
      declaration_type: rus.declaration_type,
      price_per_kg: rus.price_per_kg,
      price_per_unit: rus.price_per_unit,
      price_per_additional_unit: rus.price_per_additional_unit,
      info_type: rus.info_type,
      decision_code_ts: rus.decision_code_ts,
      recall_decision_code: rus.recall_decision_code,
      customs_value_ts: rus.customs_value_ts,
      previous_customs_value: rus.previous_customs_value,
      total_customs_payments: rus.total_customs_payments,
      customs_region: rus.customs_region,
      customs_procedure: rus.customs_procedure,
      additional_bdecl1: rus.additional_bdecl1,
      customs_op: None,
      incoterm: None,
      category: None,
      eval: rus.eval,
      eval_sec: rus.eval_sec,
    }
  }
}

impl GeneralSchema
{
  /// Validates and converts the raw eval field to the appropriate TypeT enum value
  pub fn validate_and_convert_eval(
    raw_eval: Option<String>,
  ) -> Option<String>
  {
    match raw_eval {
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
    }
  }

  pub fn from_csv_file_path(
    path: &PathBuf,
    seg: SegmentT,
  ) -> Result<Vec<GeneralSchema>, Box<dyn std::error::Error>>
  {
    debug_assert!(path.as_path().exists());
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    let mut result = Vec::new();

    match seg {
      SegmentT::Kz => {
        for record in reader.deserialize() {
          let row: KzSchema = record?;
          let mut schema: GeneralSchema = GeneralSchema::from(row);
          schema.eval =
            GeneralSchema::validate_and_convert_eval(schema.eval);
          result.push(schema);
        }
      }
      SegmentT::Rus => {
        for record in reader.deserialize() {
          let row: RusSchema = record?;
          let mut schema: GeneralSchema = GeneralSchema::from(row);
          schema.eval =
            GeneralSchema::validate_and_convert_eval(schema.eval);
          result.push(schema);
        }
      }
      SegmentT::Eas => {
        for record in reader.deserialize() {
          let row: EasSchema = record?;
          let mut schema: GeneralSchema = GeneralSchema::from(row);
          schema.eval =
            GeneralSchema::validate_and_convert_eval(schema.eval);
          result.push(schema);
        }
      }
    }

    Ok(result)
  }
}
