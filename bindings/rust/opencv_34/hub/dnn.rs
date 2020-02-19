//! # Deep Neural Network module
//!   This module contains:
//!       - API for new layers creation, layers are building bricks of neural networks;
//!       - set of built-in most-useful Layers;
//!       - API to construct and modify comprehensive neural networks from layers;
//!       - functionality for loading serialized networks models from different frameworks.
//! 
//!   Functionality of this module is designed only for forward pass computations (i.e. network testing).
//!   A network training is in principle not supported.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::DictValueTrait, super::DictTrait, super::LayerParamsTrait, super::BackendNodeTrait, super::BackendWrapper, super::LayerTrait, super::NetTrait, super::LayerFactoryTrait, super::BlankLayerTrait, super::ConstLayerTrait, super::LSTMLayer, super::RNNLayer, super::BaseConvolutionLayerTrait, super::ConvolutionLayerTrait, super::DeconvolutionLayerTrait, super::LRNLayerTrait, super::PoolingLayerTrait, super::SoftmaxLayerTrait, super::InnerProductLayerTrait, super::MVNLayerTrait, super::ReshapeLayerTrait, super::FlattenLayerTrait, super::ConcatLayerTrait, super::SplitLayerTrait, super::SliceLayerTrait, super::PermuteLayerTrait, super::ShuffleChannelLayerTrait, super::PaddingLayerTrait, super::ActivationLayer, super::ReLULayer, super::ReLU6Layer, super::ChannelsPReLULayer, super::ELULayer, super::TanHLayer, super::SwishLayer, super::MishLayer, super::SigmoidLayer, super::BNLLLayer, super::AbsLayer, super::PowerLayer, super::CropLayerTrait, super::EltwiseLayerTrait, super::BatchNormLayer, super::MaxUnpoolLayerTrait, super::ScaleLayerTrait, super::ShiftLayerTrait, super::PriorBoxLayerTrait, super::ReorgLayerTrait, super::RegionLayerTrait, super::DetectionOutputLayerTrait, super::NormalizeBBoxLayerTrait, super::ResizeLayerTrait, super::InterpLayerTrait, super::ProposalLayerTrait, super::CropAndResizeLayerTrait, super::_RangeTrait };
}

pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NGRAPH: &'static str = "NGRAPH";
pub const CV_DNN_BACKEND_INFERENCE_ENGINE_NN_BUILDER_API: &'static str = "NN_BUILDER";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_2: &'static str = "Myriad2";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_MYRIAD_X: &'static str = "MyriadX";
pub const CV_DNN_INFERENCE_ENGINE_VPU_TYPE_UNSPECIFIED: &'static str = "";
/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
/// OpenCV is built with Intel's Inference Engine library or
/// DNN_BACKEND_OPENCV otherwise.
pub const DNN_BACKEND_DEFAULT: i32 = 0;
/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
/// OpenCV is built with Intel's Inference Engine library or
/// DNN_BACKEND_OPENCV otherwise.
pub const DNN_BACKEND_HALIDE: i32 = 1;
/// Intel's Inference Engine computational backend
/// ## See also
/// setInferenceEngineBackendType
pub const DNN_BACKEND_INFERENCE_ENGINE: i32 = 2;
pub const DNN_BACKEND_OPENCV: i32 = 3;
pub const DNN_TARGET_CPU: i32 = 0;
/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
pub const DNN_TARGET_FPGA: i32 = 4;
pub const DNN_TARGET_MYRIAD: i32 = 3;
pub const DNN_TARGET_OPENCL: i32 = 1;
pub const DNN_TARGET_OPENCL_FP16: i32 = 2;
/// Enum of computation backends supported by layers.
/// ## See also
/// Net::setPreferableBackend
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Backend {
	/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
	/// OpenCV is built with Intel's Inference Engine library or
	/// DNN_BACKEND_OPENCV otherwise.
	DNN_BACKEND_DEFAULT = 0 as isize,
	/// DNN_BACKEND_DEFAULT equals to DNN_BACKEND_INFERENCE_ENGINE if
	/// OpenCV is built with Intel's Inference Engine library or
	/// DNN_BACKEND_OPENCV otherwise.
	DNN_BACKEND_HALIDE = 1 as isize,
	/// Intel's Inference Engine computational backend
	/// ## See also
	/// setInferenceEngineBackendType
	DNN_BACKEND_INFERENCE_ENGINE = 2 as isize,
	DNN_BACKEND_OPENCV = 3 as isize,
}

/// Enum of target devices for computations.
/// ## See also
/// Net::setPreferableTarget
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Target {
	DNN_TARGET_CPU = 0 as isize,
	DNN_TARGET_OPENCL = 1 as isize,
	DNN_TARGET_OPENCL_FP16 = 2 as isize,
	DNN_TARGET_MYRIAD = 3 as isize,
	/// FPGA device with CPU fallbacks using Inference Engine's Heterogeneous plugin.
	DNN_TARGET_FPGA = 4 as isize,
}

/// Each Layer class must provide this function to the factory
pub type LayerFactory_Constructor = Option<extern "C" fn(*mut c_void) -> *mut c_void>;
pub type MatShape = types::VectorOfi32;
/// Container for strings and integers.
pub type Net_LayerId = crate::dnn::DictValue;
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes_f64(bboxes: &types::VectorOfRect2d, scores: &types::VectorOff32, score_threshold: f32, nms_threshold: f32, indices: &mut types::VectorOfi32, eta: f32, top_k: i32) -> Result<()> {
	unsafe { sys::cv_dnn_NMSBoxes_const_vector_Rect2d_X_const_vector_float_X_float_float_vector_int_X_float_int(bboxes.as_raw_VectorOfRect2d(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_VectorOfi32(), eta, top_k) }.into_result()
}

/// Performs non maximum suppression given boxes and corresponding scores.
/// 
/// ## Parameters
/// * bboxes: a set of bounding boxes to apply NMS.
/// * scores: a set of corresponding confidences.
/// * score_threshold: a threshold used to filter boxes by score.
/// * nms_threshold: a threshold used in non maximum suppression.
/// * indices: the kept indices of bboxes after NMS.
/// * eta: a coefficient in adaptive threshold formula: ![inline formula](https://latex.codecogs.com/png.latex?nms%5C%5Fthreshold%5F%7Bi%2B1%7D%3Deta%5Ccdot%20nms%5C%5Fthreshold%5Fi).
/// * top_k: if `>0`, keep at most @p top_k picked indices.
/// 
/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes(bboxes: &types::VectorOfRect, scores: &types::VectorOff32, score_threshold: f32, nms_threshold: f32, indices: &mut types::VectorOfi32, eta: f32, top_k: i32) -> Result<()> {
	unsafe { sys::cv_dnn_NMSBoxes_const_vector_Rect_X_const_vector_float_X_float_float_vector_int_X_float_int(bboxes.as_raw_VectorOfRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_VectorOfi32(), eta, top_k) }.into_result()
}

/// ## C++ default parameters
/// * eta: 1.f
/// * top_k: 0
pub fn nms_boxes_rotated(bboxes: &types::VectorOfRotatedRect, scores: &types::VectorOff32, score_threshold: f32, nms_threshold: f32, indices: &mut types::VectorOfi32, eta: f32, top_k: i32) -> Result<()> {
	unsafe { sys::cv_dnn_NMSBoxes_const_vector_RotatedRect_X_const_vector_float_X_float_float_vector_int_X_float_int(bboxes.as_raw_VectorOfRotatedRect(), scores.as_raw_VectorOff32(), score_threshold, nms_threshold, indices.as_raw_VectorOfi32(), eta, top_k) }.into_result()
}

/// Creates 4-dimensional blob from image.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
/// 
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_image_to(image: &dyn core::ToInputArray, blob: &mut dyn core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(image);
	output_array_arg!(blob);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayX_const__OutputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int(image.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth) }.into_result()
}

