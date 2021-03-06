//! # Super Resolution
//!
//! The Super Resolution module contains a set of functions and classes that can be used to solve the
//! problem of resolution enhancement. There are a few methods implemented, most of them are described in
//! the papers [Farsiu03](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Farsiu03) and [Mitzel09](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Mitzel09) .
use crate::{mod_prelude::*, core, sys, types};
use crate::core::{_InputArrayTrait, _OutputArrayTrait};


///
/// ## C++ default parameters
/// * device_id: 0
pub fn create_frame_source_camera(device_id: i32) -> Result<types::PtrOfFrameSource> {
    unsafe { sys::cv_superres_createFrameSource_Camera_int(device_id) }.into_result().map(|ptr| types::PtrOfFrameSource { ptr })
}

pub fn create_frame_source_empty() -> Result<types::PtrOfFrameSource> {
    unsafe { sys::cv_superres_createFrameSource_Empty() }.into_result().map(|ptr| types::PtrOfFrameSource { ptr })
}

pub fn create_frame_source_video_cuda(file_name: &str) -> Result<types::PtrOfFrameSource> {
    string_arg!(file_name);
    unsafe { sys::cv_superres_createFrameSource_Video_CUDA_String(file_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfFrameSource { ptr })
}

pub fn create_frame_source_video(file_name: &str) -> Result<types::PtrOfFrameSource> {
    string_arg!(file_name);
    unsafe { sys::cv_superres_createFrameSource_Video_String(file_name.as_ptr()) }.into_result().map(|ptr| types::PtrOfFrameSource { ptr })
}

/// Create Bilateral TV-L1 Super Resolution.
///
/// This class implements Super Resolution algorithm described in the papers [Farsiu03](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Farsiu03) and
/// [Mitzel09](https://docs.opencv.org/3.4.9/d0/de3/citelist.html#CITEREF_Mitzel09) .
///
/// Here are important members of the class that control the algorithm, which you can set after
/// constructing the class instance:
///
/// *   **int scale** Scale factor.
/// *   **int iterations** Iteration count.
/// *   **double tau** Asymptotic value of steepest descent method.
/// *   **double lambda** Weight parameter to balance data term and smoothness term.
/// *   **double alpha** Parameter of spacial distribution in Bilateral-TV.
/// *   **int btvKernelSize** Kernel size of Bilateral-TV filter.
/// *   **int blurKernelSize** Gaussian blur kernel size.
/// *   **double blurSigma** Gaussian blur sigma.
/// *   **int temporalAreaRadius** Radius of the temporal search area.
/// *   **Ptr\<DenseOpticalFlowExt\> opticalFlow** Dense optical flow algorithm.
pub fn create_super_resolution_btvl1() -> Result<types::PtrOfSuperResolution> {
    unsafe { sys::cv_superres_createSuperResolution_BTVL1() }.into_result().map(|ptr| types::PtrOfSuperResolution { ptr })
}

pub fn create_super_resolution_btvl1_cuda() -> Result<types::PtrOfSuperResolution> {
    unsafe { sys::cv_superres_createSuperResolution_BTVL1_CUDA() }.into_result().map(|ptr| types::PtrOfSuperResolution { ptr })
}

// Generating impl for trait crate::superres::FrameSource
pub trait FrameSource {
    fn as_raw_FrameSource(&self) -> *mut c_void;
    fn next_frame(&mut self, frame: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(frame);
        unsafe { sys::cv_superres_FrameSource_nextFrame__OutputArray(self.as_raw_FrameSource(), frame.as_raw__OutputArray()) }.into_result()
    }
    
    fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_superres_FrameSource_reset(self.as_raw_FrameSource()) }.into_result()
    }
    
}

// Generating impl for trait crate::superres::SuperResolution
/// Base class for Super Resolution algorithms.
///
/// The class is only used to define the common interface for the whole family of Super Resolution
/// algorithms.
pub trait SuperResolution: core::AlgorithmTrait + crate::superres::FrameSource {
    fn as_raw_SuperResolution(&self) -> *mut c_void;
    /// Set input frame source for Super Resolution algorithm.
    ///
    /// ## Parameters
    /// * frameSource: Input frame source
    fn set_input(&mut self, frame_source: &types::PtrOfFrameSource) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setInput_PtrOfFrameSource(self.as_raw_SuperResolution(), frame_source.as_raw_PtrOfFrameSource()) }.into_result()
    }
    
    /// Process next frame from input and return output result.
    ///
    /// ## Parameters
    /// * frame: Output result
    fn next_frame(&mut self, frame: &mut dyn core::ToOutputArray) -> Result<()> {
        output_array_arg!(frame);
        unsafe { sys::cv_superres_SuperResolution_nextFrame__OutputArray(self.as_raw_SuperResolution(), frame.as_raw__OutputArray()) }.into_result()
    }
    
    fn reset(&mut self) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_reset(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// Clear all inner buffers.
    fn collect_garbage(&mut self) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_collectGarbage(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @see setScale
    fn get_scale(&self) -> Result<i32> {
        unsafe { sys::cv_superres_SuperResolution_getScale_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getScale @see getScale
    fn set_scale(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setScale_int(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setIterations
    fn get_iterations(&self) -> Result<i32> {
        unsafe { sys::cv_superres_SuperResolution_getIterations_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getIterations @see getIterations
    fn set_iterations(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setIterations_int(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setTau
    fn get_tau(&self) -> Result<f64> {
        unsafe { sys::cv_superres_SuperResolution_getTau_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getTau @see getTau
    fn set_tau(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setTau_double(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setLabmda
    fn get_labmda(&self) -> Result<f64> {
        unsafe { sys::cv_superres_SuperResolution_getLabmda_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getLabmda @see getLabmda
    fn set_labmda(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setLabmda_double(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setAlpha
    fn get_alpha(&self) -> Result<f64> {
        unsafe { sys::cv_superres_SuperResolution_getAlpha_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getAlpha @see getAlpha
    fn set_alpha(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setAlpha_double(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setKernelSize
    fn get_kernel_size(&self) -> Result<i32> {
        unsafe { sys::cv_superres_SuperResolution_getKernelSize_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getKernelSize @see getKernelSize
    fn set_kernel_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setKernelSize_int(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setBlurKernelSize
    fn get_blur_kernel_size(&self) -> Result<i32> {
        unsafe { sys::cv_superres_SuperResolution_getBlurKernelSize_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getBlurKernelSize @see getBlurKernelSize
    fn set_blur_kernel_size(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setBlurKernelSize_int(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setBlurSigma
    fn get_blur_sigma(&self) -> Result<f64> {
        unsafe { sys::cv_superres_SuperResolution_getBlurSigma_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getBlurSigma @see getBlurSigma
    fn set_blur_sigma(&mut self, val: f64) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setBlurSigma_double(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
    /// @see setTemporalAreaRadius
    fn get_temporal_area_radius(&self) -> Result<i32> {
        unsafe { sys::cv_superres_SuperResolution_getTemporalAreaRadius_const(self.as_raw_SuperResolution()) }.into_result()
    }
    
    /// @copybrief getTemporalAreaRadius @see getTemporalAreaRadius
    fn set_temporal_area_radius(&mut self, val: i32) -> Result<()> {
        unsafe { sys::cv_superres_SuperResolution_setTemporalAreaRadius_int(self.as_raw_SuperResolution(), val) }.into_result()
    }
    
}

