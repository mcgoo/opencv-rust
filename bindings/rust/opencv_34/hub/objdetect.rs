//! # Object Detection
//! 
//! Haar Feature-based Cascade Classifier for Object Detection
//! ----------------------------------------------------------
//! 
//! The object detector described below has been initially proposed by Paul Viola [Viola01](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Viola01) and
//! improved by Rainer Lienhart [Lienhart02](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Lienhart02) .
//! 
//! First, a classifier (namely a *cascade of boosted classifiers working with haar-like features*) is
//! trained with a few hundred sample views of a particular object (i.e., a face or a car), called
//! positive examples, that are scaled to the same size (say, 20x20), and negative examples - arbitrary
//! images of the same size.
//! 
//! After a classifier is trained, it can be applied to a region of interest (of the same size as used
//! during the training) in an input image. The classifier outputs a "1" if the region is likely to show
//! the object (i.e., face/car), and "0" otherwise. To search for the object in the whole image one can
//! move the search window across the image and check every location using the classifier. The
//! classifier is designed so that it can be easily "resized" in order to be able to find the objects of
//! interest at different sizes, which is more efficient than resizing the image itself. So, to find an
//! object of an unknown size in the image the scan procedure should be done several times at different
//! scales.
//! 
//! The word "cascade" in the classifier name means that the resultant classifier consists of several
//! simpler classifiers (*stages*) that are applied subsequently to a region of interest until at some
//! stage the candidate is rejected or all the stages are passed. The word "boosted" means that the
//! classifiers at every stage of the cascade are complex themselves and they are built out of basic
//! classifiers using one of four different boosting techniques (weighted voting). Currently Discrete
//! Adaboost, Real Adaboost, Gentle Adaboost and Logitboost are supported. The basic classifiers are
//! decision-tree classifiers with at least 2 leaves. Haar-like features are the input to the basic
//! classifiers, and are calculated as described below. The current algorithm uses the following
//! Haar-like features:
//! 
//! ![image](https://docs.opencv.org/3.4.9/haarfeatures.png)
//! 
//! The feature used in a particular classifier is specified by its shape (1a, 2b etc.), position within
//! the region of interest and the scale (this scale is not the same as the scale used at the detection
//! stage, though these two scales are multiplied). For example, in the case of the third line feature
//! (2c) the response is calculated as the difference between the sum of image pixels under the
//! rectangle covering the whole feature (including the two white stripes and the black stripe in the
//! middle) and the sum of the image pixels under the black stripe multiplied by 3 in order to
//! compensate for the differences in the size of areas. The sums of pixel values over a rectangular
//! regions are calculated rapidly using integral images (see below and the integral description).
//! 
//! To see the object detector at work, have a look at the facedetect demo:
//! <https://github.com/opencv/opencv/tree/3.4/samples/cpp/dbt_face_detection.cpp>
//! 
//! The following reference is for the detection part only. There is a separate application called
//! opencv_traincascade that can train a cascade of boosted classifiers from a set of samples.
//! 
//! 
//! Note: In the new C++ interface it is also possible to use LBP (local binary pattern) features in
//! addition to Haar-like features. .. [Viola01] Paul Viola and Michael J. Jones. Rapid Object Detection
//! using a Boosted Cascade of Simple Features. IEEE CVPR, 2001. The paper is available online at
//! <http://research.microsoft.com/en-us/um/people/viola/Pubs/Detect/violaJones_CVPR2001.pdf>
//!    # C API
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::SimilarRectsTrait, super::BaseCascadeClassifier_MaskGenerator, super::BaseCascadeClassifier, super::CascadeClassifierTrait, super::DetectionROITrait, super::HOGDescriptorTrait, super::QRCodeDetectorTrait, super::DetectionBasedTracker_ParametersTrait, super::DetectionBasedTracker_IDetector, super::DetectionBasedTracker_ExtObjectTrait, super::DetectionBasedTrackerTrait };
}

pub const CASCADE_DO_CANNY_PRUNING: i32 = 1;
pub const CASCADE_DO_ROUGH_SEARCH: i32 = 8;
pub const CASCADE_FIND_BIGGEST_OBJECT: i32 = 4;
pub const CASCADE_SCALE_IMAGE: i32 = 2;
pub const CV_HAAR_DO_CANNY_PRUNING: i32 = 1;
pub const CV_HAAR_DO_ROUGH_SEARCH: i32 = 8;
pub const CV_HAAR_FEATURE_MAX: i32 = 3;
pub const CV_HAAR_FIND_BIGGEST_OBJECT: i32 = 4;
pub const CV_HAAR_MAGIC_VAL: i32 = 0x42500000;
pub const CV_HAAR_SCALE_IMAGE: i32 = 2;
pub const CV_HAAR_STAGE_MAX: i32 = 1000;
pub const CV_TYPE_NAME_HAAR: &'static str = "opencv-haar-classifier";
/// Default nlevels value.
pub const HOGDescriptor_DEFAULT_NLEVELS: i32 = 64;
/// Default histogramNormType
pub const HOGDescriptor_L2Hys: i32 = 0;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DetectionBasedTracker_ObjectStatus {
	DETECTED_NOT_SHOWN_YET = 0 as isize,
	DETECTED = 1 as isize,
	DETECTED_TEMPORARY_LOST = 2 as isize,
	WRONG_OBJECT = 3 as isize,
}

pub fn create_face_detection_mask_generator() -> Result<types::PtrOfBaseCascadeClassifier_MaskGenerator> {
	unsafe { sys::cv_createFaceDetectionMaskGenerator() }.into_result().map(|ptr| types::PtrOfBaseCascadeClassifier_MaskGenerator { ptr })
}