/// Creates 4-dimensional blob from image. Optionally resizes and crops @p image from center,
/// subtract @p mean values, scales values by @p scalefactor, swap Blue and Red channels.
/// ## Parameters
/// * image: input image (with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
/// to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p image values.
/// * swapRB: flag which indicates that swap first and last channels
/// in 3-channel image is necessary.
/// * crop: flag which indicates whether image will be cropped after resize or not
/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
/// @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
/// dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
/// If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
/// ## Returns
/// 4-dimensional Mat with NCHW dimensions order.
/// 
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_image(image: &dyn core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
	input_array_arg!(image);
	unsafe { sys::cv_dnn_blobFromImage_const__InputArrayX_double_const_SizeX_const_ScalarX_bool_bool_int(image.as_raw__InputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Creates 4-dimensional blob from series of images.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
/// 
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_images_to(images: &dyn core::ToInputArray, blob: &mut dyn core::ToOutputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<()> {
	input_array_arg!(images);
	output_array_arg!(blob);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayX_const__OutputArrayX_double_Size_const_ScalarX_bool_bool_int(images.as_raw__InputArray(), blob.as_raw__OutputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth) }.into_result()
}

/// Creates 4-dimensional blob from series of images. Optionally resizes and
/// crops @p images from center, subtract @p mean values, scales values by @p scalefactor,
/// swap Blue and Red channels.
/// ## Parameters
/// * images: input images (all with 1-, 3- or 4-channels).
/// * size: spatial size for output image
/// * mean: scalar with mean values which are subtracted from channels. Values are intended
/// to be in (mean-R, mean-G, mean-B) order if @p image has BGR ordering and @p swapRB is true.
/// * scalefactor: multiplier for @p images values.
/// * swapRB: flag which indicates that swap first and last channels
/// in 3-channel image is necessary.
/// * crop: flag which indicates whether image will be cropped after resize or not
/// * ddepth: Depth of output blob. Choose CV_32F or CV_8U.
/// @details if @p crop is true, input image is resized so one side after resize is equal to corresponding
/// dimension in @p size and another one is equal or larger. Then, crop from the center is performed.
/// If @p crop is false, direct resize without cropping and preserving aspect ratio is performed.
/// ## Returns
/// 4-dimensional Mat with NCHW dimensions order.
/// 
/// ## C++ default parameters
/// * scalefactor: 1.0
/// * size: Size()
/// * mean: Scalar()
/// * swap_rb: false
/// * crop: false
/// * ddepth: CV_32F
pub fn blob_from_images(images: &dyn core::ToInputArray, scalefactor: f64, size: core::Size, mean: core::Scalar, swap_rb: bool, crop: bool, ddepth: i32) -> Result<core::Mat> {
	input_array_arg!(images);
	unsafe { sys::cv_dnn_blobFromImages_const__InputArrayX_double_Size_const_ScalarX_bool_bool_int(images.as_raw__InputArray(), scalefactor, &size, &mean, swap_rb, crop, ddepth) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn clamp_2(r: &core::Range, axis_size: i32) -> Result<core::Range> {
	unsafe { sys::cv_dnn_clamp_const_RangeX_int(r.as_raw_Range(), axis_size) }.into_result().map(|ptr| core::Range { ptr })
}

pub fn clamp_1(ax: i32, shape: &crate::dnn::MatShape) -> Result<i32> {
	unsafe { sys::cv_dnn_clamp_int_const_MatShapeX(ax, shape.as_raw_VectorOfi32()) }.into_result()
}

pub fn clamp(ax: i32, dims: i32) -> Result<i32> {
	unsafe { sys::cv_dnn_clamp_int_int(ax, dims) }.into_result()
}

pub fn concat(a: &crate::dnn::MatShape, b: &crate::dnn::MatShape) -> Result<types::VectorOfi32> {
	unsafe { sys::cv_dnn_concat_const_MatShapeX_const_MatShapeX(a.as_raw_VectorOfi32(), b.as_raw_VectorOfi32()) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
}

pub fn get_available_targets(be: crate::dnn::Backend) -> Result<types::VectorOfTarget> {
	unsafe { sys::cv_dnn_getAvailableTargets_Backend(be) }.into_result().map(|ptr| types::VectorOfTarget { ptr })
}

/// Returns Inference Engine internal backend API.
/// 
/// See values of `CV_DNN_BACKEND_INFERENCE_ENGINE_*` macros.
/// 
/// Default value is controlled through `OPENCV_DNN_BACKEND_INFERENCE_ENGINE_TYPE` runtime parameter (environment variable).
pub fn get_inference_engine_backend_type() -> Result<String> {
	unsafe { sys::cv_dnn_getInferenceEngineBackendType() }.into_result().map(crate::templ::receive_string)
}

/// Returns Inference Engine VPU type.
/// 
/// See values of `CV_DNN_INFERENCE_ENGINE_VPU_TYPE_*` macros.
pub fn get_inference_engine_vpu_type() -> Result<String> {
	unsafe { sys::cv_dnn_getInferenceEngineVPUType() }.into_result().map(crate::templ::receive_string)
}

pub fn get_plane(m: &core::Mat, n: i32, cn: i32) -> Result<core::Mat> {
	unsafe { sys::cv_dnn_getPlane_const_MatX_int_int(m.as_raw_Mat(), n, cn) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Parse a 4D blob and output the images it contains as 2D arrays through a simpler data structure
/// (std::vector<cv::Mat>).
/// ## Parameters
/// * blob_: 4 dimensional array (images, channels, height, width) in floating point precision (CV_32F) from
/// which you would like to extract the images.
/// * images_:[out] array of 2D Mat containing the images extracted from the blob in floating point precision
/// (CV_32F). They are non normalized neither mean added. The number of returned images equals the first dimension
/// of the blob (batch size). Every image has a number of channels equals to the second dimension of the blob (depth).
pub fn images_from_blob(blob_: &core::Mat, images_: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(images_);
	unsafe { sys::cv_dnn_imagesFromBlob_const_MatX_const__OutputArrayX(blob_.as_raw_Mat(), images_.as_raw__OutputArray()) }.into_result()
}

/// ## C++ default parameters
/// * name: ""
pub fn print(shape: &crate::dnn::MatShape, name: &str) -> Result<()> {
	string_arg!(name);
	unsafe { sys::cv_dnn_print_const_MatShapeX_const_StringX(shape.as_raw_VectorOfi32(), name.as_ptr()) }.into_result()
}

/// Reads a network model stored in <a href="http://caffe.berkeleyvision.org">Caffe</a> framework's format.
/// ## Parameters
/// * prototxt: path to the .prototxt file with text description of the network architecture.
/// * caffeModel: path to the .caffemodel file with learned network.
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * caffe_model: String()
pub fn read_net_from_caffe(prototxt: &str, caffe_model: &str) -> Result<crate::dnn::Net> {
	string_arg!(prototxt);
	string_arg!(caffe_model);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_StringX_const_StringX(prototxt.as_ptr(), caffe_model.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in Caffe model in memory.
/// @details This is an overloaded member function, provided for convenience.
/// It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * lenProto: length of bufferProto
/// * bufferModel: buffer containing the content of the .caffemodel file
/// * lenModel: length of bufferModel
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * buffer_model: NULL
/// * len_model: 0
pub fn read_net_from_caffe_str(buffer_proto: &str, len_proto: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	string_arg!(buffer_proto);
	string_arg!(buffer_model);
	unsafe { sys::cv_dnn_readNetFromCaffe_const_charX_size_t_const_charX_size_t(buffer_proto.as_ptr(), len_proto, buffer_model.as_ptr(), len_model) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in Caffe model in memory.
/// ## Parameters
/// * bufferProto: buffer containing the content of the .prototxt file
/// * bufferModel: buffer containing the content of the .caffemodel file
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * buffer_model: std::vector<uchar>()
pub fn read_net_from_caffe_buffer(buffer_proto: &types::VectorOfu8, buffer_model: &types::VectorOfu8) -> Result<crate::dnn::Net> {
	unsafe { sys::cv_dnn_readNetFromCaffe_const_vector_unsigned_char_X_const_vector_unsigned_char_X(buffer_proto.as_raw_VectorOfu8(), buffer_model.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * cfgFile: path to the .cfg file with text description of the network architecture.
/// * darknetModel: path to the .weights file with learned network.
/// ## Returns
/// Network object that ready to do forward, throw an exception in failure cases.
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * darknet_model: String()
pub fn read_net_from_darknet(cfg_file: &str, darknet_model: &str) -> Result<crate::dnn::Net> {
	string_arg!(cfg_file);
	string_arg!(darknet_model);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_StringX_const_StringX(cfg_file.as_ptr(), darknet_model.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg: A buffer contains a content of .cfg file with text description of the network architecture.
/// * lenCfg: Number of bytes to read from bufferCfg
/// * bufferModel: A buffer contains a content of .weights file with learned network.
/// * lenModel: Number of bytes to read from bufferModel
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * buffer_model: NULL
/// * len_model: 0
pub fn read_net_from_darknet_str(buffer_cfg: &str, len_cfg: size_t, buffer_model: &str, len_model: size_t) -> Result<crate::dnn::Net> {
	string_arg!(buffer_cfg);
	string_arg!(buffer_model);
	unsafe { sys::cv_dnn_readNetFromDarknet_const_charX_size_t_const_charX_size_t(buffer_cfg.as_ptr(), len_cfg, buffer_model.as_ptr(), len_model) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="https://pjreddie.com/darknet/">Darknet</a> model files.
/// ## Parameters
/// * bufferCfg: A buffer contains a content of .cfg file with text description of the network architecture.
/// * bufferModel: A buffer contains a content of .weights file with learned network.
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * buffer_model: std::vector<uchar>()
pub fn read_net_from_darknet_buffer(buffer_cfg: &types::VectorOfu8, buffer_model: &types::VectorOfu8) -> Result<crate::dnn::Net> {
	unsafe { sys::cv_dnn_readNetFromDarknet_const_vector_unsigned_char_X_const_vector_unsigned_char_X(buffer_cfg.as_raw_VectorOfu8(), buffer_model.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * xml: XML configuration file with network's topology.
/// * bin: Binary file with trained weights.
/// ## Returns
/// Net object.
/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
/// backend.
pub fn read_net_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
	string_arg!(xml);
	string_arg!(bin);
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_StringX_const_StringX(xml.as_ptr(), bin.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * bufferModelConfigPtr: Pointer to buffer which contains XML configuration with network's topology.
/// * bufferModelConfigSize: Binary size of XML configuration data.
/// * bufferWeightsPtr: Pointer to buffer which contains binary data with trained weights.
/// * bufferWeightsSize: Binary size of trained weights data.
/// ## Returns
/// Net object.
/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
/// backend.
pub fn read_net_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Load a network from Intel's Model Optimizer intermediate representation.
/// ## Parameters
/// * bufferModelConfig: Buffer contains XML configuration with network's topology.
/// * bufferWeights: Buffer contains binary data with trained weights.
/// ## Returns
/// Net object.
/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
/// backend.
pub fn read_net_from_model_optimizer_1(buffer_model_config: &types::VectorOfu8, buffer_weights: &types::VectorOfu8) -> Result<crate::dnn::Net> {
	unsafe { sys::cv_dnn_readNetFromModelOptimizer_const_vector_unsigned_char_X_const_vector_unsigned_char_X(buffer_model_config.as_raw_VectorOfu8(), buffer_weights.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model <a href="https://onnx.ai/">ONNX</a>.
/// ## Parameters
/// * onnxFile: path to the .onnx file with text description of the network architecture.
/// ## Returns
/// Network object that ready to do forward, throw an exception in failure cases.
pub fn read_net_from_onnx(onnx_file: &str) -> Result<crate::dnn::Net> {
	string_arg!(onnx_file);
	unsafe { sys::cv_dnn_readNetFromONNX_const_StringX(onnx_file.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model from <a href="https://onnx.ai/">ONNX</a>
///        in-memory buffer.
/// ## Parameters
/// * buffer: memory address of the first byte of the buffer.
/// * sizeBuffer: size of the buffer.
/// ## Returns
/// Network object that ready to do forward, throw an exception
///       in failure cases.
pub fn read_net_from_onnx_str(buffer: &str, size_buffer: size_t) -> Result<crate::dnn::Net> {
	string_arg!(buffer);
	unsafe { sys::cv_dnn_readNetFromONNX_const_charX_size_t(buffer.as_ptr(), size_buffer) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model from <a href="https://onnx.ai/">ONNX</a>
///        in-memory buffer.
/// ## Parameters
/// * buffer: in-memory buffer that stores the ONNX model bytes.
/// ## Returns
/// Network object that ready to do forward, throw an exception
///       in failure cases.
pub fn read_net_from_onnx_buffer(buffer: &types::VectorOfu8) -> Result<crate::dnn::Net> {
	unsafe { sys::cv_dnn_readNetFromONNX_const_vector_unsigned_char_X(buffer.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * model: path to the .pb file with binary protobuf description of the network architecture
/// * config: path to the .pbtxt file that contains text graph definition in protobuf format.
///               Resulting Net object is built by text graph using weights from a binary one that
///               let us make it more flexible.
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * config: String()
pub fn read_net_from_tensorflow(model: &str, config: &str) -> Result<crate::dnn::Net> {
	string_arg!(model);
	string_arg!(config);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_StringX_const_StringX(model.as_ptr(), config.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// @details This is an overloaded member function, provided for convenience.
/// It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * lenModel: length of bufferModel
/// * bufferConfig: buffer containing the content of the pbtxt file
/// * lenConfig: length of bufferConfig
/// 
/// ## C++ default parameters
/// * buffer_config: NULL
/// * len_config: 0
pub fn read_net_from_tensorflow_str(buffer_model: &str, len_model: size_t, buffer_config: &str, len_config: size_t) -> Result<crate::dnn::Net> {
	string_arg!(buffer_model);
	string_arg!(buffer_config);
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_charX_size_t_const_charX_size_t(buffer_model.as_ptr(), len_model, buffer_config.as_ptr(), len_config) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="https://www.tensorflow.org/">TensorFlow</a> framework's format.
/// ## Parameters
/// * bufferModel: buffer containing the content of the pb file
/// * bufferConfig: buffer containing the content of the pbtxt file
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * buffer_config: std::vector<uchar>()
pub fn read_net_from_tensorflow_buffer(buffer_model: &types::VectorOfu8, buffer_config: &types::VectorOfu8) -> Result<crate::dnn::Net> {
	unsafe { sys::cv_dnn_readNetFromTensorflow_const_vector_unsigned_char_X_const_vector_unsigned_char_X(buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Reads a network model stored in <a href="http://torch.ch">Torch7</a> framework's format.
/// ## Parameters
/// * model: path to the file, dumped from Torch by using torch.save() function.
/// * isBinary: specifies whether the network was serialized in ascii mode or binary.
/// * evaluate: specifies testing phase of network. If true, it's similar to evaluate() method in Torch.
/// ## Returns
/// Net object.
/// 
///  
/// Note: Ascii mode of Torch serializer is more preferable, because binary mode extensively use `long` type of C language,
///  which has various bit-length on different systems.
/// 
/// The loading file must contain serialized <a href="https://github.com/torch/nn/blob/master/doc/module.md">nn.Module</a> object
/// with importing network. Try to eliminate a custom objects from serialazing data to avoid importing errors.
/// 
/// List of supported layers (i.e. object instances derived from Torch nn.Module class):
/// - nn.Sequential
/// - nn.Parallel
/// - nn.Concat
/// - nn.Linear
/// - nn.SpatialConvolution
/// - nn.SpatialMaxPooling, nn.SpatialAveragePooling
/// - nn.ReLU, nn.TanH, nn.Sigmoid
/// - nn.Reshape
/// - nn.SoftMax, nn.LogSoftMax
/// 
/// Also some equivalents of these classes from cunn, cudnn, and fbcunn may be successfully imported.
/// 
/// ## C++ default parameters
/// * is_binary: true
/// * evaluate: true
pub fn read_net_from_torch(model: &str, is_binary: bool, evaluate: bool) -> Result<crate::dnn::Net> {
	string_arg!(model);
	unsafe { sys::cv_dnn_readNetFromTorch_const_StringX_bool_bool(model.as_ptr(), is_binary, evaluate) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Read deep learning network represented in one of the supported formats.
/// ## Parameters
/// * model: Binary file contains trained weights. The following file
///                  extensions are expected for models from different frameworks:
///                  * `*.caffemodel` (Caffe, http://caffe.berkeleyvision.org/)
///                  * `*.pb` (TensorFlow, https://www.tensorflow.org/)
///                  * `*.t7` | `*.net` (Torch, http://torch.ch/)
///                  * `*.weights` (Darknet, https://pjreddie.com/darknet/)
///                  * `*.bin` (DLDT, https://software.intel.com/openvino-toolkit)
///                  * `*.onnx` (ONNX, https://onnx.ai/)
/// * config: Text file contains network configuration. It could be a
///                   file with the following extensions:
///                  * `*.prototxt` (Caffe, http://caffe.berkeleyvision.org/)
///                  * `*.pbtxt` (TensorFlow, https://www.tensorflow.org/)
///                  * `*.cfg` (Darknet, https://pjreddie.com/darknet/)
///                  * `*.xml` (DLDT, https://software.intel.com/openvino-toolkit)
/// * framework: Explicit framework name tag to determine a format.
/// ## Returns
/// Net object.
/// 
/// This function automatically detects an origin framework of trained model
/// and calls an appropriate function such @ref readNetFromCaffe, @ref readNetFromTensorflow,
/// @ref readNetFromTorch or @ref readNetFromDarknet. An order of @p model and @p config
/// arguments does not matter.
/// 
/// ## C++ default parameters
/// * config: ""
/// * framework: ""
pub fn read_net(model: &str, config: &str, framework: &str) -> Result<crate::dnn::Net> {
	string_arg!(model);
	string_arg!(config);
	string_arg!(framework);
	unsafe { sys::cv_dnn_readNet_const_StringX_const_StringX_const_StringX(model.as_ptr(), config.as_ptr(), framework.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Read deep learning network represented in one of the supported formats.
/// @details This is an overloaded member function, provided for convenience.
///          It differs from the above function only in what argument(s) it accepts.
/// ## Parameters
/// * framework: Name of origin framework.
/// * bufferModel: A buffer with a content of binary file with weights
/// * bufferConfig: A buffer with a content of text file contains network configuration.
/// ## Returns
/// Net object.
/// 
/// ## C++ default parameters
/// * buffer_config: std::vector<uchar>()
pub fn read_net_1(framework: &str, buffer_model: &types::VectorOfu8, buffer_config: &types::VectorOfu8) -> Result<crate::dnn::Net> {
	string_arg!(framework);
	unsafe { sys::cv_dnn_readNet_const_StringX_const_vector_unsigned_char_X_const_vector_unsigned_char_X(framework.as_ptr(), buffer_model.as_raw_VectorOfu8(), buffer_config.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
}

/// Creates blob from .pb file.
/// ## Parameters
/// * path: to the .pb file with input tensor.
/// ## Returns
/// Mat.
pub fn read_tensor_from_onnx(path: &str) -> Result<core::Mat> {
	string_arg!(path);
	unsafe { sys::cv_dnn_readTensorFromONNX_const_StringX(path.as_ptr()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Loads blob which was serialized as torch.Tensor object of Torch7 framework.
/// @warning This function has the same limitations as readNetFromTorch().
/// 
/// ## C++ default parameters
/// * is_binary: true
pub fn read_torch_blob(filename: &str, is_binary: bool) -> Result<core::Mat> {
	string_arg!(filename);
	unsafe { sys::cv_dnn_readTorchBlob_const_StringX_bool(filename.as_ptr(), is_binary) }.into_result().map(|ptr| core::Mat { ptr })
}

/// Release a Myriad device (binded by OpenCV).
/// 
/// Single Myriad device cannot be shared across multiple processes which uses
/// Inference Engine's Myriad plugin.
pub fn reset_myriad_device() -> Result<()> {
	unsafe { sys::cv_dnn_resetMyriadDevice() }.into_result()
}

/// Specify Inference Engine internal backend API.
/// 
/// See values of `CV_DNN_BACKEND_INFERENCE_ENGINE_*` macros.
/// 
/// ## Returns
/// previous value of internal backend API
pub fn set_inference_engine_backend_type(new_backend_type: &str) -> Result<String> {
	string_arg!(new_backend_type);
	unsafe { sys::cv_dnn_setInferenceEngineBackendType_const_StringX(new_backend_type.as_ptr()) }.into_result().map(crate::templ::receive_string)
}

pub fn shape_2(sz: &core::MatSize) -> Result<types::VectorOfi32> {
	unsafe { sys::cv_dnn_shape_const_MatSizeX(sz.as_raw_MatSize()) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
}

pub fn shape_1(mat: &core::Mat) -> Result<types::VectorOfi32> {
	unsafe { sys::cv_dnn_shape_const_MatX(mat.as_raw_Mat()) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
}

pub fn shape_3(mat: &core::UMat) -> Result<types::VectorOfi32> {
	unsafe { sys::cv_dnn_shape_const_UMatX(mat.as_raw_UMat()) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
}

pub fn shape(dims: &i32, n: i32) -> Result<types::VectorOfi32> {
	unsafe { sys::cv_dnn_shape_const_intX_int(dims, n) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
}

/// ## C++ default parameters
/// * a1: -1
/// * a2: -1
/// * a3: -1
pub fn shape_4(a0: i32, a1: i32, a2: i32, a3: i32) -> Result<types::VectorOfi32> {
	unsafe { sys::cv_dnn_shape_int_int_int_int(a0, a1, a2, a3) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
}

/// Convert all weights of Caffe network to half precision floating point.
/// ## Parameters
/// * src: Path to origin model from Caffe framework contains single
///            precision floating point weights (usually has `.caffemodel` extension).
/// * dst: Path to destination model with updated weights.
/// * layersTypes: Set of layers types which parameters will be converted.
///                    By default, converts only Convolutional and Fully-Connected layers'
///                    weights.
/// 
/// 
/// Note: Shrinked model has no origin float32 weights so it can't be used
///       in origin Caffe framework anymore. However the structure of data
///       is taken from NVidia's Caffe fork: https://github.com/NVIDIA/caffe.
///       So the resulting model may be used there.
/// 
/// ## C++ default parameters
/// * layers_types: std::vector<String>()
pub fn shrink_caffe_model(src: &str, dst: &str, layers_types: &types::VectorOfString) -> Result<()> {
	string_arg!(src);
	string_arg!(dst);
	unsafe { sys::cv_dnn_shrinkCaffeModel_const_StringX_const_StringX_const_vector_String_X(src.as_ptr(), dst.as_ptr(), layers_types.as_raw_VectorOfString()) }.into_result()
}

pub fn slice(m: &core::Mat, r0: &crate::dnn::_Range) -> Result<core::Mat> {
	unsafe { sys::cv_dnn_slice_const_MatX_const__RangeX(m.as_raw_Mat(), r0.as_raw__Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn slice_1(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range) -> Result<core::Mat> {
	unsafe { sys::cv_dnn_slice_const_MatX_const__RangeX_const__RangeX(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn slice_2(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range, r2: &crate::dnn::_Range) -> Result<core::Mat> {
	unsafe { sys::cv_dnn_slice_const_MatX_const__RangeX_const__RangeX_const__RangeX(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

pub fn slice_3(m: &core::Mat, r0: &crate::dnn::_Range, r1: &crate::dnn::_Range, r2: &crate::dnn::_Range, r3: &crate::dnn::_Range) -> Result<core::Mat> {
	unsafe { sys::cv_dnn_slice_const_MatX_const__RangeX_const__RangeX_const__RangeX_const__RangeX(m.as_raw_Mat(), r0.as_raw__Range(), r1.as_raw__Range(), r2.as_raw__Range(), r3.as_raw__Range()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// ## C++ default parameters
/// * name: ""
pub fn to_string(shape: &crate::dnn::MatShape, name: &str) -> Result<String> {
	string_arg!(name);
	unsafe { sys::cv_dnn_toString_const_MatShapeX_const_StringX(shape.as_raw_VectorOfi32(), name.as_ptr()) }.into_result().map(crate::templ::receive_string)
}

/// ## C++ default parameters
/// * start: -1
/// * end: -1
pub fn total(shape: &crate::dnn::MatShape, start: i32, end: i32) -> Result<i32> {
	unsafe { sys::cv_dnn_total_const_MatShapeX_int_int(shape.as_raw_VectorOfi32(), start, end) }.into_result()
}

/// Create a text representation for a binary network stored in protocol buffer format.
/// ## Parameters
/// * model: A path to binary network.
/// * output: A path to output text file to be created.
/// 
/// 
/// Note: To reduce output file size, trained weights are not included.
pub fn write_text_graph(model: &str, output: &str) -> Result<()> {
	string_arg!(model);
	string_arg!(output);
	unsafe { sys::cv_dnn_writeTextGraph_const_StringX_const_StringX(model.as_ptr(), output.as_ptr()) }.into_result()
}

pub trait AbsLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_AbsLayer(&self) -> *mut c_void;
}

impl dyn AbsLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfAbsLayer> {
		unsafe { sys::cv_dnn_AbsLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfAbsLayer { ptr })
	}
	
}
pub trait ActivationLayer: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ActivationLayer(&self) -> *mut c_void;
	fn forward_slice(&self, src: &f32, dst: &mut f32, len: i32, out_plane_size: size_t, cn0: i32, cn1: i32) -> Result<()> {
		unsafe { sys::cv_dnn_ActivationLayer_forwardSlice_const_const_floatX_floatX_int_size_t_int_int(self.as_raw_ActivationLayer(), src, dst, len, out_plane_size, cn0, cn1) }.into_result()
	}
	
}

pub trait BNLLLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_BNLLLayer(&self) -> *mut c_void;
}

impl dyn BNLLLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBNLLLayer> {
		unsafe { sys::cv_dnn_BNLLLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBNLLLayer { ptr })
	}
	
}
/// Derivatives of this class encapsulates functions of certain backends.
pub trait BackendNodeTrait {
	fn as_raw_BackendNode(&self) -> *mut c_void;
	/// Backend identifier.
	fn backend_id(&self) -> i32 {
		unsafe { sys::cv_dnn_BackendNode_backendId_const(self.as_raw_BackendNode()) }.into_result().expect("Infallible function failed: backend_id")
	}
	
	/// Backend identifier.
	fn set_backend_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_BackendNode_setBackendId_int(self.as_raw_BackendNode(), val) }.into_result().expect("Infallible function failed: set_backend_id")
	}
	
}

/// Derivatives of this class encapsulates functions of certain backends.
pub struct BackendNode {
	pub(crate) ptr: *mut c_void
}

impl Drop for BackendNode {
	fn drop(&mut self) {
		extern "C" { fn cv_BackendNode_delete(instance: *mut c_void); }
		unsafe { cv_BackendNode_delete(self.as_raw_BackendNode()) };
	}
}

impl BackendNode {
	pub fn as_raw_BackendNode(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for BackendNode {}

impl crate::dnn::BackendNodeTrait for BackendNode {
	fn as_raw_BackendNode(&self) -> *mut c_void { self.ptr }
}

impl BackendNode {
	pub fn new(backend_id: i32) -> Result<crate::dnn::BackendNode> {
		unsafe { sys::cv_dnn_BackendNode_BackendNode_int(backend_id) }.into_result().map(|ptr| crate::dnn::BackendNode { ptr })
	}
	
}

/// Derivatives of this class wraps cv::Mat for different backends and targets.
pub trait BackendWrapper {
	fn as_raw_BackendWrapper(&self) -> *mut c_void;
	/// Backend identifier.
	fn backend_id(&self) -> i32 {
		unsafe { sys::cv_dnn_BackendWrapper_backendId_const(self.as_raw_BackendWrapper()) }.into_result().expect("Infallible function failed: backend_id")
	}
	
	/// Backend identifier.
	fn set_backend_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_BackendWrapper_setBackendId_int(self.as_raw_BackendWrapper(), val) }.into_result().expect("Infallible function failed: set_backend_id")
	}
	
	/// Target identifier.
	fn target_id(&self) -> i32 {
		unsafe { sys::cv_dnn_BackendWrapper_targetId_const(self.as_raw_BackendWrapper()) }.into_result().expect("Infallible function failed: target_id")
	}
	
	/// Target identifier.
	fn set_target_id(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_BackendWrapper_setTargetId_int(self.as_raw_BackendWrapper(), val) }.into_result().expect("Infallible function failed: set_target_id")
	}
	
	/// Transfer data to CPU host memory.
	fn copy_to_host(&mut self) -> Result<()> {
		unsafe { sys::cv_dnn_BackendWrapper_copyToHost(self.as_raw_BackendWrapper()) }.into_result()
	}
	
	/// Indicate that an actual data is on CPU.
	fn set_host_dirty(&mut self) -> Result<()> {
		unsafe { sys::cv_dnn_BackendWrapper_setHostDirty(self.as_raw_BackendWrapper()) }.into_result()
	}
	
}

pub trait BaseConvolutionLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void;
	fn kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_kernel_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: kernel")
	}
	
	fn set_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setKernel_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_kernel")
	}
	
	fn stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_stride_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: stride")
	}
	
	fn set_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setStride_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_stride")
	}
	
	fn pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_pad_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: pad")
	}
	
	fn set_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPad_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_pad")
	}
	
	fn dilation(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_dilation_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: dilation")
	}
	
	fn set_dilation(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setDilation_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_dilation")
	}
	
	fn adjust_pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_adjustPad_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: adjust_pad")
	}
	
	fn set_adjust_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setAdjustPad_Size(self.as_raw_BaseConvolutionLayer(), &val) }.into_result().expect("Infallible function failed: set_adjust_pad")
	}
	
	fn adjust_pads(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_adjust_pads(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: adjust_pads")
	}
	
	fn set_adjust_pads(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setAdjust_pads_vector_size_t_(self.as_raw_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_adjust_pads")
	}
	
	fn kernel_size(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_kernel_size(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: kernel_size")
	}
	
	fn set_kernel_size(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setKernel_size_vector_size_t_(self.as_raw_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_kernel_size")
	}
	
	fn strides(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_strides(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: strides")
	}
	
	fn set_strides(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setStrides_vector_size_t_(self.as_raw_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_strides")
	}
	
	fn dilations(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_dilations(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: dilations")
	}
	
	fn set_dilations(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setDilations_vector_size_t_(self.as_raw_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_dilations")
	}
	
	fn pads_begin(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_pads_begin(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: pads_begin")
	}
	
	fn set_pads_begin(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPads_begin_vector_size_t_(self.as_raw_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_pads_begin")
	}
	
	fn pads_end(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_pads_end(self.as_raw_BaseConvolutionLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: pads_end")
	}
	
	fn set_pads_end(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPads_end_vector_size_t_(self.as_raw_BaseConvolutionLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_pads_end")
	}
	
	fn pad_mode(&self) -> String {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_padMode_const(self.as_raw_BaseConvolutionLayer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: pad_mode")
	}
	
	fn set_pad_mode(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setPadMode_String(self.as_raw_BaseConvolutionLayer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_pad_mode")
	}
	
	fn num_output(&self) -> i32 {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_numOutput_const(self.as_raw_BaseConvolutionLayer()) }.into_result().expect("Infallible function failed: num_output")
	}
	
	fn set_num_output(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_BaseConvolutionLayer_setNumOutput_int(self.as_raw_BaseConvolutionLayer(), val) }.into_result().expect("Infallible function failed: set_num_output")
	}
	
}

pub struct BaseConvolutionLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for BaseConvolutionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_BaseConvolutionLayer_delete(instance: *mut c_void); }
		unsafe { cv_BaseConvolutionLayer_delete(self.as_raw_BaseConvolutionLayer()) };
	}
}

impl BaseConvolutionLayer {
	pub fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for BaseConvolutionLayer {}

impl core::AlgorithmTrait for BaseConvolutionLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BaseConvolutionLayerTrait for BaseConvolutionLayer {
	fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for BaseConvolutionLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl BaseConvolutionLayer {
}

pub trait BatchNormLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_BatchNormLayer(&self) -> *mut c_void;
	fn has_weights(&self) -> bool {
		unsafe { sys::cv_dnn_BatchNormLayer_hasWeights_const(self.as_raw_BatchNormLayer()) }.into_result().expect("Infallible function failed: has_weights")
	}
	
	fn set_has_weights(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_BatchNormLayer_setHasWeights_bool(self.as_raw_BatchNormLayer(), val) }.into_result().expect("Infallible function failed: set_has_weights")
	}
	
	fn has_bias(&self) -> bool {
		unsafe { sys::cv_dnn_BatchNormLayer_hasBias_const(self.as_raw_BatchNormLayer()) }.into_result().expect("Infallible function failed: has_bias")
	}
	
	fn set_has_bias(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_BatchNormLayer_setHasBias_bool(self.as_raw_BatchNormLayer(), val) }.into_result().expect("Infallible function failed: set_has_bias")
	}
	
	fn epsilon(&self) -> f32 {
		unsafe { sys::cv_dnn_BatchNormLayer_epsilon_const(self.as_raw_BatchNormLayer()) }.into_result().expect("Infallible function failed: epsilon")
	}
	
	fn set_epsilon(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_BatchNormLayer_setEpsilon_float(self.as_raw_BatchNormLayer(), val) }.into_result().expect("Infallible function failed: set_epsilon")
	}
	
}

impl dyn BatchNormLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBatchNormLayer> {
		unsafe { sys::cv_dnn_BatchNormLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBatchNormLayer { ptr })
	}
	
}
/// # Partial List of Implemented Layers
/// This subsection of dnn module contains information about built-in layers and their descriptions.
/// 
/// Classes listed here, in fact, provides C++ API for creating instances of built-in layers.
/// In addition to this way of layers instantiation, there is a more common factory API (see @ref dnnLayerFactory), it allows to create layers dynamically (by name) and register new ones.
/// You can use both API, but factory API is less convenient for native C++ programming and basically designed for use inside importers (see @ref readNetFromCaffe(), @ref readNetFromTorch(), @ref readNetFromTensorflow()).
/// 
/// Built-in layers partially reproduce functionality of corresponding Caffe and Torch7 layers.
/// In particular, the following layers and Caffe importer were tested to reproduce <a href="http://caffe.berkeleyvision.org/tutorial/layers.html">Caffe</a> functionality:
/// - Convolution
/// - Deconvolution
/// - Pooling
/// - InnerProduct
/// - TanH, ReLU, Sigmoid, BNLL, Power, AbsVal
/// - Softmax
/// - Reshape, Flatten, Slice, Split
/// - LRN
/// - MVN
/// - Dropout (since it does nothing on forward pass -))
pub trait BlankLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_BlankLayer(&self) -> *mut c_void;
}

/// # Partial List of Implemented Layers
/// This subsection of dnn module contains information about built-in layers and their descriptions.
/// 
/// Classes listed here, in fact, provides C++ API for creating instances of built-in layers.
/// In addition to this way of layers instantiation, there is a more common factory API (see @ref dnnLayerFactory), it allows to create layers dynamically (by name) and register new ones.
/// You can use both API, but factory API is less convenient for native C++ programming and basically designed for use inside importers (see @ref readNetFromCaffe(), @ref readNetFromTorch(), @ref readNetFromTensorflow()).
/// 
/// Built-in layers partially reproduce functionality of corresponding Caffe and Torch7 layers.
/// In particular, the following layers and Caffe importer were tested to reproduce <a href="http://caffe.berkeleyvision.org/tutorial/layers.html">Caffe</a> functionality:
/// - Convolution
/// - Deconvolution
/// - Pooling
/// - InnerProduct
/// - TanH, ReLU, Sigmoid, BNLL, Power, AbsVal
/// - Softmax
/// - Reshape, Flatten, Slice, Split
/// - LRN
/// - MVN
/// - Dropout (since it does nothing on forward pass -))
pub struct BlankLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for BlankLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_BlankLayer_delete(instance: *mut c_void); }
		unsafe { cv_BlankLayer_delete(self.as_raw_BlankLayer()) };
	}
}

impl BlankLayer {
	pub fn as_raw_BlankLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for BlankLayer {}

impl core::AlgorithmTrait for BlankLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BlankLayerTrait for BlankLayer {
	fn as_raw_BlankLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for BlankLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl BlankLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_BlankLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

pub trait ChannelsPReLULayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_ChannelsPReLULayer(&self) -> *mut c_void;
}

impl dyn ChannelsPReLULayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_ChannelsPReLULayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}
pub trait ConcatLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ConcatLayer(&self) -> *mut c_void;
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_ConcatLayer_axis_const(self.as_raw_ConcatLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_ConcatLayer_setAxis_int(self.as_raw_ConcatLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
	/// Add zero padding in case of concatenation of blobs with different
	/// spatial sizes.
	/// 
	/// Details: https://github.com/torch/nn/blob/master/doc/containers.md#depthconcat
	fn padding(&self) -> bool {
		unsafe { sys::cv_dnn_ConcatLayer_padding_const(self.as_raw_ConcatLayer()) }.into_result().expect("Infallible function failed: padding")
	}
	
	/// Add zero padding in case of concatenation of blobs with different
	/// spatial sizes.
	/// 
	/// Details: https://github.com/torch/nn/blob/master/doc/containers.md#depthconcat
	fn set_padding(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_ConcatLayer_setPadding_bool(self.as_raw_ConcatLayer(), val) }.into_result().expect("Infallible function failed: set_padding")
	}
	
}

pub struct ConcatLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ConcatLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ConcatLayer_delete(instance: *mut c_void); }
		unsafe { cv_ConcatLayer_delete(self.as_raw_ConcatLayer()) };
	}
}

impl ConcatLayer {
	pub fn as_raw_ConcatLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ConcatLayer {}

impl core::AlgorithmTrait for ConcatLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ConcatLayerTrait for ConcatLayer {
	fn as_raw_ConcatLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ConcatLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConcatLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfConcatLayer> {
		unsafe { sys::cv_dnn_ConcatLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfConcatLayer { ptr })
	}
	
}

/// Constant layer produces the same data blob at an every forward pass.
pub trait ConstLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ConstLayer(&self) -> *mut c_void;
}

/// Constant layer produces the same data blob at an every forward pass.
pub struct ConstLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ConstLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ConstLayer_delete(instance: *mut c_void); }
		unsafe { cv_ConstLayer_delete(self.as_raw_ConstLayer()) };
	}
}

impl ConstLayer {
	pub fn as_raw_ConstLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ConstLayer {}

impl core::AlgorithmTrait for ConstLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ConstLayerTrait for ConstLayer {
	fn as_raw_ConstLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ConstLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConstLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_ConstLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

pub trait ConvolutionLayerTrait: core::AlgorithmTrait + crate::dnn::BaseConvolutionLayerTrait + crate::dnn::LayerTrait {
	fn as_raw_ConvolutionLayer(&self) -> *mut c_void;
}

pub struct ConvolutionLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ConvolutionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ConvolutionLayer_delete(instance: *mut c_void); }
		unsafe { cv_ConvolutionLayer_delete(self.as_raw_ConvolutionLayer()) };
	}
}

impl ConvolutionLayer {
	pub fn as_raw_ConvolutionLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ConvolutionLayer {}

impl core::AlgorithmTrait for ConvolutionLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BaseConvolutionLayerTrait for ConvolutionLayer {
	fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ConvolutionLayerTrait for ConvolutionLayer {
	fn as_raw_ConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ConvolutionLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl ConvolutionLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBaseConvolutionLayer> {
		unsafe { sys::cv_dnn_ConvolutionLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
	}
	
}

pub trait CropAndResizeLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_CropAndResizeLayer(&self) -> *mut c_void;
}

pub struct CropAndResizeLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for CropAndResizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CropAndResizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_CropAndResizeLayer_delete(self.as_raw_CropAndResizeLayer()) };
	}
}

impl CropAndResizeLayer {
	pub fn as_raw_CropAndResizeLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for CropAndResizeLayer {}

impl core::AlgorithmTrait for CropAndResizeLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::CropAndResizeLayerTrait for CropAndResizeLayer {
	fn as_raw_CropAndResizeLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for CropAndResizeLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl CropAndResizeLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_CropAndResizeLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

pub trait CropLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_CropLayer(&self) -> *mut c_void;
}

pub struct CropLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for CropLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_CropLayer_delete(instance: *mut c_void); }
		unsafe { cv_CropLayer_delete(self.as_raw_CropLayer()) };
	}
}

impl CropLayer {
	pub fn as_raw_CropLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for CropLayer {}

impl core::AlgorithmTrait for CropLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::CropLayerTrait for CropLayer {
	fn as_raw_CropLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for CropLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl CropLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_CropLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

pub trait DeconvolutionLayerTrait: core::AlgorithmTrait + crate::dnn::BaseConvolutionLayerTrait + crate::dnn::LayerTrait {
	fn as_raw_DeconvolutionLayer(&self) -> *mut c_void;
}

pub struct DeconvolutionLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for DeconvolutionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_DeconvolutionLayer_delete(instance: *mut c_void); }
		unsafe { cv_DeconvolutionLayer_delete(self.as_raw_DeconvolutionLayer()) };
	}
}

impl DeconvolutionLayer {
	pub fn as_raw_DeconvolutionLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DeconvolutionLayer {}

impl core::AlgorithmTrait for DeconvolutionLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::BaseConvolutionLayerTrait for DeconvolutionLayer {
	fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::DeconvolutionLayerTrait for DeconvolutionLayer {
	fn as_raw_DeconvolutionLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for DeconvolutionLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl DeconvolutionLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfBaseConvolutionLayer> {
		unsafe { sys::cv_dnn_DeconvolutionLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfBaseConvolutionLayer { ptr })
	}
	
}

pub trait DetectionOutputLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_DetectionOutputLayer(&self) -> *mut c_void;
}

pub struct DetectionOutputLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for DetectionOutputLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_DetectionOutputLayer_delete(instance: *mut c_void); }
		unsafe { cv_DetectionOutputLayer_delete(self.as_raw_DetectionOutputLayer()) };
	}
}

impl DetectionOutputLayer {
	pub fn as_raw_DetectionOutputLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DetectionOutputLayer {}

impl core::AlgorithmTrait for DetectionOutputLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::DetectionOutputLayerTrait for DetectionOutputLayer {
	fn as_raw_DetectionOutputLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for DetectionOutputLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl DetectionOutputLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfDetectionOutputLayer> {
		unsafe { sys::cv_dnn_DetectionOutputLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfDetectionOutputLayer { ptr })
	}
	
}

/// This class implements name-value dictionary, values are instances of DictValue.
pub trait DictTrait {
	fn as_raw_Dict(&self) -> *mut c_void;
	/// Checks a presence of the @p key in the dictionary.
	fn has(&self, key: &str) -> Result<bool> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_has_const_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result()
	}
	
	/// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
	fn ptr(&mut self, key: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_ptr_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// If the @p key in the dictionary then returns pointer to its value, else returns NULL.
	/// 
	/// ## Overloaded parameters
	fn ptr_1(&self, key: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_ptr_const_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// If the @p key in the dictionary then returns its value, else an error will be generated.
	fn get(&self, key: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_get_const_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_str(&mut self, key: &str, value: &str) -> Result<String> {
		string_arg!(key);
		string_arg!(value);
		unsafe { sys::cv_dnn_Dict_set_cv_String_const_StringX_const_StringX(self.as_raw_Dict(), key.as_ptr(), value.as_ptr()) }.into_result().map(crate::templ::receive_string)
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set(&mut self, key: &str, value: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_cv_dnn_DictValue_const_StringX_const_DictValueX(self.as_raw_Dict(), key.as_ptr(), value.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_f64(&mut self, key: &str, value: &f64) -> Result<f64> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_double_const_StringX_const_doubleX(self.as_raw_Dict(), key.as_ptr(), value) }.into_result()
	}
	
	/// Sets new @p value for the @p key, or adds new key-value pair into the dictionary.
	fn set_i64(&mut self, key: &str, value: &i64) -> Result<i64> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_set_int64_t_const_StringX_const_int64_tX(self.as_raw_Dict(), key.as_ptr(), value) }.into_result()
	}
	
	/// Erase @p key from the dictionary.
	fn erase(&mut self, key: &str) -> Result<()> {
		string_arg!(key);
		unsafe { sys::cv_dnn_Dict_erase_const_StringX(self.as_raw_Dict(), key.as_ptr()) }.into_result()
	}
	
}

/// This class implements name-value dictionary, values are instances of DictValue.
pub struct Dict {
	pub(crate) ptr: *mut c_void
}

impl Drop for Dict {
	fn drop(&mut self) {
		extern "C" { fn cv_Dict_delete(instance: *mut c_void); }
		unsafe { cv_Dict_delete(self.as_raw_Dict()) };
	}
}

impl Dict {
	pub fn as_raw_Dict(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Dict {}

impl crate::dnn::DictTrait for Dict {
	fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

impl Dict {
}

/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub trait DictValueTrait {
	fn as_raw_DictValue(&self) -> *mut c_void;
	/// ## C++ default parameters
	/// * idx: -1
	fn get_str(&self, idx: i32) -> Result<String> {
		unsafe { sys::cv_dnn_DictValue_get_cv_String_const_int(self.as_raw_DictValue(), idx) }.into_result().map(crate::templ::receive_string)
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_f64(&self, idx: i32) -> Result<f64> {
		unsafe { sys::cv_dnn_DictValue_get_double_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_i32(&self, idx: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_DictValue_get_int_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_i64(&self, idx: i32) -> Result<i64> {
		unsafe { sys::cv_dnn_DictValue_get_int64_t_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	fn size(&self) -> Result<i32> {
		unsafe { sys::cv_dnn_DictValue_size_const(self.as_raw_DictValue()) }.into_result()
	}
	
	fn is_int(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_DictValue_isInt_const(self.as_raw_DictValue()) }.into_result()
	}
	
	fn is_string(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_DictValue_isString_const(self.as_raw_DictValue()) }.into_result()
	}
	
	fn is_real(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_DictValue_isReal_const(self.as_raw_DictValue()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_int_value(&self, idx: i32) -> Result<i32> {
		unsafe { sys::cv_dnn_DictValue_getIntValue_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_real_value(&self, idx: i32) -> Result<f64> {
		unsafe { sys::cv_dnn_DictValue_getRealValue_const_int(self.as_raw_DictValue(), idx) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * idx: -1
	fn get_string_value(&self, idx: i32) -> Result<String> {
		unsafe { sys::cv_dnn_DictValue_getStringValue_const_int(self.as_raw_DictValue(), idx) }.into_result().map(crate::templ::receive_string)
	}
	
}

/// This struct stores the scalar value (or array) of one of the following type: double, cv::String or int64.
/// @todo Maybe int64 is useless because double type exactly stores at least 2^52 integers.
pub struct DictValue {
	pub(crate) ptr: *mut c_void
}

impl Drop for DictValue {
	fn drop(&mut self) {
		extern "C" { fn cv_DictValue_delete(instance: *mut c_void); }
		unsafe { cv_DictValue_delete(self.as_raw_DictValue()) };
	}
}

impl DictValue {
	pub fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for DictValue {}

impl crate::dnn::DictValueTrait for DictValue {
	fn as_raw_DictValue(&self) -> *mut c_void { self.ptr }
}

impl DictValue {
	pub fn copy(r: &crate::dnn::DictValue) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_const_DictValueX(r.as_raw_DictValue()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_bool(i: bool) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_bool(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	/// ## C++ default parameters
	/// * i: 0
	pub fn from_i64(i: i64) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_int64_t(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_i32(i: i32) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_int(i) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_u32(p: u32) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_unsigned_int(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_f64(p: f64) -> Result<crate::dnn::DictValue> {
		unsafe { sys::cv_dnn_DictValue_DictValue_double(p) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
	pub fn from_str(s: &str) -> Result<crate::dnn::DictValue> {
		string_arg!(s);
		unsafe { sys::cv_dnn_DictValue_DictValue_const_charX(s.as_ptr()) }.into_result().map(|ptr| crate::dnn::DictValue { ptr })
	}
	
}

pub trait ELULayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_ELULayer(&self) -> *mut c_void;
}

impl dyn ELULayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfELULayer> {
		unsafe { sys::cv_dnn_ELULayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfELULayer { ptr })
	}
	
}
/// Element wise operation on inputs
/// 
/// Extra optional parameters:
/// - "operation" as string. Values are "sum" (default), "prod", "max", "div"
/// - "coeff" as float array. Specify weights of inputs for SUM operation
/// - "output_channels_mode" as string. Values are "same" (default, all input must have the same layout), "input_0", "input_0_truncate", "max_input_channels"
pub trait EltwiseLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_EltwiseLayer(&self) -> *mut c_void;
}

/// Element wise operation on inputs
/// 
/// Extra optional parameters:
/// - "operation" as string. Values are "sum" (default), "prod", "max", "div"
/// - "coeff" as float array. Specify weights of inputs for SUM operation
/// - "output_channels_mode" as string. Values are "same" (default, all input must have the same layout), "input_0", "input_0_truncate", "max_input_channels"
pub struct EltwiseLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for EltwiseLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_EltwiseLayer_delete(instance: *mut c_void); }
		unsafe { cv_EltwiseLayer_delete(self.as_raw_EltwiseLayer()) };
	}
}

impl EltwiseLayer {
	pub fn as_raw_EltwiseLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for EltwiseLayer {}

impl core::AlgorithmTrait for EltwiseLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::EltwiseLayerTrait for EltwiseLayer {
	fn as_raw_EltwiseLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for EltwiseLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl EltwiseLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfEltwiseLayer> {
		unsafe { sys::cv_dnn_EltwiseLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfEltwiseLayer { ptr })
	}
	
}

pub trait FlattenLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_FlattenLayer(&self) -> *mut c_void;
}

pub struct FlattenLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for FlattenLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_FlattenLayer_delete(instance: *mut c_void); }
		unsafe { cv_FlattenLayer_delete(self.as_raw_FlattenLayer()) };
	}
}

impl FlattenLayer {
	pub fn as_raw_FlattenLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for FlattenLayer {}

impl core::AlgorithmTrait for FlattenLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::FlattenLayerTrait for FlattenLayer {
	fn as_raw_FlattenLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for FlattenLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl FlattenLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfFlattenLayer> {
		unsafe { sys::cv_dnn_FlattenLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfFlattenLayer { ptr })
	}
	
}

pub trait InnerProductLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_InnerProductLayer(&self) -> *mut c_void;
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_InnerProductLayer_axis_const(self.as_raw_InnerProductLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_InnerProductLayer_setAxis_int(self.as_raw_InnerProductLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
}

pub struct InnerProductLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for InnerProductLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_InnerProductLayer_delete(instance: *mut c_void); }
		unsafe { cv_InnerProductLayer_delete(self.as_raw_InnerProductLayer()) };
	}
}

impl InnerProductLayer {
	pub fn as_raw_InnerProductLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for InnerProductLayer {}

impl core::AlgorithmTrait for InnerProductLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::InnerProductLayerTrait for InnerProductLayer {
	fn as_raw_InnerProductLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for InnerProductLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl InnerProductLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfInnerProductLayer> {
		unsafe { sys::cv_dnn_InnerProductLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfInnerProductLayer { ptr })
	}
	
}

/// Bilinear resize layer from https://github.com/cdmh/deeplab-public-ver2
/// 
/// It differs from @ref ResizeLayer in output shape and resize scales computations.
pub trait InterpLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_InterpLayer(&self) -> *mut c_void;
}

/// Bilinear resize layer from https://github.com/cdmh/deeplab-public-ver2
/// 
/// It differs from @ref ResizeLayer in output shape and resize scales computations.
pub struct InterpLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for InterpLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_InterpLayer_delete(instance: *mut c_void); }
		unsafe { cv_InterpLayer_delete(self.as_raw_InterpLayer()) };
	}
}

impl InterpLayer {
	pub fn as_raw_InterpLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for InterpLayer {}

impl core::AlgorithmTrait for InterpLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::InterpLayerTrait for InterpLayer {
	fn as_raw_InterpLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for InterpLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl InterpLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_InterpLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

pub trait LRNLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_LRNLayer(&self) -> *mut c_void;
	fn typ(&self) -> i32 {
		unsafe { sys::cv_dnn_LRNLayer_type_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setType_int(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn size(&self) -> i32 {
		unsafe { sys::cv_dnn_LRNLayer_size_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: size")
	}
	
	fn set_size(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setSize_int(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_size")
	}
	
	fn alpha(&self) -> f32 {
		unsafe { sys::cv_dnn_LRNLayer_alpha_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: alpha")
	}
	
	fn set_alpha(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setAlpha_float(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_alpha")
	}
	
	fn beta(&self) -> f32 {
		unsafe { sys::cv_dnn_LRNLayer_beta_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: beta")
	}
	
	fn set_beta(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setBeta_float(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_beta")
	}
	
	fn bias(&self) -> f32 {
		unsafe { sys::cv_dnn_LRNLayer_bias_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: bias")
	}
	
	fn set_bias(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setBias_float(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_bias")
	}
	
	fn norm_by_size(&self) -> bool {
		unsafe { sys::cv_dnn_LRNLayer_normBySize_const(self.as_raw_LRNLayer()) }.into_result().expect("Infallible function failed: norm_by_size")
	}
	
	fn set_norm_by_size(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_LRNLayer_setNormBySize_bool(self.as_raw_LRNLayer(), val) }.into_result().expect("Infallible function failed: set_norm_by_size")
	}
	
}

pub struct LRNLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for LRNLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_LRNLayer_delete(instance: *mut c_void); }
		unsafe { cv_LRNLayer_delete(self.as_raw_LRNLayer()) };
	}
}

impl LRNLayer {
	pub fn as_raw_LRNLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for LRNLayer {}

impl core::AlgorithmTrait for LRNLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LRNLayerTrait for LRNLayer {
	fn as_raw_LRNLayer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for LRNLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl LRNLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLRNLayer> {
		unsafe { sys::cv_dnn_LRNLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLRNLayer { ptr })
	}
	
}

/// LSTM recurrent layer
pub trait LSTMLayer: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_LSTMLayer(&self) -> *mut c_void;
	/// 
	/// **Deprecated**: Use LayerParams::blobs instead.
	/// Set trained weights for LSTM layer.
	/// 
	/// LSTM behavior on each step is defined by current input, previous output, previous cell state and learned weights.
	/// 
	/// Let @f$x_t@f$ be current input, @f$h_t@f$ be current output, @f$c_t@f$ be current state.
	/// Than current output and current cell state is computed as follows:
	/// @f{eqnarray*}{
	/// h_t &= o_t \odot tanh(c_t),               \\
	/// c_t &= f_t \odot c_{t-1} + i_t \odot g_t, \\
	/// @f}
	/// where @f$\odot@f$ is per-element multiply operation and @f$i_t, f_t, o_t, g_t@f$ is internal gates that are computed using learned weights.
	/// 
	/// Gates are computed as follows:
	/// @f{eqnarray*}{
	/// i_t &= sigmoid&(W_{xi} x_t + W_{hi} h_{t-1} + b_i), \\
	/// f_t &= sigmoid&(W_{xf} x_t + W_{hf} h_{t-1} + b_f), \\
	/// o_t &= sigmoid&(W_{xo} x_t + W_{ho} h_{t-1} + b_o), \\
	/// g_t &= tanh   &(W_{xg} x_t + W_{hg} h_{t-1} + b_g), \\
	/// @f}
	/// where @f$W_{x?}@f$, @f$W_{h?}@f$ and @f$b_{?}@f$ are learned weights represented as matrices:
	/// @f$W_{x?} \in R^{N_h \times N_x}@f$, @f$W_{h?} \in R^{N_h \times N_h}@f$, @f$b_? \in R^{N_h}@f$.
	/// 
	/// For simplicity and performance purposes we use @f$ W_x = [W_{xi}; W_{xf}; W_{xo}, W_{xg}] @f$
	/// (i.e. @f$W_x@f$ is vertical concatenation of @f$ W_{x?} @f$), @f$ W_x \in R^{4N_h \times N_x} @f$.
	/// The same for @f$ W_h = [W_{hi}; W_{hf}; W_{ho}, W_{hg}], W_h \in R^{4N_h \times N_h} @f$
	/// and for @f$ b = [b_i; b_f, b_o, b_g]@f$, @f$b \in R^{4N_h} @f$.
	/// 
	/// ## Parameters
	/// * Wh: is matrix defining how previous output is transformed to internal gates (i.e. according to above mentioned notation is @f$ W_h @f$)
	/// * Wx: is matrix defining how current input is transformed to internal gates (i.e. according to above mentioned notation is @f$ W_x @f$)
	/// * b: is bias vector (i.e. according to above mentioned notation is @f$ b @f$)
	#[deprecated = "Use LayerParams::blobs instead."]
	fn set_weights(&mut self, wh: &core::Mat, wx: &core::Mat, b: &core::Mat) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setWeights_const_MatX_const_MatX_const_MatX(self.as_raw_LSTMLayer(), wh.as_raw_Mat(), wx.as_raw_Mat(), b.as_raw_Mat()) }.into_result()
	}
	
	/// Specifies shape of output blob which will be [[`T`], `N`] + @p outTailShape.
	/// @details If this parameter is empty or unset then @p outTailShape = [`Wh`.size(0)] will be used,
	/// where `Wh` is parameter from setWeights().
	/// 
	/// ## C++ default parameters
	/// * out_tail_shape: MatShape()
	fn set_out_shape(&mut self, out_tail_shape: &crate::dnn::MatShape) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setOutShape_const_MatShapeX(self.as_raw_LSTMLayer(), out_tail_shape.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// 
	/// **Deprecated**: Use flag `produce_cell_output` in LayerParams.
	/// Specifies either interpret first dimension of input blob as timestamp dimenion either as sample.
	/// 
	/// If flag is set to true then shape of input blob will be interpreted as [`T`, `N`, `[data dims]`] where `T` specifies number of timestamps, `N` is number of independent streams.
	/// In this case each forward() call will iterate through `T` timestamps and update layer's state `T` times.
	/// 
	/// If flag is set to false then shape of input blob will be interpreted as [`N`, `[data dims]`].
	/// In this case each forward() call will make one iteration and produce one timestamp with shape [`N`, `[out dims]`].
	/// 
	/// ## C++ default parameters
	/// * use_: true
	#[deprecated = "Use flag `produce_cell_output` in LayerParams."]
	fn set_use_timstamps_dim(&mut self, use_: bool) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setUseTimstampsDim_bool(self.as_raw_LSTMLayer(), use_) }.into_result()
	}
	
	/// 
	/// **Deprecated**: Use flag `use_timestamp_dim` in LayerParams.
	/// If this flag is set to true then layer will produce @f$ c_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	/// 
	/// ## C++ default parameters
	/// * produce: false
	#[deprecated = "Use flag `use_timestamp_dim` in LayerParams."]
	fn set_produce_cell_output(&mut self, produce: bool) -> Result<()> {
		unsafe { sys::cv_dnn_LSTMLayer_setProduceCellOutput_bool(self.as_raw_LSTMLayer(), produce) }.into_result()
	}
	
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		string_arg!(input_name);
		unsafe { sys::cv_dnn_LSTMLayer_inputNameToIndex_String(self.as_raw_LSTMLayer(), input_name.as_ptr() as _) }.into_result()
	}
	
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_LSTMLayer_outputNameToIndex_const_StringX(self.as_raw_LSTMLayer(), output_name.as_ptr()) }.into_result()
	}
	
}

impl dyn LSTMLayer + '_ {
	/// Creates instance of LSTM layer
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLSTMLayer> {
		unsafe { sys::cv_dnn_LSTMLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLSTMLayer { ptr })
	}
	
}
/// This interface class allows to build new Layers - are building blocks of networks.
/// 
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub trait LayerTrait: core::AlgorithmTrait {
	fn as_raw_Layer(&self) -> *mut c_void;
	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	fn blobs(&mut self) -> types::VectorOfMat {
		unsafe { sys::cv_dnn_Layer_blobs(self.as_raw_Layer()) }.into_result().map(|ptr| types::VectorOfMat { ptr }).expect("Infallible function failed: blobs")
	}
	
	/// List of learned parameters must be stored here to allow read them by using Net::getParam().
	fn set_blobs(&mut self, val: types::VectorOfMat) -> () {
		unsafe { sys::cv_dnn_Layer_setBlobs_vector_Mat_(self.as_raw_Layer(), val.as_raw_VectorOfMat()) }.into_result().expect("Infallible function failed: set_blobs")
	}
	
	/// Name of the layer instance, can be used for logging or other internal purposes.
	fn name(&self) -> String {
		unsafe { sys::cv_dnn_Layer_name_const(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: name")
	}
	
	/// Name of the layer instance, can be used for logging or other internal purposes.
	fn set_name(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_Layer_setName_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_name")
	}
	
	/// Type name which was used for creating layer by layer factory.
	fn typ(&self) -> String {
		unsafe { sys::cv_dnn_Layer_type_const(self.as_raw_Layer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: typ")
	}
	
	/// Type name which was used for creating layer by layer factory.
	fn set_type(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_Layer_setType_String(self.as_raw_Layer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_type")
	}
	
	/// prefer target for layer forwarding
	fn preferable_target(&self) -> i32 {
		unsafe { sys::cv_dnn_Layer_preferableTarget_const(self.as_raw_Layer()) }.into_result().expect("Infallible function failed: preferable_target")
	}
	
	/// prefer target for layer forwarding
	fn set_preferable_target(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_Layer_setPreferableTarget_int(self.as_raw_Layer(), val) }.into_result().expect("Infallible function failed: set_preferable_target")
	}
	
	/// Computes and sets internal parameters according to inputs, outputs and blobs.
	/// ## Parameters
	/// * inputs: vector of already allocated input blobs
	/// * outputs:[out] vector of already allocated output blobs
	/// 
	/// If this method is called after network has allocated all memory for input and output blobs
	/// and before inferencing.
	fn finalize(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		unsafe { sys::cv_dnn_Layer_finalize_const__InputArrayX_const__OutputArrayX(self.as_raw_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray()) }.into_result()
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// 
	/// **Deprecated**: Use Layer::forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays) instead
	/// ## Parameters
	/// * input: the input blobs.
	/// * output:[out] allocated output blobs, which will store results of the computation.
	/// * internals:[out] allocated internal blobs
	#[deprecated = "Use Layer::forward(InputArrayOfArrays, OutputArrayOfArrays, OutputArrayOfArrays) instead"]
	fn forward_mat(&mut self, input: &mut types::VectorOfMat, output: &mut types::VectorOfMat, internals: &mut types::VectorOfMat) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_forward_vector_MatX_X_vector_Mat_X_vector_Mat_X(self.as_raw_Layer(), input.as_raw_VectorOfMat(), output.as_raw_VectorOfMat(), internals.as_raw_VectorOfMat()) }.into_result()
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * inputs: the input blobs.
	/// * outputs:[out] allocated output blobs, which will store results of the computation.
	/// * internals:[out] allocated internal blobs
	fn forward(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, internals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		output_array_arg!(internals);
		unsafe { sys::cv_dnn_Layer_forward_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(self.as_raw_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray()) }.into_result()
	}
	
	/// Given the @p input blobs, computes the output @p blobs.
	/// ## Parameters
	/// * inputs: the input blobs.
	/// * outputs:[out] allocated output blobs, which will store results of the computation.
	/// * internals:[out] allocated internal blobs
	fn forward_fallback(&mut self, inputs: &dyn core::ToInputArray, outputs: &mut dyn core::ToOutputArray, internals: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(inputs);
		output_array_arg!(outputs);
		output_array_arg!(internals);
		unsafe { sys::cv_dnn_Layer_forward_fallback_const__InputArrayX_const__OutputArrayX_const__OutputArrayX(self.as_raw_Layer(), inputs.as_raw__InputArray(), outputs.as_raw__OutputArray(), internals.as_raw__OutputArray()) }.into_result()
	}
	
	/// @brief
	///  Computes and sets internal parameters according to inputs, outputs and blobs.
	/// ## Parameters
	/// * inputs: vector of already allocated input blobs
	/// * outputs:[out] vector of already allocated output blobs
	/// 
	/// If this method is called after network has allocated all memory for input and output blobs
	/// and before inferencing.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// **Deprecated**: Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead
	#[deprecated = "Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead"]
	fn finalize_mat_to(&mut self, inputs: &types::VectorOfMat, outputs: &mut types::VectorOfMat) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_finalize_const_vector_Mat_X_vector_Mat_X(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat()) }.into_result()
	}
	
	/// @brief
	///  Computes and sets internal parameters according to inputs, outputs and blobs.
	/// ## Parameters
	/// * inputs: vector of already allocated input blobs
	/// * outputs:[out] vector of already allocated output blobs
	/// 
	/// If this method is called after network has allocated all memory for input and output blobs
	/// and before inferencing.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// **Deprecated**: Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead
	#[deprecated = "Use Layer::finalize(InputArrayOfArrays, OutputArrayOfArrays) instead"]
	fn finalize_mat(&mut self, inputs: &types::VectorOfMat) -> Result<types::VectorOfMat> {
		unsafe { sys::cv_dnn_Layer_finalize_const_vector_Mat_X(self.as_raw_Layer(), inputs.as_raw_VectorOfMat()) }.into_result().map(|ptr| types::VectorOfMat { ptr })
	}
	
	/// Allocates layer and computes output.
	/// 
	/// **Deprecated**: This method will be removed in the future release.
	#[deprecated = "This method will be removed in the future release."]
	fn run(&mut self, inputs: &types::VectorOfMat, outputs: &mut types::VectorOfMat, internals: &mut types::VectorOfMat) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_run_const_vector_Mat_X_vector_Mat_X_vector_Mat_X(self.as_raw_Layer(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), internals.as_raw_VectorOfMat()) }.into_result()
	}
	
	/// Returns index of input blob into the input array.
	/// ## Parameters
	/// * inputName: label of input blob
	/// 
	/// Each layer input and output can be labeled to easily identify them using "%<layer_name%>[.output_name]" notation.
	/// This method maps label of input blob to its index into input vector.
	fn input_name_to_index(&mut self, input_name: &str) -> Result<i32> {
		string_arg!(input_name);
		unsafe { sys::cv_dnn_Layer_inputNameToIndex_String(self.as_raw_Layer(), input_name.as_ptr() as _) }.into_result()
	}
	
	/// Returns index of output blob in output array.
	/// ## See also
	/// inputNameToIndex()
	fn output_name_to_index(&mut self, output_name: &str) -> Result<i32> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Layer_outputNameToIndex_const_StringX(self.as_raw_Layer(), output_name.as_ptr()) }.into_result()
	}
	
	/// Ask layer if it support specific backend for doing computations.
	/// ## Parameters
	/// * backendId: computation backend identifier.
	/// ## See also
	/// Backend
	fn support_backend(&mut self, backend_id: i32) -> Result<bool> {
		unsafe { sys::cv_dnn_Layer_supportBackend_int(self.as_raw_Layer(), backend_id) }.into_result()
	}
	
	/// Returns Halide backend node.
	/// ## Parameters
	/// * inputs: Input Halide buffers.
	/// ## See also
	/// BackendNode, BackendWrapper
	/// 
	/// Input buffers should be exactly the same that will be used in forward invocations.
	/// Despite we can use Halide::ImageParam based on input shape only,
	/// it helps prevent some memory management issues (if something wrong,
	/// Halide tests will be failed).
	fn init_halide(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper) -> Result<types::PtrOfBackendNode> {
		unsafe { sys::cv_dnn_Layer_initHalide_const_vector_Ptr_BackendWrapper__X(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
	}
	
	fn init_inf_engine(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper) -> Result<types::PtrOfBackendNode> {
		unsafe { sys::cv_dnn_Layer_initInfEngine_const_vector_Ptr_BackendWrapper__X(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
	}
	
	fn init_ngraph(&mut self, inputs: &types::VectorOfPtrOfBackendWrapper, nodes: &types::VectorOfPtrOfBackendNode) -> Result<types::PtrOfBackendNode> {
		unsafe { sys::cv_dnn_Layer_initNgraph_const_vector_Ptr_BackendWrapper__X_const_vector_Ptr_BackendNode__X(self.as_raw_Layer(), inputs.as_raw_VectorOfPtrOfBackendWrapper(), nodes.as_raw_VectorOfPtrOfBackendNode()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
	}
	
	/// Automatic Halide scheduling based on layer hyper-parameters.
	/// ## Parameters
	/// * node: Backend node with Halide functions.
	/// * inputs: Blobs that will be used in forward invocations.
	/// * outputs: Blobs that will be used in forward invocations.
	/// * targetId: Target identifier
	/// ## See also
	/// BackendNode, Target
	/// 
	/// Layer don't use own Halide::Func members because we can have applied
	/// layers fusing. In this way the fused function should be scheduled.
	fn apply_halide_scheduler(&self, node: &mut types::PtrOfBackendNode, inputs: &types::VectorOfMat, outputs: &types::VectorOfMat, target_id: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_applyHalideScheduler_const_Ptr_BackendNode_X_const_vector_MatX_X_const_vector_Mat_X_int(self.as_raw_Layer(), node.as_raw_PtrOfBackendNode(), inputs.as_raw_VectorOfMat(), outputs.as_raw_VectorOfMat(), target_id) }.into_result()
	}
	
	/// Implement layers fusing.
	/// ## Parameters
	/// * node: Backend node of bottom layer.
	/// ## See also
	/// BackendNode
	/// 
	/// Actual for graph-based backends. If layer attached successfully,
	/// returns non-empty cv::Ptr to node of the same backend.
	/// Fuse only over the last function.
	fn try_attach(&mut self, node: &types::PtrOfBackendNode) -> Result<types::PtrOfBackendNode> {
		unsafe { sys::cv_dnn_Layer_tryAttach_const_Ptr_BackendNode_X(self.as_raw_Layer(), node.as_raw_PtrOfBackendNode()) }.into_result().map(|ptr| types::PtrOfBackendNode { ptr })
	}
	
	/// Tries to attach to the layer the subsequent activation layer, i.e. do the layer fusion in a partial case.
	/// ## Parameters
	/// * layer: The subsequent activation layer.
	/// 
	/// Returns true if the activation layer has been attached successfully.
	fn set_activation(&mut self, layer: &types::PtrOfActivationLayer) -> Result<bool> {
		unsafe { sys::cv_dnn_Layer_setActivation_const_Ptr_ActivationLayer_X(self.as_raw_Layer(), layer.as_raw_PtrOfActivationLayer()) }.into_result()
	}
	
	/// Try to fuse current layer with a next one
	/// ## Parameters
	/// * top: Next layer to be fused.
	/// ## Returns
	/// True if fusion was performed.
	fn try_fuse(&mut self, top: &mut types::PtrOfLayer) -> Result<bool> {
		unsafe { sys::cv_dnn_Layer_tryFuse_Ptr_Layer_X(self.as_raw_Layer(), top.as_raw_PtrOfLayer()) }.into_result()
	}
	
	/// Returns parameters of layers with channel-wise multiplication and addition.
	/// ## Parameters
	/// * scale:[out] Channel-wise multipliers. Total number of values should
	///                   be equal to number of channels.
	/// * shift:[out] Channel-wise offsets. Total number of values should
	///                   be equal to number of channels.
	/// 
	/// Some layers can fuse their transformations with further layers.
	/// In example, convolution + batch normalization. This way base layer
	/// use weights from layer after it. Fused layer is skipped.
	/// By default, @p scale and @p shift are empty that means layer has no
	/// element-wise multiplications or additions.
	fn get_scale_shift(&self, scale: &mut core::Mat, shift: &mut core::Mat) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_getScaleShift_const_MatX_MatX(self.as_raw_Layer(), scale.as_raw_Mat(), shift.as_raw_Mat()) }.into_result()
	}
	
	/// "Deattaches" all the layers, attached to particular layer.
	fn unset_attached(&mut self) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_unsetAttached(self.as_raw_Layer()) }.into_result()
	}
	
	fn get_memory_shapes(&self, inputs: &types::VectorOfMatShape, required_outputs: i32, outputs: &mut types::VectorOfMatShape, internals: &mut types::VectorOfMatShape) -> Result<bool> {
		unsafe { sys::cv_dnn_Layer_getMemoryShapes_const_const_vector_MatShape_X_int_vector_MatShape_X_vector_MatShape_X(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), required_outputs, outputs.as_raw_VectorOfMatShape(), internals.as_raw_VectorOfMatShape()) }.into_result()
	}
	
	fn get_flops(&self, inputs: &types::VectorOfMatShape, outputs: &types::VectorOfMatShape) -> Result<i64> {
		unsafe { sys::cv_dnn_Layer_getFLOPS_const_const_vector_MatShape_X_const_vector_MatShape_X(self.as_raw_Layer(), inputs.as_raw_VectorOfMatShape(), outputs.as_raw_VectorOfMatShape()) }.into_result()
	}
	
	fn set_params_from(&mut self, params: &crate::dnn::LayerParams) -> Result<()> {
		unsafe { sys::cv_dnn_Layer_setParamsFrom_const_LayerParamsX(self.as_raw_Layer(), params.as_raw_LayerParams()) }.into_result()
	}
	
}

/// This interface class allows to build new Layers - are building blocks of networks.
/// 
/// Each class, derived from Layer, must implement allocate() methods to declare own outputs and forward() to compute outputs.
/// Also before using the new layer into networks you must register your layer by using one of @ref dnnLayerFactory "LayerFactory" macros.
pub struct Layer {
	pub(crate) ptr: *mut c_void
}

impl Drop for Layer {
	fn drop(&mut self) {
		extern "C" { fn cv_Layer_delete(instance: *mut c_void); }
		unsafe { cv_Layer_delete(self.as_raw_Layer()) };
	}
}

impl Layer {
	pub fn as_raw_Layer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Layer {}

impl core::AlgorithmTrait for Layer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for Layer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl Layer {
	pub fn default() -> Result<crate::dnn::Layer> {
		unsafe { sys::cv_dnn_Layer_Layer() }.into_result().map(|ptr| crate::dnn::Layer { ptr })
	}
	
	pub fn new(params: &crate::dnn::LayerParams) -> Result<crate::dnn::Layer> {
		unsafe { sys::cv_dnn_Layer_Layer_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| crate::dnn::Layer { ptr })
	}
	
}

/// %Layer factory allows to create instances of registered layers.
pub trait LayerFactoryTrait {
	fn as_raw_LayerFactory(&self) -> *mut c_void;
}

/// %Layer factory allows to create instances of registered layers.
pub struct LayerFactory {
	pub(crate) ptr: *mut c_void
}

impl Drop for LayerFactory {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerFactory_delete(instance: *mut c_void); }
		unsafe { cv_LayerFactory_delete(self.as_raw_LayerFactory()) };
	}
}

impl LayerFactory {
	pub fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for LayerFactory {}

impl crate::dnn::LayerFactoryTrait for LayerFactory {
	fn as_raw_LayerFactory(&self) -> *mut c_void { self.ptr }
}

impl LayerFactory {
	/// Registers the layer class with typename @p type and specified @p constructor. Thread-safe.
	pub fn register_layer(typ: &str, constructor: crate::dnn::LayerFactory_Constructor) -> Result<()> {
		string_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_registerLayer_const_StringX_Constructor(typ.as_ptr(), constructor) }.into_result()
	}
	
	/// Unregisters registered layer with specified type name. Thread-safe.
	pub fn unregister_layer(typ: &str) -> Result<()> {
		string_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_unregisterLayer_const_StringX(typ.as_ptr()) }.into_result()
	}
	
	/// Creates instance of registered layer.
	/// ## Parameters
	/// * type: type name of creating layer.
	/// * params: parameters which will be used for layer initialization.
	/// 
	/// Note: Thread-safe.
	pub fn create_layer_instance(typ: &str, params: &mut crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		string_arg!(typ);
		unsafe { sys::cv_dnn_LayerFactory_createLayerInstance_const_StringX_LayerParamsX(typ.as_ptr(), params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be read by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub trait LayerParamsTrait: crate::dnn::DictTrait {
	fn as_raw_LayerParams(&self) -> *mut c_void;
	/// List of learned parameters stored as blobs.
	fn blobs(&mut self) -> types::VectorOfMat {
		unsafe { sys::cv_dnn_LayerParams_blobs(self.as_raw_LayerParams()) }.into_result().map(|ptr| types::VectorOfMat { ptr }).expect("Infallible function failed: blobs")
	}
	
	/// List of learned parameters stored as blobs.
	fn set_blobs(&mut self, val: types::VectorOfMat) -> () {
		unsafe { sys::cv_dnn_LayerParams_setBlobs_vector_Mat_(self.as_raw_LayerParams(), val.as_raw_VectorOfMat()) }.into_result().expect("Infallible function failed: set_blobs")
	}
	
	/// Name of the layer instance (optional, can be used internal purposes).
	fn name(&self) -> String {
		unsafe { sys::cv_dnn_LayerParams_name_const(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: name")
	}
	
	/// Name of the layer instance (optional, can be used internal purposes).
	fn set_name(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_LayerParams_setName_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_name")
	}
	
	/// Type name which was used for creating layer by layer factory (optional).
	fn typ(&self) -> String {
		unsafe { sys::cv_dnn_LayerParams_type_const(self.as_raw_LayerParams()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: typ")
	}
	
	/// Type name which was used for creating layer by layer factory (optional).
	fn set_type(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_LayerParams_setType_String(self.as_raw_LayerParams(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_type")
	}
	
}

/// This class provides all data needed to initialize layer.
/// 
/// It includes dictionary with scalar params (which can be read by using Dict interface),
/// blob params #blobs and optional meta information: #name and #type of layer instance.
pub struct LayerParams {
	pub(crate) ptr: *mut c_void
}

impl Drop for LayerParams {
	fn drop(&mut self) {
		extern "C" { fn cv_LayerParams_delete(instance: *mut c_void); }
		unsafe { cv_LayerParams_delete(self.as_raw_LayerParams()) };
	}
}

impl LayerParams {
	pub fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for LayerParams {}

impl crate::dnn::DictTrait for LayerParams {
	fn as_raw_Dict(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerParamsTrait for LayerParams {
	fn as_raw_LayerParams(&self) -> *mut c_void { self.ptr }
}

impl LayerParams {
}

pub trait MVNLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_MVNLayer(&self) -> *mut c_void;
	fn eps(&self) -> f32 {
		unsafe { sys::cv_dnn_MVNLayer_eps_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: eps")
	}
	
	fn set_eps(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setEps_float(self.as_raw_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_eps")
	}
	
	fn norm_variance(&self) -> bool {
		unsafe { sys::cv_dnn_MVNLayer_normVariance_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: norm_variance")
	}
	
	fn set_norm_variance(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setNormVariance_bool(self.as_raw_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_norm_variance")
	}
	
	fn across_channels(&self) -> bool {
		unsafe { sys::cv_dnn_MVNLayer_acrossChannels_const(self.as_raw_MVNLayer()) }.into_result().expect("Infallible function failed: across_channels")
	}
	
	fn set_across_channels(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_MVNLayer_setAcrossChannels_bool(self.as_raw_MVNLayer(), val) }.into_result().expect("Infallible function failed: set_across_channels")
	}
	
}

pub struct MVNLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for MVNLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_MVNLayer_delete(instance: *mut c_void); }
		unsafe { cv_MVNLayer_delete(self.as_raw_MVNLayer()) };
	}
}

impl MVNLayer {
	pub fn as_raw_MVNLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for MVNLayer {}

impl core::AlgorithmTrait for MVNLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for MVNLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::MVNLayerTrait for MVNLayer {
	fn as_raw_MVNLayer(&self) -> *mut c_void { self.ptr }
}

impl MVNLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfMVNLayer> {
		unsafe { sys::cv_dnn_MVNLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfMVNLayer { ptr })
	}
	
}

pub trait MaxUnpoolLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_MaxUnpoolLayer(&self) -> *mut c_void;
	fn pool_kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_poolKernel_const(self.as_raw_MaxUnpoolLayer()) }.into_result().expect("Infallible function failed: pool_kernel")
	}
	
	fn set_pool_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_setPoolKernel_Size(self.as_raw_MaxUnpoolLayer(), &val) }.into_result().expect("Infallible function failed: set_pool_kernel")
	}
	
	fn pool_pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_poolPad_const(self.as_raw_MaxUnpoolLayer()) }.into_result().expect("Infallible function failed: pool_pad")
	}
	
	fn set_pool_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_setPoolPad_Size(self.as_raw_MaxUnpoolLayer(), &val) }.into_result().expect("Infallible function failed: set_pool_pad")
	}
	
	fn pool_stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_poolStride_const(self.as_raw_MaxUnpoolLayer()) }.into_result().expect("Infallible function failed: pool_stride")
	}
	
	fn set_pool_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_setPoolStride_Size(self.as_raw_MaxUnpoolLayer(), &val) }.into_result().expect("Infallible function failed: set_pool_stride")
	}
	
}

pub struct MaxUnpoolLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for MaxUnpoolLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_MaxUnpoolLayer_delete(instance: *mut c_void); }
		unsafe { cv_MaxUnpoolLayer_delete(self.as_raw_MaxUnpoolLayer()) };
	}
}

impl MaxUnpoolLayer {
	pub fn as_raw_MaxUnpoolLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for MaxUnpoolLayer {}

impl core::AlgorithmTrait for MaxUnpoolLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for MaxUnpoolLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::MaxUnpoolLayerTrait for MaxUnpoolLayer {
	fn as_raw_MaxUnpoolLayer(&self) -> *mut c_void { self.ptr }
}

impl MaxUnpoolLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfMaxUnpoolLayer> {
		unsafe { sys::cv_dnn_MaxUnpoolLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfMaxUnpoolLayer { ptr })
	}
	
}

pub trait MishLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_MishLayer(&self) -> *mut c_void;
}

impl dyn MishLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfMishLayer> {
		unsafe { sys::cv_dnn_MishLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfMishLayer { ptr })
	}
	
}
/// This class allows to create and manipulate comprehensive artificial neural networks.
/// 
/// Neural network is presented as directed acyclic graph (DAG), where vertices are Layer instances,
/// and edges specify relationships between layers inputs and outputs.
/// 
/// Each network layer has unique integer id and unique string name inside its network.
/// LayerId can store either layer name or layer id.
/// 
/// This class supports reference counting of its instances, i. e. copies point to the same instance.
pub trait NetTrait {
	fn as_raw_Net(&self) -> *mut c_void;
	/// Returns true if there are no layers in the network.
	fn empty(&self) -> Result<bool> {
		unsafe { sys::cv_dnn_Net_empty_const(self.as_raw_Net()) }.into_result()
	}
	
	/// Dump net to String
	/// ## Returns
	/// String with structure, hyperparameters, backend, target and fusion
	/// Call method after setInput(). To see correct backend, target and fusion run after forward().
	fn dump(&mut self) -> Result<String> {
		unsafe { sys::cv_dnn_Net_dump(self.as_raw_Net()) }.into_result().map(crate::templ::receive_string)
	}
	
	/// Dump net structure, hyperparameters, backend, target and fusion to dot file
	/// ## Parameters
	/// * path: path to output file with .dot extension
	/// ## See also
	/// dump()
	fn dump_to_file(&mut self, path: &str) -> Result<()> {
		string_arg!(path);
		unsafe { sys::cv_dnn_Net_dumpToFile_const_StringX(self.as_raw_Net(), path.as_ptr()) }.into_result()
	}
	
	/// Adds new layer to the net.
	/// ## Parameters
	/// * name: unique name of the adding layer.
	/// * type: typename of the adding layer (type must be registered in LayerRegister).
	/// * params: parameters which will be used to initialize the creating layer.
	/// ## Returns
	/// unique identifier of created layer, or -1 if a failure will happen.
	fn add_layer(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		string_arg!(name);
		string_arg!(typ);
		unsafe { sys::cv_dnn_Net_addLayer_const_StringX_const_StringX_LayerParamsX(self.as_raw_Net(), name.as_ptr(), typ.as_ptr(), params.as_raw_LayerParams()) }.into_result()
	}
	
	/// Adds new layer and connects its first input to the first output of previously added layer.
	/// ## See also
	/// addLayer()
	fn add_layer_to_prev(&mut self, name: &str, typ: &str, params: &mut crate::dnn::LayerParams) -> Result<i32> {
		string_arg!(name);
		string_arg!(typ);
		unsafe { sys::cv_dnn_Net_addLayerToPrev_const_StringX_const_StringX_LayerParamsX(self.as_raw_Net(), name.as_ptr(), typ.as_ptr(), params.as_raw_LayerParams()) }.into_result()
	}
	
	/// Converts string name of the layer to the integer identifier.
	/// ## Returns
	/// id of the layer, or -1 if the layer wasn't found.
	fn get_layer_id(&mut self, layer: &str) -> Result<i32> {
		string_arg!(layer);
		unsafe { sys::cv_dnn_Net_getLayerId_const_StringX(self.as_raw_Net(), layer.as_ptr()) }.into_result()
	}
	
	fn get_layer_names(&self) -> Result<types::VectorOfString> {
		unsafe { sys::cv_dnn_Net_getLayerNames_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfString { ptr })
	}
	
	/// Returns pointer to layer with specified id or name which the network use.
	fn get_layer(&mut self, layer_id: crate::dnn::Net_LayerId) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_Net_getLayer_LayerId(self.as_raw_Net(), layer_id.as_raw_DictValue()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
	/// Returns pointers to input layers of specific layer.
	fn get_layer_inputs(&mut self, layer_id: crate::dnn::Net_LayerId) -> Result<types::VectorOfPtrOfLayer> {
		unsafe { sys::cv_dnn_Net_getLayerInputs_LayerId(self.as_raw_Net(), layer_id.as_raw_DictValue()) }.into_result().map(|ptr| types::VectorOfPtrOfLayer { ptr })
	}
	
	/// Connects output of the first layer to input of the second layer.
	/// ## Parameters
	/// * outPin: descriptor of the first layer output.
	/// * inpPin: descriptor of the second layer input.
	/// 
	/// Descriptors have the following template <DFN>&lt;layer_name&gt;[.input_number]</DFN>:
	/// - the first part of the template <DFN>layer_name</DFN> is string name of the added layer.
	///   If this part is empty then the network input pseudo layer will be used;
	/// - the second optional part of the template <DFN>input_number</DFN>
	///   is either number of the layer input, either label one.
	///   If this part is omitted then the first layer input will be used.
	/// ## See also
	/// setNetInputs(), Layer::inputNameToIndex(), Layer::outputNameToIndex()
	fn connect_first_second(&mut self, out_pin: &str, inp_pin: &str) -> Result<()> {
		string_arg!(out_pin);
		string_arg!(inp_pin);
		unsafe { sys::cv_dnn_Net_connect_String_String(self.as_raw_Net(), out_pin.as_ptr() as _, inp_pin.as_ptr() as _) }.into_result()
	}
	
	/// Connects #@p outNum output of the first layer to #@p inNum input of the second layer.
	/// ## Parameters
	/// * outLayerId: identifier of the first layer
	/// * outNum: number of the first layer output
	/// * inpLayerId: identifier of the second layer
	/// * inpNum: number of the second layer input
	fn connect(&mut self, out_layer_id: i32, out_num: i32, inp_layer_id: i32, inp_num: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Net_connect_int_int_int_int(self.as_raw_Net(), out_layer_id, out_num, inp_layer_id, inp_num) }.into_result()
	}
	
	/// Sets outputs names of the network input pseudo layer.
	/// 
	/// Each net always has special own the network input pseudo layer with id=0.
	/// This layer stores the user blobs only and don't make any computations.
	/// In fact, this layer provides the only way to pass user data into the network.
	/// As any other layer, this layer can label its outputs and this function provides an easy way to do this.
	fn set_inputs_names(&mut self, input_blob_names: &types::VectorOfString) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setInputsNames_const_vector_String_X(self.as_raw_Net(), input_blob_names.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputName: name for layer which output is needed to get
	/// ## Returns
	/// blob for first output of specified layer.
	/// @details By default runs forward pass for the whole network.
	/// 
	/// ## C++ default parameters
	/// * output_name: String()
	fn forward_single(&mut self, output_name: &str) -> Result<core::Mat> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Net_forward_const_StringX(self.as_raw_Net(), output_name.as_ptr()) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputName: name for layer which output is needed to get
	/// @details By default runs forward pass for the whole network.
	/// 
	/// This is an asynchronous version of forward(const String&).
	/// dnn::DNN_BACKEND_INFERENCE_ENGINE backend is required.
	/// 
	/// ## C++ default parameters
	/// * output_name: String()
	fn forward_async(&mut self, output_name: &str) -> Result<core::AsyncArray> {
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Net_forwardAsync_const_StringX(self.as_raw_Net(), output_name.as_ptr()) }.into_result().map(|ptr| core::AsyncArray { ptr })
	}
	
	/// Runs forward pass to compute output of layer with name @p outputName.
	/// ## Parameters
	/// * outputBlobs: contains all output blobs for specified layer.
	/// * outputName: name for layer which output is needed to get
	/// @details If @p outputName is empty, runs forward pass for the whole network.
	/// 
	/// ## C++ default parameters
	/// * output_name: String()
	fn forward_layer(&mut self, output_blobs: &mut dyn core::ToOutputArray, output_name: &str) -> Result<()> {
		output_array_arg!(output_blobs);
		string_arg!(output_name);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayX_const_StringX(self.as_raw_Net(), output_blobs.as_raw__OutputArray(), output_name.as_ptr()) }.into_result()
	}
	
	/// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
	/// ## Parameters
	/// * outputBlobs: contains blobs for first outputs of specified layers.
	/// * outBlobNames: names for layers which outputs are needed to get
	fn forward(&mut self, output_blobs: &mut dyn core::ToOutputArray, out_blob_names: &types::VectorOfString) -> Result<()> {
		output_array_arg!(output_blobs);
		unsafe { sys::cv_dnn_Net_forward_const__OutputArrayX_const_vector_String_X(self.as_raw_Net(), output_blobs.as_raw__OutputArray(), out_blob_names.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Runs forward pass to compute outputs of layers listed in @p outBlobNames.
	/// ## Parameters
	/// * outputBlobs: contains all output blobs for each layer specified in @p outBlobNames.
	/// * outBlobNames: names for layers which outputs are needed to get
	fn forward_and_retrieve(&mut self, output_blobs: &mut types::VectorOfVectorOfMat, out_blob_names: &types::VectorOfString) -> Result<()> {
		unsafe { sys::cv_dnn_Net_forward_vector_vector_Mat__X_const_vector_String_X(self.as_raw_Net(), output_blobs.as_raw_VectorOfVectorOfMat(), out_blob_names.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Compile Halide layers.
	/// ## Parameters
	/// * scheduler: Path to YAML file with scheduling directives.
	/// ## See also
	/// setPreferableBackend
	/// 
	/// Schedule layers that support Halide backend. Then compile them for
	/// specific target. For layers that not represented in scheduling file
	/// or if no manual scheduling used at all, automatic scheduling will be applied.
	fn set_halide_scheduler(&mut self, scheduler: &str) -> Result<()> {
		string_arg!(scheduler);
		unsafe { sys::cv_dnn_Net_setHalideScheduler_const_StringX(self.as_raw_Net(), scheduler.as_ptr()) }.into_result()
	}
	
	/// Ask network to use specific computation backend where it supported.
	/// ## Parameters
	/// * backendId: backend identifier.
	/// ## See also
	/// Backend
	/// 
	/// If OpenCV is compiled with Intel's Inference Engine library, DNN_BACKEND_DEFAULT
	/// means DNN_BACKEND_INFERENCE_ENGINE. Otherwise it equals to DNN_BACKEND_OPENCV.
	fn set_preferable_backend(&mut self, backend_id: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setPreferableBackend_int(self.as_raw_Net(), backend_id) }.into_result()
	}
	
	/// Ask network to make computations on specific target device.
	/// ## Parameters
	/// * targetId: target identifier.
	/// ## See also
	/// Target
	/// 
	/// List of supported combinations backend / target:
	/// |                        | DNN_BACKEND_OPENCV | DNN_BACKEND_INFERENCE_ENGINE | DNN_BACKEND_HALIDE |
	/// |------------------------|--------------------|------------------------------|--------------------|
	/// | DNN_TARGET_CPU         |                  + |                            + |                  + |
	/// | DNN_TARGET_OPENCL      |                  + |                            + |                  + |
	/// | DNN_TARGET_OPENCL_FP16 |                  + |                            + |                    |
	/// | DNN_TARGET_MYRIAD      |                    |                            + |                    |
	/// | DNN_TARGET_FPGA        |                    |                            + |                    |
	fn set_preferable_target(&mut self, target_id: i32) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setPreferableTarget_int(self.as_raw_Net(), target_id) }.into_result()
	}
	
	/// Sets the new input value for the network
	/// ## Parameters
	/// * blob: A new blob. Should have CV_32F or CV_8U depth.
	/// * name: A name of input layer.
	/// * scalefactor: An optional normalization scale.
	/// * mean: An optional mean subtraction values.
	/// ## See also
	/// connect(String, String) to know format of the descriptor.
	/// 
	///  If scale or mean values are specified, a final input blob is computed
	///  as:
	/// ![block formula](https://latex.codecogs.com/png.latex?input%28n%2Cc%2Ch%2Cw%29%20%3D%20scalefactor%20%5Ctimes%20%28blob%28n%2Cc%2Ch%2Cw%29%20%2D%20mean%5Fc%29)
	/// 
	/// ## C++ default parameters
	/// * name: ""
	/// * scalefactor: 1.0
	/// * mean: Scalar()
	fn set_input(&mut self, blob: &dyn core::ToInputArray, name: &str, scalefactor: f64, mean: core::Scalar) -> Result<()> {
		input_array_arg!(blob);
		string_arg!(name);
		unsafe { sys::cv_dnn_Net_setInput_const__InputArrayX_const_StringX_double_const_ScalarX(self.as_raw_Net(), blob.as_raw__InputArray(), name.as_ptr(), scalefactor, &mean) }.into_result()
	}
	
	/// Sets the new value for the learned param of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// * blob: the new value.
	/// ## See also
	/// Layer::blobs
	/// 
	/// Note: If shape of the new blob differs from the previous shape,
	/// then the following forward pass may fail.
	fn set_param(&mut self, layer: crate::dnn::Net_LayerId, num_param: i32, blob: &core::Mat) -> Result<()> {
		unsafe { sys::cv_dnn_Net_setParam_LayerId_int_const_MatX(self.as_raw_Net(), layer.as_raw_DictValue(), num_param, blob.as_raw_Mat()) }.into_result()
	}
	
	/// Returns parameter blob of the layer.
	/// ## Parameters
	/// * layer: name or id of the layer.
	/// * numParam: index of the layer parameter in the Layer::blobs array.
	/// ## See also
	/// Layer::blobs
	/// 
	/// ## C++ default parameters
	/// * num_param: 0
	fn get_param(&mut self, layer: crate::dnn::Net_LayerId, num_param: i32) -> Result<core::Mat> {
		unsafe { sys::cv_dnn_Net_getParam_LayerId_int(self.as_raw_Net(), layer.as_raw_DictValue(), num_param) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	/// Returns indexes of layers with unconnected outputs.
	fn get_unconnected_out_layers(&self) -> Result<types::VectorOfi32> {
		unsafe { sys::cv_dnn_Net_getUnconnectedOutLayers_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfi32 { ptr })
	}
	
	/// Returns names of layers with unconnected outputs.
	fn get_unconnected_out_layers_names(&self) -> Result<types::VectorOfString> {
		unsafe { sys::cv_dnn_Net_getUnconnectedOutLayersNames_const(self.as_raw_Net()) }.into_result().map(|ptr| types::VectorOfString { ptr })
	}
	
	/// Returns input and output shapes for all layers in loaded model;
	///  preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShapes: shapes for all input blobs in net input layer.
	/// * layersIds: output parameter for layer IDs.
	/// * inLayersShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayersShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	fn get_layers_shapes(&self, net_input_shapes: &types::VectorOfMatShape, layers_ids: &mut types::VectorOfi32, in_layers_shapes: &mut types::VectorOfVectorOfMatShape, out_layers_shapes: &mut types::VectorOfVectorOfMatShape) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getLayersShapes_const_const_vector_MatShape_X_vector_int_X_vector_vector_MatShape__X_vector_vector_MatShape__X(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layers_ids.as_raw_VectorOfi32(), in_layers_shapes.as_raw_VectorOfVectorOfMatShape(), out_layers_shapes.as_raw_VectorOfVectorOfMatShape()) }.into_result()
	}
	
	/// Returns input and output shapes for all layers in loaded model;
	///  preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShapes: shapes for all input blobs in net input layer.
	/// * layersIds: output parameter for layer IDs.
	/// * inLayersShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayersShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	/// 
	/// ## Overloaded parameters
	fn get_layers_shapes_1(&self, net_input_shape: &crate::dnn::MatShape, layers_ids: &mut types::VectorOfi32, in_layers_shapes: &mut types::VectorOfVectorOfMatShape, out_layers_shapes: &mut types::VectorOfVectorOfMatShape) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getLayersShapes_const_const_MatShapeX_vector_int_X_vector_vector_MatShape__X_vector_vector_MatShape__X(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layers_ids.as_raw_VectorOfi32(), in_layers_shapes.as_raw_VectorOfVectorOfMatShape(), out_layers_shapes.as_raw_VectorOfVectorOfMatShape()) }.into_result()
	}
	
	/// Returns input and output shapes for layer with specified
	/// id in loaded model; preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShape: shape input blob in net input layer.
	/// * layerId: id for layer.
	/// * inLayerShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayerShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	fn get_layer_shapes(&self, net_input_shape: &crate::dnn::MatShape, layer_id: i32, in_layer_shapes: &mut types::VectorOfMatShape, out_layer_shapes: &mut types::VectorOfMatShape) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getLayerShapes_const_const_MatShapeX_int_vector_MatShape_X_vector_MatShape_X(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layer_id, in_layer_shapes.as_raw_VectorOfMatShape(), out_layer_shapes.as_raw_VectorOfMatShape()) }.into_result()
	}
	
	/// Returns input and output shapes for layer with specified
	/// id in loaded model; preliminary inferencing isn't necessary.
	/// ## Parameters
	/// * netInputShape: shape input blob in net input layer.
	/// * layerId: id for layer.
	/// * inLayerShapes: output parameter for input layers shapes;
	/// order is the same as in layersIds
	/// * outLayerShapes: output parameter for output layers shapes;
	/// order is the same as in layersIds
	/// 
	/// ## Overloaded parameters
	fn get_layer_shapes_1(&self, net_input_shapes: &types::VectorOfMatShape, layer_id: i32, in_layer_shapes: &mut types::VectorOfMatShape, out_layer_shapes: &mut types::VectorOfMatShape) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getLayerShapes_const_const_vector_MatShape_X_int_vector_MatShape_X_vector_MatShape_X(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layer_id, in_layer_shapes.as_raw_VectorOfMatShape(), out_layer_shapes.as_raw_VectorOfMatShape()) }.into_result()
	}
	
	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	fn get_flops(&self, net_input_shapes: &types::VectorOfMatShape) -> Result<i64> {
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_vector_MatShape_X(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape()) }.into_result()
	}
	
	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	/// 
	/// ## Overloaded parameters
	fn get_flops_1(&self, net_input_shape: &crate::dnn::MatShape) -> Result<i64> {
		unsafe { sys::cv_dnn_Net_getFLOPS_const_const_MatShapeX(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	/// 
	/// ## Overloaded parameters
	fn get_flops_2(&self, layer_id: i32, net_input_shapes: &types::VectorOfMatShape) -> Result<i64> {
		unsafe { sys::cv_dnn_Net_getFLOPS_const_int_const_vector_MatShape_X(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfMatShape()) }.into_result()
	}
	
	/// Computes FLOP for whole loaded model with specified input shapes.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// ## Returns
	/// computed FLOP.
	/// 
	/// ## Overloaded parameters
	fn get_flops_3(&self, layer_id: i32, net_input_shape: &crate::dnn::MatShape) -> Result<i64> {
		unsafe { sys::cv_dnn_Net_getFLOPS_const_int_const_MatShapeX(self.as_raw_Net(), layer_id, net_input_shape.as_raw_VectorOfi32()) }.into_result()
	}
	
	/// Returns list of types for layer used in model.
	/// ## Parameters
	/// * layersTypes: output parameter for returning types.
	fn get_layer_types(&self, layers_types: &mut types::VectorOfString) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getLayerTypes_const_vector_String_X(self.as_raw_Net(), layers_types.as_raw_VectorOfString()) }.into_result()
	}
	
	/// Returns count of layers of specified type.
	/// ## Parameters
	/// * layerType: type.
	/// ## Returns
	/// count of layers
	fn get_layers_count(&self, layer_type: &str) -> Result<i32> {
		string_arg!(layer_type);
		unsafe { sys::cv_dnn_Net_getLayersCount_const_const_StringX(self.as_raw_Net(), layer_type.as_ptr()) }.into_result()
	}
	
	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for model.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	fn get_memory_consumption(&self, net_input_shapes: &types::VectorOfMatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_size_tX_size_tX(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), weights, blobs) }.into_result()
	}
	
	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	/// 
	/// ## Overloaded parameters
	fn get_memory_consumption_1(&self, net_input_shape: &crate::dnn::MatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_MatShapeX_size_tX_size_tX(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), weights, blobs) }.into_result()
	}
	
	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	/// 
	/// ## Overloaded parameters
	fn get_memory_consumption_for_layer(&self, layer_id: i32, net_input_shapes: &types::VectorOfMatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_int_const_vector_MatShape_X_size_tX_size_tX(self.as_raw_Net(), layer_id, net_input_shapes.as_raw_VectorOfMatShape(), weights, blobs) }.into_result()
	}
	
	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	/// 
	/// ## Overloaded parameters
	fn get_memory_consumption_2(&self, layer_id: i32, net_input_shape: &crate::dnn::MatShape, weights: &mut size_t, blobs: &mut size_t) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_int_const_MatShapeX_size_tX_size_tX(self.as_raw_Net(), layer_id, net_input_shape.as_raw_VectorOfi32(), weights, blobs) }.into_result()
	}
	
	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	fn get_memory_consumption_for_layers(&self, net_input_shapes: &types::VectorOfMatShape, layer_ids: &mut types::VectorOfi32, weights: &mut types::VectorOfsize_t, blobs: &mut types::VectorOfsize_t) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_vector_MatShape_X_vector_int_X_vector_size_t_X_vector_size_t_X(self.as_raw_Net(), net_input_shapes.as_raw_VectorOfMatShape(), layer_ids.as_raw_VectorOfi32(), weights.as_raw_VectorOfsize_t(), blobs.as_raw_VectorOfsize_t()) }.into_result()
	}
	
	/// Computes bytes number which are required to store
	/// all weights and intermediate blobs for each layer.
	/// ## Parameters
	/// * netInputShapes: vector of shapes for all net inputs.
	/// * layerIds: output vector to save layer IDs.
	/// * weights: output parameter to store resulting bytes for weights.
	/// * blobs: output parameter to store resulting bytes for intermediate blobs.
	/// 
	/// ## Overloaded parameters
	fn get_memory_consumption_3(&self, net_input_shape: &crate::dnn::MatShape, layer_ids: &mut types::VectorOfi32, weights: &mut types::VectorOfsize_t, blobs: &mut types::VectorOfsize_t) -> Result<()> {
		unsafe { sys::cv_dnn_Net_getMemoryConsumption_const_const_MatShapeX_vector_int_X_vector_size_t_X_vector_size_t_X(self.as_raw_Net(), net_input_shape.as_raw_VectorOfi32(), layer_ids.as_raw_VectorOfi32(), weights.as_raw_VectorOfsize_t(), blobs.as_raw_VectorOfsize_t()) }.into_result()
	}
	
	/// Enables or disables layer fusion in the network.
	/// ## Parameters
	/// * fusion: true to enable the fusion, false to disable. The fusion is enabled by default.
	fn enable_fusion(&mut self, fusion: bool) -> Result<()> {
		unsafe { sys::cv_dnn_Net_enableFusion_bool(self.as_raw_Net(), fusion) }.into_result()
	}
	
	/// Returns overall time for inference and timings (in ticks) for layers.
	/// Indexes in returned vector correspond to layers ids. Some layers can be fused with others,
	/// in this case zero ticks count will be return for that skipped layers.
	/// ## Parameters
	/// * timings: vector for tick timings for all layers.
	/// ## Returns
	/// overall ticks for model inference.
	fn get_perf_profile(&mut self, timings: &mut types::VectorOff64) -> Result<i64> {
		unsafe { sys::cv_dnn_Net_getPerfProfile_vector_double_X(self.as_raw_Net(), timings.as_raw_VectorOff64()) }.into_result()
	}
	
}

/// This class allows to create and manipulate comprehensive artificial neural networks.
/// 
/// Neural network is presented as directed acyclic graph (DAG), where vertices are Layer instances,
/// and edges specify relationships between layers inputs and outputs.
/// 
/// Each network layer has unique integer id and unique string name inside its network.
/// LayerId can store either layer name or layer id.
/// 
/// This class supports reference counting of its instances, i. e. copies point to the same instance.
pub struct Net {
	pub(crate) ptr: *mut c_void
}

impl Drop for Net {
	fn drop(&mut self) {
		extern "C" { fn cv_Net_delete(instance: *mut c_void); }
		unsafe { cv_Net_delete(self.as_raw_Net()) };
	}
}

impl Net {
	pub fn as_raw_Net(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Net {}

impl crate::dnn::NetTrait for Net {
	fn as_raw_Net(&self) -> *mut c_void { self.ptr }
}

impl Net {
	pub fn default() -> Result<crate::dnn::Net> {
		unsafe { sys::cv_dnn_Net_Net() }.into_result().map(|ptr| crate::dnn::Net { ptr })
	}
	
	/// Create a network from Intel's Model Optimizer intermediate representation (IR).
	/// ## Parameters
	/// * xml: XML configuration file with network's topology.
	/// * bin: Binary file with trained weights.
	/// Networks imported from Intel's Model Optimizer are launched in Intel's Inference Engine
	/// backend.
	pub fn read_from_model_optimizer(xml: &str, bin: &str) -> Result<crate::dnn::Net> {
		string_arg!(xml);
		string_arg!(bin);
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_StringX_const_StringX(xml.as_ptr(), bin.as_ptr()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
	}
	
	/// Create a network from Intel's Model Optimizer in-memory buffers with intermediate representation (IR).
	/// ## Parameters
	/// * bufferModelConfig: buffer with model's configuration.
	/// * bufferWeights: buffer with model's trained weights.
	/// ## Returns
	/// Net object.
	pub fn read_from_model_optimizer_1(buffer_model_config: &types::VectorOfu8, buffer_weights: &types::VectorOfu8) -> Result<crate::dnn::Net> {
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_vector_unsigned_char_X_const_vector_unsigned_char_X(buffer_model_config.as_raw_VectorOfu8(), buffer_weights.as_raw_VectorOfu8()) }.into_result().map(|ptr| crate::dnn::Net { ptr })
	}
	
	/// Create a network from Intel's Model Optimizer in-memory buffers with intermediate representation (IR).
	/// ## Parameters
	/// * bufferModelConfigPtr: buffer pointer of model's configuration.
	/// * bufferModelConfigSize: buffer size of model's configuration.
	/// * bufferWeightsPtr: buffer pointer of model's trained weights.
	/// * bufferWeightsSize: buffer size of model's trained weights.
	/// ## Returns
	/// Net object.
	pub fn read_from_model_optimizer_2(buffer_model_config_ptr: &u8, buffer_model_config_size: size_t, buffer_weights_ptr: &u8, buffer_weights_size: size_t) -> Result<crate::dnn::Net> {
		unsafe { sys::cv_dnn_Net_readFromModelOptimizer_const_unsigned_charX_size_t_const_unsigned_charX_size_t(buffer_model_config_ptr, buffer_model_config_size, buffer_weights_ptr, buffer_weights_size) }.into_result().map(|ptr| crate::dnn::Net { ptr })
	}
	
}

/// ![inline formula](https://latex.codecogs.com/png.latex?%20L%5Fp%20) - normalization layer.
/// ## Parameters
/// * p: Normalization factor. The most common `p = 1` for ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F1%20) -
///          normalization or `p = 2` for ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F2%20) - normalization or a custom one.
/// * eps: Parameter ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cepsilon%20) to prevent a division by zero.
/// * across_spatial: If true, normalize an input across all non-batch dimensions.
///                       Otherwise normalize an every channel separately.
/// 
/// Across spatial:
/// @f[
/// norm = \sqrt[p]{\epsilon + \sum_{x, y, c} |src(x, y, c)|^p } \\
/// dst(x, y, c) = \frac{ src(x, y, c) }{norm}
/// @f]
/// 
/// Channel wise normalization:
/// @f[
/// norm(c) = \sqrt[p]{\epsilon + \sum_{x, y} |src(x, y, c)|^p } \\
/// dst(x, y, c) = \frac{ src(x, y, c) }{norm(c)}
/// @f]
/// 
/// Where `x, y` - spatial coordinates, `c` - channel.
/// 
/// An every sample in the batch is normalized separately. Optionally,
/// output is scaled by the trained parameters.
pub trait NormalizeBBoxLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_NormalizeBBoxLayer(&self) -> *mut c_void;
	fn pnorm(&self) -> f32 {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_pnorm_const(self.as_raw_NormalizeBBoxLayer()) }.into_result().expect("Infallible function failed: pnorm")
	}
	
	fn set_pnorm(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_setPnorm_float(self.as_raw_NormalizeBBoxLayer(), val) }.into_result().expect("Infallible function failed: set_pnorm")
	}
	
	fn epsilon(&self) -> f32 {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_epsilon_const(self.as_raw_NormalizeBBoxLayer()) }.into_result().expect("Infallible function failed: epsilon")
	}
	
	fn set_epsilon(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_setEpsilon_float(self.as_raw_NormalizeBBoxLayer(), val) }.into_result().expect("Infallible function failed: set_epsilon")
	}
	
	fn across_spatial(&self) -> bool {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_acrossSpatial_const(self.as_raw_NormalizeBBoxLayer()) }.into_result().expect("Infallible function failed: across_spatial")
	}
	
	fn set_across_spatial(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_setAcrossSpatial_bool(self.as_raw_NormalizeBBoxLayer(), val) }.into_result().expect("Infallible function failed: set_across_spatial")
	}
	
}

/// ![inline formula](https://latex.codecogs.com/png.latex?%20L%5Fp%20) - normalization layer.
/// ## Parameters
/// * p: Normalization factor. The most common `p = 1` for ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F1%20) -
///          normalization or `p = 2` for ![inline formula](https://latex.codecogs.com/png.latex?%20L%5F2%20) - normalization or a custom one.
/// * eps: Parameter ![inline formula](https://latex.codecogs.com/png.latex?%20%5Cepsilon%20) to prevent a division by zero.
/// * across_spatial: If true, normalize an input across all non-batch dimensions.
///                       Otherwise normalize an every channel separately.
/// 
/// Across spatial:
/// @f[
/// norm = \sqrt[p]{\epsilon + \sum_{x, y, c} |src(x, y, c)|^p } \\
/// dst(x, y, c) = \frac{ src(x, y, c) }{norm}
/// @f]
/// 
/// Channel wise normalization:
/// @f[
/// norm(c) = \sqrt[p]{\epsilon + \sum_{x, y} |src(x, y, c)|^p } \\
/// dst(x, y, c) = \frac{ src(x, y, c) }{norm(c)}
/// @f]
/// 
/// Where `x, y` - spatial coordinates, `c` - channel.
/// 
/// An every sample in the batch is normalized separately. Optionally,
/// output is scaled by the trained parameters.
pub struct NormalizeBBoxLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for NormalizeBBoxLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_NormalizeBBoxLayer_delete(instance: *mut c_void); }
		unsafe { cv_NormalizeBBoxLayer_delete(self.as_raw_NormalizeBBoxLayer()) };
	}
}

impl NormalizeBBoxLayer {
	pub fn as_raw_NormalizeBBoxLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for NormalizeBBoxLayer {}

impl core::AlgorithmTrait for NormalizeBBoxLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for NormalizeBBoxLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::NormalizeBBoxLayerTrait for NormalizeBBoxLayer {
	fn as_raw_NormalizeBBoxLayer(&self) -> *mut c_void { self.ptr }
}

impl NormalizeBBoxLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfNormalizeBBoxLayer> {
		unsafe { sys::cv_dnn_NormalizeBBoxLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfNormalizeBBoxLayer { ptr })
	}
	
}

/// Adds extra values for specific axes.
/// ## Parameters
/// * paddings: Vector of paddings in format
///                ```ignore
///                [ pad_before, pad_after,  // [0]th dimension
///                   pad_before, pad_after,  // [1]st dimension
///                   ...
///                   pad_before, pad_after ] // [n]th dimension
///                ```
/// 
///                that represents number of padded values at every dimension
///                starting from the first one. The rest of dimensions won't
///                be padded.
/// * value: Value to be padded. Defaults to zero.
/// * type: Padding type: 'constant', 'reflect'
/// * input_dims: Torch's parameter. If @p input_dims is not equal to the
///                   actual input dimensionality then the `[0]th` dimension
///                   is considered as a batch dimension and @p paddings are shifted
///                   to a one dimension. Defaults to `-1` that means padding
///                   corresponding to @p paddings.
pub trait PaddingLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_PaddingLayer(&self) -> *mut c_void;
}

/// Adds extra values for specific axes.
/// ## Parameters
/// * paddings: Vector of paddings in format
///                ```ignore
///                [ pad_before, pad_after,  // [0]th dimension
///                   pad_before, pad_after,  // [1]st dimension
///                   ...
///                   pad_before, pad_after ] // [n]th dimension
///                ```
/// 
///                that represents number of padded values at every dimension
///                starting from the first one. The rest of dimensions won't
///                be padded.
/// * value: Value to be padded. Defaults to zero.
/// * type: Padding type: 'constant', 'reflect'
/// * input_dims: Torch's parameter. If @p input_dims is not equal to the
///                   actual input dimensionality then the `[0]th` dimension
///                   is considered as a batch dimension and @p paddings are shifted
///                   to a one dimension. Defaults to `-1` that means padding
///                   corresponding to @p paddings.
pub struct PaddingLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for PaddingLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PaddingLayer_delete(instance: *mut c_void); }
		unsafe { cv_PaddingLayer_delete(self.as_raw_PaddingLayer()) };
	}
}

impl PaddingLayer {
	pub fn as_raw_PaddingLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for PaddingLayer {}

impl core::AlgorithmTrait for PaddingLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PaddingLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::PaddingLayerTrait for PaddingLayer {
	fn as_raw_PaddingLayer(&self) -> *mut c_void { self.ptr }
}

impl PaddingLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPaddingLayer> {
		unsafe { sys::cv_dnn_PaddingLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPaddingLayer { ptr })
	}
	
}

pub trait PermuteLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_PermuteLayer(&self) -> *mut c_void;
}

pub struct PermuteLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for PermuteLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PermuteLayer_delete(instance: *mut c_void); }
		unsafe { cv_PermuteLayer_delete(self.as_raw_PermuteLayer()) };
	}
}

impl PermuteLayer {
	pub fn as_raw_PermuteLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for PermuteLayer {}

impl core::AlgorithmTrait for PermuteLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PermuteLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::PermuteLayerTrait for PermuteLayer {
	fn as_raw_PermuteLayer(&self) -> *mut c_void { self.ptr }
}

impl PermuteLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPermuteLayer> {
		unsafe { sys::cv_dnn_PermuteLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPermuteLayer { ptr })
	}
	
}

pub trait PoolingLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_PoolingLayer(&self) -> *mut c_void;
	fn typ(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_type_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setType_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn kernel_size(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_PoolingLayer_kernel_size(self.as_raw_PoolingLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: kernel_size")
	}
	
	fn set_kernel_size(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setKernel_size_vector_size_t_(self.as_raw_PoolingLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_kernel_size")
	}
	
	fn strides(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_PoolingLayer_strides(self.as_raw_PoolingLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: strides")
	}
	
	fn set_strides(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setStrides_vector_size_t_(self.as_raw_PoolingLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_strides")
	}
	
	fn pads_begin(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_PoolingLayer_pads_begin(self.as_raw_PoolingLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: pads_begin")
	}
	
	fn set_pads_begin(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPads_begin_vector_size_t_(self.as_raw_PoolingLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_pads_begin")
	}
	
	fn pads_end(&mut self) -> types::VectorOfsize_t {
		unsafe { sys::cv_dnn_PoolingLayer_pads_end(self.as_raw_PoolingLayer()) }.into_result().map(|ptr| types::VectorOfsize_t { ptr }).expect("Infallible function failed: pads_end")
	}
	
	fn set_pads_end(&mut self, val: types::VectorOfsize_t) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPads_end_vector_size_t_(self.as_raw_PoolingLayer(), val.as_raw_VectorOfsize_t()) }.into_result().expect("Infallible function failed: set_pads_end")
	}
	
	fn kernel(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_kernel_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: kernel")
	}
	
	fn set_kernel(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setKernel_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_kernel")
	}
	
	fn stride(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_stride_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: stride")
	}
	
	fn set_stride(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setStride_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_stride")
	}
	
	fn pad(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_pad_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad")
	}
	
	fn set_pad(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPad_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_pad")
	}
	
	fn pad_l(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_pad_l_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad_l")
	}
	
	fn set_pad_l(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPad_l_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_pad_l")
	}
	
	fn pad_t(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_pad_t_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad_t")
	}
	
	fn set_pad_t(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPad_t_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_pad_t")
	}
	
	fn pad_r(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_pad_r_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad_r")
	}
	
	fn set_pad_r(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPad_r_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_pad_r")
	}
	
	fn pad_b(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_pad_b_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pad_b")
	}
	
	fn set_pad_b(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPad_b_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_pad_b")
	}
	
	fn global_pooling(&self) -> bool {
		unsafe { sys::cv_dnn_PoolingLayer_globalPooling_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: global_pooling")
	}
	
	fn set_global_pooling(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setGlobalPooling_bool(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_global_pooling")
	}
	
	fn compute_max_idx(&self) -> bool {
		unsafe { sys::cv_dnn_PoolingLayer_computeMaxIdx_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: compute_max_idx")
	}
	
	fn set_compute_max_idx(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setComputeMaxIdx_bool(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_compute_max_idx")
	}
	
	fn pad_mode(&self) -> String {
		unsafe { sys::cv_dnn_PoolingLayer_padMode_const(self.as_raw_PoolingLayer()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: pad_mode")
	}
	
	fn set_pad_mode(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_dnn_PoolingLayer_setPadMode_String(self.as_raw_PoolingLayer(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_pad_mode")
	}
	
	fn ceil_mode(&self) -> bool {
		unsafe { sys::cv_dnn_PoolingLayer_ceilMode_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: ceil_mode")
	}
	
	fn set_ceil_mode(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setCeilMode_bool(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_ceil_mode")
	}
	
	fn ave_pool_padded_area(&self) -> bool {
		unsafe { sys::cv_dnn_PoolingLayer_avePoolPaddedArea_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: ave_pool_padded_area")
	}
	
	fn set_ave_pool_padded_area(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setAvePoolPaddedArea_bool(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_ave_pool_padded_area")
	}
	
	fn pooled_size(&self) -> core::Size {
		unsafe { sys::cv_dnn_PoolingLayer_pooledSize_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: pooled_size")
	}
	
	fn set_pooled_size(&mut self, val: core::Size) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPooledSize_Size(self.as_raw_PoolingLayer(), &val) }.into_result().expect("Infallible function failed: set_pooled_size")
	}
	
	fn spatial_scale(&self) -> f32 {
		unsafe { sys::cv_dnn_PoolingLayer_spatialScale_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: spatial_scale")
	}
	
	fn set_spatial_scale(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setSpatialScale_float(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_spatial_scale")
	}
	
	fn ps_roi_out_channels(&self) -> i32 {
		unsafe { sys::cv_dnn_PoolingLayer_psRoiOutChannels_const(self.as_raw_PoolingLayer()) }.into_result().expect("Infallible function failed: ps_roi_out_channels")
	}
	
	fn set_ps_roi_out_channels(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_PoolingLayer_setPsRoiOutChannels_int(self.as_raw_PoolingLayer(), val) }.into_result().expect("Infallible function failed: set_ps_roi_out_channels")
	}
	
}

pub struct PoolingLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for PoolingLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PoolingLayer_delete(instance: *mut c_void); }
		unsafe { cv_PoolingLayer_delete(self.as_raw_PoolingLayer()) };
	}
}

impl PoolingLayer {
	pub fn as_raw_PoolingLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for PoolingLayer {}

impl core::AlgorithmTrait for PoolingLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PoolingLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::PoolingLayerTrait for PoolingLayer {
	fn as_raw_PoolingLayer(&self) -> *mut c_void { self.ptr }
}

impl PoolingLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPoolingLayer> {
		unsafe { sys::cv_dnn_PoolingLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPoolingLayer { ptr })
	}
	
}

pub trait PowerLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_PowerLayer(&self) -> *mut c_void;
	fn power(&self) -> f32 {
		unsafe { sys::cv_dnn_PowerLayer_power_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: power")
	}
	
	fn set_power(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setPower_float(self.as_raw_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_power")
	}
	
	fn scale(&self) -> f32 {
		unsafe { sys::cv_dnn_PowerLayer_scale_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: scale")
	}
	
	fn set_scale(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setScale_float(self.as_raw_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_scale")
	}
	
	fn shift(&self) -> f32 {
		unsafe { sys::cv_dnn_PowerLayer_shift_const(self.as_raw_PowerLayer()) }.into_result().expect("Infallible function failed: shift")
	}
	
	fn set_shift(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_PowerLayer_setShift_float(self.as_raw_PowerLayer(), val) }.into_result().expect("Infallible function failed: set_shift")
	}
	
}

impl dyn PowerLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPowerLayer> {
		unsafe { sys::cv_dnn_PowerLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPowerLayer { ptr })
	}
	
}
pub trait PriorBoxLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_PriorBoxLayer(&self) -> *mut c_void;
}

pub struct PriorBoxLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for PriorBoxLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_PriorBoxLayer_delete(instance: *mut c_void); }
		unsafe { cv_PriorBoxLayer_delete(self.as_raw_PriorBoxLayer()) };
	}
}

impl PriorBoxLayer {
	pub fn as_raw_PriorBoxLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for PriorBoxLayer {}

impl core::AlgorithmTrait for PriorBoxLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for PriorBoxLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::PriorBoxLayerTrait for PriorBoxLayer {
	fn as_raw_PriorBoxLayer(&self) -> *mut c_void { self.ptr }
}

impl PriorBoxLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfPriorBoxLayer> {
		unsafe { sys::cv_dnn_PriorBoxLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfPriorBoxLayer { ptr })
	}
	
}

pub trait ProposalLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ProposalLayer(&self) -> *mut c_void;
}

pub struct ProposalLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ProposalLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ProposalLayer_delete(instance: *mut c_void); }
		unsafe { cv_ProposalLayer_delete(self.as_raw_ProposalLayer()) };
	}
}

impl ProposalLayer {
	pub fn as_raw_ProposalLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ProposalLayer {}

impl core::AlgorithmTrait for ProposalLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ProposalLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ProposalLayerTrait for ProposalLayer {
	fn as_raw_ProposalLayer(&self) -> *mut c_void { self.ptr }
}

impl ProposalLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfProposalLayer> {
		unsafe { sys::cv_dnn_ProposalLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfProposalLayer { ptr })
	}
	
}

/// Classical recurrent layer
/// 
/// Accepts two inputs @f$x_t@f$ and @f$h_{t-1}@f$ and compute two outputs @f$o_t@f$ and @f$h_t@f$.
/// 
/// - input: should contain packed input @f$x_t@f$.
/// - output: should contain output @f$o_t@f$ (and @f$h_t@f$ if setProduceHiddenOutput() is set to true).
/// 
/// input[0] should have shape [`T`, `N`, `data_dims`] where `T` and `N` is number of timestamps and number of independent samples of @f$x_t@f$ respectively.
/// 
/// output[0] will have shape [`T`, `N`, @f$N_o@f$], where @f$N_o@f$ is number of rows in @f$ W_{xo} @f$ matrix.
/// 
/// If setProduceHiddenOutput() is set to true then @p output[1] will contain a Mat with shape [`T`, `N`, @f$N_h@f$], where @f$N_h@f$ is number of rows in @f$ W_{hh} @f$ matrix.
pub trait RNNLayer: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_RNNLayer(&self) -> *mut c_void;
	/// Setups learned weights.
	/// 
	/// Recurrent-layer behavior on each step is defined by current input @f$ x_t @f$, previous state @f$ h_t @f$ and learned weights as follows:
	/// @f{eqnarray*}{
	/// h_t &= tanh&(W_{hh} h_{t-1} + W_{xh} x_t + b_h),  \\
	/// o_t &= tanh&(W_{ho} h_t + b_o),
	/// @f}
	/// 
	/// ## Parameters
	/// * Wxh: is @f$ W_{xh} @f$ matrix
	/// * bh: is @f$ b_{h}  @f$ vector
	/// * Whh: is @f$ W_{hh} @f$ matrix
	/// * Who: is @f$ W_{xo} @f$ matrix
	/// * bo: is @f$ b_{o}  @f$ vector
	fn set_weights(&mut self, wxh: &core::Mat, bh: &core::Mat, whh: &core::Mat, who: &core::Mat, bo: &core::Mat) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_setWeights_const_MatX_const_MatX_const_MatX_const_MatX_const_MatX(self.as_raw_RNNLayer(), wxh.as_raw_Mat(), bh.as_raw_Mat(), whh.as_raw_Mat(), who.as_raw_Mat(), bo.as_raw_Mat()) }.into_result()
	}
	
	/// If this flag is set to true then layer will produce @f$ h_t @f$ as second output.
	/// @details Shape of the second output is the same as first output.
	/// 
	/// ## C++ default parameters
	/// * produce: false
	fn set_produce_hidden_output(&mut self, produce: bool) -> Result<()> {
		unsafe { sys::cv_dnn_RNNLayer_setProduceHiddenOutput_bool(self.as_raw_RNNLayer(), produce) }.into_result()
	}
	
}

impl dyn RNNLayer + '_ {
	/// Creates instance of RNNLayer
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfRNNLayer> {
		unsafe { sys::cv_dnn_RNNLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfRNNLayer { ptr })
	}
	
}
pub trait ReLU6Layer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_ReLU6Layer(&self) -> *mut c_void;
	fn min_value(&self) -> f32 {
		unsafe { sys::cv_dnn_ReLU6Layer_minValue_const(self.as_raw_ReLU6Layer()) }.into_result().expect("Infallible function failed: min_value")
	}
	
	fn set_min_value(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_ReLU6Layer_setMinValue_float(self.as_raw_ReLU6Layer(), val) }.into_result().expect("Infallible function failed: set_min_value")
	}
	
	fn max_value(&self) -> f32 {
		unsafe { sys::cv_dnn_ReLU6Layer_maxValue_const(self.as_raw_ReLU6Layer()) }.into_result().expect("Infallible function failed: max_value")
	}
	
	fn set_max_value(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_ReLU6Layer_setMaxValue_float(self.as_raw_ReLU6Layer(), val) }.into_result().expect("Infallible function failed: set_max_value")
	}
	
}

impl dyn ReLU6Layer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReLU6Layer> {
		unsafe { sys::cv_dnn_ReLU6Layer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReLU6Layer { ptr })
	}
	
}
pub trait ReLULayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_ReLULayer(&self) -> *mut c_void;
	fn negative_slope(&self) -> f32 {
		unsafe { sys::cv_dnn_ReLULayer_negativeSlope_const(self.as_raw_ReLULayer()) }.into_result().expect("Infallible function failed: negative_slope")
	}
	
	fn set_negative_slope(&mut self, val: f32) -> () {
		unsafe { sys::cv_dnn_ReLULayer_setNegativeSlope_float(self.as_raw_ReLULayer(), val) }.into_result().expect("Infallible function failed: set_negative_slope")
	}
	
}

impl dyn ReLULayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReLULayer> {
		unsafe { sys::cv_dnn_ReLULayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReLULayer { ptr })
	}
	
}
pub trait RegionLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_RegionLayer(&self) -> *mut c_void;
}

pub struct RegionLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for RegionLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_RegionLayer_delete(instance: *mut c_void); }
		unsafe { cv_RegionLayer_delete(self.as_raw_RegionLayer()) };
	}
}

impl RegionLayer {
	pub fn as_raw_RegionLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for RegionLayer {}

impl core::AlgorithmTrait for RegionLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for RegionLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::RegionLayerTrait for RegionLayer {
	fn as_raw_RegionLayer(&self) -> *mut c_void { self.ptr }
}

impl RegionLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfRegionLayer> {
		unsafe { sys::cv_dnn_RegionLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfRegionLayer { ptr })
	}
	
}

pub trait ReorgLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ReorgLayer(&self) -> *mut c_void;
}

pub struct ReorgLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ReorgLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReorgLayer_delete(instance: *mut c_void); }
		unsafe { cv_ReorgLayer_delete(self.as_raw_ReorgLayer()) };
	}
}

