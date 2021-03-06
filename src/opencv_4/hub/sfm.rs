//! # Structure From Motion
//!
//! The opencv_sfm module contains algorithms to perform 3d reconstruction
//! from 2d images.
//!
//! The core of the module is based on a light version of
//! [Libmv](https://developer.blender.org/project/profile/59) originally
//! developed by Sameer Agarwal and Keir Mierle.
//!
//! __Whats is libmv?__
//!
//! libmv, also known as the Library for Multiview Reconstruction (or LMV),
//! is the computer vision backend for Blender's motion tracking abilities.
//! Unlike other vision libraries with general ambitions, libmv is focused
//! on algorithms for match moving, specifically targeting [Blender](https://developer.blender.org) as the
//! primary customer. Dense reconstruction, reconstruction from unorganized
//! photo collections, image recognition, and other tasks are not a focus
//! of libmv.
//!
//! __Development__
//!
//! libmv is officially under the Blender umbrella, and so is developed
//! on developer.blender.org. The [source repository](https://developer.blender.org/diffusion/LMV) can get checked out
//! independently from Blender.
//!
//! This module has been originally developed as a project for Google Summer of Code 2012-2015.
//!
//!
//! Note:
//! - Notice that it is compiled only when Eigen, GLog and GFlags are correctly installed.
//!
//! Check installation instructions in the following tutorial: @ref tutorial_sfm_installation
//! # Conditioning
//! # Fundamental
//! # Input/Output
//! # Numeric
//! # Projection
//! # Robust Estimation
//! # Triangulation
//!
//! # Reconstruction
//!
//! Note:
//! - Notice that it is compiled only when Ceres Solver is correctly installed.
//!
//! Check installation instructions in the following tutorial: @ref tutorial_sfm_installation
//!
//!
//! # Simple Pipeline
//!
//! Note:
//! - Notice that it is compiled only when Ceres Solver is correctly installed.
//!
//! Check installation instructions in the following tutorial: @ref tutorial_sfm_installation
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};

pub const SFM_DISTORTION_MODEL_DIVISION: i32 = 1;
pub const SFM_DISTORTION_MODEL_POLYNOMIAL: i32 = 0;
pub const SFM_IO_BUNDLER: i32 = 0;
pub const SFM_IO_OPENMVG: i32 = 3;
pub const SFM_IO_OPENSFM: i32 = 2;
pub const SFM_IO_THEIASFM: i32 = 4;
pub const SFM_IO_VISUALSFM: i32 = 1;
pub const SFM_REFINE_FOCAL_LENGTH: i32 = (1 << 0);
pub const SFM_REFINE_PRINCIPAL_POINT: i32 = (1 << 1);
pub const SFM_REFINE_RADIAL_DISTORTION_K1: i32 = (1 << 2);
pub const SFM_REFINE_RADIAL_DISTORTION_K2: i32 = (1 << 4);

/// Data structure describing the camera model and its parameters.
/// ## Parameters
/// * _distortion_model: Type of camera model.
/// * _focal_length_x: focal length of the camera (in pixels).
/// * _focal_length_y: focal length of the camera (in pixels).
/// * _principal_point_x: principal point of the camera in the x direction (in pixels).
/// * _principal_point_y: principal point of the camera in the y direction (in pixels).
/// * _polynomial_k1: radial distortion parameter.
/// * _polynomial_k2: radial distortion parameter.
/// * _polynomial_k3: radial distortion parameter.
/// * _polynomial_p1: radial distortion parameter.
/// * _polynomial_p2: radial distortion parameter.
///
/// Is assumed that modern cameras have their principal point in the image center.
///
/// In case that the camera model was SFM_DISTORTION_MODEL_DIVISION, it's only needed to provide
/// _polynomial_k1 and _polynomial_k2 which will be assigned as division distortion parameters.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct libmv_CameraIntrinsicsOptions {
    pub distortion_model: i32,
    pub image_width: i32,
    pub image_height: i32,
    pub focal_length_x: f64,
    pub focal_length_y: f64,
    pub principal_point_x: f64,
    pub principal_point_y: f64,
    pub polynomial_k1: f64,
    pub polynomial_k2: f64,
    pub polynomial_k3: f64,
    pub polynomial_p1: f64,
    pub polynomial_p2: f64,
    pub division_k1: f64,
    pub division_k2: f64,
}

/// Data structure describing the reconstruction options.
/// ## Parameters
/// * _keyframe1: first keyframe used in order to initialize the reconstruction.
/// * _keyframe2: second keyframe used in order to initialize the reconstruction.
/// * _refine_intrinsics: camera parameter or combination of parameters to refine.
/// * _select_keyframes: allows to select automatically the initial keyframes. If 1 then autoselection is enabled. If 0 then is disabled.
/// * _verbosity_level: verbosity logs level for Glog. If -1 then logs are disabled, otherwise the log level will be the input integer.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct libmv_ReconstructionOptions {
    pub keyframe1: i32,
    pub keyframe2: i32,
    pub refine_intrinsics: i32,
    pub select_keyframes: i32,
    pub verbosity_level: i32,
}