/// Decode QR code in image and return text that is encrypted in QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Input vector of vertices of a quadrangle of minimal area that describes QR code.
/// * decoded_info: String information that is encrypted in QR code.
/// * straight_qrcode: Matrix of the type CV_8UC1 containing an binary straight QR code.
/// 
/// ## C++ default parameters
/// * straight_qrcode: noArray()
pub fn decode_qr_code(in_: &dyn core::ToInputArray, points: &dyn core::ToInputArray, decoded_info: &mut String, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<bool> {
	input_array_arg!(in_);
	input_array_arg!(points);
	string_arg_output_send!(via decoded_info_via);
	output_array_arg!(straight_qrcode);
	let out = unsafe { sys::cv_decodeQRCode_const__InputArrayX_const__InputArrayX_stringX_const__OutputArrayX(in_.as_raw__InputArray(), points.as_raw__InputArray(), &mut decoded_info_via, straight_qrcode.as_raw__OutputArray()) }.into_result();
	string_arg_output_receive!(out, decoded_info_via => decoded_info);
	out
}

/// Detect QR code in image and return minimum area of quadrangle that describes QR code.
/// ## Parameters
/// * in: Matrix of the type CV_8UC1 containing an image where QR code are detected.
/// * points: Output vector of vertices of a quadrangle of minimal area that describes QR code.
/// * eps_x: Epsilon neighborhood, which allows you to determine the horizontal pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// * eps_y: Epsilon neighborhood, which allows you to determine the vertical pattern of the scheme 1:1:3:1:1 according to QR code standard.
/// 
/// ## C++ default parameters
/// * eps_x: 0.2
/// * eps_y: 0.1
pub fn detect_qr_code(in_: &dyn core::ToInputArray, points: &mut types::VectorOfPoint, eps_x: f64, eps_y: f64) -> Result<bool> {
	input_array_arg!(in_);
	unsafe { sys::cv_detectQRCode_const__InputArrayX_vector_Point_X_double_double(in_.as_raw__InputArray(), points.as_raw_VectorOfPoint(), eps_x, eps_y) }.into_result()
}

/// ## C++ default parameters
/// * detect_threshold: 0.0
/// * win_det_size: Size(64,128)
pub fn group_rectangles_meanshift(rect_list: &mut types::VectorOfRect, found_weights: &mut types::VectorOff64, found_scales: &mut types::VectorOff64, detect_threshold: f64, win_det_size: core::Size) -> Result<()> {
	unsafe { sys::cv_groupRectangles_meanshift_vector_Rect_X_vector_double_X_vector_double_X_double_Size(rect_list.as_raw_VectorOfRect(), found_weights.as_raw_VectorOff64(), found_scales.as_raw_VectorOff64(), detect_threshold, &win_det_size) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## C++ default parameters
/// * eps: 0.2
pub fn group_rectangles(rect_list: &mut types::VectorOfRect, group_threshold: i32, eps: f64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_X_int_double(rect_list.as_raw_VectorOfRect(), group_threshold, eps) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## Overloaded parameters
pub fn group_rectangles_levelweights(rect_list: &mut types::VectorOfRect, group_threshold: i32, eps: f64, weights: &mut types::VectorOfi32, level_weights: &mut types::VectorOff64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_X_int_double_vector_int_X_vector_double_X(rect_list.as_raw_VectorOfRect(), group_threshold, eps, weights.as_raw_VectorOfi32(), level_weights.as_raw_VectorOff64()) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * eps: 0.2
pub fn group_rectangles_weights(rect_list: &mut types::VectorOfRect, weights: &mut types::VectorOfi32, group_threshold: i32, eps: f64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_X_vector_int_X_int_double(rect_list.as_raw_VectorOfRect(), weights.as_raw_VectorOfi32(), group_threshold, eps) }.into_result()
}

/// Groups the object candidate rectangles.
/// 
/// ## Parameters
/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped
/// rectangles. (The Python list is not modified in place.)
/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a
/// group of rectangles to retain it.
/// * eps: Relative difference between sides of the rectangles to merge them into a group.
/// 
/// The function is a wrapper for the generic function partition . It clusters all the input rectangles
/// using the rectangle equivalence criteria that combines rectangles with similar sizes and similar
/// locations. The similarity is defined by eps. When eps=0 , no clustering is done at all. If
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Beps%7D%5Crightarrow%20%2B%5Cinf) , all the rectangles are put in one cluster. Then, the small
/// clusters containing less than or equal to groupThreshold rectangles are rejected. In each other
/// cluster, the average rectangle is computed and put into the output rectangle list.
/// 
/// ## Overloaded parameters
/// 
/// ## C++ default parameters
/// * eps: 0.2
pub fn group_rectangles_levels(rect_list: &mut types::VectorOfRect, reject_levels: &mut types::VectorOfi32, level_weights: &mut types::VectorOff64, group_threshold: i32, eps: f64) -> Result<()> {
	unsafe { sys::cv_groupRectangles_vector_Rect_X_vector_int_X_vector_double_X_int_double(rect_list.as_raw_VectorOfRect(), reject_levels.as_raw_VectorOfi32(), level_weights.as_raw_VectorOff64(), group_threshold, eps) }.into_result()
}

pub trait BaseCascadeClassifier: core::AlgorithmTrait {
	fn as_raw_BaseCascadeClassifier(&self) -> *mut c_void;
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_BaseCascadeClassifier_empty_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn load(&mut self, filename: &str) -> Result<bool> {
		string_arg!(filename);
		unsafe { sys::cv_BaseCascadeClassifier_load_const_StringX(self.as_raw_BaseCascadeClassifier(), filename.as_ptr()) }.into_result()
	}
	
	fn detect_multi_scale(&mut self, image: &dyn core::ToInputArray, objects: &mut types::VectorOfRect, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_double_int_int_Size_Size(self.as_raw_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_VectorOfRect(), scale_factor, min_neighbors, flags, &min_size, &max_size) }.into_result()
	}
	
	fn detect_multi_scale_num(&mut self, image: &dyn core::ToInputArray, objects: &mut types::VectorOfRect, num_detections: &mut types::VectorOfi32, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size(self.as_raw_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_VectorOfRect(), num_detections.as_raw_VectorOfi32(), scale_factor, min_neighbors, flags, &min_size, &max_size) }.into_result()
	}
	
	fn detect_multi_scale_levels(&mut self, image: &dyn core::ToInputArray, objects: &mut types::VectorOfRect, reject_levels: &mut types::VectorOfi32, level_weights: &mut types::VectorOff64, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool(self.as_raw_BaseCascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_VectorOfRect(), reject_levels.as_raw_VectorOfi32(), level_weights.as_raw_VectorOff64(), scale_factor, min_neighbors, flags, &min_size, &max_size, output_reject_levels) }.into_result()
	}
	
	fn is_old_format_cascade(&self) -> Result<bool> {
		unsafe { sys::cv_BaseCascadeClassifier_isOldFormatCascade_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn get_original_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_BaseCascadeClassifier_getOriginalWindowSize_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn get_feature_type(&self) -> Result<i32> {
		unsafe { sys::cv_BaseCascadeClassifier_getFeatureType_const(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn get_old_cascade(&mut self) -> Result<*mut c_void> {
		unsafe { sys::cv_BaseCascadeClassifier_getOldCascade(self.as_raw_BaseCascadeClassifier()) }.into_result()
	}
	
	fn set_mask_generator(&mut self, mask_generator: &types::PtrOfBaseCascadeClassifier_MaskGenerator) -> Result<()> {
		unsafe { sys::cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_X(self.as_raw_BaseCascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator()) }.into_result()
	}
	
	fn get_mask_generator(&mut self) -> Result<types::PtrOfBaseCascadeClassifier_MaskGenerator> {
		unsafe { sys::cv_BaseCascadeClassifier_getMaskGenerator(self.as_raw_BaseCascadeClassifier()) }.into_result().map(|ptr| types::PtrOfBaseCascadeClassifier_MaskGenerator { ptr })
	}
	
}

pub trait BaseCascadeClassifier_MaskGenerator {
	fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void;
	fn generate_mask(&mut self, src: &core::Mat) -> Result<core::Mat> {
		unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatX(self.as_raw_BaseCascadeClassifier_MaskGenerator(), src.as_raw_Mat()) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	fn initialize_mask(&mut self, unnamed: &core::Mat) -> Result<()> {
		unsafe { sys::cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatX(self.as_raw_BaseCascadeClassifier_MaskGenerator(), unnamed.as_raw_Mat()) }.into_result()
	}
	
}

/// @example samples/cpp/facedetect.cpp
/// This program demonstrates usage of the Cascade classifier class
/// \image html Cascade_Classifier_Tutorial_Result_Haar.jpg "Sample screenshot" width=321 height=254
/// 
/// Cascade classifier class for object detection.
pub trait CascadeClassifierTrait {
	fn as_raw_CascadeClassifier(&self) -> *mut c_void;
	fn cc(&mut self) -> types::PtrOfBaseCascadeClassifier {
		unsafe { sys::cv_CascadeClassifier_cc(self.as_raw_CascadeClassifier()) }.into_result().map(|ptr| types::PtrOfBaseCascadeClassifier { ptr }).expect("Infallible function failed: cc")
	}
	
	fn set_cc(&mut self, val: types::PtrOfBaseCascadeClassifier) -> () {
		unsafe { sys::cv_CascadeClassifier_setCc_Ptr_BaseCascadeClassifier_(self.as_raw_CascadeClassifier(), val.as_raw_PtrOfBaseCascadeClassifier()) }.into_result().expect("Infallible function failed: set_cc")
	}
	
	/// Checks whether the classifier has been loaded.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_CascadeClassifier_empty_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	/// Loads a classifier from a file.
	/// 
	/// ## Parameters
	/// * filename: Name of the file from which the classifier is loaded. The file may contain an old
	/// HAAR classifier trained by the haartraining application or a new cascade classifier trained by the
	/// traincascade application.
	fn load(&mut self, filename: &str) -> Result<bool> {
		string_arg!(filename);
		unsafe { sys::cv_CascadeClassifier_load_const_StringX(self.as_raw_CascadeClassifier(), filename.as_ptr()) }.into_result()
	}
	
	/// Reads a classifier from a FileStorage node.
	/// 
	/// 
	/// Note: The file may contain a new cascade classifier (trained traincascade application) only.
	fn read(&mut self, node: &core::FileNode) -> Result<bool> {
		unsafe { sys::cv_CascadeClassifier_read_const_FileNodeX(self.as_raw_CascadeClassifier(), node.as_raw_FileNode()) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// 
	/// ## Parameters
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	/// rectangles may be partially outside the original image.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	/// to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	/// cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// The function is parallelized with the TBB library.
	/// 
	/// 
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	/// 
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	fn detect_multi_scale(&mut self, image: &dyn core::ToInputArray, objects: &mut types::VectorOfRect, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_double_int_int_Size_Size(self.as_raw_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_VectorOfRect(), scale_factor, min_neighbors, flags, &min_size, &max_size) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// 
	/// ## Parameters
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	/// rectangles may be partially outside the original image.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	/// to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	/// cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// The function is parallelized with the TBB library.
	/// 
	/// 
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	/// 
	/// ## Overloaded parameters
	/// 
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	///    rectangles may be partially outside the original image.
	/// * numDetections: Vector of detection numbers for the corresponding objects. An object's number
	///    of detections is the number of neighboring positively classified rectangles that were joined
	///    together to form the object.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	///    to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	///    cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	fn detect_multi_scale2(&mut self, image: &dyn core::ToInputArray, objects: &mut types::VectorOfRect, num_detections: &mut types::VectorOfi32, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_double_int_int_Size_Size(self.as_raw_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_VectorOfRect(), num_detections.as_raw_VectorOfi32(), scale_factor, min_neighbors, flags, &min_size, &max_size) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// 
	/// ## Parameters
	/// * image: Matrix of the type CV_8U containing an image where objects are detected.
	/// * objects: Vector of rectangles where each rectangle contains the detected object, the
	/// rectangles may be partially outside the original image.
	/// * scaleFactor: Parameter specifying how much the image size is reduced at each image scale.
	/// * minNeighbors: Parameter specifying how many neighbors each candidate rectangle should have
	/// to retain it.
	/// * flags: Parameter with the same meaning for an old cascade as in the function
	/// cvHaarDetectObjects. It is not used for a new cascade.
	/// * minSize: Minimum possible object size. Objects smaller than that are ignored.
	/// * maxSize: Maximum possible object size. Objects larger than that are ignored. If `maxSize == minSize` model is evaluated on single scale.
	/// 
	/// The function is parallelized with the TBB library.
	/// 
	/// 
	/// Note:
	///    *   (Python) A face detection example using cascade classifiers can be found at
	///        opencv_source_code/samples/python/facedetect.py
	/// 
	/// ## Overloaded parameters
	/// 
	///    This function allows you to retrieve the final stage decision certainty of classification.
	///    For this, one needs to set `outputRejectLevels` on true and provide the `rejectLevels` and `levelWeights` parameter.
	///    For each resulting detection, `levelWeights` will then contain the certainty of classification at the final stage.
	///    This value can then be used to separate strong from weaker classifications.
	/// 
	///    A code sample on how to use it efficiently can be found below:
	///    ```ignore
	///    Mat img;
	///    vector<double> weights;
	///    vector<int> levels;
	///    vector<Rect> detections;
	///    CascadeClassifier model("/path/to/your/model.xml");
	///    model.detectMultiScale(img, detections, levels, weights, 1.1, 3, 0, Size(), Size(), true);
	///    cerr << "Detection " << detections[0] << " with weight " << weights[0] << endl;
	///    ```
	/// 
	/// 
	/// ## C++ default parameters
	/// * scale_factor: 1.1
	/// * min_neighbors: 3
	/// * flags: 0
	/// * min_size: Size()
	/// * max_size: Size()
	/// * output_reject_levels: false
	fn detect_multi_scale3(&mut self, image: &dyn core::ToInputArray, objects: &mut types::VectorOfRect, reject_levels: &mut types::VectorOfi32, level_weights: &mut types::VectorOff64, scale_factor: f64, min_neighbors: i32, flags: i32, min_size: core::Size, max_size: core::Size, output_reject_levels: bool) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_CascadeClassifier_detectMultiScale_const__InputArrayX_vector_Rect_X_vector_int_X_vector_double_X_double_int_int_Size_Size_bool(self.as_raw_CascadeClassifier(), image.as_raw__InputArray(), objects.as_raw_VectorOfRect(), reject_levels.as_raw_VectorOfi32(), level_weights.as_raw_VectorOff64(), scale_factor, min_neighbors, flags, &min_size, &max_size, output_reject_levels) }.into_result()
	}
	
	fn is_old_format_cascade(&self) -> Result<bool> {
		unsafe { sys::cv_CascadeClassifier_isOldFormatCascade_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn get_original_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_CascadeClassifier_getOriginalWindowSize_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn get_feature_type(&self) -> Result<i32> {
		unsafe { sys::cv_CascadeClassifier_getFeatureType_const(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn get_old_cascade(&mut self) -> Result<*mut c_void> {
		unsafe { sys::cv_CascadeClassifier_getOldCascade(self.as_raw_CascadeClassifier()) }.into_result()
	}
	
	fn set_mask_generator(&mut self, mask_generator: &types::PtrOfBaseCascadeClassifier_MaskGenerator) -> Result<()> {
		unsafe { sys::cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_X(self.as_raw_CascadeClassifier(), mask_generator.as_raw_PtrOfBaseCascadeClassifier_MaskGenerator()) }.into_result()
	}
	
	fn get_mask_generator(&mut self) -> Result<types::PtrOfBaseCascadeClassifier_MaskGenerator> {
		unsafe { sys::cv_CascadeClassifier_getMaskGenerator(self.as_raw_CascadeClassifier()) }.into_result().map(|ptr| types::PtrOfBaseCascadeClassifier_MaskGenerator { ptr })
	}
	
}

/// @example samples/cpp/facedetect.cpp
/// This program demonstrates usage of the Cascade classifier class
/// \image html Cascade_Classifier_Tutorial_Result_Haar.jpg "Sample screenshot" width=321 height=254
/// 
/// Cascade classifier class for object detection.
pub struct CascadeClassifier {
	pub(crate) ptr: *mut c_void
}

impl Drop for CascadeClassifier {
	fn drop(&mut self) {
		extern "C" { fn cv_CascadeClassifier_delete(instance: *mut c_void); }
		unsafe { cv_CascadeClassifier_delete(self.as_raw_CascadeClassifier()) };
	}
}

impl CascadeClassifier {
	pub fn as_raw_CascadeClassifier(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for CascadeClassifier {}

impl crate::objdetect::CascadeClassifierTrait for CascadeClassifier {
	fn as_raw_CascadeClassifier(&self) -> *mut c_void { self.ptr }
}

impl CascadeClassifier {
	pub fn default() -> Result<crate::objdetect::CascadeClassifier> {
		unsafe { sys::cv_CascadeClassifier_CascadeClassifier() }.into_result().map(|ptr| crate::objdetect::CascadeClassifier { ptr })
	}
	
	/// Loads a classifier from a file.
	/// 
	/// ## Parameters
	/// * filename: Name of the file from which the classifier is loaded.
	pub fn new(filename: &str) -> Result<crate::objdetect::CascadeClassifier> {
		string_arg!(filename);
		unsafe { sys::cv_CascadeClassifier_CascadeClassifier_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| crate::objdetect::CascadeClassifier { ptr })
	}
	
	pub fn convert(oldcascade: &str, newcascade: &str) -> Result<bool> {
		string_arg!(oldcascade);
		string_arg!(newcascade);
		unsafe { sys::cv_CascadeClassifier_convert_const_StringX_const_StringX(oldcascade.as_ptr(), newcascade.as_ptr()) }.into_result()
	}
	
}

pub trait DetectionBasedTrackerTrait {
	fn as_raw_DetectionBasedTracker(&self) -> *mut c_void;
	fn run(&mut self) -> Result<bool> {
		unsafe { sys::cv_DetectionBasedTracker_run(self.as_raw_DetectionBasedTracker()) }.into_result()
	}
	
	fn stop(&mut self) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_stop(self.as_raw_DetectionBasedTracker()) }.into_result()
	}
	
	fn reset_tracking(&mut self) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_resetTracking(self.as_raw_DetectionBasedTracker()) }.into_result()
	}
	
	fn process(&mut self, image_gray: &core::Mat) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_process_const_MatX(self.as_raw_DetectionBasedTracker(), image_gray.as_raw_Mat()) }.into_result()
	}
	
	fn set_parameters(&mut self, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<bool> {
		unsafe { sys::cv_DetectionBasedTracker_setParameters_const_ParametersX(self.as_raw_DetectionBasedTracker(), params.as_raw_DetectionBasedTracker_Parameters()) }.into_result()
	}
	
	fn get_parameters(&self) -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
		unsafe { sys::cv_DetectionBasedTracker_getParameters_const(self.as_raw_DetectionBasedTracker()) }.into_result().map(|ptr| crate::objdetect::DetectionBasedTracker_Parameters { ptr })
	}
	
	fn get_objects(&self, result: &mut types::VectorOfRect) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vector_Rect_X(self.as_raw_DetectionBasedTracker(), result.as_raw_VectorOfRect()) }.into_result()
	}
	
	fn get_objects_1(&self, result: &mut types::VectorOfDetectionBasedTracker_ExtObject) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_X(self.as_raw_DetectionBasedTracker(), result.as_raw_VectorOfDetectionBasedTracker_ExtObject()) }.into_result()
	}
	
	fn add_object(&mut self, location: core::Rect) -> Result<i32> {
		unsafe { sys::cv_DetectionBasedTracker_addObject_const_RectX(self.as_raw_DetectionBasedTracker(), &location) }.into_result()
	}
	
}

pub struct DetectionBasedTracker {
	pub(crate) ptr: *mut c_void
}

impl Drop for DetectionBasedTracker {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionBasedTracker_delete(instance: *mut c_void); }
		unsafe { cv_DetectionBasedTracker_delete(self.as_raw_DetectionBasedTracker()) };
	}
}

impl DetectionBasedTracker {
	pub fn as_raw_DetectionBasedTracker(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DetectionBasedTracker {}

impl crate::objdetect::DetectionBasedTrackerTrait for DetectionBasedTracker {
	fn as_raw_DetectionBasedTracker(&self) -> *mut c_void { self.ptr }
}

impl DetectionBasedTracker {
	pub fn new(main_detector: types::PtrOfDetectionBasedTracker_IDetector, tracking_detector: types::PtrOfDetectionBasedTracker_IDetector, params: &crate::objdetect::DetectionBasedTracker_Parameters) -> Result<crate::objdetect::DetectionBasedTracker> {
		unsafe { sys::cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersX(main_detector.as_raw_PtrOfDetectionBasedTracker_IDetector(), tracking_detector.as_raw_PtrOfDetectionBasedTracker_IDetector(), params.as_raw_DetectionBasedTracker_Parameters()) }.into_result().map(|ptr| crate::objdetect::DetectionBasedTracker { ptr })
	}
	
}

pub trait DetectionBasedTracker_ExtObjectTrait {
	fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *mut c_void;
	fn id(&self) -> i32 {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_id_const(self.as_raw_DetectionBasedTracker_ExtObject()) }.into_result().expect("Infallible function failed: id")
	}
	
	fn set_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_setId_int(self.as_raw_DetectionBasedTracker_ExtObject(), val) }.into_result().expect("Infallible function failed: set_id")
	}
	
	fn location(&self) -> core::Rect {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_location_const(self.as_raw_DetectionBasedTracker_ExtObject()) }.into_result().expect("Infallible function failed: location")
	}
	
	fn set_location(&mut self, val: core::Rect) -> () {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_setLocation_Rect(self.as_raw_DetectionBasedTracker_ExtObject(), &val) }.into_result().expect("Infallible function failed: set_location")
	}
	
	fn status(&self) -> crate::objdetect::DetectionBasedTracker_ObjectStatus {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_status_const(self.as_raw_DetectionBasedTracker_ExtObject()) }.into_result().expect("Infallible function failed: status")
	}
	
	fn set_status(&mut self, val: crate::objdetect::DetectionBasedTracker_ObjectStatus) -> () {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_setStatus_ObjectStatus(self.as_raw_DetectionBasedTracker_ExtObject(), val) }.into_result().expect("Infallible function failed: set_status")
	}
	
}