impl ReorgLayer {
	pub fn as_raw_ReorgLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ReorgLayer {}

impl core::AlgorithmTrait for ReorgLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ReorgLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ReorgLayerTrait for ReorgLayer {
	fn as_raw_ReorgLayer(&self) -> *mut c_void { self.ptr }
}

impl ReorgLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReorgLayer> {
		unsafe { sys::cv_dnn_ReorgLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReorgLayer { ptr })
	}
	
}

pub trait ReshapeLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ReshapeLayer(&self) -> *mut c_void;
	fn new_shape_desc(&mut self) -> types::VectorOfi32 {
		unsafe { sys::cv_dnn_ReshapeLayer_newShapeDesc(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| types::VectorOfi32 { ptr }).expect("Infallible function failed: new_shape_desc")
	}
	
	fn set_new_shape_desc(&mut self, val: crate::dnn::MatShape) -> () {
		unsafe { sys::cv_dnn_ReshapeLayer_setNewShapeDesc_MatShape(self.as_raw_ReshapeLayer(), val.as_raw_VectorOfi32()) }.into_result().expect("Infallible function failed: set_new_shape_desc")
	}
	
	fn new_shape_range(&mut self) -> core::Range {
		unsafe { sys::cv_dnn_ReshapeLayer_newShapeRange(self.as_raw_ReshapeLayer()) }.into_result().map(|ptr| core::Range { ptr }).expect("Infallible function failed: new_shape_range")
	}
	
	fn set_new_shape_range(&mut self, val: core::Range) -> () {
		unsafe { sys::cv_dnn_ReshapeLayer_setNewShapeRange_Range(self.as_raw_ReshapeLayer(), val.as_raw_Range()) }.into_result().expect("Infallible function failed: set_new_shape_range")
	}
	
}

