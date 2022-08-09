use bigfixed::*;
use Index::{Position};
use Rounding::*;

type D = u8;

#[test]
fn positive() {
    // 00000000 00000011.10000001 10000000 00000000
    let a: BigFixed<D> = BigFixed {
        head: 0b00000000,
        body: vec![0b10000000, 0b10000001, 0b00000011],
        position: Position(-2)
    };

    test(&a, Floor, -3, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Floor, -2, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Floor, -1, &[0b00000000, 0b00000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Floor,  0, &[0b00000000, 0b00000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Floor,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&a, Floor,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);

    test(&a, Ceiling, -3, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Ceiling, -2, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Ceiling, -1, &[0b00000000, 0b00000000, 0b10000010, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Ceiling,  0, &[0b00000000, 0b00000000, 0b00000000, 0b00000100, 0b00000000, 0b00000000]);
    test(&a, Ceiling,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000]);
    test(&a, Ceiling,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001]);

    test(&a, Round, -3, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Round, -2, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Round, -1, &[0b00000000, 0b00000000, 0b10000010, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, Round,  0, &[0b00000000, 0b00000000, 0b00000000, 0b00000100, 0b00000000, 0b00000000]);
    test(&a, Round,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&a, Round,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);

    test(&a, TowardsZero, -3, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, TowardsZero, -2, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, TowardsZero, -1, &[0b00000000, 0b00000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, TowardsZero,  0, &[0b00000000, 0b00000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, TowardsZero,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&a, TowardsZero,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);

    test(&a, AwayFromZero, -3, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, AwayFromZero, -2, &[0b00000000, 0b10000000, 0b10000001, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, AwayFromZero, -1, &[0b00000000, 0b00000000, 0b10000010, 0b00000011, 0b00000000, 0b00000000]);
    test(&a, AwayFromZero,  0, &[0b00000000, 0b00000000, 0b00000000, 0b00000100, 0b00000000, 0b00000000]);
    test(&a, AwayFromZero,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000]);
    test(&a, AwayFromZero,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001]);
}

#[test]
fn negative() {
    // 11111111 11110001.10000001 10000000 00000000
    let b: BigFixed<D> = BigFixed {
        head: 0b11111111,
        body: vec![0b10000000, 0b10000001, 0b11110001],
        position: Position(-2)
    };

    test(&b, Floor, -3, &[0b00000000, 0b10000000, 0b10000001, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Floor, -2, &[0b00000000, 0b10000000, 0b10000001, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Floor, -1, &[0b00000000, 0b00000000, 0b10000001, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Floor,  0, &[0b00000000, 0b00000000, 0b00000000, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Floor,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b11111111]);
    test(&b, Floor,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11111111]);

    test(&b, Ceiling, -3, &[0b00000000, 0b10000000, 0b10000001, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Ceiling, -2, &[0b00000000, 0b10000000, 0b10000001, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Ceiling, -1, &[0b00000000, 0b00000000, 0b10000010, 0b11110001, 0b11111111, 0b11111111]);
    test(&b, Ceiling,  0, &[0b00000000, 0b00000000, 0b00000000, 0b11110010, 0b11111111, 0b11111111]);
    test(&b, Ceiling,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&b, Ceiling,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
}

#[test]
fn nine() {
    // 00000000 11111111.00000000
    let e: BigFixed<D> = BigFixed {
        head: 0b00000000,
        body: vec![0b11111111],
        position: Position(0)
    };

    test(&e, Floor, -1, &[0b00000000, 0b11111111, 0b00000000, 0b00000000]);
    test(&e, Floor,  0, &[0b00000000, 0b11111111, 0b00000000, 0b00000000]);
    test(&e, Floor,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&e, Floor,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000]);

    test(&e, Ceiling, -1, &[0b00000000, 0b11111111, 0b00000000, 0b00000000]);
    test(&e, Ceiling,  0, &[0b00000000, 0b11111111, 0b00000000, 0b00000000]);
    test(&e, Ceiling,  1, &[0b00000000, 0b00000000, 0b00000001, 0b00000000]);
    test(&e, Ceiling,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000001]);
}

// unformatted cases
#[test]
fn unformatted_positive() {
    // 00000000 00000000 11111111 11111111.00000000 00000000
    let c: BigFixed<D> = BigFixed {
        head: 0b00000000,
        body: vec![0b00000000, 0b11111111, 0b11111111, 0b00000000],
        position: Position(-1)
    };

    test(&c, Floor, -2, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Floor, -1, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Floor,  0, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Floor,  1, &[0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Floor,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Floor,  3, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Floor,  4, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);

    test(&c, Ceiling, -2, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Ceiling, -1, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Ceiling,  0, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b00000000, 0b00000000, 0b00000000]);
    test(&c, Ceiling,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000, 0b00000000]);
    test(&c, Ceiling,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000, 0b00000000]);
    test(&c, Ceiling,  3, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001, 0b00000000]);
    test(&c, Ceiling,  4, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000001]);
}

#[test]
fn unformatted_negative() {
    // 11111111 11111111.00000000 00000000
    let d: BigFixed<D> = BigFixed {
        head: 0b11111111,
        body: vec![0b00000000, 0b11111111],
        position: Position(-1)
    };

    test(&d, Floor, -2, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111]);
    test(&d, Floor, -1, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111]);
    test(&d, Floor,  0, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111]);
    test(&d, Floor,  1, &[0b00000000, 0b00000000, 0b00000000, 0b11111111, 0b11111111]);
    test(&d, Floor,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b11111111]);

    test(&d, Ceiling, -2, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111]);
    test(&d, Ceiling, -1, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111]);
    test(&d, Ceiling,  0, &[0b00000000, 0b00000000, 0b11111111, 0b11111111, 0b11111111]);
    test(&d, Ceiling,  1, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
    test(&d, Ceiling,  2, &[0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000]);
}

// result is the expected read values starting from just below a's position and going until just above a's body high
fn test(a: &BigFixed<D>, round: Rounding, cutoff_index: isize, result: &[D]) {
    assert!(a.properly_positioned());
    let mut on = (a.position - Position(1)).unwrap();
    let top = a.body_high().unwrap();
    assert_eq!((top - on).unwrap().unsigned_value().unwrap() + 2, result.len(), "mismatch in expected number of terms");
    for r in result {
        let mut res = !r; // won't be equal if nothing changed
        a.index_cutoff_result_full(round, Position(cutoff_index), on, &mut res).unwrap();
        assert_eq!(res, *r, "{:?} {:?} {} {}", a, round, cutoff_index, on);
        on += Position(1);
    }
}
