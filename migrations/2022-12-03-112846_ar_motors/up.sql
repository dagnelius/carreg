-- Your SQL goes here
create table ar_motors (
  id int not null primary key auto_increment,
  motora_tilpums int not null,
  ar_degviela_id int,
  FOREIGN KEY (ar_degviela_id) REFERENCES ar_degviela(id)
);