pub struct ReshapeLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ReshapeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ReshapeLayer_delete(instance: *mut c_void); }
		unsafe { cv_ReshapeLayer_delete(self.as_raw_ReshapeLayer()) };
	}
}

impl ReshapeLayer {
	pub fn as_raw_ReshapeLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ReshapeLayer {}

impl core::AlgorithmTrait for ReshapeLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ReshapeLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ReshapeLayerTrait for ReshapeLayer {
	fn as_raw_ReshapeLayer(&self) -> *mut c_void { self.ptr }
}

impl ReshapeLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfReshapeLayer> {
		unsafe { sys::cv_dnn_ReshapeLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfReshapeLayer { ptr })
	}
	
}

/// Resize input 4-dimensional blob by nearest neighbor or bilinear strategy.
/// 
/// Layer is used to support TensorFlow's resize_nearest_neighbor and resize_bilinear ops.
pub trait ResizeLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ResizeLayer(&self) -> *mut c_void;
}

/// Resize input 4-dimensional blob by nearest neighbor or bilinear strategy.
/// 
/// Layer is used to support TensorFlow's resize_nearest_neighbor and resize_bilinear ops.
pub struct ResizeLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ResizeLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ResizeLayer_delete(instance: *mut c_void); }
		unsafe { cv_ResizeLayer_delete(self.as_raw_ResizeLayer()) };
	}
}

