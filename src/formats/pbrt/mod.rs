mod param_set;

//use core::Scene;
//use linalg::{Transform, Vector3};

//use std::str::from_utf8;
//use std::str::FromStr;

//use self::para_set::{ParamSet, ParamType};


#[cfg(test)]
#[test]
fn test_signed_f64() {
    assert_eq!(signed_f64(&b"1"[..]), Done(&b""[..], Result::Ok(1.0)));
    assert_eq!(signed_f64(&b"0"[..]), Done(&b""[..], Result::Ok(0.0)));
    assert_eq!(signed_f64(&b"0.145"[..]), Done(&b""[..], Result::Ok(0.145)));
    assert_eq!(signed_f64(&b"2.45"[..]), Done(&b""[..], Result::Ok(2.45)));
}

#[cfg(test)]
#[test]
fn test_vector3() {
    assert_eq!(vector3(&b"0.0 10.0 100.0"[..]),
               Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));

    assert_eq!(vector3(&b"0 10 100"[..]),
               Done(&b""[..], Vector3::new(0.0, 10.0, 100.0)));

    assert_eq!(vector3(&b"0 -10 100"[..]),
               Done(&b""[..], Vector3::new(0.0, -10.0, 100.0)));
}

#[cfg(test)]
#[test]
fn test_look_at() {
    assert_eq!(look_at(&b"LookAt 0 10 100 0 -1 0 0 1 0"[..]),
               Done(&b""[..],
                    Transform::look_at(Vector3::new(0.0, 10.0, 100.0),
                                       Vector3::new(0.0, -1.0, 0.0),
                                       Vector3::new(0.0, 1.0, 0.0))));
}
