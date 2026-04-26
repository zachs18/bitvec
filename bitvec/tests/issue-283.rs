// https://github.com/ferrilab/bitvec/issues/283
// This used to be miscompiled in release mode

use std::cell::Cell;
use bitvec::prelude::*;

#[test]
#[cfg_attr(debug_assertions, ignore = "should be run in release mode")]
fn assert_release() {
    assert!(!cfg!(debug_assertions), "tests/issue-283.rs should be run in release mode");
}

#[test]
fn test_cell_bitvec() {
    let cell: Cell<u8> = Cell::new(0);

    for index in 0..8 {
        assert_eq!(*cell.view_bits::<Lsb0>().get(index).unwrap(), false);
    }

    for index in 0..8 {
        cell.view_bits::<Lsb0>().set_aliased(index, true);
    }

    for index in 0..8 {
        assert_eq!(*cell.view_bits::<Lsb0>().get(index).unwrap(), true);
    }
}

#[test]
fn test_cell_bitvec_unchecked() {
    let cell: Cell<u8> = Cell::new(0);

    for index in 0..8 {
        assert_eq!(*unsafe { cell.view_bits::<Lsb0>().get_unchecked(index) }, false);
    }

    for index in 0..8 {
        unsafe { cell.view_bits::<Lsb0>().set_aliased_unchecked(index, true) }
    }

    for index in 0..8 {
        assert_eq!(*unsafe { cell.view_bits::<Lsb0>().get_unchecked(index) }, true);
    }
}
