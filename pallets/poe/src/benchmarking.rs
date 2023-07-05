use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite, account};
use frame_system::RawOrigin;

benchmarks! {
  create_claim {
    let d in 0 .. T::MaxClaimLength::get();
    let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
    let caller = whitelisted_caller();
  }: _(RawOrigin::Signed(caller), claim)

  revoke_claim {
    let d in 0 .. T::MaxClaimLength::get();
    let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
    let caller: T::AccountId = whitelisted_caller();
    assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
  }: _(RawOrigin::Signed(caller), claim)

  transfer_claim {
    let d in 0 .. T::MaxClaimLength::get();
    let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
    let caller: T::AccountId = whitelisted_caller();
    assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
  }: _(RawOrigin::Signed(caller), claim, account("Alice", 0, 1))

  impl_benchmark_test_suite!(
    PoeModule,
    crate::mock::new_test_ext(),
    crate::mock::Test,
  );
}