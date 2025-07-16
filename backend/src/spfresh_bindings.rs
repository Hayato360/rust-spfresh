use std::os::raw::{c_char, c_int, c_float};
use std::ffi::{CString, CStr};
use std::ptr;

#[repr(C)]
pub struct SPFreshIndex {
    _private: [u8; 0],
}

#[link(name = "spfresh_c_api")]
extern "C" {
    pub fn spfresh_create_index(dimension: c_int) -> *mut SPFreshIndex;
    pub fn spfresh_destroy_index(index: *mut SPFreshIndex);
    pub fn spfresh_add_vector(index: *mut SPFreshIndex, data: *const c_float, metadata: *const c_char) -> c_int;
    pub fn spfresh_search(index: *mut SPFreshIndex, query: *const c_float, k: c_int, result_ids: *mut c_int, result_distances: *mut c_float) -> c_int;
    pub fn spfresh_get_vector_count(index: *mut SPFreshIndex) -> c_int;
}

pub struct SPFreshIndexWrapper {
    inner: *mut SPFreshIndex,
    dimension: usize,
}

impl SPFreshIndexWrapper {
    pub fn new(dimension: usize) -> Result<Self, String> {
        let index = unsafe { spfresh_create_index(dimension as c_int) };
        
        if index.is_null() {
            Err("Failed to create SPFresh index".to_string())
        } else {
            Ok(Self { inner: index, dimension })
        }
    }
    
    pub fn add_vector(&self, vector: &[f32], metadata: Option<&str>) -> Result<i32, String> {
        if vector.len() != self.dimension {
            return Err(format!("Vector dimension mismatch: expected {}, got {}", self.dimension, vector.len()));
        }
        
        let metadata_cstr = metadata.map(|s| CString::new(s).unwrap());
        let metadata_ptr = metadata_cstr.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        
        let result = unsafe {
            spfresh_add_vector(self.inner, vector.as_ptr(), metadata_ptr)
        };
        
        if result >= 0 {
            Ok(result)
        } else {
            Err("Failed to add vector to index".to_string())
        }
    }
    
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<(i32, f32)>, String> {
        if query.len() != self.dimension {
            return Err(format!("Query dimension mismatch: expected {}, got {}", self.dimension, query.len()));
        }
        
        let mut result_ids = vec![0i32; k];
        let mut result_distances = vec![0.0f32; k];
        
        let count = unsafe {
            spfresh_search(
                self.inner,
                query.as_ptr(),
                k as c_int,
                result_ids.as_mut_ptr(),
                result_distances.as_mut_ptr()
            )
        };
        
        if count < 0 {
            return Err("Search failed".to_string());
        }
        
        let mut results = Vec::new();
        for i in 0..(count as usize) {
            results.push((result_ids[i], result_distances[i]));
        }
        
        Ok(results)
    }
    
    pub fn get_vector_count(&self) -> usize {
        unsafe { spfresh_get_vector_count(self.inner) as usize }
    }
}

impl Drop for SPFreshIndexWrapper {
    fn drop(&mut self) {
        unsafe {
            spfresh_destroy_index(self.inner);
        }
    }
}

unsafe impl Send for SPFreshIndexWrapper {}
unsafe impl Sync for SPFreshIndexWrapper {}
