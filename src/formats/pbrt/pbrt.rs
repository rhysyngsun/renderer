use std::str::{self, FromStr};

use nom::{digit, space};

use linalg::{Transform, Vector3};

named!(unsigned_float<f64>, map_res!(
  map_res!(
    recognize!(
      alt_complete!(
        delimited!(digit, tag!("."), opt!(complete!(digit))) |
        delimited!(opt!(digit), tag!("."), digit)            |
        digit
      )
    ),
    str::from_utf8
  ),
  FromStr::from_str
));

named!(signed_float<f64>, map!(
    pair!(
        opt!(alt!(tag!("+") | tag!("-"))),
        unsigned_float
    ),
    |(sign, value): (Option<&[u8]>, f64)| {
        sign.and_then(
            |s| if s[0] == ('-' as u8) { Some(-1f64) } else { None }
        ).unwrap_or(1f64) * value
    }
));

named!(vector3<Vector3>, do_parse!(
    x: signed_float >>
              space >>
    y: signed_float >>
              space >>
    z: signed_float >>
    (Vector3::new(x, y, z))
));

named!(look_at<Transform>, do_parse!(
    tag!("LookAt") >>
             space >>
      eye: vector3 >>
             space >>
     look: vector3 >>
             space >>
       up: vector3 >>
    (Transform::look_at(eye, look, up))
));

named!(transformation<Transform>, alt!(
    look_at
));

#[cfg(test)]
mod tests {
    use nom::IResult::Done;

    use linalg::Vector3;

    use super::*;

    #[test]
    fn test_unsigned_float() {
        assert_eq!(unsigned_float(&b"134"[..]),   Done(&b""[..], 134.0));
        assert_eq!(unsigned_float(&b"1234."[..]), Done(&b""[..], 1234.0));
        assert_eq!(unsigned_float(&b"0.145"[..]), Done(&b""[..], 0.145));
        assert_eq!(unsigned_float(&b"2.45"[..]),  Done(&b""[..], 2.45));
    }

    #[test]
    fn test_signed_float() {
        assert_eq!(signed_float(&b"134"[..]),    Done(&b""[..], 134.0));
        assert_eq!(signed_float(&b"-1234."[..]), Done(&b""[..], -1234.0));
        assert_eq!(signed_float(&b"0.145"[..]),  Done(&b""[..], 0.145));
        assert_eq!(signed_float(&b"-2.45"[..]),  Done(&b""[..], -2.45));
    }

    #[test]
    fn test_vector3() {
        assert_eq!(vector3(&b"0.0 10.0 100.0"[..]),
                   Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));

        assert_eq!(vector3(&b"0 10 100"[..]),
                   Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));

        assert_eq!(vector3(&b"0 -10 100"[..]),
                   Done(&b""[..], Vector3::new(0.0, -10.0, 100.0)));
    }

    #[test]
    #[ignore]
    fn test_look_at() {
        assert_eq!(
            look_at(&b"LookAt 0 10 100 0 -1 0 0 1 0"[..]),
            Done(
                &b""[..],
                Transform::look_at(
                    Vector3::new(0.0, 10.0, 100.0),
                    Vector3::new(0.0, -1.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0)
                )
            )
        );
    }
}
