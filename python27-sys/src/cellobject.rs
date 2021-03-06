use libc::c_int;
use pyport::Py_ssize_t;
use object::*;

#[repr(C)]
#[derive(Copy, Clone)]
struct PyCellObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    pub _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
    pub ob_ref: *mut PyObject
}

extern "C" {
    pub static mut PyCell_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyCell_Check(op: *mut PyObject) -> c_int {
     (Py_TYPE(op) == &mut PyCell_Type) as c_int
}

extern "C" {
    pub fn PyCell_New(obj: *mut PyObject) -> *mut PyObject;
    pub fn PyCell_Get(op: *mut PyObject) -> *mut PyObject;
    pub fn PyCell_Set(op: *mut PyObject, obj: *mut PyObject) -> c_int;
}

#[inline(always)]
pub unsafe fn PyCell_GET(op: *mut PyObject) -> *mut PyObject {
    (*(op as *mut PyCellObject)).ob_ref
}

#[inline(always)]
pub unsafe fn PyCell_SET(op: *mut PyObject, obj: *mut PyObject) {
    (*(op as *mut PyCellObject)).ob_ref = obj;
}

