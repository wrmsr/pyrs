#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;


#[pyfunction]
fn ret_one(line: &str, needle: &str) -> usize {
    1
}

#[pymodule]
fn word_count(_py: Python, m: &PyModule) -> PyResult<()> {
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
