#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

use pydicts::{FromPyObject, IntoPyObject};


#[pyfunction]
fn ret_one() -> i32 {
    add_one::add_one(0)
}

#[derive(FromPyObject, IntoPyObject)]
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