pub struct DetectionBasedTracker_ExtObject {
	pub(crate) ptr: *mut c_void
}

impl Drop for DetectionBasedTracker_ExtObject {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionBasedTracker_ExtObject_delete(instance: *mut c_void); }
		unsafe { cv_DetectionBasedTracker_ExtObject_delete(self.as_raw_DetectionBasedTracker_ExtObject()) };
	}
}

impl DetectionBasedTracker_ExtObject {
	pub fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DetectionBasedTracker_ExtObject {}

impl crate::objdetect::DetectionBasedTracker_ExtObjectTrait for DetectionBasedTracker_ExtObject {
	fn as_raw_DetectionBasedTracker_ExtObject(&self) -> *mut c_void { self.ptr }
}

impl DetectionBasedTracker_ExtObject {
	pub fn new(_id: i32, _location: core::Rect, _status: crate::objdetect::DetectionBasedTracker_ObjectStatus) -> Result<crate::objdetect::DetectionBasedTracker_ExtObject> {
		unsafe { sys::cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(_id, &_location, _status) }.into_result().map(|ptr| crate::objdetect::DetectionBasedTracker_ExtObject { ptr })
	}
	
}

pub trait DetectionBasedTracker_IDetector {
	fn as_raw_DetectionBasedTracker_IDetector(&self) -> *mut c_void;
	fn detect(&mut self, image: &core::Mat, objects: &mut types::VectorOfRect) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_detect_const_MatX_vector_Rect_X(self.as_raw_DetectionBasedTracker_IDetector(), image.as_raw_Mat(), objects.as_raw_VectorOfRect()) }.into_result()
	}
	
	fn set_min_object_size(&mut self, min: core::Size) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeX(self.as_raw_DetectionBasedTracker_IDetector(), &min) }.into_result()
	}
	
	fn set_max_object_size(&mut self, max: core::Size) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeX(self.as_raw_DetectionBasedTracker_IDetector(), &max) }.into_result()
	}
	
	fn get_min_object_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn get_max_object_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn get_scale_factor(&mut self) -> Result<f32> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getScaleFactor(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn set_scale_factor(&mut self, value: f32) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setScaleFactor_float(self.as_raw_DetectionBasedTracker_IDetector(), value) }.into_result()
	}
	
	fn get_min_neighbours(&mut self) -> Result<i32> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_getMinNeighbours(self.as_raw_DetectionBasedTracker_IDetector()) }.into_result()
	}
	
	fn set_min_neighbours(&mut self, value: i32) -> Result<()> {
		unsafe { sys::cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(self.as_raw_DetectionBasedTracker_IDetector(), value) }.into_result()
	}
	
}

