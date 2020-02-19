use matches::assert_matches;

use opencv::{
    core::{self, Point2d, Point3i, Scalar, SparseMat_Hdr},
    Error,
    prelude::*,
    Result,
    types::{
        VectorOfbool,
        VectorOff64,
        VectorOfi32,
        VectorOfi8,
        VectorOfMat,
        VectorOfPoint2d,
        VectorOfPoint3i,
        VectorOfString,
        VectorOfu8,
        VectorOfVectorOfPoint3i,
    },
};

#[test]
fn boxed() -> Result<()> {
    let mut vec = VectorOfMat::new();
    vec.push(Mat::new_rows_cols_with_default(1, 3, u8::typ(), Scalar::all(1.))?);
    vec.push(Mat::new_rows_cols_with_default(1, 3, u16::typ(), Scalar::all(2.))?);
    vec.push(Mat::new_rows_cols_with_default(1, 3, i32::typ(), Scalar::all(3.))?);
    assert_eq!(u8::typ(), vec.get(0)?.typ()?);
    assert_eq!(1, *vec.get(0)?.at_2d::<u8>(0, 1)?);
    assert_eq!(u16::typ(), vec.get(1)?.typ()?);
    assert_eq!(2, *vec.get(1)?.at_2d::<u16>(0, 1)?);
    assert_eq!(i32::typ(), vec.get(2)?.typ()?);
    assert_eq!(3, *vec.get(2)?.at_2d::<i32>(0, 1)?);
    vec.set(0, Mat::new_rows_cols_with_default(1, 3, f32::typ(), Scalar::all(3.))?)?;
    unsafe { vec.set_unchecked(1, Mat::new_rows_cols_with_default(1, 3, f64::typ(), Scalar::all(4.))?); }
    vec.set(2, Mat::new_rows_cols_with_default(1, 3, i16::typ(), Scalar::all(5.))?)?;
    assert_eq!(f32::typ(), unsafe { vec.get_unchecked(0) }.typ()?);
    assert_eq!(3., *unsafe { vec.get_unchecked(0) }.at_2d::<f32>(0, 1)?);
    assert_eq!(f64::typ(), unsafe { vec.get_unchecked(1) }.typ()?);
    assert_eq!(4., *unsafe { vec.get_unchecked(1) }.at_2d::<f64>(0, 1)?);
    assert_eq!(i16::typ(), unsafe { vec.get_unchecked(2) }.typ()?);
    assert_eq!(5, *unsafe { vec.get_unchecked(2) }.at_2d::<i16>(0, 1)?);

    Ok(())
}

#[test]
fn string() -> Result<()> {
    let mut vec = VectorOfString::new();
    vec.push("123");
    vec.push("456");
    vec.push("789");
    vec.push("888\0999");
    vec.push("\0999");
    assert_eq!(5, vec.len());
    assert_eq!("123", vec.get(0)?);
    assert_eq!("456", vec.get(1)?);
    assert_eq!("789", vec.get(2)?);
    assert_eq!("888", vec.get(3)?);
    assert_eq!("", vec.get(4)?);
    vec.set(0, "qqq")?;
    unsafe { vec.set_unchecked(1, "www"); }
    vec.set(2, "eee")?;
    assert_eq!("qqq", unsafe { vec.get_unchecked(0) });
    assert_eq!("www", unsafe { vec.get_unchecked(1) });
    assert_eq!("eee", unsafe { vec.get_unchecked(2) });
    Ok(())
}

#[test]
fn boolean() -> Result<()> {
    let mut vec = VectorOfbool::new();
    vec.push(true);
    vec.push(true);
    vec.push(false);
    assert_eq!(true, vec.get(0)?);
    assert_eq!(true, vec.get(1)?);
    assert_eq!(false, vec.get(2)?);
    vec.set(0, false)?;
    unsafe { vec.set_unchecked(1, true); }
    vec.set(2, true)?;
    assert_eq!(false, unsafe { vec.get_unchecked(0) });
    assert_eq!(true, unsafe { vec.get_unchecked(1) });
    assert_eq!(true, unsafe { vec.get_unchecked(2) });
    Ok(())
}

#[test]
fn primitive() -> Result<()> {
    let mut vec = VectorOfi32::with_capacity(10);
    vec.push(1);
    vec.push(2);
    vec.push(3);
    assert_eq!(1, vec.get(0)?);
    assert_eq!(2, vec.get(1)?);
    assert_eq!(3, vec.get(2)?);
    vec.set(0, 4)?;
    vec.set(1, 5)?;
    unsafe { vec.set_unchecked(2, 6) };
    assert_eq!(4, unsafe { vec.get_unchecked(0) });
    assert_eq!(5, unsafe { vec.get_unchecked(1) });
    assert_eq!(6, unsafe { vec.get_unchecked(2) });
    Ok(())
}

