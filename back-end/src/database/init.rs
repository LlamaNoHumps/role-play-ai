/*
:project: transfery
:author: L-ING
:copyright: (C) 2024 L-ING <hlf01@icloud.com>
:license: MIT, see LICENSE for more details.
*/

use super::Database;
use crate::database::models::{conversation, users, voices};
use anyhow::Result;
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityName, EntityTrait, Schema, Statement};

impl Database {
    pub async fn create_database_if_not_exists(connection: &DatabaseConnection) -> Result<()> {
        let sql = format!("create database if not exists `{}`", super::DB_NAME);

        connection
            .execute(Statement::from_string(
                connection.get_database_backend(),
                sql,
            ))
            .await?;

        Ok(())
    }

    pub async fn init(&self) -> Result<()> {
        self.create_table_if_not_exists(users::Entity).await?;
        self.create_table_if_not_exists(conversation::Entity)
            .await?;
        self.create_table_if_not_exists(voices::Entity).await?;

        Ok(())
    }

    async fn is_table_exists<E>(&self) -> bool
    where
        E: EntityTrait,
    {
        let result = E::find().all(&self.connection).await;

        result.is_ok()
    }

    async fn create_table_if_not_exists<E>(&self, entity: E) -> Result<()>
    where
        E: EntityTrait + EntityName,
    {
        if !self.is_table_exists::<E>().await {
            let backend = self.connection.get_database_backend();

            let table_create_statement = Schema::new(backend).create_table_from_entity(entity);

            self.connection
                .execute(backend.build(&table_create_statement))
                .await?;
        }

        Ok(())
    }
}
