#[macro_use]
extern crate cpython;

use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use cpython::{PyResult, Python};
use midas_rs::{default, Float, Int, MidasR as MidasR_};

py_module_initializer!(midas, initmidas, PyInit_midas, |py, module| {
    module.add(py, "__doc__", "MidasR implementation")?;
    module.add(py, "hash", py_fn!(py, hash(string: String)))?;
    module.add_class::<MidasR>(py)?;
    Ok(())
});

fn hash(_py: Python, string: String) -> PyResult<Int> {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    Ok(hasher.finish() as Int)
}

py_class!(class MidasR |py| {
    data value: RefCell<MidasR_>;

    def __new__(
        _cls,
        rows: Int = default::NUM_ROWS,
        buckets: Int = default::NUM_BUCKETS,
        m_value: Int = default::M_VALUE,
        factor: Float = default::ALPHA
    ) -> PyResult<MidasR> {
        MidasR::create_instance(py, RefCell::new(MidasR_::new(rows, buckets, m_value, factor)))
    }

    def insert(&self, source: Int, dest: Int, time: Int) -> PyResult<Float> {
        Ok(self.value(py).borrow_mut().insert((source, dest, time)))
    }

    def query(&self, source: Int, dest: Int) -> PyResult<Float> {
        Ok(self.value(py).borrow().query(source, dest))
    }

    def current_time(&self) -> PyResult<Int> {
        Ok(self.value(py).borrow().current_time())
    }

    def factor(&self) -> PyResult<Float> {
        Ok(self.value(py).borrow().factor())
    }
});
