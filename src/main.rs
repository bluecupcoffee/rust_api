mod models;

use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use sqlx::{Pool, Error};
use sqlx_mysql::{MySql, MySqlPool, MySqlQueryResult};
use crate::models::{Person, PersonGenerator};

#[tokio::main]
async fn main() {
    let db_url = "mysql://root:my-secret-pw@127.0.0.1:3306/rust_api";
    let pool = MySqlPool::connect(db_url).await.unwrap();

    let mut pg = PersonGenerator::new(String::from("name_list.txt"), 10);
    let load_res = pg.load_names();
    match load_res {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{:#?}", e.to_string());
            std::process::exit(1);
        }
    }

    let people = pg.generate_population();
    let v = insert_people(&pool, &people).await;
    for i in &v {
        if let Err(e) = i {
            eprintln!("Error occurred while inserting: {e}");
            std::process::exit(1);
        }
    }
    println!("{} records successfully created",v.len());

    // let db_url = "mysql://root:my-secret-pw@127.0.0.1:3306/rust_api";
    // let pool = MySqlPool::connect(db_url).await.unwrap();
    //
    // let query = "CREATE OR REPLACE TABLE rust_api.test_table (
    //     uid BIGINT UNSIGNED auto_increment NOT NULL,
    //     name varchar(100) NULL,
    //     color ENUM ('red', 'blue', 'yellow'),
    //     CONSTRAINT test_table_pk PRIMARY KEY (uid)
    // )
    // ENGINE=InnoDB
    // DEFAULT CHARSET=utf8mb4
    // COLLATE=utf8mb4_general_ci;";
    //
    // let create_table = sqlx::query(query)
    //     .execute(&pool)
    //     .await;


}

async fn replace_test_table(pool: &MySqlPool) -> Result<MySqlQueryResult, Error> {
    let query = "CREATE OR REPLACE TABLE rust_api.test_table (
        uid BIGINT UNSIGNED auto_increment NOT NULL,
        name varchar(100) NULL,
        color ENUM ('red', 'blue', 'yellow'),
        CONSTRAINT test_table_pk PRIMARY KEY (uid)
    )
    ENGINE=InnoDB
    DEFAULT CHARSET=utf8mb4
    COLLATE=utf8mb4_general_ci;";

    let create_table = sqlx::query(query)
        .execute(pool)
        .await;

    return create_table
}

async fn insert_people(pool: &MySqlPool, vp: &Vec<Person>) -> Vec<Result<MySqlQueryResult, Error>> {
    let mut v: Vec<Result<MySqlQueryResult, Error>> = Vec::new();
    for p in vp {
        let insert_query = format!(
            "INSERT INTO rust_api.test_table (name, color) VALUES (\'{}\', \'{}\');",
            p.name, p.color
        );
        println!("{insert_query}");
        let insert_res = sqlx::query(insert_query.as_str())
            .execute(pool)
            .await;

        v.push(insert_res);
    }
    v
}