pub trait DetectionBasedTracker_ParametersTrait {
	fn as_raw_DetectionBasedTracker_Parameters(&self) -> *mut c_void;
	fn max_track_lifetime(&self) -> i32 {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_maxTrackLifetime_const(self.as_raw_DetectionBasedTracker_Parameters()) }.into_result().expect("Infallible function failed: max_track_lifetime")
	}
	
	fn set_max_track_lifetime(&mut self, val: i32) -> () {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_setMaxTrackLifetime_int(self.as_raw_DetectionBasedTracker_Parameters(), val) }.into_result().expect("Infallible function failed: set_max_track_lifetime")
	}
	
	fn min_detection_period(&self) -> i32 {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_minDetectionPeriod_const(self.as_raw_DetectionBasedTracker_Parameters()) }.into_result().expect("Infallible function failed: min_detection_period")
	}
	
	fn set_min_detection_period(&mut self, val: i32) -> () {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_setMinDetectionPeriod_int(self.as_raw_DetectionBasedTracker_Parameters(), val) }.into_result().expect("Infallible function failed: set_min_detection_period")
	}
	
}

pub struct DetectionBasedTracker_Parameters {
	pub(crate) ptr: *mut c_void
}

impl Drop for DetectionBasedTracker_Parameters {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionBasedTracker_Parameters_delete(instance: *mut c_void); }
		unsafe { cv_DetectionBasedTracker_Parameters_delete(self.as_raw_DetectionBasedTracker_Parameters()) };
	}
}

