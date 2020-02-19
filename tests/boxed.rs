use std::mem::transmute;

use opencv::{
    core::Scalar,
    prelude::*,
    Result,
};

#[test]
fn layout() -> Result<()> {
    let mat = Mat::new_rows_cols_with_default(1, 3, f32::typ(), Scalar::all(10.))?;
    let mut mat_ptr = mat.as_raw_Mat();
    let mat_ref: &mut Mat = unsafe { transmute(&mut mat_ptr) };
    assert_eq!(mat.size()?, mat_ref.size()?);
    assert_eq!(mat.typ()?, mat_ref.typ()?);
    assert_eq!(mat.rows(), mat_ref.rows());
    assert_eq!(mat.cols(), mat_ref.cols());
    assert_eq!(mat.at_2d::<f32>(0, 1)?, mat_ref.at_2d::<f32>(0, 1)?);
    Ok(())
}
