#!/bin/bash
set -e

echo "Trying to create database with name: {$POSTGRES_USER}"

psql --username $POSTGRES_USER --dbname $POSTGRES_DB <<EOSQL
  CREATE DATABASE "$POSTGRES_USER";
  \connect "$POSTGRES_USER";
  
  CREATE TABLE sample_table(uid uuid NOT NULL PRIMARY KEY);
  SELECT * FROM sample_table;
EOSQL