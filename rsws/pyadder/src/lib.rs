#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

use add_one;


#[pyfunction]
fn ret_one() -> i32 {
    add_one::add_one(0)
}

#[pymodule]
fn pyadder(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ret_one))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
