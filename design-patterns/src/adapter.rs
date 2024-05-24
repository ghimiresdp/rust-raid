/**
 * -----------------------------------------------------------------------------
 * ADAPTER PATTERN
 *
 * To execute, please run: cargo run --bin adapter
 * To run tests, please run: cargo test --bin adapter
 * -----------------------------------------------------------------------------
 *
 * An adapter pattern is a structural design pattern which allows objects with
 * incompatible interfaces or traits to work together.
 *
 * An adapter works as a wrapper or an interpreter which allows to convert one
 * type of information to another so that the information can be passed to
 * another object. This process includes transformation of information.
 *
 * The main components of adapter pattern are as follows:
 *
 * 1. Target Interface  : It is an interface where interaction needs to be done.
 * 2. Adaptee   : It is a data structure which is incompatible with the
 *                existing target interface.
 * 3. Adapter   : It creates the compatibility layer between the target
 *                interface and adaptee.
 * 4. Client    : It is the part of the code that uses adaptee to perform tasks
 *                that are incompatible with target interface.
 *
 * Adapter pattern is useful when creating plugins, parsers, media converters,
 * media players, etc.
 *
 * The example below shows an adapter that integrates with an existing ORM to
 * generate database queries directly from the json data.
 *
 **/

mod my_orm {
    pub(crate) trait QueryTrait {
        fn db_query(table_name: &str) -> Self;
        fn select(&mut self, columns: Vec<&str>) -> &mut Self;
        fn filter(&mut self, condition: &str) -> &mut Self;
    }

    #[derive(Default, Clone)]
    pub(crate) struct Query {
        table: String,
        columns: Vec<String>,
        filters: Vec<String>,
    }

    impl Query {
        fn build_condition(&self) -> String {
            if self.filters.len() > 0 {
                return format!(" WHERE {}", self.filters.join(" AND "));
            }
            return String::new();
        }
        pub(crate) fn evaluate(&self) -> String {
            let mut raw_query = String::from("SELECT ");
            let len = self.columns.len();
            if len == 0 {
                raw_query.push_str("*")
            } else {
                raw_query.push_str(self.columns.join(", ").as_str())
            }
            raw_query.push_str(format!(" FROM {}", self.table).as_str());
            raw_query.push_str(self.build_condition().as_str());

            raw_query.push(';');

            raw_query
        }
    }

    impl QueryTrait for Query {
        fn db_query(table_name: &str) -> Self {
            let mut q = Query::default();
            q.table = table_name.trim().to_string();
            q.to_owned()
        }

        fn select(&mut self, columns: Vec<&str>) -> &mut Self {
            self.columns = columns.iter().map(|f| f.trim().to_string()).collect();
            self
        }

        fn filter(&mut self, condition: &str) -> &mut Self {
            self.filters.push(condition.trim().to_string());
            self
        }
    }
}

mod json_adapter {
    use super::my_orm::{Query, QueryTrait};
    use serde::Deserialize;

    // adapter for json data
    pub(crate) trait QueryUtilTrait {
        fn get_query_from_json(json_string: &str) -> Result<String, String>;
    }

    #[derive(Deserialize)]
    pub(crate) struct QueryAdapter {
        _select: Vec<String>,
        _from: String,
        _where: Vec<String>,
    }

    impl QueryUtilTrait for QueryAdapter {
        fn get_query_from_json(json_string: &str) -> Result<String, String> {
            let query_adapter = serde_json::from_str::<QueryAdapter>(json_string);
            match query_adapter {
                Ok(adapter) => {
                    let mut q = Query::db_query(adapter._from.as_str());
                    q.select(adapter._select.iter().map(|v| v.as_str()).collect());
                    for condition in adapter._where {
                        q.filter(&condition);
                    }
                    Ok(q.evaluate())
                }
                Err(err) => {
                    println!("{}", err);
                    Err("Invalid Json Structure".to_string())
                }
            }
        }
    }
}

use json_adapter::{QueryAdapter, QueryUtilTrait};
use my_orm::{Query, QueryTrait};

fn main() {
    // using query without adapter
    let query1 = Query::db_query("person")
        .select(vec!["id", "name"])
        .filter("id > 5")
        .filter("id < 10")
        .evaluate();

    let query2 = Query::db_query("table_name").filter("id > 5").evaluate();

    println!("Raw query 1 is:\t{}", query1);
    println!("Raw query 2 is:\t{}", query2);

    // using query with adapter

    let query_result = QueryAdapter::get_query_from_json(
        r#"{"_select": ["id", "name"],"_from": "Employee","_where": ["id>5", "id<10"]}"#,
    );
    match query_result {
        Ok(query) => println!("Raw query using adapter:\t{}", query),
        Err(message) => println!("[Error]:\t{}", message),
    }
}