#[test]
fn simple_struct() -> Result<()> {
    let mut vec = VectorOfPoint2d::new();
    vec.push(Point2d::new(10., 10.));
    vec.push(Point2d::new(20., 20.));
    vec.push(Point2d::new(30., 30.));
    assert_eq!(Point2d::new(10., 10.), vec.get(0)?);
    assert_eq!(Point2d::new(20., 20.), vec.get(1)?);
    assert_eq!(Point2d::new(30., 30.), vec.get(2)?);
    unsafe { vec.set_unchecked(0, Point2d::new(40., 50.)) };
    vec.set(1, Point2d::new(50., 60.))?;
    vec.set(2, Point2d::new(60., 70.))?;
    assert_eq!(Point2d::new(40., 50.), unsafe { vec.get_unchecked(0) });
    assert_eq!(Point2d::new(50., 60.), unsafe { vec.get_unchecked(1) });
    assert_eq!(Point2d::new(60., 70.), unsafe { vec.get_unchecked(2) });
    Ok(())
}

#[test]
fn vector_of_vector_simple_struct() -> Result<()> {
    #[inline(never)]
    fn make_vec() -> VectorOfVectorOfPoint3i {
        let mut outer = VectorOfVectorOfPoint3i::new();
        outer.push({
            let mut inner = VectorOfPoint3i::new();
            inner.push(Point3i::new(1, 1, 1));
            inner.push(Point3i::new(2, 2, 2));
            inner.push(Point3i::new(3, 3, 3));
            inner
        });
        outer.push({
            let mut inner = VectorOfPoint3i::new();
            inner.push(Point3i::new(4, 4, 4));
            inner.push(Point3i::new(5, 5, 5));
            inner.push(Point3i::new(6, 6, 6));
            inner
        });
        outer.push({
            let mut inner = VectorOfPoint3i::new();
            inner.push(Point3i::new(7, 7, 7));
            inner.push(Point3i::new(8, 8, 8));
            inner.push(Point3i::new(9, 9, 9));
            inner
        });
        outer
    }

    let mut outer = make_vec();
    assert_eq!(6, outer.get(1)?.get(2)?.x);
    outer.remove(1)?;
    assert_eq!(9, outer.get(1)?.get(2)?.x);
    Ok(())
}

#[test]
fn capacity() {
    {
        let mut vec = VectorOfi32::with_capacity(0);
        assert_eq!(0, vec.len());
        assert!(vec.is_empty());
        assert_eq!(0, vec.capacity());
        vec.shrink_to_fit();
        assert_eq!(0, vec.capacity());
    }

    {
        let mut vec = VectorOfi32::with_capacity(10);
        assert_eq!(0, vec.len());
        assert!(vec.is_empty());
        assert_eq!(10, vec.capacity());
        vec.push(1);
        assert_eq!(1, vec.len());
        assert!(!vec.is_empty());
        assert_eq!(10, vec.capacity());
        vec.reserve(10);
        assert_eq!(1, vec.len());
        assert_eq!(11, vec.capacity());
        vec.reserve(10);
        assert!(!vec.is_empty());
        assert_eq!(11, vec.capacity());
        vec.shrink_to_fit();
        assert_eq!(1, vec.capacity());
    }

    {
        let mut vec = VectorOfbool::new();
        assert_eq!(0, vec.len());
        assert!(vec.is_empty());
        assert_eq!(0, vec.capacity());
        vec.push(true);
        assert_eq!(1, vec.len());
        assert!(!vec.is_empty());
        assert!(vec.capacity() > 0);
    }
}

#[test]
fn insert() -> Result<()> {
    let mut vec = VectorOfi32::from_iter(vec![1, 2, 3]);
    vec.insert(1, 4)?;
    assert_eq!(4, vec.len());
    assert_eq!(4, vec.get(1)?);
    vec.insert(0, 8)?;
    assert_eq!(5, vec.len());
    assert_eq!(8, vec.get(0)?);
    assert_eq!(1, vec.get(1)?);
    assert_matches!(vec.insert(10, 10), Err(Error { code: core::StsOutOfRange, .. }));
    vec.insert(5, 10)?;
    assert_eq!(6, vec.len());
    assert_eq!(10, vec.get(5)?);
    Ok(())
}

#[test]
fn remove() -> Result<()> {
    let mut vec = VectorOfi32::with_capacity(10);
    vec.push(10);
    vec.push(20);
    vec.push(30);
    assert_eq!(3, vec.len());
    assert_eq!(10, vec.capacity());
    vec.remove(1)?;
    assert_eq!(2, vec.len());
    assert_eq!(10, vec.get(0)?);
    assert_eq!(30, vec.get(1)?);
    vec.remove(0)?;
    assert_eq!(1, vec.len());
    assert_eq!(30, vec.get(0)?);
    vec.clear();
    assert_eq!(0, vec.len());
    assert_eq!(10, vec.capacity());
    assert_matches!(vec.remove(0), Err(Error { code: core::StsOutOfRange, .. }));
    Ok(())
}