impl ResizeLayer {
	pub fn as_raw_ResizeLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ResizeLayer {}

impl core::AlgorithmTrait for ResizeLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ResizeLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ResizeLayerTrait for ResizeLayer {
	fn as_raw_ResizeLayer(&self) -> *mut c_void { self.ptr }
}

impl ResizeLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfResizeLayer> {
		unsafe { sys::cv_dnn_ResizeLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfResizeLayer { ptr })
	}
	
}

pub trait ScaleLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ScaleLayer(&self) -> *mut c_void;
	fn has_bias(&self) -> bool {
		unsafe { sys::cv_dnn_ScaleLayer_hasBias_const(self.as_raw_ScaleLayer()) }.into_result().expect("Infallible function failed: has_bias")
	}
	
	fn set_has_bias(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_ScaleLayer_setHasBias_bool(self.as_raw_ScaleLayer(), val) }.into_result().expect("Infallible function failed: set_has_bias")
	}
	
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_ScaleLayer_axis_const(self.as_raw_ScaleLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_ScaleLayer_setAxis_int(self.as_raw_ScaleLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
}

pub struct ScaleLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ScaleLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ScaleLayer_delete(instance: *mut c_void); }
		unsafe { cv_ScaleLayer_delete(self.as_raw_ScaleLayer()) };
	}
}