/// Get K, R and t from projection matrix P, decompose using the RQ decomposition.
/// ## Parameters
/// * P: Input 3x4 projection matrix.
/// * K: Output 3x3 camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * R: Output 3x3 rotation matrix.
/// * t: Output 3x1 translation vector.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) A4.1.1 pag.579
pub fn k_rt_from_projection(p: &dyn core::ToInputArray, k: &mut dyn core::ToOutputArray, r: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(p);
    output_array_arg!(k);
    output_array_arg!(r);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_KRtFromProjection__InputArray__OutputArray__OutputArray__OutputArray(p.as_raw__InputArray(), k.as_raw__OutputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray()) }.into_result()
}

/// Apply Transformation to points.
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Input 3x3 transformation matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20T%2AX), where ![inline formula](https://latex.codecogs.com/png.latex?X) are the points to transform and ![inline formula](https://latex.codecogs.com/png.latex?x) the transformed points.
/// * transformed_points: Output vector of N-dimensional transformed points.
pub fn apply_transformation_to_points(points: &dyn core::ToInputArray, t: &dyn core::ToInputArray, transformed_points: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(points);
    input_array_arg!(t);
    output_array_arg!(transformed_points);
    unsafe { sys::cv_sfm_applyTransformationToPoints__InputArray__InputArray__OutputArray(points.as_raw__InputArray(), t.as_raw__InputArray(), transformed_points.as_raw__OutputArray()) }.into_result()
}

/// Computes Absolute or Exterior Orientation (Pose Estimation) between 2 sets of 3D point.
/// ## Parameters
/// * x1: Input first 3xN or 2xN array of points.
/// * x2: Input second 3xN or 2xN array of points.
/// * R: Output 3x3 computed rotation matrix.
/// * t: Output 3x1 computed translation vector.
/// * s: Output computed scale factor.
///
/// Find the best transformation such that xp=projection*(s*R*x+t) (same as Pose Estimation, ePNP).
/// The routines below are only for the orthographic case for now.
pub fn compute_orientation(x1: &dyn core::ToInputArray, x2: &dyn core::ToInputArray, r: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray, s: f64) -> Result<()> {
    input_array_arg!(x1);
    input_array_arg!(x2);
    output_array_arg!(r);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_computeOrientation__InputArray__InputArray__OutputArray__OutputArray_double(x1.as_raw__InputArray(), x2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray(), s) }.into_result()
}

/// Returns the depth of a point transformed by a rigid transform.
/// ## Parameters
/// * R: Input 3x3 rotation matrix.
/// * t: Input 3x1 translation vector.
/// * X: Input 3x1 or 4x1 vector with the 3d point.
pub fn depth(r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, x: &dyn core::ToInputArray) -> Result<f64> {
    input_array_arg!(r);
    input_array_arg!(t);
    input_array_arg!(x);
    unsafe { sys::cv_sfm_depth__InputArray__InputArray__InputArray(r.as_raw__InputArray(), t.as_raw__InputArray(), x.as_raw__InputArray()) }.into_result()
}

/// Get Essential matrix from Fundamental and Camera matrices.
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * K1: Input 3x3 first camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * E: Output 3x3 essential matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12)
pub fn essential_from_fundamental(f: &dyn core::ToInputArray, k1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, e: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(f);
    input_array_arg!(k1);
    input_array_arg!(k2);
    output_array_arg!(e);
    unsafe { sys::cv_sfm_essentialFromFundamental__InputArray__InputArray__InputArray__OutputArray(f.as_raw__InputArray(), k1.as_raw__InputArray(), k2.as_raw__InputArray(), e.as_raw__OutputArray()) }.into_result()
}

/// Get Essential matrix from Motion (R's and t's ).
/// ## Parameters
/// * R1: Input 3x3 first camera rotation matrix.
/// * t1: Input 3x1 first camera translation vector.
/// * R2: Input 3x3 second camera rotation matrix.
/// * t2: Input 3x1 second camera translation vector.
/// * E: Output 3x3 essential matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12)
pub fn essential_from_rt(r1: &dyn core::ToInputArray, t1: &dyn core::ToInputArray, r2: &dyn core::ToInputArray, t2: &dyn core::ToInputArray, e: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(r1);
    input_array_arg!(t1);
    input_array_arg!(r2);
    input_array_arg!(t2);
    output_array_arg!(e);
    unsafe { sys::cv_sfm_essentialFromRt__InputArray__InputArray__InputArray__InputArray__OutputArray(r1.as_raw__InputArray(), t1.as_raw__InputArray(), r2.as_raw__InputArray(), t2.as_raw__InputArray(), e.as_raw__OutputArray()) }.into_result()
}

