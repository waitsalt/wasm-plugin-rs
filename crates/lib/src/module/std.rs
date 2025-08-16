use serde::{Serialize, de::DeserializeOwned};

use super::{FFIResult, Ptr, Rid};

#[link(wasm_import_module = "std")]
unsafe extern "C" {
    pub fn destroy(rid: Rid);
}

pub fn encode<T: Serialize>(result: &T) -> anyhow::Result<Ptr> {
    let mut bytes = postcard::to_allocvec(result)?;
    let bytes_len = (bytes.len() as i32).to_le_bytes();
    let bytes_cap = (bytes.capacity() as i32).to_le_bytes();
    bytes.splice(0..0, [0; 8]);
    bytes[0..4].copy_from_slice(&bytes_len);
    bytes[4..8].copy_from_slice(&bytes_cap);
    let ptr = bytes.as_ptr() as Ptr;
    core::mem::forget(bytes);
    Ok(ptr)
}

pub fn decode<T: DeserializeOwned>(ptr: Ptr) -> anyhow::Result<T> {
    let ptr = ptr as *const u8;
    let len_vec = unsafe { core::slice::from_raw_parts(ptr, 4) };
    let len = i32::from_le_bytes([len_vec[0], len_vec[1], len_vec[2], len_vec[3]]);
    let cap_vec = unsafe { core::slice::from_raw_parts(ptr.add(4), 4) };
    let cap = i32::from_le_bytes([cap_vec[0], cap_vec[1], cap_vec[2], cap_vec[3]]);
    let original_vec: Vec<u8> =
        unsafe { Vec::from_raw_parts(ptr.add(8) as *mut u8, len as usize, cap as usize) };
    let result = postcard::from_bytes::<T>(&original_vec)?;
    Ok(result)
}

// pub fn buffer_read(ptr: Ptr) -> anyhow::Result<Vec<u8>> {
//     let ptr = ptr as *const u8;
//     let len_vec = unsafe { core::slice::from_raw_parts(ptr, 4) };
//     let len = i32::from_le_bytes([len_vec[0], len_vec[1], len_vec[2], len_vec[3]]);
//     let cap_vec = unsafe { core::slice::from_raw_parts(ptr.add(4), 4) };
//     let cap = i32::from_le_bytes([cap_vec[0], cap_vec[1], cap_vec[2], cap_vec[3]]);
//     let original_vec: Vec<u8> =
//         unsafe { Vec::from_raw_parts(ptr.add(8) as *mut u8, len as usize, cap as usize) };
//     Ok(original_vec)
// }

// pub fn buffer_write(ptr: Ptr, buffer: &[u8]) -> anyhow::Result<()> {
//     Ok(())
// }

// pub fn buffer_free(ptr: Ptr) -> anyhow::Result<()> {
//     Ok(())
// }