impl ScaleLayer {
	pub fn as_raw_ScaleLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ScaleLayer {}

impl core::AlgorithmTrait for ScaleLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ScaleLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ScaleLayerTrait for ScaleLayer {
	fn as_raw_ScaleLayer(&self) -> *mut c_void { self.ptr }
}

impl ScaleLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfScaleLayer> {
		unsafe { sys::cv_dnn_ScaleLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfScaleLayer { ptr })
	}
	
}

pub trait ShiftLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ShiftLayer(&self) -> *mut c_void;
}

pub struct ShiftLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ShiftLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ShiftLayer_delete(instance: *mut c_void); }
		unsafe { cv_ShiftLayer_delete(self.as_raw_ShiftLayer()) };
	}
}

impl ShiftLayer {
	pub fn as_raw_ShiftLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ShiftLayer {}

impl core::AlgorithmTrait for ShiftLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ShiftLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ShiftLayerTrait for ShiftLayer {
	fn as_raw_ShiftLayer(&self) -> *mut c_void { self.ptr }
}

impl ShiftLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_ShiftLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

/// Permute channels of 4-dimensional input blob.
/// ## Parameters
/// * group: Number of groups to split input channels and pick in turns
///              into output blob.
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%20groupSize%20%3D%20%5Cfrac%7Bnumber%5C%20of%5C%20channels%7D%7Bgroup%7D%20)
/// ![block formula](https://latex.codecogs.com/png.latex?%20output%28n%2C%20c%2C%20h%2C%20w%29%20%3D%20input%28n%2C%20groupSize%20%5Ctimes%20%28c%20%5C%25%20group%29%20%2B%20%5Clfloor%20%5Cfrac%7Bc%7D%7Bgroup%7D%20%5Crfloor%2C%20h%2C%20w%29%20)
/// Read more at https://arxiv.org/pdf/1707.01083.pdf
pub trait ShuffleChannelLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_ShuffleChannelLayer(&self) -> *mut c_void;
	fn group(&self) -> i32 {
		unsafe { sys::cv_dnn_ShuffleChannelLayer_group_const(self.as_raw_ShuffleChannelLayer()) }.into_result().expect("Infallible function failed: group")
	}
	
	fn set_group(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_ShuffleChannelLayer_setGroup_int(self.as_raw_ShuffleChannelLayer(), val) }.into_result().expect("Infallible function failed: set_group")
	}
	
}