impl DetectionBasedTracker_Parameters {
	pub fn as_raw_DetectionBasedTracker_Parameters(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DetectionBasedTracker_Parameters {}

impl crate::objdetect::DetectionBasedTracker_ParametersTrait for DetectionBasedTracker_Parameters {
	fn as_raw_DetectionBasedTracker_Parameters(&self) -> *mut c_void { self.ptr }
}

impl DetectionBasedTracker_Parameters {
	pub fn default() -> Result<crate::objdetect::DetectionBasedTracker_Parameters> {
		unsafe { sys::cv_DetectionBasedTracker_Parameters_Parameters() }.into_result().map(|ptr| crate::objdetect::DetectionBasedTracker_Parameters { ptr })
	}
	
}

/// struct for detection region of interest (ROI)
pub trait DetectionROITrait {
	fn as_raw_DetectionROI(&self) -> *mut c_void;
	/// scale(size) of the bounding box
	fn scale(&self) -> f64 {
		unsafe { sys::cv_DetectionROI_scale_const(self.as_raw_DetectionROI()) }.into_result().expect("Infallible function failed: scale")
	}
	
	/// scale(size) of the bounding box
	fn set_scale(&mut self, val: f64) -> () {
		unsafe { sys::cv_DetectionROI_setScale_double(self.as_raw_DetectionROI(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	/// set of requested locations to be evaluated
	fn locations(&mut self) -> types::VectorOfPoint {
		unsafe { sys::cv_DetectionROI_locations(self.as_raw_DetectionROI()) }.into_result().map(|ptr| types::VectorOfPoint { ptr }).expect("Infallible function failed: locations")
	}
	
	/// set of requested locations to be evaluated
	fn set_locations(&mut self, val: types::VectorOfPoint) -> () {
		unsafe { sys::cv_DetectionROI_setLocations_vector_Point_(self.as_raw_DetectionROI(), val.as_raw_VectorOfPoint()) }.into_result().expect("Infallible function failed: set_locations")
	}
	
	/// vector that will contain confidence values for each location
	fn confidences(&mut self) -> types::VectorOff64 {
		unsafe { sys::cv_DetectionROI_confidences(self.as_raw_DetectionROI()) }.into_result().map(|ptr| types::VectorOff64 { ptr }).expect("Infallible function failed: confidences")
	}
	
	/// vector that will contain confidence values for each location
	fn set_confidences(&mut self, val: types::VectorOff64) -> () {
		unsafe { sys::cv_DetectionROI_setConfidences_vector_double_(self.as_raw_DetectionROI(), val.as_raw_VectorOff64()) }.into_result().expect("Infallible function failed: set_confidences")
	}
	
}

/// struct for detection region of interest (ROI)
pub struct DetectionROI {
	pub(crate) ptr: *mut c_void
}

impl Drop for DetectionROI {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionROI_delete(instance: *mut c_void); }
		unsafe { cv_DetectionROI_delete(self.as_raw_DetectionROI()) };
	}
}

impl DetectionROI {
	pub fn as_raw_DetectionROI(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DetectionROI {}

impl crate::objdetect::DetectionROITrait for DetectionROI {
	fn as_raw_DetectionROI(&self) -> *mut c_void { self.ptr }
}

impl DetectionROI {
}

/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
/// 
/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Dalal2005) .
/// 
/// useful links:
/// 
/// https://hal.inria.fr/inria-00548512/document/
/// 
/// https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients
/// 
/// https://software.intel.com/en-us/ipp-dev-reference-histogram-of-oriented-gradients-hog-descriptor
/// 
/// http://www.learnopencv.com/histogram-of-oriented-gradients
/// 
/// http://www.learnopencv.com/handwritten-digits-classification-an-opencv-c-python-tutorial
pub trait HOGDescriptorTrait {
	fn as_raw_HOGDescriptor(&self) -> *mut c_void;
	/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
	fn win_size(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_winSize_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: win_size")
	}
	
	/// Detection window size. Align to block size and block stride. Default value is Size(64,128).
	fn set_win_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setWinSize_Size(self.as_raw_HOGDescriptor(), &val) }.into_result().expect("Infallible function failed: set_win_size")
	}
	
	/// Block size in pixels. Align to cell size. Default value is Size(16,16).
	fn block_size(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_blockSize_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: block_size")
	}
	
	/// Block size in pixels. Align to cell size. Default value is Size(16,16).
	fn set_block_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setBlockSize_Size(self.as_raw_HOGDescriptor(), &val) }.into_result().expect("Infallible function failed: set_block_size")
	}
	
	/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
	fn block_stride(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_blockStride_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: block_stride")
	}
	
	/// Block stride. It must be a multiple of cell size. Default value is Size(8,8).
	fn set_block_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setBlockStride_Size(self.as_raw_HOGDescriptor(), &val) }.into_result().expect("Infallible function failed: set_block_stride")
	}
	
	/// Cell size. Default value is Size(8,8).
	fn cell_size(&self) -> core::Size {
		unsafe { sys::cv_HOGDescriptor_cellSize_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: cell_size")
	}
	
	/// Cell size. Default value is Size(8,8).
	fn set_cell_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_HOGDescriptor_setCellSize_Size(self.as_raw_HOGDescriptor(), &val) }.into_result().expect("Infallible function failed: set_cell_size")
	}
	
	/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
	fn nbins(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_nbins_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: nbins")
	}
	
	/// Number of bins used in the calculation of histogram of gradients. Default value is 9.
	fn set_nbins(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setNbins_int(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_nbins")
	}
	
	/// not documented
	fn deriv_aperture(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_derivAperture_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: deriv_aperture")
	}
	
	/// not documented
	fn set_deriv_aperture(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setDerivAperture_int(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_deriv_aperture")
	}
	
	/// Gaussian smoothing window parameter.
	fn win_sigma(&self) -> f64 {
		unsafe { sys::cv_HOGDescriptor_winSigma_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: win_sigma")
	}
	
	/// Gaussian smoothing window parameter.
	fn set_win_sigma(&mut self, val: f64) -> () {
		unsafe { sys::cv_HOGDescriptor_setWinSigma_double(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_win_sigma")
	}
	
	/// histogramNormType
	fn histogram_norm_type(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_histogramNormType_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: histogram_norm_type")
	}
	
	/// histogramNormType
	fn set_histogram_norm_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setHistogramNormType_int(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_histogram_norm_type")
	}
	
	/// L2-Hys normalization method shrinkage.
	fn l2_hys_threshold(&self) -> f64 {
		unsafe { sys::cv_HOGDescriptor_L2HysThreshold_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: l2_hys_threshold")
	}
	
	/// L2-Hys normalization method shrinkage.
	fn set_l2_hys_threshold(&mut self, val: f64) -> () {
		unsafe { sys::cv_HOGDescriptor_setL2HysThreshold_double(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_l2_hys_threshold")
	}
	
	/// Flag to specify whether the gamma correction preprocessing is required or not.
	fn gamma_correction(&self) -> bool {
		unsafe { sys::cv_HOGDescriptor_gammaCorrection_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: gamma_correction")
	}
	
	/// Flag to specify whether the gamma correction preprocessing is required or not.
	fn set_gamma_correction(&mut self, val: bool) -> () {
		unsafe { sys::cv_HOGDescriptor_setGammaCorrection_bool(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_gamma_correction")
	}
	
	/// coefficients for the linear SVM classifier.
	fn svm_detector(&mut self) -> types::VectorOff32 {
		unsafe { sys::cv_HOGDescriptor_svmDetector(self.as_raw_HOGDescriptor()) }.into_result().map(|ptr| types::VectorOff32 { ptr }).expect("Infallible function failed: svm_detector")
	}
	
	/// coefficients for the linear SVM classifier.
	fn set_svm_detector_vec(&mut self, val: types::VectorOff32) -> () {
		unsafe { sys::cv_HOGDescriptor_setSvmDetector_vector_float_(self.as_raw_HOGDescriptor(), val.as_raw_VectorOff32()) }.into_result().expect("Infallible function failed: set_svm_detector_vec")
	}
	
	/// coefficients for the linear SVM classifier used when OpenCL is enabled
	fn ocl_svm_detector(&mut self) -> core::UMat {
		unsafe { sys::cv_HOGDescriptor_oclSvmDetector(self.as_raw_HOGDescriptor()) }.into_result().map(|ptr| core::UMat { ptr }).expect("Infallible function failed: ocl_svm_detector")
	}
	
	/// coefficients for the linear SVM classifier used when OpenCL is enabled
	fn set_ocl_svm_detector(&mut self, val: core::UMat) -> () {
		unsafe { sys::cv_HOGDescriptor_setOclSvmDetector_UMat(self.as_raw_HOGDescriptor(), val.as_raw_UMat()) }.into_result().expect("Infallible function failed: set_ocl_svm_detector")
	}
	
	/// not documented
	fn free_coef(&self) -> f32 {
		unsafe { sys::cv_HOGDescriptor_free_coef_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: free_coef")
	}
	
	/// not documented
	fn set_free_coef(&mut self, val: f32) -> () {
		unsafe { sys::cv_HOGDescriptor_setFree_coef_float(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_free_coef")
	}
	
	/// Maximum number of detection window increases. Default value is 64
	fn nlevels(&self) -> i32 {
		unsafe { sys::cv_HOGDescriptor_nlevels_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: nlevels")
	}
	
	/// Maximum number of detection window increases. Default value is 64
	fn set_nlevels(&mut self, val: i32) -> () {
		unsafe { sys::cv_HOGDescriptor_setNlevels_int(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_nlevels")
	}
	
	/// Indicates signed gradient will be used or not
	fn signed_gradient(&self) -> bool {
		unsafe { sys::cv_HOGDescriptor_signedGradient_const(self.as_raw_HOGDescriptor()) }.into_result().expect("Infallible function failed: signed_gradient")
	}
	
	/// Indicates signed gradient will be used or not
	fn set_signed_gradient(&mut self, val: bool) -> () {
		unsafe { sys::cv_HOGDescriptor_setSignedGradient_bool(self.as_raw_HOGDescriptor(), val) }.into_result().expect("Infallible function failed: set_signed_gradient")
	}
	
	/// Returns the number of coefficients required for the classification.
	fn get_descriptor_size(&self) -> Result<size_t> {
		unsafe { sys::cv_HOGDescriptor_getDescriptorSize_const(self.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// Checks if detector size equal to descriptor size.
	fn check_detector_size(&self) -> Result<bool> {
		unsafe { sys::cv_HOGDescriptor_checkDetectorSize_const(self.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// Returns winSigma value
	fn get_win_sigma(&self) -> Result<f64> {
		unsafe { sys::cv_HOGDescriptor_getWinSigma_const(self.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// @example samples/cpp/peopledetect.cpp
	/// /
	/// Sets coefficients for the linear SVM classifier.
	/// ## Parameters
	/// * _svmdetector: coefficients for the linear SVM classifier.
	fn set_svm_detector(&mut self, _svmdetector: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(_svmdetector);
		unsafe { sys::cv_HOGDescriptor_setSVMDetector_const__InputArrayX(self.as_raw_HOGDescriptor(), _svmdetector.as_raw__InputArray()) }.into_result()
	}
	
	/// Reads HOGDescriptor parameters from a file node.
	/// ## Parameters
	/// * fn: File node
	fn read(&mut self, fn_: &mut core::FileNode) -> Result<bool> {
		unsafe { sys::cv_HOGDescriptor_read_FileNodeX(self.as_raw_HOGDescriptor(), fn_.as_raw_FileNode()) }.into_result()
	}
	
	/// Stores HOGDescriptor parameters in a file storage.
	/// ## Parameters
	/// * fs: File storage
	/// * objname: Object name
	fn write(&self, fs: &mut core::FileStorage, objname: &str) -> Result<()> {
		string_arg!(objname);
		unsafe { sys::cv_HOGDescriptor_write_const_FileStorageX_const_StringX(self.as_raw_HOGDescriptor(), fs.as_raw_FileStorage(), objname.as_ptr()) }.into_result()
	}
	
	/// loads coefficients for the linear SVM classifier from a file
	/// ## Parameters
	/// * filename: Name of the file to read.
	/// * objname: The optional name of the node to read (if empty, the first top-level node will be used).
	/// 
	/// ## C++ default parameters
	/// * objname: String()
	fn load(&mut self, filename: &str, objname: &str) -> Result<bool> {
		string_arg!(filename);
		string_arg!(objname);
		unsafe { sys::cv_HOGDescriptor_load_const_StringX_const_StringX(self.as_raw_HOGDescriptor(), filename.as_ptr(), objname.as_ptr()) }.into_result()
	}
	
	/// saves coefficients for the linear SVM classifier to a file
	/// ## Parameters
	/// * filename: File name
	/// * objname: Object name
	/// 
	/// ## C++ default parameters
	/// * objname: String()
	fn save(&self, filename: &str, objname: &str) -> Result<()> {
		string_arg!(filename);
		string_arg!(objname);
		unsafe { sys::cv_HOGDescriptor_save_const_const_StringX_const_StringX(self.as_raw_HOGDescriptor(), filename.as_ptr(), objname.as_ptr()) }.into_result()
	}
	
	/// clones the HOGDescriptor
	/// ## Parameters
	/// * c: cloned HOGDescriptor
	fn copy_to(&self, c: &mut crate::objdetect::HOGDescriptor) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_copyTo_const_HOGDescriptorX(self.as_raw_HOGDescriptor(), c.as_raw_HOGDescriptor()) }.into_result()
	}
	
	/// @example samples/cpp/train_HOG.cpp
	/// /
	/// Computes HOG descriptors of given image.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U containing an image where HOG features will be calculated.
	/// * descriptors: Matrix of the type CV_32F
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * locations: Vector of Point
	/// 
	/// ## C++ default parameters
	/// * win_stride: Size()
	/// * padding: Size()
	/// * locations: std::vector<Point>()
	fn compute(&self, img: &dyn core::ToInputArray, descriptors: &mut types::VectorOff32, win_stride: core::Size, padding: core::Size, locations: &types::VectorOfPoint) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_HOGDescriptor_compute_const_const__InputArrayX_vector_float_X_Size_Size_const_vector_Point_X(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), descriptors.as_raw_VectorOff32(), &win_stride, &padding, locations.as_raw_VectorOfPoint()) }.into_result()
	}
	
	/// Performs object detection without a multi-scale window.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
	/// * weights: Vector that will contain confidence values for each detected object.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * searchLocations: Vector of Point includes set of requested locations to be evaluated.
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * search_locations: std::vector<Point>()
	fn detect_weights(&self, img: &core::Mat, found_locations: &mut types::VectorOfPoint, weights: &mut types::VectorOff64, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &types::VectorOfPoint) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatX_vector_Point_X_vector_double_X_double_Size_Size_const_vector_Point_X(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfPoint(), weights.as_raw_VectorOff64(), hit_threshold, &win_stride, &padding, search_locations.as_raw_VectorOfPoint()) }.into_result()
	}
	
	/// Performs object detection without a multi-scale window.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of point where each point contains left-top corner point of detected object boundaries.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * searchLocations: Vector of Point includes locations to search.
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * search_locations: std::vector<Point>()
	fn detect(&self, img: &core::Mat, found_locations: &mut types::VectorOfPoint, hit_threshold: f64, win_stride: core::Size, padding: core::Size, search_locations: &types::VectorOfPoint) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detect_const_const_MatX_vector_Point_X_double_Size_Size_const_vector_Point_X(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfPoint(), hit_threshold, &win_stride, &padding, search_locations.as_raw_VectorOfPoint()) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
	/// * foundWeights: Vector that will contain confidence values for each detected object.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * scale: Coefficient of the detection window increase.
	/// * finalThreshold: Final threshold
	/// * useMeanshiftGrouping: indicates grouping algorithm
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * scale: 1.05
	/// * final_threshold: 2.0
	/// * use_meanshift_grouping: false
	fn detect_multi_scale_weights(&self, img: &dyn core::ToInputArray, found_locations: &mut types::VectorOfRect, found_weights: &mut types::VectorOff64, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_vector_double_X_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_VectorOfRect(), found_weights.as_raw_VectorOff64(), hit_threshold, &win_stride, &padding, scale, final_threshold, use_meanshift_grouping) }.into_result()
	}
	
	/// Detects objects of different sizes in the input image. The detected objects are returned as a list
	/// of rectangles.
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane.
	/// Usually it is 0 and should be specified in the detector coefficients (as the last free coefficient).
	/// But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * winStride: Window stride. It must be a multiple of block stride.
	/// * padding: Padding
	/// * scale: Coefficient of the detection window increase.
	/// * finalThreshold: Final threshold
	/// * useMeanshiftGrouping: indicates grouping algorithm
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	/// * scale: 1.05
	/// * final_threshold: 2.0
	/// * use_meanshift_grouping: false
	fn detect_multi_scale(&self, img: &dyn core::ToInputArray, found_locations: &mut types::VectorOfRect, hit_threshold: f64, win_stride: core::Size, padding: core::Size, scale: f64, final_threshold: f64, use_meanshift_grouping: bool) -> Result<()> {
		input_array_arg!(img);
		unsafe { sys::cv_HOGDescriptor_detectMultiScale_const_const__InputArrayX_vector_Rect_X_double_Size_Size_double_double_bool(self.as_raw_HOGDescriptor(), img.as_raw__InputArray(), found_locations.as_raw_VectorOfRect(), hit_threshold, &win_stride, &padding, scale, final_threshold, use_meanshift_grouping) }.into_result()
	}
	
	///  Computes gradients and quantized gradient orientations.
	/// ## Parameters
	/// * img: Matrix contains the image to be computed
	/// * grad: Matrix of type CV_32FC2 contains computed gradients
	/// * angleOfs: Matrix of type CV_8UC2 contains quantized gradient orientations
	/// * paddingTL: Padding from top-left
	/// * paddingBR: Padding from bottom-right
	/// 
	/// ## C++ default parameters
	/// * padding_tl: Size()
	/// * padding_br: Size()
	fn compute_gradient(&self, img: &core::Mat, grad: &mut core::Mat, angle_ofs: &mut core::Mat, padding_tl: core::Size, padding_br: core::Size) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_computeGradient_const_const_MatX_MatX_MatX_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), grad.as_raw_Mat(), angle_ofs.as_raw_Mat(), &padding_tl, &padding_br) }.into_result()
	}
	
	/// evaluate specified ROI and return confidence value for each location
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * locations: Vector of Point
	/// * foundLocations: Vector of Point where each Point is detected object's top-left point.
	/// * confidences: confidences
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually
	/// it is 0 and should be specified in the detector coefficients (as the last free coefficient). But if
	/// the free coefficient is omitted (which is allowed), you can specify it manually here
	/// * winStride: winStride
	/// * padding: padding
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * win_stride: Size()
	/// * padding: Size()
	fn detect_roi(&self, img: &core::Mat, locations: &types::VectorOfPoint, found_locations: &mut types::VectorOfPoint, confidences: &mut types::VectorOff64, hit_threshold: f64, win_stride: core::Size, padding: core::Size) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detectROI_const_const_MatX_const_vector_Point_X_vector_Point_X_vector_double_X_double_Size_Size(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), locations.as_raw_VectorOfPoint(), found_locations.as_raw_VectorOfPoint(), confidences.as_raw_VectorOff64(), hit_threshold, &win_stride, &padding) }.into_result()
	}
	
	/// evaluate specified ROI and return confidence value for each location in multiple scales
	/// ## Parameters
	/// * img: Matrix of the type CV_8U or CV_8UC3 containing an image where objects are detected.
	/// * foundLocations: Vector of rectangles where each rectangle contains the detected object.
	/// * locations: Vector of DetectionROI
	/// * hitThreshold: Threshold for the distance between features and SVM classifying plane. Usually it is 0 and should be specified
	/// in the detector coefficients (as the last free coefficient). But if the free coefficient is omitted (which is allowed), you can specify it manually here.
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
	/// 
	/// ## C++ default parameters
	/// * hit_threshold: 0
	/// * group_threshold: 0
	fn detect_multi_scale_roi(&self, img: &core::Mat, found_locations: &mut types::VectorOfRect, locations: &mut types::VectorOfDetectionROI, hit_threshold: f64, group_threshold: i32) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_detectMultiScaleROI_const_const_MatX_vector_Rect_X_vector_DetectionROI_X_double_int(self.as_raw_HOGDescriptor(), img.as_raw_Mat(), found_locations.as_raw_VectorOfRect(), locations.as_raw_VectorOfDetectionROI(), hit_threshold, group_threshold) }.into_result()
	}
	
	/// read/parse Dalal's alt model file
	/// ## Parameters
	/// * modelfile: Path of Dalal's alt model file.
	fn read_alt_model(&mut self, modelfile: &str) -> Result<()> {
		string_arg!(modelfile);
		unsafe { sys::cv_HOGDescriptor_readALTModel_String(self.as_raw_HOGDescriptor(), modelfile.as_ptr() as _) }.into_result()
	}
	
	/// Groups the object candidate rectangles.
	/// ## Parameters
	/// * rectList: Input/output vector of rectangles. Output vector includes retained and grouped rectangles. (The Python list is not modified in place.)
	/// * weights: Input/output vector of weights of rectangles. Output vector includes weights of retained and grouped rectangles. (The Python list is not modified in place.)
	/// * groupThreshold: Minimum possible number of rectangles minus 1. The threshold is used in a group of rectangles to retain it.
	/// * eps: Relative difference between sides of the rectangles to merge them into a group.
	fn group_rectangles(&self, rect_list: &mut types::VectorOfRect, weights: &mut types::VectorOff64, group_threshold: i32, eps: f64) -> Result<()> {
		unsafe { sys::cv_HOGDescriptor_groupRectangles_const_vector_Rect_X_vector_double_X_int_double(self.as_raw_HOGDescriptor(), rect_list.as_raw_VectorOfRect(), weights.as_raw_VectorOff64(), group_threshold, eps) }.into_result()
	}
	
}

/// Implementation of HOG (Histogram of Oriented Gradients) descriptor and object detector.
/// 
/// the HOG descriptor algorithm introduced by Navneet Dalal and Bill Triggs [Dalal2005](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Dalal2005) .
/// 
/// useful links:
/// 
/// https://hal.inria.fr/inria-00548512/document/
/// 
/// https://en.wikipedia.org/wiki/Histogram_of_oriented_gradients
/// 
/// https://software.intel.com/en-us/ipp-dev-reference-histogram-of-oriented-gradients-hog-descriptor
/// 
/// http://www.learnopencv.com/histogram-of-oriented-gradients
/// 
/// http://www.learnopencv.com/handwritten-digits-classification-an-opencv-c-python-tutorial
pub struct HOGDescriptor {
	pub(crate) ptr: *mut c_void
}

impl Drop for HOGDescriptor {
	fn drop(&mut self) {
		extern "C" { fn cv_HOGDescriptor_delete(instance: *mut c_void); }
		unsafe { cv_HOGDescriptor_delete(self.as_raw_HOGDescriptor()) };
	}
}

impl HOGDescriptor {
	pub fn as_raw_HOGDescriptor(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for HOGDescriptor {}

impl crate::objdetect::HOGDescriptorTrait for HOGDescriptor {
	fn as_raw_HOGDescriptor(&self) -> *mut c_void { self.ptr }
}

impl HOGDescriptor {
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	pub fn default() -> Result<crate::objdetect::HOGDescriptor> {
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor() }.into_result().map(|ptr| crate::objdetect::HOGDescriptor { ptr })
	}
	
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * _winSize: sets winSize with given value.
	/// * _blockSize: sets blockSize with given value.
	/// * _blockStride: sets blockStride with given value.
	/// * _cellSize: sets cellSize with given value.
	/// * _nbins: sets nbins with given value.
	/// * _derivAperture: sets derivAperture with given value.
	/// * _winSigma: sets winSigma with given value.
	/// * _histogramNormType: sets histogramNormType with given value.
	/// * _L2HysThreshold: sets L2HysThreshold with given value.
	/// * _gammaCorrection: sets gammaCorrection with given value.
	/// * _nlevels: sets nlevels with given value.
	/// * _signedGradient: sets signedGradient with given value.
	/// 
	/// ## C++ default parameters
	/// * _deriv_aperture: 1
	/// * _win_sigma: -1
	/// * _histogram_norm_type: HOGDescriptor::L2Hys
	/// * _l2_hys_threshold: 0.2
	/// * _gamma_correction: false
	/// * _nlevels: HOGDescriptor::DEFAULT_NLEVELS
	/// * _signed_gradient: false
	pub fn new(_win_size: core::Size, _block_size: core::Size, _block_stride: core::Size, _cell_size: core::Size, _nbins: i32, _deriv_aperture: i32, _win_sigma: f64, _histogram_norm_type: i32, _l2_hys_threshold: f64, _gamma_correction: bool, _nlevels: i32, _signed_gradient: bool) -> Result<crate::objdetect::HOGDescriptor> {
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_int_double_bool_int_bool(&_win_size, &_block_size, &_block_stride, &_cell_size, _nbins, _deriv_aperture, _win_sigma, _histogram_norm_type, _l2_hys_threshold, _gamma_correction, _nlevels, _signed_gradient) }.into_result().map(|ptr| crate::objdetect::HOGDescriptor { ptr })
	}
	
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * filename: the file name containing  HOGDescriptor properties and coefficients of the trained classifier
	pub fn new_from_file(filename: &str) -> Result<crate::objdetect::HOGDescriptor> {
		string_arg!(filename);
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_StringX(filename.as_ptr()) }.into_result().map(|ptr| crate::objdetect::HOGDescriptor { ptr })
	}
	
	/// Creates the HOG descriptor and detector with default params.
	/// 
	/// aqual to HOGDescriptor(Size(64,128), Size(16,16), Size(8,8), Size(8,8), 9, 1 )
	/// 
	/// ## Overloaded parameters
	/// 
	/// ## Parameters
	/// * d: the HOGDescriptor which cloned to create a new one.
	pub fn copy(d: &crate::objdetect::HOGDescriptor) -> Result<crate::objdetect::HOGDescriptor> {
		unsafe { sys::cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorX(d.as_raw_HOGDescriptor()) }.into_result().map(|ptr| crate::objdetect::HOGDescriptor { ptr })
	}
	
	/// Returns coefficients of the classifier trained for people detection (for 64x128 windows).
	pub fn get_default_people_detector() -> Result<types::VectorOff32> {
		unsafe { sys::cv_HOGDescriptor_getDefaultPeopleDetector() }.into_result().map(|ptr| types::VectorOff32 { ptr })
	}
	
	/// @example samples/tapi/hog.cpp
	/// /
	/// Returns coefficients of the classifier trained for people detection (for 48x96 windows).
	pub fn get_daimler_people_detector() -> Result<types::VectorOff32> {
		unsafe { sys::cv_HOGDescriptor_getDaimlerPeopleDetector() }.into_result().map(|ptr| types::VectorOff32 { ptr })
	}
	
}

pub trait QRCodeDetectorTrait {
	fn as_raw_QRCodeDetector(&self) -> *mut c_void;
	/// sets the epsilon used during the horizontal scan of QR code stop marker detection.
	/// ## Parameters
	/// * epsX: Epsilon neighborhood, which allows you to determine the horizontal pattern
	/// of the scheme 1:1:3:1:1 according to QR code standard.
	fn set_eps_x(&mut self, eps_x: f64) -> Result<()> {
		unsafe { sys::cv_QRCodeDetector_setEpsX_double(self.as_raw_QRCodeDetector(), eps_x) }.into_result()
	}
	
	/// sets the epsilon used during the vertical scan of QR code stop marker detection.
	/// ## Parameters
	/// * epsY: Epsilon neighborhood, which allows you to determine the vertical pattern
	/// of the scheme 1:1:3:1:1 according to QR code standard.
	fn set_eps_y(&mut self, eps_y: f64) -> Result<()> {
		unsafe { sys::cv_QRCodeDetector_setEpsY_double(self.as_raw_QRCodeDetector(), eps_y) }.into_result()
	}
	
	/// Detects QR code in image and returns the quadrangle containing the code.
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing (or not) QR code.
	/// * points: Output vector of vertices of the minimum-area quadrangle containing the code.
	fn detect(&self, img: &dyn core::ToInputArray, points: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(img);
		output_array_arg!(points);
		unsafe { sys::cv_QRCodeDetector_detect_const_const__InputArrayX_const__OutputArrayX(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray()) }.into_result()
	}
	
	/// Decodes QR code in image once it's found by the detect() method.
	/// Returns UTF8-encoded output string or empty string if the code cannot be decoded.
	/// 
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: Quadrangle vertices found by detect() method (or some other algorithm).
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	/// 
	/// ## C++ default parameters
	/// * straight_qrcode: noArray()
	fn decode(&mut self, img: &dyn core::ToInputArray, points: &dyn core::ToInputArray, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<String> {
		input_array_arg!(img);
		input_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_decode_const__InputArrayX_const__InputArrayX_const__OutputArrayX(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__InputArray(), straight_qrcode.as_raw__OutputArray()) }.into_result().map(crate::templ::receive_string)
	}
	
	/// Both detects and decodes QR code
	/// 
	/// ## Parameters
	/// * img: grayscale or color (BGR) image containing QR code.
	/// * points: opiotnal output array of vertices of the found QR code quadrangle. Will be empty if not found.
	/// * straight_qrcode: The optional output image containing rectified and binarized QR code
	/// 
	/// ## C++ default parameters
	/// * points: noArray()
	/// * straight_qrcode: noArray()
	fn detect_and_decode(&mut self, img: &dyn core::ToInputArray, points: &mut dyn core::ToOutputArray, straight_qrcode: &mut dyn core::ToOutputArray) -> Result<String> {
		input_array_arg!(img);
		output_array_arg!(points);
		output_array_arg!(straight_qrcode);
		unsafe { sys::cv_QRCodeDetector_detectAndDecode_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(self.as_raw_QRCodeDetector(), img.as_raw__InputArray(), points.as_raw__OutputArray(), straight_qrcode.as_raw__OutputArray()) }.into_result().map(crate::templ::receive_string)
	}
	
}

pub struct QRCodeDetector {
	pub(crate) ptr: *mut c_void
}

impl Drop for QRCodeDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_QRCodeDetector_delete(instance: *mut c_void); }
		unsafe { cv_QRCodeDetector_delete(self.as_raw_QRCodeDetector()) };
	}
}

impl QRCodeDetector {
	pub fn as_raw_QRCodeDetector(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for QRCodeDetector {}

impl crate::objdetect::QRCodeDetectorTrait for QRCodeDetector {
	fn as_raw_QRCodeDetector(&self) -> *mut c_void { self.ptr }
}

impl QRCodeDetector {
	pub fn default() -> Result<crate::objdetect::QRCodeDetector> {
		unsafe { sys::cv_QRCodeDetector_QRCodeDetector() }.into_result().map(|ptr| crate::objdetect::QRCodeDetector { ptr })
	}
	
}

/// class for grouping object candidates, detected by Cascade Classifier, HOG etc.
/// instance of the class is to be passed to cv::partition (see cxoperations.hpp)
pub trait SimilarRectsTrait {
	fn as_raw_SimilarRects(&self) -> *mut c_void;
	fn eps(&self) -> f64 {
		unsafe { sys::cv_SimilarRects_eps_const(self.as_raw_SimilarRects()) }.into_result().expect("Infallible function failed: eps")
	}
	
	fn set_eps(&mut self, val: f64) -> () {
		unsafe { sys::cv_SimilarRects_setEps_double(self.as_raw_SimilarRects(), val) }.into_result().expect("Infallible function failed: set_eps")
	}
	
}

/// class for grouping object candidates, detected by Cascade Classifier, HOG etc.
/// instance of the class is to be passed to cv::partition (see cxoperations.hpp)
pub struct SimilarRects {
	pub(crate) ptr: *mut c_void
}

impl Drop for SimilarRects {
	fn drop(&mut self) {
		extern "C" { fn cv_SimilarRects_delete(instance: *mut c_void); }
		unsafe { cv_SimilarRects_delete(self.as_raw_SimilarRects()) };
	}
}

impl SimilarRects {
	pub fn as_raw_SimilarRects(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for SimilarRects {}

impl crate::objdetect::SimilarRectsTrait for SimilarRects {
	fn as_raw_SimilarRects(&self) -> *mut c_void { self.ptr }
}

impl SimilarRects {
	pub fn new(_eps: f64) -> Result<crate::objdetect::SimilarRects> {
		unsafe { sys::cv_SimilarRects_SimilarRects_double(_eps) }.into_result().map(|ptr| crate::objdetect::SimilarRects { ptr })
	}
	
}