/// Converts points from Euclidean to homogeneous space. E.g., ((x,y)->(x,y,1))
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N+1-dimensional points.
pub fn euclidean_to_homogeneous(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(src);
    output_array_arg!(dst);
    unsafe { sys::cv_sfm_euclideanToHomogeneous__InputArray__OutputArray(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x_2%5ET%20F%20x_1%3D0).
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
/// The number of iterations is controlled using the following equation:
/// ![inline formula](https://latex.codecogs.com/png.latex?k%20%3D%20%5Cfrac%7Blog%281-p%29%7D%7Blog%281.0%20-%20w%5En%20%29%7D) where ![inline formula](https://latex.codecogs.com/png.latex?k), ![inline formula](https://latex.codecogs.com/png.latex?w) and ![inline formula](https://latex.codecogs.com/png.latex?n) are the number of
/// iterations, the inliers ratio and minimun number of selected independent samples.
/// The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 7 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## C++ default parameters
/// * outliers_probability: 1e-2
pub fn fundamental_from_correspondences7_point_robust(x1: &dyn core::ToInputArray, x2: &dyn core::ToInputArray, max_error: f64, f: &mut dyn core::ToOutputArray, inliers: &mut dyn core::ToOutputArray, outliers_probability: f64) -> Result<f64> {
    input_array_arg!(x1);
    input_array_arg!(x2);
    output_array_arg!(f);
    output_array_arg!(inliers);
    unsafe { sys::cv_sfm_fundamentalFromCorrespondences7PointRobust__InputArray__InputArray_double__OutputArray__OutputArray_double(x1.as_raw__InputArray(), x2.as_raw__InputArray(), max_error, f.as_raw__OutputArray(), inliers.as_raw__OutputArray(), outliers_probability) }.into_result()
}

/// Estimate robustly the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * max_error: maximum error (in pixels).
/// * F: Output 3x3 fundamental matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x_2%5ET%20F%20x_1%3D0).
/// * inliers: Output 1xN vector that contains the indexes of the detected inliers.
/// * outliers_probability: outliers probability (in ]0,1[).
/// The number of iterations is controlled using the following equation:
/// ![inline formula](https://latex.codecogs.com/png.latex?k%20%3D%20%5Cfrac%7Blog%281-p%29%7D%7Blog%281.0%20-%20w%5En%20%29%7D) where ![inline formula](https://latex.codecogs.com/png.latex?k), ![inline formula](https://latex.codecogs.com/png.latex?w) and ![inline formula](https://latex.codecogs.com/png.latex?n) are the number of
/// iterations, the inliers ratio and minimun number of selected independent samples.
/// The more this value is high, the less the function selects ramdom samples.
///
/// The fundamental solver relies on the 8 point solution. Returns the best error (in pixels), associated to the solution F.
///
/// ## C++ default parameters
/// * outliers_probability: 1e-2
pub fn fundamental_from_correspondences8_point_robust(x1: &dyn core::ToInputArray, x2: &dyn core::ToInputArray, max_error: f64, f: &mut dyn core::ToOutputArray, inliers: &mut dyn core::ToOutputArray, outliers_probability: f64) -> Result<f64> {
    input_array_arg!(x1);
    input_array_arg!(x2);
    output_array_arg!(f);
    output_array_arg!(inliers);
    unsafe { sys::cv_sfm_fundamentalFromCorrespondences8PointRobust__InputArray__InputArray_double__OutputArray__OutputArray_double(x1.as_raw__InputArray(), x2.as_raw__InputArray(), max_error, f.as_raw__OutputArray(), inliers.as_raw__OutputArray(), outliers_probability) }.into_result()
}

/// Get Essential matrix from Fundamental and Camera matrices.
/// ## Parameters
/// * E: Input 3x3 essential matrix.
/// * K1: Input 3x3 first camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * F: Output 3x3 fundamental matrix.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 257 (formula 9.12) or http://ai.stanford.edu/~birch/projective/node20.html
pub fn fundamental_from_essential(e: &dyn core::ToInputArray, k1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, f: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(e);
    input_array_arg!(k1);
    input_array_arg!(k2);
    output_array_arg!(f);
    unsafe { sys::cv_sfm_fundamentalFromEssential__InputArray__InputArray__InputArray__OutputArray(e.as_raw__InputArray(), k1.as_raw__InputArray(), k2.as_raw__InputArray(), f.as_raw__OutputArray()) }.into_result()
}

/// Get Fundamental matrix from Projection matrices.
/// ## Parameters
/// * P1: Input 3x4 first projection matrix.
/// * P2: Input 3x4 second projection matrix.
/// * F: Output 3x3 fundamental matrix.
pub fn fundamental_from_projections(p1: &dyn core::ToInputArray, p2: &dyn core::ToInputArray, f: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(p1);
    input_array_arg!(p2);
    output_array_arg!(f);
    unsafe { sys::cv_sfm_fundamentalFromProjections__InputArray__InputArray__OutputArray(p1.as_raw__InputArray(), p2.as_raw__InputArray(), f.as_raw__OutputArray()) }.into_result()
}

/// Converts point coordinates from homogeneous to euclidean pixel coordinates. E.g., ((x,y,z)->(x/z, y/z))
/// ## Parameters
/// * src: Input vector of N-dimensional points.
/// * dst: Output vector of N-1-dimensional points.
pub fn homogeneous_to_euclidean(src: &dyn core::ToInputArray, dst: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(src);
    output_array_arg!(dst);
    unsafe { sys::cv_sfm_homogeneousToEuclidean__InputArray__OutputArray(src.as_raw__InputArray(), dst.as_raw__OutputArray()) }.into_result()
}

/// Import a reconstruction file.
/// ## Parameters
/// * file: The path to the file.
/// * Rs: Output vector of 3x3 rotations of the camera
/// * Ts: Output vector of 3x1 translations of the camera.
/// * Ks: Output vector of 3x3 instrinsics of the camera.
/// * points3d: Output array with 3d points. Is 3 x N.
/// * file_format: The format of the file to import.
///
/// The function supports reconstructions from Bundler.
///
/// ## C++ default parameters
/// * file_format: SFM_IO_BUNDLER
pub fn import_reconstruction(file: &str, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, ks: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray, file_format: i32) -> Result<()> {
    string_arg!(file);
    output_array_arg!(rs);
    output_array_arg!(ts);
    output_array_arg!(ks);
    output_array_arg!(points3d);
    unsafe { sys::cv_sfm_importReconstruction_String__OutputArray__OutputArray__OutputArray__OutputArray_int(file.as_ptr(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ks.as_raw__OutputArray(), points3d.as_raw__OutputArray(), file_format) }.into_result()
}

/// Point conditioning (isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Output 3x3 transformation matrix.
///
/// Computes the transformation matrix such that each coordinate direction will be scaled equally,
/// bringing the centroid to the origin with an average centroid ![inline formula](https://latex.codecogs.com/png.latex?%281%2C1%2C1%29%5ET).
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.107.
pub fn isotropic_preconditioner_from_points(points: &dyn core::ToInputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(points);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_isotropicPreconditionerFromPoints__InputArray__OutputArray(points.as_raw__InputArray(), t.as_raw__OutputArray()) }.into_result()
}

/// Computes the mean and variance of a given matrix along its rows.
/// ## Parameters
/// * A: Input NxN matrix.
/// * mean: Output Nx1 matrix with computed mean.
/// * variance: Output Nx1 matrix with computed variance.
///
/// It computes in the same way as woud do @ref reduce but with \a Variance function.
pub fn mean_and_variance_along_rows(a: &dyn core::ToInputArray, mean: &mut dyn core::ToOutputArray, variance: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(a);
    output_array_arg!(mean);
    output_array_arg!(variance);
    unsafe { sys::cv_sfm_meanAndVarianceAlongRows__InputArray__OutputArray__OutputArray(a.as_raw__InputArray(), mean.as_raw__OutputArray(), variance.as_raw__OutputArray()) }.into_result()
}

/// Choose one of the four possible motion solutions from an essential matrix.
/// ## Parameters
/// * Rs: Input vector of 3x3 rotation matrices.
/// * ts: Input vector of 3x1 translation vectors.
/// * K1: Input 3x3 first camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * x1: Input 2x1 vector with first 2d point.
/// * K2: Input 3x3 second camera matrix. The parameters are similar to K1.
/// * x2: Input 2x1 vector with second 2d point.
///
/// Decides the right solution by checking that the triangulation of a match
/// x1--x2 lies in front of the cameras. Return index of the right solution or -1 if no solution.
///
/// Reference: See [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 259 (9.6.3 Geometrical interpretation of the 4 solutions).
pub fn motion_from_essential_choose_solution(rs: &dyn core::ToInputArray, ts: &dyn core::ToInputArray, k1: &dyn core::ToInputArray, x1: &dyn core::ToInputArray, k2: &dyn core::ToInputArray, x2: &dyn core::ToInputArray) -> Result<i32> {
    input_array_arg!(rs);
    input_array_arg!(ts);
    input_array_arg!(k1);
    input_array_arg!(x1);
    input_array_arg!(k2);
    input_array_arg!(x2);
    unsafe { sys::cv_sfm_motionFromEssentialChooseSolution__InputArray__InputArray__InputArray__InputArray__InputArray__InputArray(rs.as_raw__InputArray(), ts.as_raw__InputArray(), k1.as_raw__InputArray(), x1.as_raw__InputArray(), k2.as_raw__InputArray(), x2.as_raw__InputArray()) }.into_result()
}

/// Get Motion (R's and t's ) from Essential matrix.
/// ## Parameters
/// * E: Input 3x3 essential matrix.
/// * Rs: Output vector of 3x3 rotation matrices.
/// * ts: Output vector of 3x1 translation vectors.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 9.6 pag 259 (Result 9.19)
pub fn motion_from_essential(e: &dyn core::ToInputArray, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(e);
    output_array_arg!(rs);
    output_array_arg!(ts);
    unsafe { sys::cv_sfm_motionFromEssential__InputArray__OutputArray__OutputArray(e.as_raw__InputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray()) }.into_result()
}

/// Normalizes the Fundamental matrix.
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * F_normalized: Output 3x3 normalized fundamental matrix.
///
/// By default divides the fundamental matrix by its L2 norm.
pub fn normalize_fundamental(f: &dyn core::ToInputArray, f_normalized: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(f);
    output_array_arg!(f_normalized);
    unsafe { sys::cv_sfm_normalizeFundamental__InputArray__OutputArray(f.as_raw__InputArray(), f_normalized.as_raw__OutputArray()) }.into_result()
}

/// This function normalizes points. (isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * normalized_points: Output vector of the same N-dimensional points but with mean 0 and average norm ![inline formula](https://latex.codecogs.com/png.latex?%5Csqrt%7B2%7D).
/// * T: Output 3x3 transform matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20T%2AX), where ![inline formula](https://latex.codecogs.com/png.latex?X) are the points to normalize and ![inline formula](https://latex.codecogs.com/png.latex?x) the normalized points.
///
/// Internally calls @ref preconditionerFromPoints in order to get the scaling matrix before applying @ref applyTransformationToPoints.
/// This operation is an essential step before applying the DLT algorithm in order to consider the result as optimal.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.107.
pub fn normalize_isotropic_points(points: &dyn core::ToInputArray, normalized_points: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(points);
    output_array_arg!(normalized_points);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_normalizeIsotropicPoints__InputArray__OutputArray__OutputArray(points.as_raw__InputArray(), normalized_points.as_raw__OutputArray(), t.as_raw__OutputArray()) }.into_result()
}

/// This function normalizes points (non isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * normalized_points: Output vector of the same N-dimensional points but with mean 0 and average norm ![inline formula](https://latex.codecogs.com/png.latex?%5Csqrt%7B2%7D).
/// * T: Output 3x3 transform matrix such that ![inline formula](https://latex.codecogs.com/png.latex?x%20%3D%20T%2AX), where ![inline formula](https://latex.codecogs.com/png.latex?X) are the points to normalize and ![inline formula](https://latex.codecogs.com/png.latex?x) the normalized points.
///
/// Internally calls @ref preconditionerFromPoints in order to get the scaling matrix before applying @ref applyTransformationToPoints.
/// This operation is an essential step before applying the DLT algorithm in order to consider the result as optimal.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.109
pub fn normalize_points(points: &dyn core::ToInputArray, normalized_points: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(points);
    output_array_arg!(normalized_points);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_normalizePoints__InputArray__OutputArray__OutputArray(points.as_raw__InputArray(), normalized_points.as_raw__OutputArray(), t.as_raw__OutputArray()) }.into_result()
}

