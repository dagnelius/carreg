-- Your SQL goes here
create table ar_modelis (
  id int not null primary key auto_increment,
  ar_marka_id int not null,
  FOREIGN KEY (ar_marka_id) REFERENCES ar_marka(id)
);