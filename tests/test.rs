use amm_pinocchio::states::pool::Pool;

#[test]
fn test_pool_size() {
    println!("Pool::LEN = {} bytes", Pool::LEN);
    assert_eq!(Pool::LEN, core::mem::size_of::<Pool>());
}