/// Estimate the fundamental matrix between two dataset of 2D point (image coords space).
/// ## Parameters
/// * x1: Input 2xN Array of 2D points in view 1.
/// * x2: Input 2xN Array of 2D points in view 2.
/// * F: Output 3x3 fundamental matrix.
///
/// Uses the normalized 8-point fundamental matrix solver.
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 11.2 pag.281 (x1 = x, x2 = x')
pub fn normalized_eight_point_solver(x1: &dyn core::ToInputArray, x2: &dyn core::ToInputArray, f: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(x1);
    input_array_arg!(x2);
    output_array_arg!(f);
    unsafe { sys::cv_sfm_normalizedEightPointSolver__InputArray__InputArray__OutputArray(x1.as_raw__InputArray(), x2.as_raw__InputArray(), f.as_raw__OutputArray()) }.into_result()
}

/// Point conditioning (non isotropic).
/// ## Parameters
/// * points: Input vector of N-dimensional points.
/// * T: Output 3x3 transformation matrix.
///
/// Computes the transformation matrix such that the two principal moments of the set of points are equal to unity,
/// forming an approximately symmetric circular cloud of points of radius 1 about the origin.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 4.4.4 pag.109
pub fn preconditioner_from_points(points: &dyn core::ToInputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(points);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_preconditionerFromPoints__InputArray__OutputArray(points.as_raw__InputArray(), t.as_raw__OutputArray()) }.into_result()
}

