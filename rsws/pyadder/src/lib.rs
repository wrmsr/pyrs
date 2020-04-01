#[macro_use]
extern crate pyo3;
extern crate serde;

use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

use pydicts::{FromPyObject, IntoPyObject};


#[pyfunction]
fn ret_one() -> i32 {
    add_one::add_one(0)
}

#[derive(Serialize, Deserialize, Debug, FromPyObject, IntoPyObject)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[pyfunction]
fn get_contact_info(user: User) -> PyResult<String> {
    Ok(format!("{} - {}", user.name, user.email))
}

#[pyfunction]
fn get_default_user() -> PyResult<User> {
    Ok(User {
        name: "Default".to_owned(),
        email: "default@user.com".to_owned(),
        age: 27,
    })
}

#[pymodule]
fn pyadder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ret_one))?;
    m.add_wrapped(wrap_pyfunction!(get_contact_info))?;
    m.add_wrapped(wrap_pyfunction!(get_default_user))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::User;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_serde() {
        let user = User { name: "name!".to_owned(), email: "email!".to_owned(), age: 420 };

        let serialized = serde_json::to_string(&user).unwrap();
        println!("serialized = {}", serialized);

        let deserialized: User = serde_json::from_str(&serialized).unwrap();
        println!("deserialized = {:?}", deserialized);
    }
}
