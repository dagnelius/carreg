-- Your SQL goes here
create table ar_auto (
  id int not null primary key auto_increment,
  ar_pase_id int not null,
  ar_modelis_id int not null,
  vin varchar(17) not null,
  tipa_apstiprinajuma_nr varchar(30) not null,
  ar_krasa_id int not null,
  sedvietas int not null,
  ar_veids_id int not null,
  piezimes varchar(255),
  FOREIGN KEY (ar_pase_id) REFERENCES ar_pase(id),
  FOREIGN KEY (ar_modelis_id) REFERENCES ar_modelis(id),
  FOREIGN KEY (ar_krasa_id) REFERENCES ar_krasa(id),
  FOREIGN KEY (ar_veids_id) REFERENCES ar_veids(id)
);