/// Get projection matrix P from K, R and t.
/// ## Parameters
/// * K: Input 3x3 camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D).
/// * R: Input 3x3 rotation matrix.
/// * t: Input 3x1 translation vector.
/// * P: Output 3x4 projection matrix.
///
/// This function estimate the projection matrix by solving the following equation: ![inline formula](https://latex.codecogs.com/png.latex?P%20%3D%20K%20%2A%20%5BR%7Ct%5D)
pub fn projection_from_k_rt(k: &dyn core::ToInputArray, r: &dyn core::ToInputArray, t: &dyn core::ToInputArray, p: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(k);
    input_array_arg!(r);
    input_array_arg!(t);
    output_array_arg!(p);
    unsafe { sys::cv_sfm_projectionFromKRt__InputArray__InputArray__InputArray__OutputArray(k.as_raw__InputArray(), r.as_raw__InputArray(), t.as_raw__InputArray(), p.as_raw__OutputArray()) }.into_result()
}

/// Get projection matrices from Fundamental matrix
/// ## Parameters
/// * F: Input 3x3 fundamental matrix.
/// * P1: Output 3x4 one possible projection matrix.
/// * P2: Output 3x4 another possible projection matrix.
pub fn projections_from_fundamental(f: &dyn core::ToInputArray, p1: &mut dyn core::ToOutputArray, p2: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(f);
    output_array_arg!(p1);
    output_array_arg!(p2);
    unsafe { sys::cv_sfm_projectionsFromFundamental__InputArray__OutputArray__OutputArray(f.as_raw__InputArray(), p1.as_raw__OutputArray(), p2.as_raw__OutputArray()) }.into_result()
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///
/// Note:
/// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
/// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
/// - To see a working example for scene reconstruction, check the following tutorial: @ref tutorial_sfm_scene_reconstruction.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct(images: &types::VectorOfString, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, k: &mut dyn core::ToInputOutputArray, points3d: &mut dyn core::ToOutputArray, is_projective: bool) -> Result<()> {
    output_array_arg!(rs);
    output_array_arg!(ts);
    input_output_array_arg!(k);
    output_array_arg!(points3d);
    unsafe { sys::cv_sfm_reconstruct_VectorOfString__OutputArray__OutputArray__InputOutputArray__OutputArray_bool(images.as_raw_VectorOfString(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), k.as_raw__InputOutputArray(), points3d.as_raw__OutputArray(), is_projective) }.into_result()
}

