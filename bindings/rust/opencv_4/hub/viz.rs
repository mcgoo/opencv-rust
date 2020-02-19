//! # 3D Visualizer
//! 
//! This section describes 3D visualization window as well as classes and methods that are used to
//! interact with it.
//! 
//! 3D visualization window (see Viz3d) is used to display widgets (see Widget), and it provides several
//! methods to interact with scene and widgets.
//!    # Widget
//! 
//! In this section, the widget framework is explained. Widgets represent 2D or 3D objects, varying from
//! simple ones such as lines to complex ones such as point clouds and meshes.
//! 
//! Widgets are **implicitly shared**. Therefore, one can add a widget to the scene, and modify the
//! widget without re-adding the widget.
//! 
//! ```ignore
//! // Create a cloud widget
//! viz::WCloud cw(cloud, viz::Color::red());
//! // Display it in a window
//! myWindow.showWidget("CloudWidget1", cw);
//! // Modify it, and it will be modified in the window.
//! cw.setColor(viz::Color::yellow());
//! ```
//! 
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ColorTrait, super::MeshTrait, super::CameraTrait, super::KeyboardEventTrait, super::MouseEventTrait, super::WidgetTrait, super::Widget3DTrait, super::Widget2DTrait, super::WLineTrait, super::WPlaneTrait, super::WSphereTrait, super::WArrowTrait, super::WCircleTrait, super::WConeTrait, super::WCylinderTrait, super::WCubeTrait, super::WPolyLineTrait, super::WTextTrait, super::WText3DTrait, super::WImageOverlayTrait, super::WImage3DTrait, super::WCoordinateSystemTrait, super::WGridTrait, super::WCameraPositionTrait, super::WTrajectoryTrait, super::WTrajectoryFrustumsTrait, super::WTrajectorySpheresTrait, super::WCloudTrait, super::WPaintedCloudTrait, super::WCloudCollectionTrait, super::WCloudNormalsTrait, super::WMeshTrait, super::WWidgetMergerTrait, super::Viz3dTrait };
}

pub const AMBIENT: i32 = 7;
pub const FONT_SIZE: i32 = 3;
pub const IMMEDIATE_RENDERING: i32 = 5;
pub const KeyboardEvent_ALT: i32 = 1;
pub const KeyboardEvent_CTRL: i32 = 2;
pub const KeyboardEvent_NONE: i32 = 0;
pub const KeyboardEvent_SHIFT: i32 = 4;
pub const LIGHTING: i32 = 8;
pub const LINE_WIDTH: i32 = 2;
pub const Mesh_LOAD_AUTO: i32 = 0;
pub const Mesh_LOAD_OBJ: i32 = 2;
pub const Mesh_LOAD_PLY: i32 = 1;
pub const OPACITY: i32 = 1;
pub const POINT_SIZE: i32 = 0;
pub const REPRESENTATION: i32 = 4;
pub const REPRESENTATION_POINTS: i32 = 0;
pub const REPRESENTATION_SURFACE: i32 = 2;
pub const REPRESENTATION_WIREFRAME: i32 = 1;
pub const SHADING: i32 = 6;
pub const SHADING_FLAT: i32 = 0;
pub const SHADING_GOURAUD: i32 = 1;
pub const SHADING_PHONG: i32 = 2;
pub const WTrajectory_BOTH: i32 = 3;
pub const WTrajectory_FRAMES: i32 = 1;
pub const WTrajectory_PATH: i32 = 2;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyboardEvent_Action {
	KEY_UP = 0 as isize,
	KEY_DOWN = 1 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent_MouseButton {
	NoButton = 0 as isize,
	LeftButton = 1 as isize,
	MiddleButton = 2 as isize,
	RightButton = 3 as isize,
	VScroll = 4 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MouseEvent_Type {
	MouseMove = 1 as isize,
	MouseButtonPress = 2 as isize,
	MouseButtonRelease = 3 as isize,
	MouseScrollDown = 4 as isize,
	MouseScrollUp = 5 as isize,
	MouseDblClick = 6 as isize,
}

/// //////////////////////////////////////////////////////////////////////////
/// Widget rendering properties
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RenderingProperties {
	POINT_SIZE = 0 as isize,
	OPACITY = 1 as isize,
	LINE_WIDTH = 2 as isize,
	FONT_SIZE = 3 as isize,
	REPRESENTATION = 4 as isize,
	IMMEDIATE_RENDERING = 5 as isize,
	SHADING = 6 as isize,
	AMBIENT = 7 as isize,
	LIGHTING = 8 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RepresentationValues {
	REPRESENTATION_POINTS = 0 as isize,
	REPRESENTATION_WIREFRAME = 1 as isize,
	REPRESENTATION_SURFACE = 2 as isize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShadingValues {
	SHADING_FLAT = 0 as isize,
	SHADING_GOURAUD = 1 as isize,
	SHADING_PHONG = 2 as isize,
}

pub type Viz3d_Color = crate::viz::Color;
pub type Viz3d_KeyboardCallback = Option<Box<dyn FnMut(*mut c_void) -> () + Send + Sync + 'static>>;
pub type Viz3d_MouseCallback = Option<Box<dyn FnMut(*mut c_void) -> () + Send + Sync + 'static>>;
/// ////////////////////////////////////////////////////////////////////////////////////////////
/// Computing normals for mesh
/// ## Parameters
/// * mesh: Input mesh.
/// * normals: Normals at very point in the mesh of type CV_64FC3.
pub fn compute_normals(mesh: &crate::viz::Mesh, normals: &mut dyn core::ToOutputArray) -> Result<()> {
	output_array_arg!(normals);
	unsafe { sys::cv_viz_computeNormals_const_MeshX_const__OutputArrayX(mesh.as_raw_Mesh(), normals.as_raw__OutputArray()) }.into_result()
}

/// Retrieves a window by its name.
/// 
/// ## Parameters
/// * window_name: Name of the window that is to be retrieved.
/// 
/// This function returns a Viz3d object with the given name.
/// 
/// 
/// Note: If the window with that name already exists, that window is returned. Otherwise, new window is
/// created with the given name, and it is returned.
/// 
/// 
/// Note: Window names are automatically prefixed by "Viz - " if it is not done by the user.
///    ```ignore
///    /// window and window_2 are the same windows.
///    viz::Viz3d window   = viz::getWindowByName("myWindow");
///    viz::Viz3d window_2 = viz::getWindowByName("Viz - myWindow");
///    ```
/// 
pub fn get_window_by_name(window_name: &str) -> Result<crate::viz::Viz3d> {
	string_arg!(window_name);
	unsafe { sys::cv_viz_getWindowByName_const_StringX(window_name.as_ptr()) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
}

/// Displays image in specified window
/// 
/// ## C++ default parameters
/// * window_size: Size(-1,-1)
pub fn imshow(window_name: &str, image: &dyn core::ToInputArray, window_size: core::Size) -> Result<crate::viz::Viz3d> {
	string_arg!(window_name);
	input_array_arg!(image);
	unsafe { sys::cv_viz_imshow_const_StringX_const__InputArrayX_const_SizeX(window_name.as_ptr(), image.as_raw__InputArray(), &window_size) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
}

/// ## Parameters
/// * file: Filename with extension. Supported formats: PLY, XYZ, OBJ and STL.
/// * colors: Used by PLY and STL formats only.
/// * normals: Used by PLY, OBJ and STL formats only.
/// ## Returns
/// A mat containing the point coordinates with depth CV_32F or CV_64F and number of
///        channels 3 or 4 with only 1 row.
/// 
/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
pub fn read_cloud(file: &str, colors: &mut dyn core::ToOutputArray, normals: &mut dyn core::ToOutputArray) -> Result<core::Mat> {
	string_arg!(file);
	output_array_arg!(colors);
	output_array_arg!(normals);
	unsafe { sys::cv_viz_readCloud_const_StringX_const__OutputArrayX_const__OutputArrayX(file.as_ptr(), colors.as_raw__OutputArray(), normals.as_raw__OutputArray()) }.into_result().map(|ptr| core::Mat { ptr })
}

/// ////////////////////////////////////////////////////////////////////////////////////////////
/// Reads mesh. Only ply format is supported now and no texture load support
pub fn read_mesh(file: &str) -> Result<crate::viz::Mesh> {
	string_arg!(file);
	unsafe { sys::cv_viz_readMesh_const_StringX(file.as_ptr()) }.into_result().map(|ptr| crate::viz::Mesh { ptr })
}

/// takes vector<Affine3<T>> with T = float/dobule and loads poses from sequence of files
/// 
/// ## Parameters
/// * traj: Output array containing a lists of poses. It can be
///            - std::vector<cv::Affine3f>, std::vector<cv::Affine3d>
///            - cv::Mat
/// * files_format: Format specifier string for constructing filenames.
///                    The only placeholder in the string should support `int`.
/// * start: The initial counter for files_format. It must be greater than or equal to 0.
/// * end: The final  counter for files_format.
/// * tag: Name of the matrix in the file.
/// 
/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * end: INT_MAX
/// * tag: "pose"
pub fn read_trajectory(traj: &mut dyn core::ToOutputArray, files_format: &str, start: i32, end: i32, tag: &str) -> Result<()> {
	output_array_arg!(traj);
	string_arg!(files_format);
	string_arg!(tag);
	unsafe { sys::cv_viz_readTrajectory_const__OutputArrayX_const_StringX_int_int_const_StringX(traj.as_raw__OutputArray(), files_format.as_ptr(), start, end, tag.as_ptr()) }.into_result()
}

/// Unregisters all Viz windows from internal database. After it 'getWindowByName()' will create new windows instead of getting existing from the database.
pub fn unregister_all_windows() -> Result<()> {
	unsafe { sys::cv_viz_unregisterAllWindows() }.into_result()
}

/// ## Parameters
/// * file: Filename with extension. Supported formats: PLY, XYZ and OBJ.
/// * cloud: Supported depths: CV_32F and CV_64F. Supported channels: 3 and 4.
/// * colors: Used by PLY format only. Supported depth: CV_8U. Supported channels: 1, 3 and 4.
/// * normals: Used by PLY and OBJ format only. Supported depths: CV_32F and CV_64F.
///                Supported channels: 3 and 4.
/// * binary: Used only for PLY format.
/// 
/// ## C++ default parameters
/// * colors: noArray()
/// * normals: noArray()
/// * binary: false
pub fn write_cloud(file: &str, cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray, binary: bool) -> Result<()> {
	string_arg!(file);
	input_array_arg!(cloud);
	input_array_arg!(colors);
	input_array_arg!(normals);
	unsafe { sys::cv_viz_writeCloud_const_StringX_const__InputArrayX_const__InputArrayX_const__InputArrayX_bool(file.as_ptr(), cloud.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray(), binary) }.into_result()
}

/// takes vector<Affine3<T>> with T = float/dobule and writes to a sequence of files with given filename format
/// ## Parameters
/// * traj: Trajectory containing a list of poses. It can be
///          - std::vector<cv::Mat>, each cv::Mat is of type CV_32F16 or CV_64FC16
///          - std::vector<cv::Affine3f>, std::vector<cv::Affine3d>
///          - cv::Mat of type CV_32FC16 OR CV_64F16
/// * files_format: Format specifier string for constructing filenames.
///                    The only placeholder in the string should support `int`.
/// * start: The initial counter for files_format.
/// * tag: Name of the matrix in the file.
/// 
/// ## C++ default parameters
/// * files_format: "pose%05d.xml"
/// * start: 0
/// * tag: "pose"
pub fn write_trajectory(traj: &dyn core::ToInputArray, files_format: &str, start: i32, tag: &str) -> Result<()> {
	input_array_arg!(traj);
	string_arg!(files_format);
	string_arg!(tag);
	unsafe { sys::cv_viz_writeTrajectory_const__InputArrayX_const_StringX_int_const_StringX(traj.as_raw__InputArray(), files_format.as_ptr(), start, tag.as_ptr()) }.into_result()
}

/// This class wraps intrinsic parameters of a camera.
/// 
/// It provides several constructors that can extract the intrinsic parameters from field of
/// view, intrinsic matrix and projection matrix. :
pub trait CameraTrait {
	fn as_raw_Camera(&self) -> *mut c_void;
	fn get_clip(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getClip_const(self.as_raw_Camera()) }.into_result()
	}
	
	fn set_clip(&mut self, clip: core::Vec2d) -> Result<()> {
		unsafe { sys::cv_viz_Camera_setClip_const_Vec2dX(self.as_raw_Camera(), &clip) }.into_result()
	}
	
	fn get_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_viz_Camera_getWindowSize_const(self.as_raw_Camera()) }.into_result()
	}
	
	fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
		unsafe { sys::cv_viz_Camera_setWindowSize_const_SizeX(self.as_raw_Camera(), &window_size) }.into_result()
	}
	
	fn get_fov(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getFov_const(self.as_raw_Camera()) }.into_result()
	}
	
	fn set_fov(&mut self, fov: core::Vec2d) -> Result<()> {
		unsafe { sys::cv_viz_Camera_setFov_const_Vec2dX(self.as_raw_Camera(), &fov) }.into_result()
	}
	
	fn get_principal_point(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getPrincipalPoint_const(self.as_raw_Camera()) }.into_result()
	}
	
	fn get_focal_length(&self) -> Result<core::Vec2d> {
		unsafe { sys::cv_viz_Camera_getFocalLength_const(self.as_raw_Camera()) }.into_result()
	}
	
}

