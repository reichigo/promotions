#!/bin/sh

# Espera até que o MongoDB esteja pronto para aceitar conexões
echo "Aguardando o MongoDB iniciar..."
until mongosh --host mongodb --username root --password 1234 --eval 'db.stats()'; do
  sleep 2
done

echo "MongoDB iniciado."

# Comandos para criar o banco de dados e a coleção
mongosh --host mongodb --username root --password 1234 --eval '
  db = db.getSiblingDB("promotions");
  db.createCollection("initialCollection");
'

echo "Banco de dados e coleção criados."