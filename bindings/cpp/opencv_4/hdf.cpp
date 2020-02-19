#include "hdf.hpp"
#include "hdf_types.hpp"

extern "C" {
	Result<void*> cv_hdf_open_const_StringX(const char* HDF5Filename) {
		try {
			cv::Ptr<cv::hdf::HDF5> ret = cv::hdf::open(std::string(HDF5Filename));
			return Ok<void*>(new cv::Ptr<cv::hdf::HDF5>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_hdf_HDF5_close(void* instance) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->close();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_grcreate_const_StringX(void* instance, const char* grlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->grcreate(std::string(grlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<bool> cv_hdf_HDF5_hlexists_const_const_StringX(void* instance, const char* label) {
		try {
			bool ret = reinterpret_cast<cv::hdf::HDF5*>(instance)->hlexists(std::string(label));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result<bool> cv_hdf_HDF5_atexists_const_const_StringX(void* instance, const char* atlabel) {
		try {
			bool ret = reinterpret_cast<cv::hdf::HDF5*>(instance)->atexists(std::string(atlabel));
			return Ok<bool>(ret);
		} OCVRS_CATCH(Result<bool>)
	}
	
	Result_void cv_hdf_HDF5_atdelete_const_StringX(void* instance, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atdelete(std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atwrite_int_const_StringX(void* instance, int value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atwrite(value, std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atread_intX_const_StringX(void* instance, int* value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atread(value, std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atwrite_double_const_StringX(void* instance, double value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atwrite(value, std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atread_doubleX_const_StringX(void* instance, double* value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atread(value, std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atwrite_const_StringX_const_StringX(void* instance, const char* value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atwrite(std::string(value), std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atread_StringX_const_StringX(void* instance, void** value, const char* atlabel) {
		try {
			std::string value_out;
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atread(&value_out, std::string(atlabel));
			*value = ocvrs_create_string(value_out.c_str());
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atwrite_const__InputArrayX_const_StringX(void* instance, void* value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atwrite(*reinterpret_cast<const cv::_InputArray*>(value), std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_atread_const__OutputArrayX_const_StringX(void* instance, void* value, const char* atlabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->atread(*reinterpret_cast<const cv::_OutputArray*>(value), std::string(atlabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_const_StringX(void* instance, int rows, int cols, int type, const char* dslabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(rows, cols, type, std::string(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_const_StringX_int(void* instance, int rows, int cols, int type, const char* dslabel, int compresslevel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(rows, cols, type, std::string(dslabel), compresslevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_const_StringX_int_const_vector_int_X(void* instance, int rows, int cols, int type, const char* dslabel, int compresslevel, void* dims_chunks) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(rows, cols, type, std::string(dslabel), compresslevel, *reinterpret_cast<const std::vector<int>*>(dims_chunks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_int_int_const_StringX_int_const_intX(void* instance, int rows, int cols, int type, const char* dslabel, int compresslevel, const int* dims_chunks) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(rows, cols, type, std::string(dslabel), compresslevel, dims_chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_const_intX_int_const_StringX(void* instance, int n_dims, const int* sizes, int type, const char* dslabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(n_dims, sizes, type, std::string(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_const_intX_int_const_StringX_int(void* instance, int n_dims, const int* sizes, int type, const char* dslabel, int compresslevel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(n_dims, sizes, type, std::string(dslabel), compresslevel);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_const_vector_int_X_int_const_StringX_int_const_vector_int_X(void* instance, void* sizes, int type, const char* dslabel, int compresslevel, void* dims_chunks) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(*reinterpret_cast<const std::vector<int>*>(sizes), type, std::string(dslabel), compresslevel, *reinterpret_cast<const std::vector<int>*>(dims_chunks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dscreate_const_int_const_intX_int_const_StringX_int_const_intX(void* instance, int n_dims, const int* sizes, int type, const char* dslabel, int compresslevel, const int* dims_chunks) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dscreate(n_dims, sizes, type, std::string(dslabel), compresslevel, dims_chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_hdf_HDF5_dsgetsize_const_const_StringX_int(void* instance, const char* dslabel, int dims_flag) {
		try {
			std::vector<int> ret = reinterpret_cast<cv::hdf::HDF5*>(instance)->dsgetsize(std::string(dslabel), dims_flag);
			return Ok<void*>(new std::vector<int>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_hdf_HDF5_dsgettype_const_const_StringX(void* instance, const char* dslabel) {
		try {
			int ret = reinterpret_cast<cv::hdf::HDF5*>(instance)->dsgettype(std::string(dslabel));
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_const_StringX(void* instance, void* Array, const char* dslabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dswrite(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_const_StringX_const_intX(void* instance, void* Array, const char* dslabel, const int* dims_offset) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dswrite(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel), dims_offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_const_StringX_const_vector_int_X_const_vector_int_X(void* instance, void* Array, const char* dslabel, void* dims_offset, void* dims_counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dswrite(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel), *reinterpret_cast<const std::vector<int>*>(dims_offset), *reinterpret_cast<const std::vector<int>*>(dims_counts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dswrite_const_const__InputArrayX_const_StringX_const_intX_const_intX(void* instance, void* Array, const char* dslabel, const int* dims_offset, const int* dims_counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dswrite(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel), dims_offset, dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_const_StringX(void* instance, void* Array, const char* dslabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsinsert(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_const_StringX_const_intX(void* instance, void* Array, const char* dslabel, const int* dims_offset) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsinsert(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel), dims_offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_const_StringX_const_vector_int_X_const_vector_int_X(void* instance, void* Array, const char* dslabel, void* dims_offset, void* dims_counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsinsert(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel), *reinterpret_cast<const std::vector<int>*>(dims_offset), *reinterpret_cast<const std::vector<int>*>(dims_counts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsinsert_const_const__InputArrayX_const_StringX_const_intX_const_intX(void* instance, void* Array, const char* dslabel, const int* dims_offset, const int* dims_counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsinsert(*reinterpret_cast<const cv::_InputArray*>(Array), std::string(dslabel), dims_offset, dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_const_StringX(void* instance, void* Array, const char* dslabel) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsread(*reinterpret_cast<const cv::_OutputArray*>(Array), std::string(dslabel));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_const_StringX_const_intX(void* instance, void* Array, const char* dslabel, const int* dims_offset) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsread(*reinterpret_cast<const cv::_OutputArray*>(Array), std::string(dslabel), dims_offset);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_const_StringX_const_vector_int_X_const_vector_int_X(void* instance, void* Array, const char* dslabel, void* dims_offset, void* dims_counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsread(*reinterpret_cast<const cv::_OutputArray*>(Array), std::string(dslabel), *reinterpret_cast<const std::vector<int>*>(dims_offset), *reinterpret_cast<const std::vector<int>*>(dims_counts));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_dsread_const_const__OutputArrayX_const_StringX_const_intX_const_intX(void* instance, void* Array, const char* dslabel, const int* dims_offset, const int* dims_counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->dsread(*reinterpret_cast<const cv::_OutputArray*>(Array), std::string(dslabel), dims_offset, dims_counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_hdf_HDF5_kpgetsize_const_const_StringX_int(void* instance, const char* kplabel, int dims_flag) {
		try {
			int ret = reinterpret_cast<cv::hdf::HDF5*>(instance)->kpgetsize(std::string(kplabel), dims_flag);
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_hdf_HDF5_kpcreate_const_int_const_StringX_int_int(void* instance, int size, const char* kplabel, int compresslevel, int chunks) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->kpcreate(size, std::string(kplabel), compresslevel, chunks);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_kpwrite_const_vector_KeyPoint__const_StringX_int_int(void* instance, void* keypoints, const char* kplabel, int offset, int counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->kpwrite(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), std::string(kplabel), offset, counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_kpinsert_const_vector_KeyPoint__const_StringX_int_int(void* instance, void* keypoints, const char* kplabel, int offset, int counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->kpinsert(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), std::string(kplabel), offset, counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_hdf_HDF5_kpread_const_vector_KeyPoint_X_const_StringX_int_int(void* instance, void* keypoints, const char* kplabel, int offset, int counts) {
		try {
			reinterpret_cast<cv::hdf::HDF5*>(instance)->kpread(*reinterpret_cast<std::vector<cv::KeyPoint>*>(keypoints), std::string(kplabel), offset, counts);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