/// Reconstruct 3d points from 2d images while performing autocalibration.
/// ## Parameters
/// * images: a vector of string with the images paths.
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///
/// Note:
/// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
/// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct_1(images: &types::VectorOfString, ps: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray, k: &mut dyn core::ToInputOutputArray, is_projective: bool) -> Result<()> {
    output_array_arg!(ps);
    output_array_arg!(points3d);
    input_output_array_arg!(k);
    unsafe { sys::cv_sfm_reconstruct_VectorOfString__OutputArray__OutputArray__InputOutputArray_bool(images.as_raw_VectorOfString(), ps.as_raw__OutputArray(), points3d.as_raw__OutputArray(), k.as_raw__InputOutputArray(), is_projective) }.into_result()
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Rs: Output vector of 3x3 rotations of the camera.
/// * Ts: Output vector of 3x1 translations of the camera.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// Internally calls libmv simple pipeline routine with some default parameters by instatiating SFMLibmvEuclideanReconstruction class.
///
///
/// Note:
/// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
/// - To see a working example for camera motion reconstruction, check the following tutorial: @ref tutorial_sfm_trajectory_estimation.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct_2(points2d: &dyn core::ToInputArray, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, k: &mut dyn core::ToInputOutputArray, points3d: &mut dyn core::ToOutputArray, is_projective: bool) -> Result<()> {
    input_array_arg!(points2d);
    output_array_arg!(rs);
    output_array_arg!(ts);
    input_output_array_arg!(k);
    output_array_arg!(points3d);
    unsafe { sys::cv_sfm_reconstruct__InputArray__OutputArray__OutputArray__InputOutputArray__OutputArray_bool(points2d.as_raw__InputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), k.as_raw__InputOutputArray(), points3d.as_raw__OutputArray(), is_projective) }.into_result()
}

/// Reconstruct 3d points from 2d correspondences while performing autocalibration.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
/// * Ps: Output vector with the 3x4 projections matrices of each image.
/// * points3d: Output array with estimated 3d points.
/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
/// * is_projective: if true, the cameras are supposed to be projective.
///
/// This method calls below signature and extracts projection matrices from estimated K, R and t.
///
///
/// Note:
/// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
///
/// ## C++ default parameters
/// * is_projective: false
pub fn reconstruct_3(points2d: &dyn core::ToInputArray, ps: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray, k: &mut dyn core::ToInputOutputArray, is_projective: bool) -> Result<()> {
    input_array_arg!(points2d);
    output_array_arg!(ps);
    output_array_arg!(points3d);
    input_output_array_arg!(k);
    unsafe { sys::cv_sfm_reconstruct__InputArray__OutputArray__OutputArray__InputOutputArray_bool(points2d.as_raw__InputArray(), ps.as_raw__OutputArray(), points3d.as_raw__OutputArray(), k.as_raw__InputOutputArray(), is_projective) }.into_result()
}

/// Computes the relative camera motion between two cameras.
/// ## Parameters
/// * R1: Input 3x3 first camera rotation matrix.
/// * t1: Input 3x1 first camera translation vector.
/// * R2: Input 3x3 second camera rotation matrix.
/// * t2: Input 3x1 second camera translation vector.
/// * R: Output 3x3 relative rotation matrix.
/// * t: Output 3x1 relative translation vector.
///
/// Given the motion parameters of two cameras, computes the motion parameters
/// of the second one assuming the first one to be at the origin.
/// If T1 and T2 are the camera motions, the computed relative motion is ![inline formula](https://latex.codecogs.com/png.latex?T%20%3D%20T_2%20T_1%5E%7B-1%7D)
pub fn relative_camera_motion(r1: &dyn core::ToInputArray, t1: &dyn core::ToInputArray, r2: &dyn core::ToInputArray, t2: &dyn core::ToInputArray, r: &mut dyn core::ToOutputArray, t: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(r1);
    input_array_arg!(t1);
    input_array_arg!(r2);
    input_array_arg!(t2);
    output_array_arg!(r);
    output_array_arg!(t);
    unsafe { sys::cv_sfm_relativeCameraMotion__InputArray__InputArray__InputArray__InputArray__OutputArray__OutputArray(r1.as_raw__InputArray(), t1.as_raw__InputArray(), r2.as_raw__InputArray(), t2.as_raw__InputArray(), r.as_raw__OutputArray(), t.as_raw__OutputArray()) }.into_result()
}

