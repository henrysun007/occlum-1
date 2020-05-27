use super::*;

#[occlum_test]
pub fn test_unix_new() {
    UnixSocketFile::new(libc::SOCK_STREAM, 0).unwrap();
}

#[occlum_test]
pub fn test_iov() {
    let matrix: Vec<Vec<u8>> = vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]];
    let slices: Vec<&[u8]> = matrix.iter().map(|s| &s[..]).collect();
    let iovs = Iovs::new(slices);
    assert_eq!(iovs.total_bytes(), 9);
    assert_eq!(
        iovs.as_slices()
            .iter()
            .map(|s| s.to_vec())
            .flatten()
            .collect::<Vec<u8>>(),
        &[0, 1, 2, 1, 2, 3, 2, 3, 4]
    );
}
