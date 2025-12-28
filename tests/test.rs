use amm_pinocchio::states::pool::Pool;

#[test]
fn test_pool_size() {
    println!("Size of Pool::LEN = {} bytes", Pool::LEN);
    println!(
        "Size of mem::size_of = {} bytes",
        core::mem::size_of::<Pool>()
    );
    assert_eq!(Pool::LEN, core::mem::size_of::<Pool>());
}
