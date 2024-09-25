struct Point {
    x: i32,
    y: i32,
}
pub fn ref_to_ref() {
    let point = Point { x: 1, y: 2 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;

    // `.` operator follows as many references as it takes to find its target
    assert_eq!(rrr.y, 2);
    // - or we can manually deref
    assert_eq!((***rrr).y, 2);

    // comparison operators work similarly
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &&x;
    let rry = &&y;
    assert!(rry == rrx);
    // we `std::ptr::eq` to compare address, instead of pointee
    assert!(!std::ptr::eq(rx, ry));
}