/// This class wraps intrinsic parameters of a camera.
/// 
/// It provides several constructors that can extract the intrinsic parameters from field of
/// view, intrinsic matrix and projection matrix. :
pub struct Camera {
	pub(crate) ptr: *mut c_void
}

impl Drop for Camera {
	fn drop(&mut self) {
		extern "C" { fn cv_Camera_delete(instance: *mut c_void); }
		unsafe { cv_Camera_delete(self.as_raw_Camera()) };
	}
}

impl Camera {
	pub fn as_raw_Camera(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Camera {}

impl crate::viz::CameraTrait for Camera {
	fn as_raw_Camera(&self) -> *mut c_void { self.ptr }
}

impl Camera {
	/// Constructs a Camera.
	/// 
	/// ## Parameters
	/// * fx: Horizontal focal length.
	/// * fy: Vertical focal length.
	/// * cx: x coordinate of the principal point.
	/// * cy: y coordinate of the principal point.
	/// * window_size: Size of the window. This together with focal length and principal
	/// point determines the field of view.
	pub fn new(fx: f64, fy: f64, cx: f64, cy: f64, window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_Camera_double_double_double_double_const_SizeX(fx, fy, cx, cy, &window_size) }.into_result().map(|ptr| crate::viz::Camera { ptr })
	}
	
	/// Constructs a Camera.
	/// 
	/// ## Parameters
	/// * fx: Horizontal focal length.
	/// * fy: Vertical focal length.
	/// * cx: x coordinate of the principal point.
	/// * cy: y coordinate of the principal point.
	/// * window_size: Size of the window. This together with focal length and principal
	/// point determines the field of view.
	/// 
	/// ## Overloaded parameters
	/// 
	/// * fov: Field of view (horizontal, vertical)
	/// * window_size: Size of the window. Principal point is at the center of the window
	///            by default.
	pub fn new_1(fov: core::Vec2d, window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_Camera_const_Vec2dX_const_SizeX(&fov, &window_size) }.into_result().map(|ptr| crate::viz::Camera { ptr })
	}
	
	/// Creates a Kinect Camera with
	///   - fx = fy = 525
	///   - cx = 320
	///   - cy = 240
	/// 
	/// ## Parameters
	/// * window_size: Size of the window. This together with intrinsic matrix of a Kinect Camera
	/// determines the field of view.
	pub fn kinect_camera(window_size: core::Size) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Camera_KinectCamera_const_SizeX(&window_size) }.into_result().map(|ptr| crate::viz::Camera { ptr })
	}
	
}

/// This class represents color in BGR order.
pub trait ColorTrait {
	fn as_raw_Color(&self) -> *mut c_void;
	fn to_vec3b(&self) -> Result<core::Vec3b> {
		unsafe { sys::cv_viz_Color_operator_cv_Vec3b_const(self.as_raw_Color()) }.into_result()
	}
	
}

/// This class represents color in BGR order.
pub struct Color {
	pub(crate) ptr: *mut c_void
}

impl Drop for Color {
	fn drop(&mut self) {
		extern "C" { fn cv_Color_delete(instance: *mut c_void); }
		unsafe { cv_Color_delete(self.as_raw_Color()) };
	}
}

impl Color {
	pub fn as_raw_Color(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Color {}

impl crate::viz::ColorTrait for Color {
	fn as_raw_Color(&self) -> *mut c_void { self.ptr }
}

impl Color {
	/// ///////////////////////////////////////////////////////////////////////////////////////////////////
	/// cv::viz::Color
	pub fn default() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	/// The three channels will have the same value equal to gray.
	pub fn new(gray: f64) -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color_double(gray) }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn new_1(blue: f64, green: f64, red: f64) -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color_double_double_double(blue, green, red) }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn new_2(color: core::Scalar) -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_Color_const_ScalarX(&color) }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn black() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_black() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn blue() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_blue() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn green() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_green() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn red() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_red() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn cyan() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_cyan() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn yellow() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_yellow() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn magenta() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_magenta() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn white() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_white() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn gray() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_gray() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn silver() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_silver() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn mlab() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_mlab() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn navy() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_navy() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn maroon() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_maroon() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn teal() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_teal() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn olive() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_olive() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn purple() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_purple() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn azure() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_azure() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn chartreuse() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_chartreuse() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn rose() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_rose() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn lime() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_lime() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn gold() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_gold() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn orange() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_orange() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn orange_red() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_orange_red() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn indigo() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_indigo() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn brown() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_brown() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn apricot() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_apricot() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn pink() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_pink() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn raspberry() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_raspberry() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn cherry() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_cherry() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn violet() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_violet() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn amethyst() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_amethyst() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn bluberry() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_bluberry() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn celestial_blue() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_celestial_blue() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn turquoise() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_turquoise() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
	pub fn not_set() -> Result<crate::viz::Color> {
		unsafe { sys::cv_viz_Color_not_set() }.into_result().map(|ptr| crate::viz::Color { ptr })
	}
	
}

/// This class represents a keyboard event.
pub trait KeyboardEventTrait {
	fn as_raw_KeyboardEvent(&self) -> *mut c_void;
	fn action(&self) -> crate::viz::KeyboardEvent_Action {
		unsafe { sys::cv_viz_KeyboardEvent_action_const(self.as_raw_KeyboardEvent()) }.into_result().expect("Infallible function failed: action")
	}
	
	fn set_action(&mut self, val: crate::viz::KeyboardEvent_Action) -> () {
		unsafe { sys::cv_viz_KeyboardEvent_setAction_Action(self.as_raw_KeyboardEvent(), val) }.into_result().expect("Infallible function failed: set_action")
	}
	
	fn symbol(&self) -> String {
		unsafe { sys::cv_viz_KeyboardEvent_symbol_const(self.as_raw_KeyboardEvent()) }.into_result().map(crate::templ::receive_string).expect("Infallible function failed: symbol")
	}
	
	fn set_symbol(&mut self, val: &str) -> () {
		string_arg_infallible!(val);
		unsafe { sys::cv_viz_KeyboardEvent_setSymbol_String(self.as_raw_KeyboardEvent(), val.as_ptr() as _) }.into_result().expect("Infallible function failed: set_symbol")
	}
	
	fn code(&self) -> u8 {
		unsafe { sys::cv_viz_KeyboardEvent_code_const(self.as_raw_KeyboardEvent()) }.into_result().expect("Infallible function failed: code")
	}
	
	fn set_code(&mut self, val: u8) -> () {
		unsafe { sys::cv_viz_KeyboardEvent_setCode_unsigned_char(self.as_raw_KeyboardEvent(), val) }.into_result().expect("Infallible function failed: set_code")
	}
	
	fn modifiers(&self) -> i32 {
		unsafe { sys::cv_viz_KeyboardEvent_modifiers_const(self.as_raw_KeyboardEvent()) }.into_result().expect("Infallible function failed: modifiers")
	}
	
	fn set_modifiers(&mut self, val: i32) -> () {
		unsafe { sys::cv_viz_KeyboardEvent_setModifiers_int(self.as_raw_KeyboardEvent(), val) }.into_result().expect("Infallible function failed: set_modifiers")
	}
	
}

/// This class represents a keyboard event.
pub struct KeyboardEvent {
	pub(crate) ptr: *mut c_void
}

impl Drop for KeyboardEvent {
	fn drop(&mut self) {
		extern "C" { fn cv_KeyboardEvent_delete(instance: *mut c_void); }
		unsafe { cv_KeyboardEvent_delete(self.as_raw_KeyboardEvent()) };
	}
}