/// Permute channels of 4-dimensional input blob.
/// ## Parameters
/// * group: Number of groups to split input channels and pick in turns
///              into output blob.
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%20groupSize%20%3D%20%5Cfrac%7Bnumber%5C%20of%5C%20channels%7D%7Bgroup%7D%20)
/// ![block formula](https://latex.codecogs.com/png.latex?%20output%28n%2C%20c%2C%20h%2C%20w%29%20%3D%20input%28n%2C%20groupSize%20%5Ctimes%20%28c%20%5C%25%20group%29%20%2B%20%5Clfloor%20%5Cfrac%7Bc%7D%7Bgroup%7D%20%5Crfloor%2C%20h%2C%20w%29%20)
/// Read more at https://arxiv.org/pdf/1707.01083.pdf
pub struct ShuffleChannelLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for ShuffleChannelLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_ShuffleChannelLayer_delete(instance: *mut c_void); }
		unsafe { cv_ShuffleChannelLayer_delete(self.as_raw_ShuffleChannelLayer()) };
	}
}

impl ShuffleChannelLayer {
	pub fn as_raw_ShuffleChannelLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for ShuffleChannelLayer {}

impl core::AlgorithmTrait for ShuffleChannelLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for ShuffleChannelLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::ShuffleChannelLayerTrait for ShuffleChannelLayer {
	fn as_raw_ShuffleChannelLayer(&self) -> *mut c_void { self.ptr }
}

impl ShuffleChannelLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfLayer> {
		unsafe { sys::cv_dnn_ShuffleChannelLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfLayer { ptr })
	}
	
}

