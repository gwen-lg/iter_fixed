#![cfg_attr(feature = "nightly_features", allow(incomplete_features))]
#![cfg_attr(feature = "nightly_features", feature(generic_const_exprs))]

extern crate iter_fixed;

use iter_fixed::IntoIteratorFixed;

#[test]
fn test() {
    let res: [_; 4] = [1u32, 2, 3, 4]
        .into_iter_fixed()
        .zip([4u32, 3, 2, 1])
        .map(|(a, b)| a + b)
        .collect();

    assert_eq!(res, [5, 5, 5, 5]);

    let res: [(_, _); 3] = [1, 2, 3]
        .into_iter_fixed()
        .zip(core::iter::repeat(42))
        .collect();
    assert_eq!(res, [(1, 42), (2, 42), (3, 42)]);
}

#[cfg(feature = "nightly_features")]
#[test]
fn test_changing_length() {
    let res: [_; 3] = [1, 2, 3, 4].into_iter_fixed().skip::<1>().collect();

    assert_eq!(res, [2, 3, 4]);

    let res: [_; 3] = [1, 2, 3, 4, 5].into_iter_fixed().step_by::<2>().collect();

    assert_eq!(res, [1, 3, 5]);

    let res: [_; 3] = [1, 2, 3, 4, 5, 6]
        .into_iter_fixed()
        .step_by::<2>()
        .collect();

    assert_eq!(res, [1, 3, 5]);

    let res: [_; 4] = [1, 2, 3, 4, 5, 6, 7]
        .into_iter_fixed()
        .step_by::<2>()
        .collect();

    assert_eq!(res, [1, 3, 5, 7]);

    let res: [_; 4] = [1, 2].into_iter_fixed().chain([3, 4]).collect();

    assert_eq!(res, [1, 2, 3, 4]);

    let res: [_; 2] = [1, 2, 3, 4].into_iter_fixed().take::<2>().collect();

    assert_eq!(res, [1, 2]);

    let res: [_; 4] = [[1, 2], [3, 4]].into_iter_fixed().flatten().collect();

    assert_eq!(res, [1, 2, 3, 4]);

    let res: [_; 6] = [1, 2, 3].into_iter_fixed().flat_map(|x| [x, x]).collect();

    assert_eq!(res, [1, 1, 2, 2, 3, 3]);

    let res: [_; 6] = [1, 2, 3]
        .into_iter_fixed()
        .flat_map(|x| IntoIteratorFixed::<2>::into_iter_fixed(core::iter::repeat(x)))
        .collect();

    assert_eq!(res, [1, 1, 2, 2, 3, 3]);
}
