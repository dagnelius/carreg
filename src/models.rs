use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct ARAuto {
  pub id: i32,
  pub ar_pase_id: i32,
  pub ar_modelis_id: i32,
  pub vin: String,
  pub tipa_apstiprinajuma_nr: String,
  pub ar_krasa_id: i32,
  pub sedvietas: i32,
  pub ar_veids_id: i32,
  pub piezimes: Option<String>,
}

#[derive(Queryable)]
pub struct ARDegviela {
  pub id: i32,
  pub degviela: String,
}

#[derive(Queryable)]
pub struct ARKrasa {
  pub id: i32,
  pub krasa: String,
}

#[derive(Queryable)]
pub struct ARMarka {
  pub id: i32,
  pub marka: String,
}

#[derive(Queryable)]
pub struct ARModelis {
  pub id: i32,
  pub ar_marka_id: i32,
  pub modelis: String,
}

#[derive(Queryable)]
pub struct ARMotors {
  pub id: i32,
  pub motora_tilpums: i32,
  pub ar_degviela_id: i32, 
}

#[derive(Queryable)]
pub struct ARPase {
  pub id: i32,
  pub apliecibas_nr: String,
  pub registracijas_nr: String,
  pub datums_no: String,
  pub ar_turetaja_adrese_id: i32,
}

#[derive(Queryable)]
pub struct ARTuretajaAdrese {
  pub id: i32,
  pub pilseta: String,
  pub iela: String, 
}

#[derive(Queryable)]
pub struct ARVeids {
  pub id: i32,
  pub veids: String,
}