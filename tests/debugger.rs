use amm_pinocchio::states::pool::Pool;

#[test]
fn test_pool_size() {
    assert_eq!(Pool::LEN, core::mem::size_of::<Pool>());
}