impl KeyboardEvent {
	pub fn as_raw_KeyboardEvent(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for KeyboardEvent {}

impl crate::viz::KeyboardEventTrait for KeyboardEvent {
	fn as_raw_KeyboardEvent(&self) -> *mut c_void { self.ptr }
}

impl KeyboardEvent {
	/// Constructs a KeyboardEvent.
	/// 
	/// ## Parameters
	/// * action: Signals if key is pressed or released.
	/// * symbol: Name of the key.
	/// * code: Code of the key.
	/// * modifiers: Signals if alt, ctrl or shift are pressed or their combination.
	pub fn new(action: crate::viz::KeyboardEvent_Action, symbol: &str, code: u8, modifiers: i32) -> Result<crate::viz::KeyboardEvent> {
		string_arg!(symbol);
		unsafe { sys::cv_viz_KeyboardEvent_KeyboardEvent_Action_const_StringX_unsigned_char_int(action, symbol.as_ptr(), code, modifiers) }.into_result().map(|ptr| crate::viz::KeyboardEvent { ptr })
	}
	
}

/// This class wraps mesh attributes, and it can load a mesh from a ply file. :
pub trait MeshTrait {
	fn as_raw_Mesh(&self) -> *mut c_void;
	/// point coordinates of type CV_32FC3 or CV_64FC3 with only 1 row
	fn cloud(&mut self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_cloud(self.as_raw_Mesh()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: cloud")
	}
	
	/// point coordinates of type CV_32FC3 or CV_64FC3 with only 1 row
	fn set_cloud(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_viz_Mesh_setCloud_Mat(self.as_raw_Mesh(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_cloud")
	}
	
	/// point color of type CV_8UC3 or CV_8UC4 with only 1 row
	fn colors(&mut self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_colors(self.as_raw_Mesh()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: colors")
	}
	
	/// point color of type CV_8UC3 or CV_8UC4 with only 1 row
	fn set_colors(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_viz_Mesh_setColors_Mat(self.as_raw_Mesh(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_colors")
	}
	
	/// point normals of type CV_32FC3, CV_32FC4, CV_64FC3 or CV_64FC4 with only 1 row
	fn normals(&mut self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_normals(self.as_raw_Mesh()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: normals")
	}
	
	/// point normals of type CV_32FC3, CV_32FC4, CV_64FC3 or CV_64FC4 with only 1 row
	fn set_normals(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_viz_Mesh_setNormals_Mat(self.as_raw_Mesh(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_normals")
	}
	
	/// CV_32SC1 with only 1 row
	fn polygons(&mut self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_polygons(self.as_raw_Mesh()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: polygons")
	}
	
	/// CV_32SC1 with only 1 row
	fn set_polygons(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_viz_Mesh_setPolygons_Mat(self.as_raw_Mesh(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_polygons")
	}
	
	fn texture(&mut self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_texture(self.as_raw_Mesh()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: texture")
	}
	
	fn set_texture(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_viz_Mesh_setTexture_Mat(self.as_raw_Mesh(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_texture")
	}
	
	/// CV_32FC2 or CV_64FC2 with only 1 row
	fn tcoords(&mut self) -> core::Mat {
		unsafe { sys::cv_viz_Mesh_tcoords(self.as_raw_Mesh()) }.into_result().map(|ptr| core::Mat { ptr }).expect("Infallible function failed: tcoords")
	}
	
	/// CV_32FC2 or CV_64FC2 with only 1 row
	fn set_tcoords(&mut self, val: core::Mat) -> () {
		unsafe { sys::cv_viz_Mesh_setTcoords_Mat(self.as_raw_Mesh(), val.as_raw_Mat()) }.into_result().expect("Infallible function failed: set_tcoords")
	}
	
}

/// This class wraps mesh attributes, and it can load a mesh from a ply file. :
pub struct Mesh {
	pub(crate) ptr: *mut c_void
}

impl Drop for Mesh {
	fn drop(&mut self) {
		extern "C" { fn cv_Mesh_delete(instance: *mut c_void); }
		unsafe { cv_Mesh_delete(self.as_raw_Mesh()) };
	}
}

impl Mesh {
	pub fn as_raw_Mesh(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Mesh {}

impl crate::viz::MeshTrait for Mesh {
	fn as_raw_Mesh(&self) -> *mut c_void { self.ptr }
}

impl Mesh {
	/// Loads a mesh from a ply or a obj file.
	/// 
	/// ## Parameters
	/// * file: File name
	/// * type: File type (for now only PLY and OBJ are supported)
	/// 
	/// **File type** can be one of the following:
	/// *   **LOAD_PLY**
	/// *   **LOAD_OBJ**
	/// 
	/// ## C++ default parameters
	/// * typ: LOAD_PLY
	pub fn load(file: &str, typ: i32) -> Result<crate::viz::Mesh> {
		string_arg!(file);
		unsafe { sys::cv_viz_Mesh_load_const_StringX_int(file.as_ptr(), typ) }.into_result().map(|ptr| crate::viz::Mesh { ptr })
	}
	
}

/// This class represents a mouse event.
pub trait MouseEventTrait {
	fn as_raw_MouseEvent(&self) -> *mut c_void;
	fn typ(&self) -> crate::viz::MouseEvent_Type {
		unsafe { sys::cv_viz_MouseEvent_type_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: typ")
	}
	
	fn set_type(&mut self, val: crate::viz::MouseEvent_Type) -> () {
		unsafe { sys::cv_viz_MouseEvent_setType_Type(self.as_raw_MouseEvent(), val) }.into_result().expect("Infallible function failed: set_type")
	}
	
	fn button(&self) -> crate::viz::MouseEvent_MouseButton {
		unsafe { sys::cv_viz_MouseEvent_button_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: button")
	}
	
	fn set_button(&mut self, val: crate::viz::MouseEvent_MouseButton) -> () {
		unsafe { sys::cv_viz_MouseEvent_setButton_MouseButton(self.as_raw_MouseEvent(), val) }.into_result().expect("Infallible function failed: set_button")
	}
	
	fn pointer(&self) -> core::Point {
		unsafe { sys::cv_viz_MouseEvent_pointer_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: pointer")
	}
	
	fn set_pointer(&mut self, val: core::Point) -> () {
		unsafe { sys::cv_viz_MouseEvent_setPointer_Point(self.as_raw_MouseEvent(), &val) }.into_result().expect("Infallible function failed: set_pointer")
	}
	
	fn modifiers(&self) -> i32 {
		unsafe { sys::cv_viz_MouseEvent_modifiers_const(self.as_raw_MouseEvent()) }.into_result().expect("Infallible function failed: modifiers")
	}
	
	fn set_modifiers(&mut self, val: i32) -> () {
		unsafe { sys::cv_viz_MouseEvent_setModifiers_int(self.as_raw_MouseEvent(), val) }.into_result().expect("Infallible function failed: set_modifiers")
	}
	
}

/// This class represents a mouse event.
pub struct MouseEvent {
	pub(crate) ptr: *mut c_void
}

impl Drop for MouseEvent {
	fn drop(&mut self) {
		extern "C" { fn cv_MouseEvent_delete(instance: *mut c_void); }
		unsafe { cv_MouseEvent_delete(self.as_raw_MouseEvent()) };
	}
}

impl MouseEvent {
	pub fn as_raw_MouseEvent(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for MouseEvent {}

impl crate::viz::MouseEventTrait for MouseEvent {
	fn as_raw_MouseEvent(&self) -> *mut c_void { self.ptr }
}

impl MouseEvent {
	/// Constructs a MouseEvent.
	/// 
	/// ## Parameters
	/// * type: Type of the event. This can be **MouseMove**, **MouseButtonPress**,
	/// **MouseButtonRelease**, **MouseScrollDown**, **MouseScrollUp**, **MouseDblClick**.
	/// * button: Mouse button. This can be **NoButton**, **LeftButton**, **MiddleButton**,
	/// **RightButton**, **VScroll**.
	/// * pointer: Position of the event.
	/// * modifiers: Signals if alt, ctrl or shift are pressed or their combination.
	pub fn new(typ: crate::viz::MouseEvent_Type, button: crate::viz::MouseEvent_MouseButton, pointer: core::Point, modifiers: i32) -> Result<crate::viz::MouseEvent> {
		unsafe { sys::cv_viz_MouseEvent_MouseEvent_const_TypeX_const_MouseButtonX_const_PointX_int(&typ, &button, &pointer, modifiers) }.into_result().map(|ptr| crate::viz::MouseEvent { ptr })
	}
	
}

/// The Viz3d class represents a 3D visualizer window. This class is implicitly shared.
pub trait Viz3dTrait {
	fn as_raw_Viz3d(&self) -> *mut c_void;
	/// Removes a widget from the window.
	/// 
	/// ## Parameters
	/// * id: The id of the widget that will be removed.
	fn remove_widget(&mut self, id: &str) -> Result<()> {
		string_arg!(id);
		unsafe { sys::cv_viz_Viz3d_removeWidget_const_StringX(self.as_raw_Viz3d(), id.as_ptr()) }.into_result()
	}
	
	/// Retrieves a widget from the window.
	/// 
	/// A widget is implicitly shared; that is, if the returned widget is modified, the changes
	/// will be immediately visible in the window.
	/// 
	/// ## Parameters
	/// * id: The id of the widget that will be returned.
	fn get_widget(&self, id: &str) -> Result<crate::viz::Widget> {
		string_arg!(id);
		unsafe { sys::cv_viz_Viz3d_getWidget_const_const_StringX(self.as_raw_Viz3d(), id.as_ptr()) }.into_result().map(|ptr| crate::viz::Widget { ptr })
	}
	
	/// Removes all widgets from the window.
	fn remove_all_widgets(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_removeAllWidgets(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Removed all widgets and displays image scaled to whole window area.
	/// 
	/// ## Parameters
	/// * image: Image to be displayed.
	/// * window_size: Size of Viz3d window. Default value means no change.
	/// 
	/// ## C++ default parameters
	/// * window_size: Size(-1,-1)
	fn show_image(&mut self, image: &dyn core::ToInputArray, window_size: core::Size) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_Viz3d_showImage_const__InputArrayX_const_SizeX(self.as_raw_Viz3d(), image.as_raw__InputArray(), &window_size) }.into_result()
	}
	
	/// Sets the intrinsic parameters of the viewer using Camera.
	/// 
	/// ## Parameters
	/// * camera: Camera object wrapping intrinsic parameters.
	fn set_camera(&mut self, camera: &crate::viz::Camera) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setCamera_const_CameraX(self.as_raw_Viz3d(), camera.as_raw_Camera()) }.into_result()
	}
	
	/// Returns a camera object that contains intrinsic parameters of the current viewer.
	fn get_camera(&self) -> Result<crate::viz::Camera> {
		unsafe { sys::cv_viz_Viz3d_getCamera_const(self.as_raw_Viz3d()) }.into_result().map(|ptr| crate::viz::Camera { ptr })
	}
	
	/// Resets camera viewpoint to a 3D widget in the scene.
	/// 
	/// ## Parameters
	/// * id: Id of a 3D widget.
	fn reset_camera_viewpoint(&mut self, id: &str) -> Result<()> {
		string_arg!(id);
		unsafe { sys::cv_viz_Viz3d_resetCameraViewpoint_const_StringX(self.as_raw_Viz3d(), id.as_ptr()) }.into_result()
	}
	
	/// Resets camera.
	fn reset_camera(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_resetCamera(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Transforms a point in world coordinate system to window coordinate system.
	/// 
	/// ## Parameters
	/// * pt: Point in world coordinate system.
	/// * window_coord: Output point in window coordinate system.
	fn convert_to_window_coordinates(&mut self, pt: core::Point3d, window_coord: &mut core::Point3d) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_convertToWindowCoordinates_const_Point3dX_Point3dX(self.as_raw_Viz3d(), &pt, window_coord) }.into_result()
	}
	
	/// Transforms a point in window coordinate system to a 3D ray in world coordinate system.
	/// 
	/// ## Parameters
	/// * window_coord: Point in window coordinate system. @param origin Output origin of the ray.
	/// * direction: Output direction of the ray.
	fn conver_to3_d_ray(&mut self, window_coord: core::Point3d, origin: &mut core::Point3d, direction: &mut core::Vec3d) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_converTo3DRay_const_Point3dX_Point3dX_Vec3dX(self.as_raw_Viz3d(), &window_coord, origin, direction) }.into_result()
	}
	
	/// Returns the current size of the window.
	fn get_window_size(&self) -> Result<core::Size> {
		unsafe { sys::cv_viz_Viz3d_getWindowSize_const(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Sets the size of the window.
	/// 
	/// ## Parameters
	/// * window_size: New size of the window.
	fn set_window_size(&mut self, window_size: core::Size) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setWindowSize_const_SizeX(self.as_raw_Viz3d(), &window_size) }.into_result()
	}
	
	/// Returns the name of the window which has been set in the constructor.
	/// `Viz - ` is prepended to the name if necessary.
	fn get_window_name(&self) -> Result<String> {
		unsafe { sys::cv_viz_Viz3d_getWindowName_const(self.as_raw_Viz3d()) }.into_result().map(crate::templ::receive_string)
	}
	
	/// Returns the Mat screenshot of the current scene.
	fn get_screenshot(&self) -> Result<core::Mat> {
		unsafe { sys::cv_viz_Viz3d_getScreenshot_const(self.as_raw_Viz3d()) }.into_result().map(|ptr| core::Mat { ptr })
	}
	
	/// Saves screenshot of the current scene.
	/// 
	/// ## Parameters
	/// * file: Name of the file.
	fn save_screenshot(&mut self, file: &str) -> Result<()> {
		string_arg!(file);
		unsafe { sys::cv_viz_Viz3d_saveScreenshot_const_StringX(self.as_raw_Viz3d(), file.as_ptr()) }.into_result()
	}
	
	/// Sets the position of the window in the screen.
	/// 
	/// ## Parameters
	/// * window_position: coordinates of the window
	fn set_window_position(&mut self, window_position: core::Point) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setWindowPosition_const_PointX(self.as_raw_Viz3d(), &window_position) }.into_result()
	}
	
	/// Sets or unsets full-screen rendering mode.
	/// 
	/// ## Parameters
	/// * mode: If true, window will use full-screen mode.
	/// 
	/// ## C++ default parameters
	/// * mode: true
	fn set_full_screen(&mut self, mode: bool) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setFullScreen_bool(self.as_raw_Viz3d(), mode) }.into_result()
	}
	
	/// Sets background color.
	/// 
	/// ## C++ default parameters
	/// * color: Color::black()
	/// * color2: Color::not_set()
	fn set_background_color(&mut self, color: &crate::viz::Viz3d_Color, color2: &crate::viz::Viz3d_Color) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setBackgroundColor_const_ColorX_const_ColorX(self.as_raw_Viz3d(), color.as_raw_Color(), color2.as_raw_Color()) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * image: noArray()
	fn set_background_texture(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_Viz3d_setBackgroundTexture_const__InputArrayX(self.as_raw_Viz3d(), image.as_raw__InputArray()) }.into_result()
	}
	
	fn set_background_mesh_lab(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setBackgroundMeshLab(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// The window renders and starts the event loop.
	fn spin(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_spin(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Starts the event loop for a given time.
	/// 
	/// ## Parameters
	/// * time: Amount of time in milliseconds for the event loop to keep running.
	/// * force_redraw: If true, window renders.
	/// 
	/// ## C++ default parameters
	/// * time: 1
	/// * force_redraw: false
	fn spin_once(&mut self, time: i32, force_redraw: bool) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_spinOnce_int_bool(self.as_raw_Viz3d(), time, force_redraw) }.into_result()
	}
	
	/// Create a window in memory instead of on the screen.
	fn set_off_screen_rendering(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setOffScreenRendering(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Remove all lights from the current scene.
	fn remove_all_lights(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_removeAllLights(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Add a light in the scene.
	/// 
	/// ## Parameters
	/// * position: The position of the light.
	/// * focalPoint: The point at which the light is shining
	/// * color: The color of the light
	/// * diffuseColor: The diffuse color of the light
	/// * ambientColor: The ambient color of the light
	/// * specularColor: The specular color of the light
	/// 
	/// ## C++ default parameters
	/// * focal_point: Vec3d(0,0,0)
	/// * color: Color::white()
	/// * diffuse_color: Color::white()
	/// * ambient_color: Color::black()
	/// * specular_color: Color::white()
	fn add_light(&mut self, position: core::Vec3d, focal_point: core::Vec3d, color: &crate::viz::Viz3d_Color, diffuse_color: &crate::viz::Viz3d_Color, ambient_color: &crate::viz::Viz3d_Color, specular_color: &crate::viz::Viz3d_Color) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_addLight_const_Vec3dX_const_Vec3dX_const_ColorX_const_ColorX_const_ColorX_const_ColorX(self.as_raw_Viz3d(), &position, &focal_point, color.as_raw_Color(), diffuse_color.as_raw_Color(), ambient_color.as_raw_Color(), specular_color.as_raw_Color()) }.into_result()
	}
	
	/// Returns whether the event loop has been stopped.
	fn was_stopped(&self) -> Result<bool> {
		unsafe { sys::cv_viz_Viz3d_wasStopped_const(self.as_raw_Viz3d()) }.into_result()
	}
	
	fn close(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_close(self.as_raw_Viz3d()) }.into_result()
	}
	
	/// Sets keyboard handler.
	/// 
	/// ## Parameters
	/// * callback: Keyboard callback (void (\*KeyboardCallbackFunction(const
	/// KeyboardEvent&, void\*)).
	/// * cookie: The optional parameter passed to the callback.
	/// 
	/// ## C++ default parameters
	/// * cookie: 0
	fn register_keyboard_callback(&mut self, callback: crate::viz::Viz3d_KeyboardCallback) -> Result<()> {
		callback_arg!(callback_trampoline(unnamed: *mut c_void, unnamed_1: *mut c_void) -> () => unnamed_1 in callbacks => callback(unnamed: *mut c_void) -> ());
		userdata_arg!(cookie in callbacks => callback);
		unsafe { sys::cv_viz_Viz3d_registerKeyboardCallback_KeyboardCallback_voidX(self.as_raw_Viz3d(), callback_trampoline, cookie) }.into_result()
	}
	
	/// Sets mouse handler.
	/// 
	/// ## Parameters
	/// * callback: Mouse callback (void (\*MouseCallback)(const MouseEvent&, void\*)).
	/// * cookie: The optional parameter passed to the callback.
	/// 
	/// ## C++ default parameters
	/// * cookie: 0
	fn register_mouse_callback(&mut self, callback: crate::viz::Viz3d_MouseCallback) -> Result<()> {
		callback_arg!(callback_trampoline(unnamed: *mut c_void, unnamed_1: *mut c_void) -> () => unnamed_1 in callbacks => callback(unnamed: *mut c_void) -> ());
		userdata_arg!(cookie in callbacks => callback);
		unsafe { sys::cv_viz_Viz3d_registerMouseCallback_MouseCallback_voidX(self.as_raw_Viz3d(), callback_trampoline, cookie) }.into_result()
	}
	
	/// Sets rendering property of a widget.
	/// 
	/// ## Parameters
	/// * id: Id of the widget.
	/// * property: Property that will be modified.
	/// * value: The new value of the property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// IMMEDIATE_RENDERING:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	fn set_rendering_property(&mut self, id: &str, property: i32, value: f64) -> Result<()> {
		string_arg!(id);
		unsafe { sys::cv_viz_Viz3d_setRenderingProperty_const_StringX_int_double(self.as_raw_Viz3d(), id.as_ptr(), property, value) }.into_result()
	}
	
	/// Returns rendering property of a widget.
	/// 
	/// ## Parameters
	/// * id: Id of the widget.
	/// * property: Property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// IMMEDIATE_RENDERING:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	fn get_rendering_property(&mut self, id: &str, property: i32) -> Result<f64> {
		string_arg!(id);
		unsafe { sys::cv_viz_Viz3d_getRenderingProperty_const_StringX_int(self.as_raw_Viz3d(), id.as_ptr(), property) }.into_result()
	}
	
	/// Sets geometry representation of the widgets to surface, wireframe or points.
	/// 
	/// ## Parameters
	/// * representation: Geometry representation which can be one of the following:
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	fn set_representation(&mut self, representation: i32) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setRepresentation_int(self.as_raw_Viz3d(), representation) }.into_result()
	}
	
	/// ## C++ default parameters
	/// * enabled: false
	fn set_global_warnings(&mut self, enabled: bool) -> Result<()> {
		unsafe { sys::cv_viz_Viz3d_setGlobalWarnings_bool(self.as_raw_Viz3d(), enabled) }.into_result()
	}
	
}

/// The Viz3d class represents a 3D visualizer window. This class is implicitly shared.
pub struct Viz3d {
	pub(crate) ptr: *mut c_void
}

impl Drop for Viz3d {
	fn drop(&mut self) {
		extern "C" { fn cv_Viz3d_delete(instance: *mut c_void); }
		unsafe { cv_Viz3d_delete(self.as_raw_Viz3d()) };
	}
}

impl Viz3d {
	pub fn as_raw_Viz3d(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Viz3d {}

impl crate::viz::Viz3dTrait for Viz3d {
	fn as_raw_Viz3d(&self) -> *mut c_void { self.ptr }
}

impl Viz3d {
	/// The constructors.
	/// 
	/// ## Parameters
	/// * window_name: Name of the window.
	/// 
	/// ## C++ default parameters
	/// * window_name: String()
	pub fn new(window_name: &str) -> Result<crate::viz::Viz3d> {
		string_arg!(window_name);
		unsafe { sys::cv_viz_Viz3d_Viz3d_const_StringX(window_name.as_ptr()) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
	}
	
	pub fn copy(unnamed: &crate::viz::Viz3d) -> Result<crate::viz::Viz3d> {
		unsafe { sys::cv_viz_Viz3d_Viz3d_const_Viz3dX(unnamed.as_raw_Viz3d()) }.into_result().map(|ptr| crate::viz::Viz3d { ptr })
	}
	
}

/// This 3D Widget defines an arrow.
pub trait WArrowTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WArrow(&self) -> *mut c_void;
}

/// This 3D Widget defines an arrow.
pub struct WArrow {
	pub(crate) ptr: *mut c_void
}

impl Drop for WArrow {
	fn drop(&mut self) {
		extern "C" { fn cv_WArrow_delete(instance: *mut c_void); }
		unsafe { cv_WArrow_delete(self.as_raw_WArrow()) };
	}
}

impl WArrow {
	pub fn as_raw_WArrow(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WArrow {}

impl crate::viz::WArrowTrait for WArrow {
	fn as_raw_WArrow(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WArrow {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WArrow {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WArrow {
	/// Constructs an WArrow.
	/// 
	/// ## Parameters
	/// * pt1: Start point of the arrow.
	/// * pt2: End point of the arrow.
	/// * thickness: Thickness of the arrow. Thickness of arrow head is also adjusted
	/// accordingly.
	/// * color: Color of the arrow.
	/// 
	/// Arrow head is located at the end point of the arrow.
	/// 
	/// ## C++ default parameters
	/// * thickness: 0.03
	/// * color: Color::white()
	pub fn new(pt1: core::Point3d, pt2: core::Point3d, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WArrow> {
		unsafe { sys::cv_viz_WArrow_WArrow_const_Point3dX_const_Point3dX_double_const_ColorX(&pt1, &pt2, thickness, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WArrow { ptr })
	}
	
}

/// This 3D Widget represents camera position in a scene by its axes or viewing frustum. :
pub trait WCameraPositionTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCameraPosition(&self) -> *mut c_void;
}

/// This 3D Widget represents camera position in a scene by its axes or viewing frustum. :
pub struct WCameraPosition {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCameraPosition {
	fn drop(&mut self) {
		extern "C" { fn cv_WCameraPosition_delete(instance: *mut c_void); }
		unsafe { cv_WCameraPosition_delete(self.as_raw_WCameraPosition()) };
	}
}

impl WCameraPosition {
	pub fn as_raw_WCameraPosition(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCameraPosition {}

impl crate::viz::WCameraPositionTrait for WCameraPosition {
	fn as_raw_WCameraPosition(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCameraPosition {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCameraPosition {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCameraPosition {
	/// Creates camera coordinate frame at the origin.
	/// 
	/// ![Camera coordinate frame](https://docs.opencv.org/4.2.0/cpw1.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	pub fn new(scale: f64) -> Result<crate::viz::WCameraPosition> {
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_double(scale) }.into_result().map(|ptr| crate::viz::WCameraPosition { ptr })
	}
	
	/// Display the viewing frustum
	/// ## Parameters
	/// * fov: Field of view of the camera (horizontal, vertical).
	/// * scale: Scale of the frustum.
	/// * color: Color of the frustum.
	/// 
	/// Creates viewing frustum of the camera based on its field of view fov.
	/// 
	/// ![Camera viewing frustum](https://docs.opencv.org/4.2.0/cpw2.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	pub fn new_1(fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Vec2dX_double_const_ColorX(&fov, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCameraPosition { ptr })
	}
	
	///  Display image on the far plane of the viewing frustum
	/// 
	/// ## Parameters
	/// * fov: Field of view of the camera (horizontal, vertical).
	/// * image: BGR or Gray-Scale image that is going to be displayed on the far plane of the frustum.
	/// * scale: Scale of the frustum and image.
	/// * color: Color of the frustum.
	/// 
	/// Creates viewing frustum of the camera based on its intrinsic matrix K, and displays image on
	/// the far end plane.
	/// 
	/// ![Camera viewing frustum with image](https://docs.opencv.org/4.2.0/cpw3.png)
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	/// * color: Color::white()
	pub fn new_2(fov: core::Vec2d, image: &dyn core::ToInputArray, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCameraPosition> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WCameraPosition_WCameraPosition_const_Vec2dX_const__InputArrayX_double_const_ColorX(&fov, image.as_raw__InputArray(), scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCameraPosition { ptr })
	}
	
}

/// This 3D Widget defines a circle.
pub trait WCircleTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCircle(&self) -> *mut c_void;
}

/// This 3D Widget defines a circle.
pub struct WCircle {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCircle {
	fn drop(&mut self) {
		extern "C" { fn cv_WCircle_delete(instance: *mut c_void); }
		unsafe { cv_WCircle_delete(self.as_raw_WCircle()) };
	}
}

impl WCircle {
	pub fn as_raw_WCircle(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCircle {}

impl crate::viz::WCircleTrait for WCircle {
	fn as_raw_WCircle(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCircle {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCircle {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCircle {
	/// Constructs default planar circle centered at origin with plane normal along z-axis
	/// 
	/// ## Parameters
	/// * radius: Radius of the circle.
	/// * thickness: Thickness of the circle.
	/// * color: Color of the circle.
	/// 
	/// ## C++ default parameters
	/// * thickness: 0.01
	/// * color: Color::white()
	pub fn new(radius: f64, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
		unsafe { sys::cv_viz_WCircle_WCircle_double_double_const_ColorX(radius, thickness, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCircle { ptr })
	}
	
	/// Constructs repositioned planar circle.
	/// 
	/// ## Parameters
	/// * radius: Radius of the circle.
	/// * center: Center of the circle.
	/// * normal: Normal of the plane in which the circle lies.
	/// * thickness: Thickness of the circle.
	/// * color: Color of the circle.
	/// 
	/// ## C++ default parameters
	/// * thickness: 0.01
	/// * color: Color::white()
	pub fn new_1(radius: f64, center: core::Point3d, normal: core::Vec3d, thickness: f64, color: &crate::viz::Color) -> Result<crate::viz::WCircle> {
		unsafe { sys::cv_viz_WCircle_WCircle_double_const_Point3dX_const_Vec3dX_double_const_ColorX(radius, &center, &normal, thickness, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCircle { ptr })
	}
	
}

/// This 3D Widget defines a point cloud. :
/// 
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub trait WCloudTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCloud(&self) -> *mut c_void;
}

/// This 3D Widget defines a point cloud. :
/// 
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub struct WCloud {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCloud {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloud_delete(instance: *mut c_void); }
		unsafe { cv_WCloud_delete(self.as_raw_WCloud()) };
	}
}

impl WCloud {
	pub fn as_raw_WCloud(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCloud {}

impl crate::viz::WCloudTrait for WCloud {
	fn as_raw_WCloud(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCloud {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCloud {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCloud {
	/// Constructs a WCloud.
	/// 
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * colors: Set of colors. It has to be of the same size with cloud.
	/// 
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	pub fn new(cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayX_const__InputArrayX(cloud.as_raw__InputArray(), colors.as_raw__InputArray()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
	}
	
	/// Constructs a WCloud.
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * color: A single Color for the whole cloud.
	/// 
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	pub fn new_1(cloud: &dyn core::ToInputArray, color: &crate::viz::Color) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayX_const_ColorX(cloud.as_raw__InputArray(), color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
	}
	
	/// Constructs a WCloud.
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * colors: Set of colors. It has to be of the same size with cloud.
	/// * normals: Normals for each point in cloud. Size and type should match with the cloud parameter.
	/// 
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	pub fn new_2(cloud: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(colors);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayX_const__InputArrayX_const__InputArrayX(cloud.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
	}
	
	/// Constructs a WCloud.
	/// ## Parameters
	/// * cloud: Set of points which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * color: A single Color for the whole cloud.
	/// * normals: Normals for each point in cloud.
	/// 
	/// Size and type should match with the cloud parameter.
	/// Points in the cloud belong to mask when they are set to (NaN, NaN, NaN).
	pub fn new_3(cloud: &dyn core::ToInputArray, color: &crate::viz::Color, normals: &dyn core::ToInputArray) -> Result<crate::viz::WCloud> {
		input_array_arg!(cloud);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WCloud_WCloud_const__InputArrayX_const_ColorX_const__InputArrayX(cloud.as_raw__InputArray(), color.as_raw_Color(), normals.as_raw__InputArray()) }.into_result().map(|ptr| crate::viz::WCloud { ptr })
	}
	
}

/// This 3D Widget defines a collection of clouds. :
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub trait WCloudCollectionTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCloudCollection(&self) -> *mut c_void;
	/// Finalizes cloud data by repacking to single cloud.
	/// 
	/// Useful for large cloud collections to reduce memory usage
	fn finalize(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_WCloudCollection_finalize(self.as_raw_WCloudCollection()) }.into_result()
	}
	
}

/// This 3D Widget defines a collection of clouds. :
/// 
/// Note: In case there are four channels in the cloud, fourth channel is ignored.
pub struct WCloudCollection {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCloudCollection {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloudCollection_delete(instance: *mut c_void); }
		unsafe { cv_WCloudCollection_delete(self.as_raw_WCloudCollection()) };
	}
}

impl WCloudCollection {
	pub fn as_raw_WCloudCollection(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCloudCollection {}

impl crate::viz::WCloudCollectionTrait for WCloudCollection {
	fn as_raw_WCloudCollection(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCloudCollection {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCloudCollection {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCloudCollection {
	pub fn default() -> Result<crate::viz::WCloudCollection> {
		unsafe { sys::cv_viz_WCloudCollection_WCloudCollection() }.into_result().map(|ptr| crate::viz::WCloudCollection { ptr })
	}
	
}

/// This 3D Widget represents normals of a point cloud. :
pub trait WCloudNormalsTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCloudNormals(&self) -> *mut c_void;
}

/// This 3D Widget represents normals of a point cloud. :
pub struct WCloudNormals {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCloudNormals {
	fn drop(&mut self) {
		extern "C" { fn cv_WCloudNormals_delete(instance: *mut c_void); }
		unsafe { cv_WCloudNormals_delete(self.as_raw_WCloudNormals()) };
	}
}

impl WCloudNormals {
	pub fn as_raw_WCloudNormals(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCloudNormals {}

impl crate::viz::WCloudNormalsTrait for WCloudNormals {
	fn as_raw_WCloudNormals(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCloudNormals {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCloudNormals {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCloudNormals {
	/// Constructs a WCloudNormals.
	/// 
	/// ## Parameters
	/// * cloud: Point set which can be of type: CV_32FC3, CV_32FC4, CV_64FC3, CV_64FC4.
	/// * normals: A set of normals that has to be of same type with cloud.
	/// * level: Display only every level th normal.
	/// * scale: Scale of the arrows that represent normals.
	/// * color: Color of the arrows that represent normals.
	/// 
	/// 
	/// Note: In case there are four channels in the cloud, fourth channel is ignored.
	/// 
	/// ## C++ default parameters
	/// * level: 64
	/// * scale: 0.1
	/// * color: Color::white()
	pub fn new(cloud: &dyn core::ToInputArray, normals: &dyn core::ToInputArray, level: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WCloudNormals> {
		input_array_arg!(cloud);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WCloudNormals_WCloudNormals_const__InputArrayX_const__InputArrayX_int_double_const_ColorX(cloud.as_raw__InputArray(), normals.as_raw__InputArray(), level, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCloudNormals { ptr })
	}
	
}

/// This 3D Widget defines a cone. :
pub trait WConeTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCone(&self) -> *mut c_void;
}

/// This 3D Widget defines a cone. :
pub struct WCone {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCone {
	fn drop(&mut self) {
		extern "C" { fn cv_WCone_delete(instance: *mut c_void); }
		unsafe { cv_WCone_delete(self.as_raw_WCone()) };
	}
}

impl WCone {
	pub fn as_raw_WCone(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCone {}

impl crate::viz::WConeTrait for WCone {
	fn as_raw_WCone(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCone {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCone {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCone {
	/// Constructs default cone oriented along x-axis with center of its base located at origin
	/// 
	/// ## Parameters
	/// * length: Length of the cone.
	/// * radius: Radius of the cone.
	/// * resolution: Resolution of the cone.
	/// * color: Color of the cone.
	/// 
	/// ## C++ default parameters
	/// * resolution: 6.0
	/// * color: Color::white()
	pub fn new(length: f64, radius: f64, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
		unsafe { sys::cv_viz_WCone_WCone_double_double_int_const_ColorX(length, radius, resolution, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCone { ptr })
	}
	
	/// Constructs repositioned planar cone.
	/// 
	/// ## Parameters
	/// * radius: Radius of the cone.
	/// * center: Center of the cone base.
	/// * tip: Tip of the cone.
	/// * resolution: Resolution of the cone.
	/// * color: Color of the cone.
	/// 
	/// ## C++ default parameters
	/// * resolution: 6.0
	/// * color: Color::white()
	pub fn new_1(radius: f64, center: core::Point3d, tip: core::Point3d, resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WCone> {
		unsafe { sys::cv_viz_WCone_WCone_double_const_Point3dX_const_Point3dX_int_const_ColorX(radius, &center, &tip, resolution, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCone { ptr })
	}
	
}

/// This 3D Widget represents a coordinate system. :
pub trait WCoordinateSystemTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCoordinateSystem(&self) -> *mut c_void;
}

/// This 3D Widget represents a coordinate system. :
pub struct WCoordinateSystem {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCoordinateSystem {
	fn drop(&mut self) {
		extern "C" { fn cv_WCoordinateSystem_delete(instance: *mut c_void); }
		unsafe { cv_WCoordinateSystem_delete(self.as_raw_WCoordinateSystem()) };
	}
}

impl WCoordinateSystem {
	pub fn as_raw_WCoordinateSystem(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCoordinateSystem {}

impl crate::viz::WCoordinateSystemTrait for WCoordinateSystem {
	fn as_raw_WCoordinateSystem(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCoordinateSystem {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCoordinateSystem {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCoordinateSystem {
	/// Constructs a WCoordinateSystem.
	/// 
	/// ## Parameters
	/// * scale: Determines the size of the axes.
	/// 
	/// ## C++ default parameters
	/// * scale: 1.0
	pub fn new(scale: f64) -> Result<crate::viz::WCoordinateSystem> {
		unsafe { sys::cv_viz_WCoordinateSystem_WCoordinateSystem_double(scale) }.into_result().map(|ptr| crate::viz::WCoordinateSystem { ptr })
	}
	
}

/// This 3D Widget defines a cube.
pub trait WCubeTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCube(&self) -> *mut c_void;
}

/// This 3D Widget defines a cube.
pub struct WCube {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCube {
	fn drop(&mut self) {
		extern "C" { fn cv_WCube_delete(instance: *mut c_void); }
		unsafe { cv_WCube_delete(self.as_raw_WCube()) };
	}
}

impl WCube {
	pub fn as_raw_WCube(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCube {}

impl crate::viz::WCubeTrait for WCube {
	fn as_raw_WCube(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCube {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCube {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCube {
	/// Constructs a WCube.
	/// 
	/// ## Parameters
	/// * min_point: Specifies minimum point of the bounding box.
	/// * max_point: Specifies maximum point of the bounding box.
	/// * wire_frame: If true, cube is represented as wireframe.
	/// * color: Color of the cube.
	/// 
	/// ![Cube Widget](https://docs.opencv.org/4.2.0/cube_widget.png)
	/// 
	/// ## C++ default parameters
	/// * min_point: Vec3d::all(-0.5)
	/// * max_point: Vec3d::all(0.5)
	/// * wire_frame: true
	/// * color: Color::white()
	pub fn new(min_point: core::Point3d, max_point: core::Point3d, wire_frame: bool, color: &crate::viz::Color) -> Result<crate::viz::WCube> {
		unsafe { sys::cv_viz_WCube_WCube_const_Point3dX_const_Point3dX_bool_const_ColorX(&min_point, &max_point, wire_frame, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCube { ptr })
	}
	
}

/// This 3D Widget defines a cylinder. :
pub trait WCylinderTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WCylinder(&self) -> *mut c_void;
}

/// This 3D Widget defines a cylinder. :
pub struct WCylinder {
	pub(crate) ptr: *mut c_void
}

impl Drop for WCylinder {
	fn drop(&mut self) {
		extern "C" { fn cv_WCylinder_delete(instance: *mut c_void); }
		unsafe { cv_WCylinder_delete(self.as_raw_WCylinder()) };
	}
}

impl WCylinder {
	pub fn as_raw_WCylinder(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WCylinder {}

impl crate::viz::WCylinderTrait for WCylinder {
	fn as_raw_WCylinder(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WCylinder {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WCylinder {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WCylinder {
	/// Constructs a WCylinder.
	/// 
	/// ## Parameters
	/// * axis_point1: A point1 on the axis of the cylinder.
	/// * axis_point2: A point2 on the axis of the cylinder.
	/// * radius: Radius of the cylinder.
	/// * numsides: Resolution of the cylinder.
	/// * color: Color of the cylinder.
	/// 
	/// ## C++ default parameters
	/// * numsides: 30
	/// * color: Color::white()
	pub fn new(axis_point1: core::Point3d, axis_point2: core::Point3d, radius: f64, numsides: i32, color: &crate::viz::Color) -> Result<crate::viz::WCylinder> {
		unsafe { sys::cv_viz_WCylinder_WCylinder_const_Point3dX_const_Point3dX_double_int_const_ColorX(&axis_point1, &axis_point2, radius, numsides, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WCylinder { ptr })
	}
	
}

/// This 3D Widget defines a grid. :
pub trait WGridTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WGrid(&self) -> *mut c_void;
}

/// This 3D Widget defines a grid. :
pub struct WGrid {
	pub(crate) ptr: *mut c_void
}

impl Drop for WGrid {
	fn drop(&mut self) {
		extern "C" { fn cv_WGrid_delete(instance: *mut c_void); }
		unsafe { cv_WGrid_delete(self.as_raw_WGrid()) };
	}
}

impl WGrid {
	pub fn as_raw_WGrid(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WGrid {}

impl crate::viz::WGridTrait for WGrid {
	fn as_raw_WGrid(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WGrid {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WGrid {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WGrid {
	/// Constructs a WGrid.
	/// 
	/// ## Parameters
	/// * cells: Number of cell columns and rows, respectively.
	/// * cells_spacing: Size of each cell, respectively.
	/// * color: Color of the grid.
	/// 
	/// ## C++ default parameters
	/// * cells: Vec2i::all(10)
	/// * cells_spacing: Vec2d::all(1.0)
	/// * color: Color::white()
	pub fn new(cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
		unsafe { sys::cv_viz_WGrid_WGrid_const_Vec2iX_const_Vec2dX_const_ColorX(&cells, &cells_spacing, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WGrid { ptr })
	}
	
	/// Creates repositioned grid
	/// 
	/// ## C++ default parameters
	/// * cells: Vec2i::all(10)
	/// * cells_spacing: Vec2d::all(1.0)
	/// * color: Color::white()
	pub fn new_1(center: core::Point3d, normal: core::Vec3d, new_yaxis: core::Vec3d, cells: core::Vec2i, cells_spacing: core::Vec2d, color: &crate::viz::Color) -> Result<crate::viz::WGrid> {
		unsafe { sys::cv_viz_WGrid_WGrid_const_Point3dX_const_Vec3dX_const_Vec3dX_const_Vec2iX_const_Vec2dX_const_ColorX(&center, &normal, &new_yaxis, &cells, &cells_spacing, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WGrid { ptr })
	}
	
}

/// This 3D Widget represents an image in 3D space. :
pub trait WImage3DTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WImage3D(&self) -> *mut c_void;
	/// Sets the image content of the widget.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	fn set_image(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImage3D_setImage_const__InputArrayX(self.as_raw_WImage3D(), image.as_raw__InputArray()) }.into_result()
	}
	
	/// Sets the image size of the widget.
	/// 
	/// ## Parameters
	/// * size: the new size of the image.
	fn set_size(&mut self, size: core::Size) -> Result<()> {
		unsafe { sys::cv_viz_WImage3D_setSize_const_SizeX(self.as_raw_WImage3D(), &size) }.into_result()
	}
	
}

/// This 3D Widget represents an image in 3D space. :
pub struct WImage3D {
	pub(crate) ptr: *mut c_void
}

impl Drop for WImage3D {
	fn drop(&mut self) {
		extern "C" { fn cv_WImage3D_delete(instance: *mut c_void); }
		unsafe { cv_WImage3D_delete(self.as_raw_WImage3D()) };
	}
}

impl WImage3D {
	pub fn as_raw_WImage3D(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WImage3D {}

impl crate::viz::WImage3DTrait for WImage3D {
	fn as_raw_WImage3D(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WImage3D {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WImage3D {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WImage3D {
	/// Constructs an WImage3D.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	/// * size: Size of the image.
	pub fn new(image: &dyn core::ToInputArray, size: core::Size2d) -> Result<crate::viz::WImage3D> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImage3D_WImage3D_const__InputArrayX_const_Size2dX(image.as_raw__InputArray(), &size) }.into_result().map(|ptr| crate::viz::WImage3D { ptr })
	}
	
	/// Constructs an WImage3D.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	/// * size: Size of the image.
	/// * center: Position of the image.
	/// * normal: Normal of the plane that represents the image.
	/// * up_vector: Determines orientation of the image.
	pub fn new_1(image: &dyn core::ToInputArray, size: core::Size2d, center: core::Vec3d, normal: core::Vec3d, up_vector: core::Vec3d) -> Result<crate::viz::WImage3D> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImage3D_WImage3D_const__InputArrayX_const_Size2dX_const_Vec3dX_const_Vec3dX_const_Vec3dX(image.as_raw__InputArray(), &size, &center, &normal, &up_vector) }.into_result().map(|ptr| crate::viz::WImage3D { ptr })
	}
	
}

/// This 2D Widget represents an image overlay. :
pub trait WImageOverlayTrait: crate::viz::WidgetTrait + crate::viz::Widget2DTrait {
	fn as_raw_WImageOverlay(&self) -> *mut c_void;
	/// Sets the image content of the widget.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	fn set_image(&mut self, image: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImageOverlay_setImage_const__InputArrayX(self.as_raw_WImageOverlay(), image.as_raw__InputArray()) }.into_result()
	}
	
}

/// This 2D Widget represents an image overlay. :
pub struct WImageOverlay {
	pub(crate) ptr: *mut c_void
}

impl Drop for WImageOverlay {
	fn drop(&mut self) {
		extern "C" { fn cv_WImageOverlay_delete(instance: *mut c_void); }
		unsafe { cv_WImageOverlay_delete(self.as_raw_WImageOverlay()) };
	}
}

impl WImageOverlay {
	pub fn as_raw_WImageOverlay(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WImageOverlay {}

impl crate::viz::WImageOverlayTrait for WImageOverlay {
	fn as_raw_WImageOverlay(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WImageOverlay {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget2DTrait for WImageOverlay {
	fn as_raw_Widget2D(&self) -> *mut c_void { self.ptr }
}

impl WImageOverlay {
	/// Constructs an WImageOverlay.
	/// 
	/// ## Parameters
	/// * image: BGR or Gray-Scale image.
	/// * rect: Image is scaled and positioned based on rect.
	pub fn new(image: &dyn core::ToInputArray, rect: core::Rect) -> Result<crate::viz::WImageOverlay> {
		input_array_arg!(image);
		unsafe { sys::cv_viz_WImageOverlay_WImageOverlay_const__InputArrayX_const_RectX(image.as_raw__InputArray(), &rect) }.into_result().map(|ptr| crate::viz::WImageOverlay { ptr })
	}
	
}

/// This 3D Widget defines a finite line.
pub trait WLineTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WLine(&self) -> *mut c_void;
}

/// This 3D Widget defines a finite line.
pub struct WLine {
	pub(crate) ptr: *mut c_void
}

impl Drop for WLine {
	fn drop(&mut self) {
		extern "C" { fn cv_WLine_delete(instance: *mut c_void); }
		unsafe { cv_WLine_delete(self.as_raw_WLine()) };
	}
}

impl WLine {
	pub fn as_raw_WLine(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WLine {}

impl crate::viz::WLineTrait for WLine {
	fn as_raw_WLine(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WLine {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WLine {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WLine {
	/// Constructs a WLine.
	/// 
	/// ## Parameters
	/// * pt1: Start point of the line.
	/// * pt2: End point of the line.
	/// * color: Color of the line.
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	pub fn new(pt1: core::Point3d, pt2: core::Point3d, color: &crate::viz::Color) -> Result<crate::viz::WLine> {
		unsafe { sys::cv_viz_WLine_WLine_const_Point3dX_const_Point3dX_const_ColorX(&pt1, &pt2, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WLine { ptr })
	}
	
}

/// Constructs a WMesh.
/// 
/// ## Parameters
/// * mesh: Mesh object that will be displayed.
/// * cloud: Points of the mesh object.
/// * polygons: Points of the mesh object.
/// * colors: Point colors.
/// * normals: Point normals.
pub trait WMeshTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WMesh(&self) -> *mut c_void;
}

/// Constructs a WMesh.
/// 
/// ## Parameters
/// * mesh: Mesh object that will be displayed.
/// * cloud: Points of the mesh object.
/// * polygons: Points of the mesh object.
/// * colors: Point colors.
/// * normals: Point normals.
pub struct WMesh {
	pub(crate) ptr: *mut c_void
}

impl Drop for WMesh {
	fn drop(&mut self) {
		extern "C" { fn cv_WMesh_delete(instance: *mut c_void); }
		unsafe { cv_WMesh_delete(self.as_raw_WMesh()) };
	}
}

impl WMesh {
	pub fn as_raw_WMesh(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WMesh {}

impl crate::viz::WMeshTrait for WMesh {
	fn as_raw_WMesh(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WMesh {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WMesh {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WMesh {
	pub fn new(mesh: &crate::viz::Mesh) -> Result<crate::viz::WMesh> {
		unsafe { sys::cv_viz_WMesh_WMesh_const_MeshX(mesh.as_raw_Mesh()) }.into_result().map(|ptr| crate::viz::WMesh { ptr })
	}
	
	/// ## C++ default parameters
	/// * colors: noArray()
	/// * normals: noArray()
	pub fn new_1(cloud: &dyn core::ToInputArray, polygons: &dyn core::ToInputArray, colors: &dyn core::ToInputArray, normals: &dyn core::ToInputArray) -> Result<crate::viz::WMesh> {
		input_array_arg!(cloud);
		input_array_arg!(polygons);
		input_array_arg!(colors);
		input_array_arg!(normals);
		unsafe { sys::cv_viz_WMesh_WMesh_const__InputArrayX_const__InputArrayX_const__InputArrayX_const__InputArrayX(cloud.as_raw__InputArray(), polygons.as_raw__InputArray(), colors.as_raw__InputArray(), normals.as_raw__InputArray()) }.into_result().map(|ptr| crate::viz::WMesh { ptr })
	}
	
}

pub trait WPaintedCloudTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WPaintedCloud(&self) -> *mut c_void;
}

pub struct WPaintedCloud {
	pub(crate) ptr: *mut c_void
}

impl Drop for WPaintedCloud {
	fn drop(&mut self) {
		extern "C" { fn cv_WPaintedCloud_delete(instance: *mut c_void); }
		unsafe { cv_WPaintedCloud_delete(self.as_raw_WPaintedCloud()) };
	}
}

impl WPaintedCloud {
	pub fn as_raw_WPaintedCloud(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WPaintedCloud {}

impl crate::viz::WPaintedCloudTrait for WPaintedCloud {
	fn as_raw_WPaintedCloud(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WPaintedCloud {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WPaintedCloud {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WPaintedCloud {
	/// Paint cloud with default gradient between cloud bounds points
	pub fn new(cloud: &dyn core::ToInputArray) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayX(cloud.as_raw__InputArray()) }.into_result().map(|ptr| crate::viz::WPaintedCloud { ptr })
	}
	
	/// Paint cloud with default gradient between given points
	pub fn new_1(cloud: &dyn core::ToInputArray, p1: core::Point3d, p2: core::Point3d) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayX_const_Point3dX_const_Point3dX(cloud.as_raw__InputArray(), &p1, &p2) }.into_result().map(|ptr| crate::viz::WPaintedCloud { ptr })
	}
	
	/// Paint cloud with gradient specified by given colors between given points
	pub fn new_2(cloud: &dyn core::ToInputArray, p1: core::Point3d, p2: core::Point3d, c1: &crate::viz::Color, c2: crate::viz::Color) -> Result<crate::viz::WPaintedCloud> {
		input_array_arg!(cloud);
		unsafe { sys::cv_viz_WPaintedCloud_WPaintedCloud_const__InputArrayX_const_Point3dX_const_Point3dX_const_ColorX_Color(cloud.as_raw__InputArray(), &p1, &p2, c1.as_raw_Color(), c2.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WPaintedCloud { ptr })
	}
	
}

/// This 3D Widget defines a finite plane.
pub trait WPlaneTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WPlane(&self) -> *mut c_void;
}

/// This 3D Widget defines a finite plane.
pub struct WPlane {
	pub(crate) ptr: *mut c_void
}

impl Drop for WPlane {
	fn drop(&mut self) {
		extern "C" { fn cv_WPlane_delete(instance: *mut c_void); }
		unsafe { cv_WPlane_delete(self.as_raw_WPlane()) };
	}
}

impl WPlane {
	pub fn as_raw_WPlane(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WPlane {}

impl crate::viz::WPlaneTrait for WPlane {
	fn as_raw_WPlane(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WPlane {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WPlane {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WPlane {
	/// Constructs a default plane with center point at origin and normal oriented along z-axis.
	/// 
	/// ## Parameters
	/// * size: Size of the plane
	/// * color: Color of the plane.
	/// 
	/// ## C++ default parameters
	/// * size: Size2d(1.0,1.0)
	/// * color: Color::white()
	pub fn new(size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
		unsafe { sys::cv_viz_WPlane_WPlane_const_Size2dX_const_ColorX(&size, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WPlane { ptr })
	}
	
	/// Constructs a repositioned plane
	/// 
	/// ## Parameters
	/// * center: Center of the plane
	/// * normal: Plane normal orientation
	/// * new_yaxis: Up-vector. New orientation of plane y-axis.
	/// * size: 
	/// * color: Color of the plane.
	/// 
	/// ## C++ default parameters
	/// * size: Size2d(1.0,1.0)
	/// * color: Color::white()
	pub fn new_1(center: core::Point3d, normal: core::Vec3d, new_yaxis: core::Vec3d, size: core::Size2d, color: &crate::viz::Color) -> Result<crate::viz::WPlane> {
		unsafe { sys::cv_viz_WPlane_WPlane_const_Point3dX_const_Vec3dX_const_Vec3dX_const_Size2dX_const_ColorX(&center, &normal, &new_yaxis, &size, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WPlane { ptr })
	}
	
}

/// This 3D Widget defines a poly line. :
pub trait WPolyLineTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WPolyLine(&self) -> *mut c_void;
}

/// This 3D Widget defines a poly line. :
pub struct WPolyLine {
	pub(crate) ptr: *mut c_void
}

impl Drop for WPolyLine {
	fn drop(&mut self) {
		extern "C" { fn cv_WPolyLine_delete(instance: *mut c_void); }
		unsafe { cv_WPolyLine_delete(self.as_raw_WPolyLine()) };
	}
}

impl WPolyLine {
	pub fn as_raw_WPolyLine(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WPolyLine {}

impl crate::viz::WPolyLineTrait for WPolyLine {
	fn as_raw_WPolyLine(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WPolyLine {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WPolyLine {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WPolyLine {
	pub fn new(points: &dyn core::ToInputArray, colors: &dyn core::ToInputArray) -> Result<crate::viz::WPolyLine> {
		input_array_arg!(points);
		input_array_arg!(colors);
		unsafe { sys::cv_viz_WPolyLine_WPolyLine_const__InputArrayX_const__InputArrayX(points.as_raw__InputArray(), colors.as_raw__InputArray()) }.into_result().map(|ptr| crate::viz::WPolyLine { ptr })
	}
	
	/// Constructs a WPolyLine.
	/// 
	/// ## Parameters
	/// * points: Point set.
	/// * color: Color of the poly line.
	/// 
	/// ## C++ default parameters
	/// * color: Color::white()
	pub fn new_1(points: &dyn core::ToInputArray, color: &crate::viz::Color) -> Result<crate::viz::WPolyLine> {
		input_array_arg!(points);
		unsafe { sys::cv_viz_WPolyLine_WPolyLine_const__InputArrayX_const_ColorX(points.as_raw__InputArray(), color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WPolyLine { ptr })
	}
	
}

/// This 3D Widget defines a sphere. :
pub trait WSphereTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WSphere(&self) -> *mut c_void;
}

/// This 3D Widget defines a sphere. :
pub struct WSphere {
	pub(crate) ptr: *mut c_void
}

impl Drop for WSphere {
	fn drop(&mut self) {
		extern "C" { fn cv_WSphere_delete(instance: *mut c_void); }
		unsafe { cv_WSphere_delete(self.as_raw_WSphere()) };
	}
}

impl WSphere {
	pub fn as_raw_WSphere(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WSphere {}

impl crate::viz::WSphereTrait for WSphere {
	fn as_raw_WSphere(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WSphere {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WSphere {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WSphere {
	/// Constructs a WSphere.
	/// 
	/// ## Parameters
	/// * center: Center of the sphere.
	/// * radius: Radius of the sphere.
	/// * sphere_resolution: Resolution of the sphere.
	/// * color: Color of the sphere.
	/// 
	/// ## C++ default parameters
	/// * sphere_resolution: 10
	/// * color: Color::white()
	pub fn new(center: core::Point3d, radius: f64, sphere_resolution: i32, color: &crate::viz::Color) -> Result<crate::viz::WSphere> {
		unsafe { sys::cv_viz_WSphere_WSphere_const_Point3dX_double_int_const_ColorX(&center, radius, sphere_resolution, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WSphere { ptr })
	}
	
}

/// This 2D Widget represents text overlay.
pub trait WTextTrait: crate::viz::WidgetTrait + crate::viz::Widget2DTrait {
	fn as_raw_WText(&self) -> *mut c_void;
	/// Sets the text content of the widget.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	fn set_text(&mut self, text: &str) -> Result<()> {
		string_arg!(text);
		unsafe { sys::cv_viz_WText_setText_const_StringX(self.as_raw_WText(), text.as_ptr()) }.into_result()
	}
	
	/// Returns the current text content of the widget.
	fn get_text(&self) -> Result<String> {
		unsafe { sys::cv_viz_WText_getText_const(self.as_raw_WText()) }.into_result().map(crate::templ::receive_string)
	}
	
}

/// This 2D Widget represents text overlay.
pub struct WText {
	pub(crate) ptr: *mut c_void
}

impl Drop for WText {
	fn drop(&mut self) {
		extern "C" { fn cv_WText_delete(instance: *mut c_void); }
		unsafe { cv_WText_delete(self.as_raw_WText()) };
	}
}

impl WText {
	pub fn as_raw_WText(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WText {}

impl crate::viz::WTextTrait for WText {
	fn as_raw_WText(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WText {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget2DTrait for WText {
	fn as_raw_Widget2D(&self) -> *mut c_void { self.ptr }
}

impl WText {
	/// Constructs a WText.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	/// * pos: Position of the text.
	/// * font_size: Font size.
	/// * color: Color of the text.
	/// 
	/// ## C++ default parameters
	/// * font_size: 20
	/// * color: Color::white()
	pub fn new(text: &str, pos: core::Point, font_size: i32, color: &crate::viz::Color) -> Result<crate::viz::WText> {
		string_arg!(text);
		unsafe { sys::cv_viz_WText_WText_const_StringX_const_PointX_int_const_ColorX(text.as_ptr(), &pos, font_size, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WText { ptr })
	}
	
}

/// This 3D Widget represents 3D text. The text always faces the camera.
pub trait WText3DTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WText3D(&self) -> *mut c_void;
	/// Sets the text content of the widget.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	fn set_text(&mut self, text: &str) -> Result<()> {
		string_arg!(text);
		unsafe { sys::cv_viz_WText3D_setText_const_StringX(self.as_raw_WText3D(), text.as_ptr()) }.into_result()
	}
	
	/// Returns the current text content of the widget.
	fn get_text(&self) -> Result<String> {
		unsafe { sys::cv_viz_WText3D_getText_const(self.as_raw_WText3D()) }.into_result().map(crate::templ::receive_string)
	}
	
}

/// This 3D Widget represents 3D text. The text always faces the camera.
pub struct WText3D {
	pub(crate) ptr: *mut c_void
}

impl Drop for WText3D {
	fn drop(&mut self) {
		extern "C" { fn cv_WText3D_delete(instance: *mut c_void); }
		unsafe { cv_WText3D_delete(self.as_raw_WText3D()) };
	}
}

impl WText3D {
	pub fn as_raw_WText3D(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WText3D {}

impl crate::viz::WText3DTrait for WText3D {
	fn as_raw_WText3D(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WText3D {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WText3D {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WText3D {
	/// Constructs a WText3D.
	/// 
	/// ## Parameters
	/// * text: Text content of the widget.
	/// * position: Position of the text.
	/// * text_scale: Size of the text.
	/// * face_camera: If true, text always faces the camera.
	/// * color: Color of the text.
	/// 
	/// ## C++ default parameters
	/// * text_scale: 1.
	/// * face_camera: true
	/// * color: Color::white()
	pub fn new(text: &str, position: core::Point3d, text_scale: f64, face_camera: bool, color: &crate::viz::Color) -> Result<crate::viz::WText3D> {
		string_arg!(text);
		unsafe { sys::cv_viz_WText3D_WText3D_const_StringX_const_Point3dX_double_bool_const_ColorX(text.as_ptr(), &position, text_scale, face_camera, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WText3D { ptr })
	}
	
}

/// This 3D Widget represents a trajectory. :
pub trait WTrajectoryTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WTrajectory(&self) -> *mut c_void;
}

/// This 3D Widget represents a trajectory. :
pub struct WTrajectory {
	pub(crate) ptr: *mut c_void
}

impl Drop for WTrajectory {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectory_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectory_delete(self.as_raw_WTrajectory()) };
	}
}

impl WTrajectory {
	pub fn as_raw_WTrajectory(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WTrajectory {}

impl crate::viz::WTrajectoryTrait for WTrajectory {
	fn as_raw_WTrajectory(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WTrajectory {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WTrajectory {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WTrajectory {
	/// Constructs a WTrajectory.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * display_mode: Display mode. This can be PATH, FRAMES, and BOTH.
	/// * scale: Scale of the frames. Polyline is not affected.
	/// * color: Color of the polyline that represents path.
	/// 
	/// Frames are not affected.
	/// Displays trajectory of the given path as follows:
	/// *   PATH : Displays a poly line that represents the path.
	/// *   FRAMES : Displays coordinate frames at each pose.
	/// *   PATH & FRAMES : Displays both poly line and coordinate frames.
	/// 
	/// ## C++ default parameters
	/// * display_mode: WTrajectory::PATH
	/// * scale: 1.0
	/// * color: Color::white()
	pub fn new(path: &dyn core::ToInputArray, display_mode: i32, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectory> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectory_WTrajectory_const__InputArrayX_int_double_const_ColorX(path.as_raw__InputArray(), display_mode, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WTrajectory { ptr })
	}
	
}

/// This 3D Widget represents a trajectory. :
pub trait WTrajectoryFrustumsTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WTrajectoryFrustums(&self) -> *mut c_void;
}

/// This 3D Widget represents a trajectory. :
pub struct WTrajectoryFrustums {
	pub(crate) ptr: *mut c_void
}

impl Drop for WTrajectoryFrustums {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectoryFrustums_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectoryFrustums_delete(self.as_raw_WTrajectoryFrustums()) };
	}
}

impl WTrajectoryFrustums {
	pub fn as_raw_WTrajectoryFrustums(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WTrajectoryFrustums {}

impl crate::viz::WTrajectoryFrustumsTrait for WTrajectoryFrustums {
	fn as_raw_WTrajectoryFrustums(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WTrajectoryFrustums {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WTrajectoryFrustums {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WTrajectoryFrustums {
	/// Constructs a WTrajectoryFrustums.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * fov: Field of view of the camera (horizontal, vertical).
	/// * scale: Scale of the frustums.
	/// * color: Color of the frustums.
	/// 
	/// Displays frustums at each pose of the trajectory.
	/// 
	/// ## C++ default parameters
	/// * scale: 1.
	/// * color: Color::white()
	pub fn new(path: &dyn core::ToInputArray, fov: core::Vec2d, scale: f64, color: &crate::viz::Color) -> Result<crate::viz::WTrajectoryFrustums> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectoryFrustums_WTrajectoryFrustums_const__InputArrayX_const_Vec2dX_double_const_ColorX(path.as_raw__InputArray(), &fov, scale, color.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WTrajectoryFrustums { ptr })
	}
	
}

/// This 3D Widget represents a trajectory using spheres and lines
/// 
/// where spheres represent the positions of the camera, and lines represent the direction from
/// previous position to the current. :
pub trait WTrajectorySpheresTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WTrajectorySpheres(&self) -> *mut c_void;
}

/// This 3D Widget represents a trajectory using spheres and lines
/// 
/// where spheres represent the positions of the camera, and lines represent the direction from
/// previous position to the current. :
pub struct WTrajectorySpheres {
	pub(crate) ptr: *mut c_void
}

impl Drop for WTrajectorySpheres {
	fn drop(&mut self) {
		extern "C" { fn cv_WTrajectorySpheres_delete(instance: *mut c_void); }
		unsafe { cv_WTrajectorySpheres_delete(self.as_raw_WTrajectorySpheres()) };
	}
}

impl WTrajectorySpheres {
	pub fn as_raw_WTrajectorySpheres(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WTrajectorySpheres {}

impl crate::viz::WTrajectorySpheresTrait for WTrajectorySpheres {
	fn as_raw_WTrajectorySpheres(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WTrajectorySpheres {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WTrajectorySpheres {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WTrajectorySpheres {
	/// Constructs a WTrajectorySpheres.
	/// 
	/// ## Parameters
	/// * path: List of poses on a trajectory. Takes std::vector\<Affine\<T\>\> with T == [float | double]
	/// * line_length: Max length of the lines which point to previous position
	/// * radius: Radius of the spheres.
	/// * from: Color for first sphere.
	/// * to: Color for last sphere. Intermediate spheres will have interpolated color.
	/// 
	/// ## C++ default parameters
	/// * line_length: 0.05
	/// * radius: 0.007
	/// * from: Color::red()
	/// * to: Color::white()
	pub fn new(path: &dyn core::ToInputArray, line_length: f64, radius: f64, from: &crate::viz::Color, to: &crate::viz::Color) -> Result<crate::viz::WTrajectorySpheres> {
		input_array_arg!(path);
		unsafe { sys::cv_viz_WTrajectorySpheres_WTrajectorySpheres_const__InputArrayX_double_double_const_ColorX_const_ColorX(path.as_raw__InputArray(), line_length, radius, from.as_raw_Color(), to.as_raw_Color()) }.into_result().map(|ptr| crate::viz::WTrajectorySpheres { ptr })
	}
	
}

/// This class allows to merge several widgets to single one.
/// 
/// It has quite limited functionality and can't merge widgets with different attributes. For
/// instance, if widgetA has color array and widgetB has only global color defined, then result
/// of merge won't have color at all. The class is suitable for merging large amount of similar
/// widgets. :
pub trait WWidgetMergerTrait: crate::viz::WidgetTrait + crate::viz::Widget3DTrait {
	fn as_raw_WWidgetMerger(&self) -> *mut c_void;
	/// Repacks internal structure to single widget
	fn finalize(&mut self) -> Result<()> {
		unsafe { sys::cv_viz_WWidgetMerger_finalize(self.as_raw_WWidgetMerger()) }.into_result()
	}
	
}

/// This class allows to merge several widgets to single one.
/// 
/// It has quite limited functionality and can't merge widgets with different attributes. For
/// instance, if widgetA has color array and widgetB has only global color defined, then result
/// of merge won't have color at all. The class is suitable for merging large amount of similar
/// widgets. :
pub struct WWidgetMerger {
	pub(crate) ptr: *mut c_void
}

impl Drop for WWidgetMerger {
	fn drop(&mut self) {
		extern "C" { fn cv_WWidgetMerger_delete(instance: *mut c_void); }
		unsafe { cv_WWidgetMerger_delete(self.as_raw_WWidgetMerger()) };
	}
}

impl WWidgetMerger {
	pub fn as_raw_WWidgetMerger(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for WWidgetMerger {}

impl crate::viz::WWidgetMergerTrait for WWidgetMerger {
	fn as_raw_WWidgetMerger(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::WidgetTrait for WWidgetMerger {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for WWidgetMerger {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl WWidgetMerger {
	pub fn default() -> Result<crate::viz::WWidgetMerger> {
		unsafe { sys::cv_viz_WWidgetMerger_WWidgetMerger() }.into_result().map(|ptr| crate::viz::WWidgetMerger { ptr })
	}
	
}

/// Base class of all widgets. Widget is implicitly shared.
pub trait WidgetTrait {
	fn as_raw_Widget(&self) -> *mut c_void;
	/// Sets rendering property of the widget.
	/// 
	/// ## Parameters
	/// * property: Property that will be modified.
	/// * value: The new value of the property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// IMMEDIATE_RENDERING:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	fn set_rendering_property(&mut self, property: i32, value: f64) -> Result<()> {
		unsafe { sys::cv_viz_Widget_setRenderingProperty_int_double(self.as_raw_Widget(), property, value) }.into_result()
	}
	
	/// Returns rendering property of the widget.
	/// 
	/// ## Parameters
	/// * property: Property.
	/// 
	/// Rendering property can be one of the following:
	/// *   **POINT_SIZE**
	/// *   **OPACITY**
	/// *   **LINE_WIDTH**
	/// *   **FONT_SIZE**
	/// *   **AMBIENT**
	/// 
	/// REPRESENTATION: Expected values are
	/// *   **REPRESENTATION_POINTS**
	/// *   **REPRESENTATION_WIREFRAME**
	/// *   **REPRESENTATION_SURFACE**
	/// 
	/// **IMMEDIATE_RENDERING**:
	/// *   Turn on immediate rendering by setting the value to 1.
	/// *   Turn off immediate rendering by setting the value to 0.
	/// 
	/// SHADING: Expected values are
	/// *   **SHADING_FLAT**
	/// *   **SHADING_GOURAUD**
	/// *   **SHADING_PHONG**
	fn get_rendering_property(&self, property: i32) -> Result<f64> {
		unsafe { sys::cv_viz_Widget_getRenderingProperty_const_int(self.as_raw_Widget(), property) }.into_result()
	}
	
}

/// Base class of all widgets. Widget is implicitly shared.
pub struct Widget {
	pub(crate) ptr: *mut c_void
}

impl Drop for Widget {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget_delete(instance: *mut c_void); }
		unsafe { cv_Widget_delete(self.as_raw_Widget()) };
	}
}

impl Widget {
	pub fn as_raw_Widget(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Widget {}

impl crate::viz::WidgetTrait for Widget {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl Widget {
	pub fn default() -> Result<crate::viz::Widget> {
		unsafe { sys::cv_viz_Widget_Widget() }.into_result().map(|ptr| crate::viz::Widget { ptr })
	}
	
	pub fn copy(other: &crate::viz::Widget) -> Result<crate::viz::Widget> {
		unsafe { sys::cv_viz_Widget_Widget_const_WidgetX(other.as_raw_Widget()) }.into_result().map(|ptr| crate::viz::Widget { ptr })
	}
	
	/// Creates a widget from ply file.
	/// 
	/// ## Parameters
	/// * file_name: Ply file name.
	pub fn from_ply_file(file_name: &str) -> Result<crate::viz::Widget> {
		string_arg!(file_name);
		unsafe { sys::cv_viz_Widget_fromPlyFile_const_StringX(file_name.as_ptr()) }.into_result().map(|ptr| crate::viz::Widget { ptr })
	}
	
}

/// Base class of all 2D widgets.
pub trait Widget2DTrait: crate::viz::WidgetTrait {
	fn as_raw_Widget2D(&self) -> *mut c_void;
	/// Sets the color of the widget.
	/// 
	/// ## Parameters
	/// * color: color of type Color
	fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
		unsafe { sys::cv_viz_Widget2D_setColor_const_ColorX(self.as_raw_Widget2D(), color.as_raw_Color()) }.into_result()
	}
	
}

/// Base class of all 2D widgets.
pub struct Widget2D {
	pub(crate) ptr: *mut c_void
}

impl Drop for Widget2D {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget2D_delete(instance: *mut c_void); }
		unsafe { cv_Widget2D_delete(self.as_raw_Widget2D()) };
	}
}

impl Widget2D {
	pub fn as_raw_Widget2D(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Widget2D {}

impl crate::viz::WidgetTrait for Widget2D {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget2DTrait for Widget2D {
	fn as_raw_Widget2D(&self) -> *mut c_void { self.ptr }
}

impl Widget2D {
	pub fn default() -> Result<crate::viz::Widget2D> {
		unsafe { sys::cv_viz_Widget2D_Widget2D() }.into_result().map(|ptr| crate::viz::Widget2D { ptr })
	}
	
}

/// Base class of all 3D widgets.
pub trait Widget3DTrait: crate::viz::WidgetTrait {
	fn as_raw_Widget3D(&self) -> *mut c_void;
	/// Sets the color of the widget.
	/// 
	/// ## Parameters
	/// * color: color of type Color
	fn set_color(&mut self, color: &crate::viz::Color) -> Result<()> {
		unsafe { sys::cv_viz_Widget3D_setColor_const_ColorX(self.as_raw_Widget3D(), color.as_raw_Color()) }.into_result()
	}
	
}

/// Base class of all 3D widgets.
pub struct Widget3D {
	pub(crate) ptr: *mut c_void
}

impl Drop for Widget3D {
	fn drop(&mut self) {
		extern "C" { fn cv_Widget3D_delete(instance: *mut c_void); }
		unsafe { cv_Widget3D_delete(self.as_raw_Widget3D()) };
	}
}

impl Widget3D {
	pub fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }

	pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
		Self { ptr }
	}
}

unsafe impl Send for Widget3D {}

impl crate::viz::WidgetTrait for Widget3D {
	fn as_raw_Widget(&self) -> *mut c_void { self.ptr }
}

impl crate::viz::Widget3DTrait for Widget3D {
	fn as_raw_Widget3D(&self) -> *mut c_void { self.ptr }
}

impl Widget3D {
	pub fn default() -> Result<crate::viz::Widget3D> {
		unsafe { sys::cv_viz_Widget3D_Widget3D() }.into_result().map(|ptr| crate::viz::Widget3D { ptr })
	}
	
}
