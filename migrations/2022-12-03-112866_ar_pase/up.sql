-- Your SQL goes here
create table ar_pase (
  id int not null primary key auto_increment,
  apliecibas_nr varchar(9) not null,
  registracijas_nr varchar(6) not null,
  datums_no date not null,
  ar_turetaja_adrese_id int,
  FOREIGN KEY (ar_turetaja_adrese_id) REFERENCES ar_turetaja_adrese(id)
);