use bson::Bson;
use json::arguments::Arguments;
use json::reader::SuiteContainer;
use json::eq::{self, NumEq};
use mongodb::client:: MongoClient;
use rustc_serialize::json::Json;

#[test]
fn delete_many() {
    run_suite!("tests/json/data/specs/source/crud/tests/write/deleteMany.json",
               "delete_many");
}

#[test]
fn delete_one() {
    run_suite!("tests/json/data/specs/source/crud/tests/write/deleteOne.json",
               "delete_one");
}

#[test]
fn insert_many() {
    run_suite!("tests/json/data/specs/source/crud/tests/write/insertMany.json",
               "insert_many");
}

#[test]
fn insert_one() {
    run_suite!("tests/json/data/specs/source/crud/tests/write/insertOne.json",
               "insert_one");
}

#[test]
fn update_one() {
    run_suite!("tests/json/data/specs/source/crud/tests/write/updateOne.json",
               "update_one");
}

#[test]
fn update_many() {
    run_suite!("tests/json/data/specs/source/crud/tests/write/updateMany.json",
               "update_many");
}
