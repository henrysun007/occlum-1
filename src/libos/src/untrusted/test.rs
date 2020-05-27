use super::slice_ext::*;
use super::*;

#[occlum_test]
pub fn test_empty_slice_ext() {
    let empty_vec: Vec<u32> = Vec::new();
    let empty_slice: &[u32] = &empty_vec;
    assert_eq!(empty_slice.as_ptr_and_len(), (std::ptr::null(), 0));
    assert_eq!(Some(empty_slice).as_ptr_and_len(), (std::ptr::null(), 0));
}

#[occlum_test]
pub fn test_none_slice_ext() {
    assert_eq!(
        Option::<&[u8]>::None.as_ptr_and_len(),
        (std::ptr::null(), 0)
    );
}
