use super::*;

#[occlum_test]
#[should_panic]
pub fn test_should_panic() {
    panic!();
}
