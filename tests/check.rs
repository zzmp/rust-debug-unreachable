#![feature(phase)]

#[phase(plugin)]
extern crate debug_unreachable;

#[test]
#[should_fail]
fn explodes_in_debug() {
    unsafe { debug_unreachable!() }
}