pub trait SigmoidLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_SigmoidLayer(&self) -> *mut c_void;
}

impl dyn SigmoidLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSigmoidLayer> {
		unsafe { sys::cv_dnn_SigmoidLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSigmoidLayer { ptr })
	}
	
}
/// Slice layer has several modes:
/// 1. Caffe mode
/// ## Parameters
/// * axis: Axis of split operation
/// * slice_point: Array of split points
/// 
/// Number of output blobs equals to number of split points plus one. The
/// first blob is a slice on input from 0 to @p slice_point[0] - 1 by @p axis,
/// the second output blob is a slice of input from @p slice_point[0] to
/// @p slice_point[1] - 1 by @p axis and the last output blob is a slice of
/// input from @p slice_point[-1] up to the end of @p axis size.
/// 
/// 2. TensorFlow mode
/// * begin: Vector of start indices
/// * size: Vector of sizes
/// 
/// More convenient numpy-like slice. One and only output blob
/// is a slice `input[begin[0]:begin[0]+size[0], begin[1]:begin[1]+size[1], ...]`
/// 
/// 3. Torch mode
/// * axis: Axis of split operation
/// 
/// Split input blob on the equal parts by @p axis.
pub trait SliceLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_SliceLayer(&self) -> *mut c_void;
	/// Vector of slice ranges.
	/// 
	/// The first dimension equals number of output blobs.
	/// Inner vector has slice ranges for the first number of input dimensions.
	fn slice_ranges(&mut self) -> types::VectorOfVectorOfRange {
		unsafe { sys::cv_dnn_SliceLayer_sliceRanges(self.as_raw_SliceLayer()) }.into_result().map(|ptr| types::VectorOfVectorOfRange { ptr }).expect("Infallible function failed: slice_ranges")
	}
	
	/// Vector of slice ranges.
	/// 
	/// The first dimension equals number of output blobs.
	/// Inner vector has slice ranges for the first number of input dimensions.
	fn set_slice_ranges(&mut self, val: types::VectorOfVectorOfRange) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setSliceRanges_vector_vector_Range__(self.as_raw_SliceLayer(), val.as_raw_VectorOfVectorOfRange()) }.into_result().expect("Infallible function failed: set_slice_ranges")
	}
	
	fn axis(&self) -> i32 {
		unsafe { sys::cv_dnn_SliceLayer_axis_const(self.as_raw_SliceLayer()) }.into_result().expect("Infallible function failed: axis")
	}
	
	fn set_axis(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setAxis_int(self.as_raw_SliceLayer(), val) }.into_result().expect("Infallible function failed: set_axis")
	}
	
	fn num_split(&self) -> i32 {
		unsafe { sys::cv_dnn_SliceLayer_num_split_const(self.as_raw_SliceLayer()) }.into_result().expect("Infallible function failed: num_split")
	}
	
	fn set_num_split(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SliceLayer_setNum_split_int(self.as_raw_SliceLayer(), val) }.into_result().expect("Infallible function failed: set_num_split")
	}
	
}

/// Slice layer has several modes:
/// 1. Caffe mode
/// ## Parameters
/// * axis: Axis of split operation
/// * slice_point: Array of split points
/// 
/// Number of output blobs equals to number of split points plus one. The
/// first blob is a slice on input from 0 to @p slice_point[0] - 1 by @p axis,
/// the second output blob is a slice of input from @p slice_point[0] to
/// @p slice_point[1] - 1 by @p axis and the last output blob is a slice of
/// input from @p slice_point[-1] up to the end of @p axis size.
/// 
/// 2. TensorFlow mode
/// * begin: Vector of start indices
/// * size: Vector of sizes
/// 
/// More convenient numpy-like slice. One and only output blob
/// is a slice `input[begin[0]:begin[0]+size[0], begin[1]:begin[1]+size[1], ...]`
/// 
/// 3. Torch mode
/// * axis: Axis of split operation
/// 
/// Split input blob on the equal parts by @p axis.
pub struct SliceLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for SliceLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SliceLayer_delete(instance: *mut c_void); }
		unsafe { cv_SliceLayer_delete(self.as_raw_SliceLayer()) };
	}
}

impl SliceLayer {
	pub fn as_raw_SliceLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for SliceLayer {}

impl core::AlgorithmTrait for SliceLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SliceLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::SliceLayerTrait for SliceLayer {
	fn as_raw_SliceLayer(&self) -> *mut c_void { self.ptr }
}

impl SliceLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSliceLayer> {
		unsafe { sys::cv_dnn_SliceLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSliceLayer { ptr })
	}
	
}

pub trait SoftmaxLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_SoftmaxLayer(&self) -> *mut c_void;
	fn log_soft_max(&self) -> bool {
		unsafe { sys::cv_dnn_SoftmaxLayer_logSoftMax_const(self.as_raw_SoftmaxLayer()) }.into_result().expect("Infallible function failed: log_soft_max")
	}
	
	fn set_log_soft_max(&mut self, val: bool) -> () {
		unsafe { sys::cv_dnn_SoftmaxLayer_setLogSoftMax_bool(self.as_raw_SoftmaxLayer(), val) }.into_result().expect("Infallible function failed: set_log_soft_max")
	}
	
}

pub struct SoftmaxLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for SoftmaxLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SoftmaxLayer_delete(instance: *mut c_void); }
		unsafe { cv_SoftmaxLayer_delete(self.as_raw_SoftmaxLayer()) };
	}
}

impl SoftmaxLayer {
	pub fn as_raw_SoftmaxLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for SoftmaxLayer {}

impl core::AlgorithmTrait for SoftmaxLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SoftmaxLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::SoftmaxLayerTrait for SoftmaxLayer {
	fn as_raw_SoftmaxLayer(&self) -> *mut c_void { self.ptr }
}

impl SoftmaxLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSoftmaxLayer> {
		unsafe { sys::cv_dnn_SoftmaxLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSoftmaxLayer { ptr })
	}
	
}

pub trait SplitLayerTrait: core::AlgorithmTrait + crate::dnn::LayerTrait {
	fn as_raw_SplitLayer(&self) -> *mut c_void;
	/// Number of copies that will be produced (is ignored when negative).
	fn outputs_count(&self) -> i32 {
		unsafe { sys::cv_dnn_SplitLayer_outputsCount_const(self.as_raw_SplitLayer()) }.into_result().expect("Infallible function failed: outputs_count")
	}
	
	/// Number of copies that will be produced (is ignored when negative).
	fn set_outputs_count(&mut self, val: i32) -> () {
		unsafe { sys::cv_dnn_SplitLayer_setOutputsCount_int(self.as_raw_SplitLayer(), val) }.into_result().expect("Infallible function failed: set_outputs_count")
	}
	
}

pub struct SplitLayer {
	pub(crate) ptr: *mut c_void
}

impl Drop for SplitLayer {
	fn drop(&mut self) {
		extern "C" { fn cv_SplitLayer_delete(instance: *mut c_void); }
		unsafe { cv_SplitLayer_delete(self.as_raw_SplitLayer()) };
	}
}

impl SplitLayer {
	pub fn as_raw_SplitLayer(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for SplitLayer {}

impl core::AlgorithmTrait for SplitLayer {
	fn as_raw_Algorithm(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::LayerTrait for SplitLayer {
	fn as_raw_Layer(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::SplitLayerTrait for SplitLayer {
	fn as_raw_SplitLayer(&self) -> *mut c_void { self.ptr }
}

impl SplitLayer {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSplitLayer> {
		unsafe { sys::cv_dnn_SplitLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSplitLayer { ptr })
	}
	
}

pub trait SwishLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_SwishLayer(&self) -> *mut c_void;
}

impl dyn SwishLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfSwishLayer> {
		unsafe { sys::cv_dnn_SwishLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfSwishLayer { ptr })
	}
	
}
pub trait TanHLayer: core::AlgorithmTrait + crate::dnn::ActivationLayer + crate::dnn::LayerTrait {
	fn as_raw_TanHLayer(&self) -> *mut c_void;
}

impl dyn TanHLayer + '_ {
	pub fn create(params: &crate::dnn::LayerParams) -> Result<types::PtrOfTanHLayer> {
		unsafe { sys::cv_dnn_TanHLayer_create_const_LayerParamsX(params.as_raw_LayerParams()) }.into_result().map(|ptr| types::PtrOfTanHLayer { ptr })
	}
	
}
pub trait _RangeTrait: core::RangeTrait {
	fn as_raw__Range(&self) -> *mut c_void;
}

pub struct _Range {
	pub(crate) ptr: *mut c_void
}

impl Drop for _Range {
	fn drop(&mut self) {
		extern "C" { fn cv__Range_delete(instance: *mut c_void); }
		unsafe { cv__Range_delete(self.as_raw__Range()) };
	}
}

impl _Range {
	pub fn as_raw__Range(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for _Range {}

impl core::RangeTrait for _Range {
	fn as_raw_Range(&self) -> *mut c_void { self.ptr }
}

impl crate::dnn::_RangeTrait for _Range {
	fn as_raw__Range(&self) -> *mut c_void { self.ptr }
}

impl _Range {
	pub fn new(r: &core::Range) -> Result<crate::dnn::_Range> {
		unsafe { sys::cv_dnn__Range__Range_const_RangeX(r.as_raw_Range()) }.into_result().map(|ptr| crate::dnn::_Range { ptr })
	}
	
	/// ## C++ default parameters
	/// * size_: 1
	pub fn new_1(start_: i32, size_: i32) -> Result<crate::dnn::_Range> {
		unsafe { sys::cv_dnn__Range__Range_int_int(start_, size_) }.into_result().map(|ptr| crate::dnn::_Range { ptr })
	}
	
}
pub use crate::manual::dnn::*;
