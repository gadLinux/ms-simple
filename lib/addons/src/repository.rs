#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::person;
use person::Person;

pub fn all(connection: &MysqlConnection) -> QueryResult<Vec<Person>> {
    person::table.load::<Person>(&*connection)
}

pub fn get(id: i32, connection: &MysqlConnection) -> QueryResult<Person> {
    person::table.find(id).get_result::<Person>(connection)
}
/*
pub fn insert(person: Person, connection: &MysqlConnection) -> QueryResult<Person> {
    diesel::insert_into(people::table)
        .values(&InsertablePerson::from_person(person))
        .get_result(connection)
}

pub fn update(id: i32, person: Person, connection: &MysqlConnection) -> QueryResult<Person> {
    diesel::update(people::table.find(id))
        .set(&person)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(people::table.find(id))
        .execute(connection)
}
*/

#[derive(Insertable)]
#[table_name = "person"]
struct InsertablePerson {
    first_name: String,
    last_name: String,
    //age: i32,
}
/*
impl InsertablePerson {

    fn from_person(person: Person) -> InsertablePerson {
        InsertablePerson {
            first_name: person.first_name,
            last_name: person.last_name,
            age: person.age,
            profession: person.profession,
            salary: person.salary,
        }
    }
}
*/