/// Returns the 3x3 skew symmetric matrix of a vector.
/// ## Parameters
/// * x: Input 3x1 vector.
///
/// Reference: [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00), p581, equation (A4.5).
pub fn skew(x: &dyn core::ToInputArray) -> Result<core::Mat> {
    input_array_arg!(x);
    unsafe { sys::cv_sfm_skew__InputArray(x.as_raw__InputArray()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Reconstructs bunch of points by triangulation.
/// ## Parameters
/// * points2d: Input vector of vectors of 2d points (the inner vector is per image). Has to be 2 X N.
/// * projection_matrices: Input vector with 3x4 projections matrices of each image.
/// * points3d: Output array with computed 3d points. Is 3 x N.
///
/// Triangulates the 3d position of 2d correspondences between several images.
/// Reference: Internally it uses DLT method [HartleyZ00](https://docs.opencv.org/4.2.0/d0/de3/citelist.html#CITEREF_HartleyZ00) 12.2 pag.312
pub fn triangulate_points(points2d: &dyn core::ToInputArray, projection_matrices: &dyn core::ToInputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
    input_array_arg!(points2d);
    input_array_arg!(projection_matrices);
    output_array_arg!(points3d);
    unsafe { sys::cv_sfm_triangulatePoints__InputArray__InputArray__OutputArray(points2d.as_raw__InputArray(), projection_matrices.as_raw__InputArray(), points3d.as_raw__OutputArray()) }.into_result()
}

// Generating impl for trait crate::sfm::BaseSFM
/// base class BaseSFM declares a common API that would be used in a typical scene reconstruction scenario
pub trait BaseSFM {
    fn as_raw_BaseSFM(&self) -> *mut c_void;
    fn run(&mut self, points2d: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(points2d);
        unsafe { sys::cv_sfm_BaseSFM_run__InputArray(self.as_raw_BaseSFM(), points2d.as_raw__InputArray()) }.into_result()
    }
    
    fn run_1(&mut self, points2d: &dyn core::ToInputArray, k: &mut dyn core::ToInputOutputArray, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(points2d);
        input_output_array_arg!(k);
        output_array_arg!(rs);
        output_array_arg!(ts);
        output_array_arg!(points3d);
        unsafe { sys::cv_sfm_BaseSFM_run__InputArray__InputOutputArray__OutputArray__OutputArray__OutputArray(self.as_raw_BaseSFM(), points2d.as_raw__InputArray(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray()) }.into_result()
    }
    
    fn run_2(&mut self, images: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_run_VectorOfString(self.as_raw_BaseSFM(), images.as_raw_VectorOfString()) }.into_result()
    }
    
    fn run_3(&mut self, images: &types::VectorOfString, k: &mut dyn core::ToInputOutputArray, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
        input_output_array_arg!(k);
        output_array_arg!(rs);
        output_array_arg!(ts);
        output_array_arg!(points3d);
        unsafe { sys::cv_sfm_BaseSFM_run_VectorOfString__InputOutputArray__OutputArray__OutputArray__OutputArray(self.as_raw_BaseSFM(), images.as_raw_VectorOfString(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray()) }.into_result()
    }
    
    fn get_error(&self) -> Result<f64> {
        unsafe { sys::cv_sfm_BaseSFM_getError_const(self.as_raw_BaseSFM()) }.into_result()
    }
    
    fn get_points(&mut self, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(points3d);
        unsafe { sys::cv_sfm_BaseSFM_getPoints__OutputArray(self.as_raw_BaseSFM(), points3d.as_raw__OutputArray()) }.into_result()
    }
    
    fn get_intrinsics(&self) -> Result<core::Mat> {
        unsafe { sys::cv_sfm_BaseSFM_getIntrinsics_const(self.as_raw_BaseSFM()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    fn get_cameras(&mut self, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(rs);
        output_array_arg!(ts);
        unsafe { sys::cv_sfm_BaseSFM_getCameras__OutputArray__OutputArray(self.as_raw_BaseSFM(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray()) }.into_result()
    }
    
    fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_setReconstructionOptions_libmv_ReconstructionOptions(self.as_raw_BaseSFM(), libmv_reconstruction_options) }.into_result()
    }
    
    fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
        unsafe { sys::cv_sfm_BaseSFM_setCameraIntrinsicOptions_libmv_CameraIntrinsicsOptions(self.as_raw_BaseSFM(), libmv_camera_intrinsics_options) }.into_result()
    }
    
}

// boxed class cv::sfm::SFMLibmvEuclideanReconstruction
/// SFMLibmvEuclideanReconstruction class provides an interface with the Libmv Structure From Motion pipeline.
pub struct SFMLibmvEuclideanReconstruction {
    #[doc(hidden)] pub(crate) ptr: *mut c_void
}

impl Drop for SFMLibmvEuclideanReconstruction {
    fn drop(&mut self) {
        unsafe { sys::cv_SFMLibmvEuclideanReconstruction_delete(self.ptr) };
    }
}

impl SFMLibmvEuclideanReconstruction {
    #[inline(always)] pub fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *mut c_void { self.ptr }

    pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

unsafe impl Send for SFMLibmvEuclideanReconstruction {}

impl crate::sfm::BaseSFM for SFMLibmvEuclideanReconstruction {
    #[inline(always)] fn as_raw_BaseSFM(&self) -> *mut c_void { self.ptr }
}

impl SFMLibmvEuclideanReconstruction {
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
    ///
    ///
    /// Note:
    /// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
    pub fn run(&mut self, points2d: &dyn core::ToInputArray) -> Result<()> {
        input_array_arg!(points2d);
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run__InputArray(self.as_raw_SFMLibmvEuclideanReconstruction(), points2d.as_raw__InputArray()) }.into_result()
    }
    
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
    /// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
    /// * Rs: Output vector of 3x3 rotations of the camera.
    /// * Ts: Output vector of 3x1 translations of the camera.
    /// * points3d: Output array with estimated 3d points.
    ///
    ///
    /// Note:
    /// - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
    pub fn run_1(&mut self, points2d: &dyn core::ToInputArray, k: &mut dyn core::ToInputOutputArray, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
        input_array_arg!(points2d);
        input_output_array_arg!(k);
        output_array_arg!(rs);
        output_array_arg!(ts);
        output_array_arg!(points3d);
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run__InputArray__InputOutputArray__OutputArray__OutputArray__OutputArray(self.as_raw_SFMLibmvEuclideanReconstruction(), points2d.as_raw__InputArray(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray()) }.into_result()
    }
    
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * images: a vector of string with the images paths.
    ///
    ///
    /// Note:
    /// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
    /// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
    pub fn run_2(&mut self, images: &types::VectorOfString) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_VectorOfString(self.as_raw_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString()) }.into_result()
    }
    
    /// Calls the pipeline in order to perform Eclidean reconstruction.
    /// ## Parameters
    /// * images: a vector of string with the images paths.
    /// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f_x%20%26%200%20%26%20c_x%5C%5C%200%20%26%20f_y%20%26%20c_y%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
    /// * Rs: Output vector of 3x3 rotations of the camera.
    /// * Ts: Output vector of 3x1 translations of the camera.
    /// * points3d: Output array with estimated 3d points.
    ///
    ///
    /// Note:
    /// - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
    /// - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
    pub fn run_3(&mut self, images: &types::VectorOfString, k: &mut dyn core::ToInputOutputArray, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
        input_output_array_arg!(k);
        output_array_arg!(rs);
        output_array_arg!(ts);
        output_array_arg!(points3d);
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_VectorOfString__InputOutputArray__OutputArray__OutputArray__OutputArray(self.as_raw_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray()) }.into_result()
    }
    
    /// Returns the computed reprojection error.
    pub fn get_error(&self) -> Result<f64> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(self.as_raw_SFMLibmvEuclideanReconstruction()) }.into_result()
    }
    
    /// Returns the estimated 3d points.
    /// ## Parameters
    /// * points3d: Output array with estimated 3d points.
    pub fn get_points(&mut self, points3d: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(points3d);
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getPoints__OutputArray(self.as_raw_SFMLibmvEuclideanReconstruction(), points3d.as_raw__OutputArray()) }.into_result()
    }
    
    /// Returns the refined camera calibration matrix.
    pub fn get_intrinsics(&self) -> Result<core::Mat> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(self.as_raw_SFMLibmvEuclideanReconstruction()) }.into_result().map(|ptr| core::Mat { ptr })
    }
    
    /// Returns the estimated camera extrinsic parameters.
    /// ## Parameters
    /// * Rs: Output vector of 3x3 rotations of the camera.
    /// * Ts: Output vector of 3x1 translations of the camera.
    pub fn get_cameras(&mut self, rs: &mut dyn core::ToOutputArray, ts: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(rs);
        output_array_arg!(ts);
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getCameras__OutputArray__OutputArray(self.as_raw_SFMLibmvEuclideanReconstruction(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray()) }.into_result()
    }
    
    /// Setter method for reconstruction options.
    /// ## Parameters
    /// * libmv_reconstruction_options: struct with reconstruction options such as initial keyframes,
    /// automatic keyframe selection, parameters to refine and the verbosity level.
    pub fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_libmv_ReconstructionOptions(self.as_raw_SFMLibmvEuclideanReconstruction(), libmv_reconstruction_options) }.into_result()
    }
    
    /// Setter method for camera intrinsic options.
    /// ## Parameters
    /// * libmv_camera_intrinsics_options: struct with camera intrinsic options such as camera model and
    /// the internal camera parameters.
    pub fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_libmv_CameraIntrinsicsOptions(self.as_raw_SFMLibmvEuclideanReconstruction(), libmv_camera_intrinsics_options) }.into_result()
    }
    
    /// Creates an instance of the SFMLibmvEuclideanReconstruction class. Initializes Libmv.
    ///
    /// ## C++ default parameters
    /// * camera_instrinsic_options: libmv_CameraIntrinsicsOptions()
    /// * reconstruction_options: libmv_ReconstructionOptions()
    pub fn create(camera_instrinsic_options: crate::sfm::libmv_CameraIntrinsicsOptions, reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<types::PtrOfSFMLibmvEuclideanReconstruction> {
        unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_create_libmv_CameraIntrinsicsOptions_libmv_ReconstructionOptions(camera_instrinsic_options, reconstruction_options) }.into_result().map(|ptr| types::PtrOfSFMLibmvEuclideanReconstruction { ptr })
    }
    
}