#[test]
fn swap() -> Result<()> {
    {
        let mut vec = VectorOfi32::from_iter(vec![1, 2, 3]);
        vec.swap(0, 0)?;
        vec.swap(0, 1)?;
        assert_eq!(2, vec.get(0)?);
        assert_eq!(1, vec.get(1)?);

        vec.swap(2, 0)?;
        assert_eq!(3, vec.get(0)?);
        assert_eq!(1, vec.get(1)?);

        assert_matches!(vec.swap(0, 4), Err(Error { code: core::StsOutOfRange, .. }));
        assert_matches!(vec.swap(6, 1), Err(Error { code: core::StsOutOfRange, .. }));
    }

    {
        let mut vec = VectorOfString::new();
        vec.push("123");
        vec.push("456");
        vec.push("789");
        vec.swap(0, 2)?;
        assert_eq!("789", vec.get(0)?);
        assert_eq!("456", vec.get(1)?);
        assert_eq!("123", vec.get(2)?);
    }

    Ok(())
}

#[test]
fn nth() -> Result<()> {
    {
        let mut vec = VectorOfi32::new();
        assert_eq!(None, vec.iter().nth(0));
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(Some(1), vec.iter().nth(0));
        assert_eq!(Some(2), vec.iter().nth(1));
        assert_eq!(Some(3), vec.iter().nth(2));
    }

    Ok(())
}

#[test]
fn out_of_bounds() -> Result<()> {
    let mut vec = VectorOff64::new();
    assert_matches!(vec.get(0), Err(Error { code: core::StsOutOfRange, .. }));
    vec.push(1.);
    vec.push(2.);
    assert_matches!(vec.get(3), Err(Error { code: core::StsOutOfRange, .. }));
    assert_matches!(vec.set(3, 5.), Err(Error { code: core::StsOutOfRange, .. }));
    Ok(())
}

#[test]
fn from_iter() -> Result<()> {
    {
        let vec = VectorOfi8::from_iter(vec![1, 2, 3]);
        assert_eq!(3, vec.len());
        assert_eq!(1, vec.get(0)?);
        assert_eq!(2, vec.get(1)?);
        assert_eq!(3, vec.get(2)?);
    }

    {
        let vec = VectorOfi8::from_iter([1, 2, 3].iter().map(|x| *x));
        assert_eq!(3, vec.len());
        assert_eq!(1, vec.get(0)?);
        assert_eq!(2, vec.get(1)?);
        assert_eq!(3, vec.get(2)?);
    }

    {
        let vec = VectorOfi8::from_iter(1..=3);
        assert_eq!(3, vec.len());
        assert_eq!(1, vec.get(0)?);
        assert_eq!(2, vec.get(1)?);
        assert_eq!(3, vec.get(2)?);
    }

    Ok(())
}

#[test]
fn iter() -> Result<()> {
    {
        let vec = VectorOfi32::from_iter(vec![1, 2, 3, 4]);
        let mut sum = 0;
        for i in &vec {
            sum += i;
        }
        assert_eq!(10, sum);
        for i in vec {
            sum += i;
        }
        assert_eq!(20, sum);
    }

    {
        let vec = VectorOfMat::from_iter(vec![]);
        for _ in vec {
            assert!(false, "iterator must not yield any elements")
        }
    }

    Ok(())
}

#[test]
fn to_slice() -> Result<()> {
    {
        let vec = VectorOfu8::from_iter(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec.to_slice(), &[1, 2, 3, 4, 5]);
    }
    {
        let vec = VectorOfi32::new();
        assert_eq!(vec.to_slice(), &[]);
    }
    {
        let vec = VectorOfPoint2d::from_iter(vec![Point2d::new(10., 20.), Point2d::new(60.5, 90.3), Point2d::new(-40.333, 89.)]);
        let slice = vec.to_slice();
        assert_eq!(20., slice[0].y);
        assert_eq!(60.5, slice[1].x);
        assert_eq!(Point2d::new(-40.333, 89.), slice[2]);
    }
    Ok(())
}

#[test]
fn to_vec() -> Result<()> {
    {
        let vec = VectorOfu8::from_iter(vec![1, 2, 3, 4, 5]);
        assert_eq!(vec.to_vec(), vec![1, 2, 3, 4, 5]);
    }
    {
        let vec = VectorOfu8::new();
        assert_eq!(vec.to_vec(), Vec::new());
    }
    Ok(())
}

#[test]
fn property() -> Result<()> {
    let mut hdr = SparseMat_Hdr::new(&[4, 2], i32::typ())?;
    #[inline(never)]
    fn f(hdr: &mut SparseMat_Hdr, pool: VectorOfu8) {
        hdr.set_pool(pool);
    }
    let pool = VectorOfu8::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9].iter().copied());
    f(&mut hdr, pool);
    let pool = VectorOfu8::from_iter([11, 12, 13, 14, 15, 16, 17, 18, 19].iter().copied());
    let pool_out = hdr.pool();
    assert_eq!(pool_out.get(0)?, pool.get(0)? - 10);
    assert_eq!(pool_out.get(2)?, pool.get(2)? - 10);
    assert_eq!(pool_out.get(4)?, pool.get(4)? - 10);
    assert_eq!(pool_out.get(8)?, pool.get(8)? - 10);
    assert_eq!(pool_out.len(), pool.len());
    Ok(())
}