impl libmv_CameraIntrinsicsOptions {
    ///
    /// ## C++ default parameters
    /// * _distortion_model: 0
    /// * _focal_length_x: 0
    /// * _focal_length_y: 0
    /// * _principal_point_x: 0
    /// * _principal_point_y: 0
    /// * _polynomial_k1: 0
    /// * _polynomial_k2: 0
    /// * _polynomial_k3: 0
    /// * _polynomial_p1: 0
    /// * _polynomial_p2: 0
    pub fn new(_distortion_model: i32, _focal_length_x: f64, _focal_length_y: f64, _principal_point_x: f64, _principal_point_y: f64, _polynomial_k1: f64, _polynomial_k2: f64, _polynomial_k3: f64, _polynomial_p1: f64, _polynomial_p2: f64) -> Result<crate::sfm::libmv_CameraIntrinsicsOptions> {
        unsafe { sys::cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_int_double_double_double_double_double_double_double_double_double(_distortion_model, _focal_length_x, _focal_length_y, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2) }.into_result()
    }
    
}

impl libmv_ReconstructionOptions {
    ///
    /// ## C++ default parameters
    /// * _keyframe1: 1
    /// * _keyframe2: 2
    /// * _refine_intrinsics: 1
    /// * _select_keyframes: 1
    /// * _verbosity_level: -1
    pub fn new(_keyframe1: i32, _keyframe2: i32, _refine_intrinsics: i32, _select_keyframes: i32, _verbosity_level: i32) -> Result<crate::sfm::libmv_ReconstructionOptions> {
        unsafe { sys::cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_int_int_int_int_int(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level) }.into_result()
    }
    
}

