#include "common.hpp"
#include <opencv2/line_descriptor.hpp>
#include "line_descriptor_types.hpp"

extern "C" {
	Result_void cv_line_descriptor_drawKeylines_const_MatX_const_vector_KeyLine_X_MatX_const_ScalarX_int(void* image, void* keylines, void* outImage, const cv::Scalar* color, int flags) {
		try {
			cv::line_descriptor::drawKeylines(*reinterpret_cast<const cv::Mat*>(image), *reinterpret_cast<const std::vector<cv::line_descriptor::KeyLine>*>(keylines), *reinterpret_cast<cv::Mat*>(outImage), *color, flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_drawLineMatches_const_MatX_const_vector_KeyLine_X_const_MatX_const_vector_KeyLine_X_const_vector_DMatch_X_MatX_const_ScalarX_const_ScalarX_const_vector_char_X_int(void* img1, void* keylines1, void* img2, void* keylines2, void* matches1to2, void* outImg, const cv::Scalar* matchColor, const cv::Scalar* singleLineColor, void* matchesMask, int flags) {
		try {
			cv::line_descriptor::drawLineMatches(*reinterpret_cast<const cv::Mat*>(img1), *reinterpret_cast<const std::vector<cv::line_descriptor::KeyLine>*>(keylines1), *reinterpret_cast<const cv::Mat*>(img2), *reinterpret_cast<const std::vector<cv::line_descriptor::KeyLine>*>(keylines2), *reinterpret_cast<const std::vector<cv::DMatch>*>(matches1to2), *reinterpret_cast<cv::Mat*>(outImg), *matchColor, *singleLineColor, *reinterpret_cast<const std::vector<char>*>(matchesMask), flags);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BinaryDescriptor_delete(cv::line_descriptor::BinaryDescriptor* instance) {
		delete instance;
	}
	Result<void*> cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsX(void* parameters) {
		try {
			cv::line_descriptor::BinaryDescriptor* ret = new cv::line_descriptor::BinaryDescriptor(*reinterpret_cast<const cv::line_descriptor::BinaryDescriptor::Params*>(parameters));
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor() {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptor> ret = cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor();
			return Ok<void*>(new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(void* parameters) {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptor> ret = cv::line_descriptor::BinaryDescriptor::createBinaryDescriptor(*reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(parameters));
			return Ok<void*>(new cv::Ptr<cv::line_descriptor::BinaryDescriptor>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->getNumOfOctaves();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(void* instance, int octaves) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->setNumOfOctaves(octaves);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_getWidthOfBand(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->getWidthOfBand();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(void* instance, int width) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->setWidthOfBand(width);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_getReductionRatio(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->getReductionRatio();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(void* instance, int rRatio) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->setReductionRatio(rRatio);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_read_const_FileNodeX(void* instance, void* fn) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_detect_const_MatX_vector_KeyLine_X_const_MatX(void* instance, void* image, void* keypoints, void* mask) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->detect(*reinterpret_cast<const cv::Mat*>(image), *reinterpret_cast<std::vector<cv::line_descriptor::KeyLine>*>(keypoints), *reinterpret_cast<const cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_detect_const_const_vector_Mat_X_vector_vector_KeyLine__X_const_vector_Mat_X(void* instance, void* images, void* keylines, void* masks) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->detect(*reinterpret_cast<const std::vector<cv::Mat>*>(images), *reinterpret_cast<std::vector<std::vector<cv::line_descriptor::KeyLine>>*>(keylines), *reinterpret_cast<const std::vector<cv::Mat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_compute_const_const_MatX_vector_KeyLine_X_MatX_bool(void* instance, void* image, void* keylines, void* descriptors, bool returnFloatDescr) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->compute(*reinterpret_cast<const cv::Mat*>(image), *reinterpret_cast<std::vector<cv::line_descriptor::KeyLine>*>(keylines), *reinterpret_cast<cv::Mat*>(descriptors), returnFloatDescr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_compute_const_const_vector_Mat_X_vector_vector_KeyLine__X_vector_Mat_X_bool(void* instance, void* images, void* keylines, void* descriptors, bool returnFloatDescr) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->compute(*reinterpret_cast<const std::vector<cv::Mat>*>(images), *reinterpret_cast<std::vector<std::vector<cv::line_descriptor::KeyLine>>*>(keylines), *reinterpret_cast<std::vector<cv::Mat>*>(descriptors), returnFloatDescr);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_descriptorSize_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->descriptorSize();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_descriptorType_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->descriptorType();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_defaultNorm_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor*>(instance)->defaultNorm();
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_Params_numOfOctave__const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->numOfOctave_;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_Params_setNumOfOctave__int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->numOfOctave_ = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_Params_widthOfBand__const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->widthOfBand_;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_Params_setWidthOfBand__int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->widthOfBand_ = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_Params_reductionRatio_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->reductionRatio;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_Params_setReductionRatio_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->reductionRatio = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_BinaryDescriptor_Params_ksize__const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->ksize_;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_Params_setKsize__int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->ksize_ = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BinaryDescriptor_Params_delete(cv::line_descriptor::BinaryDescriptor::Params* instance) {
		delete instance;
	}
	Result<void*> cv_line_descriptor_BinaryDescriptor_Params_Params() {
		try {
			cv::line_descriptor::BinaryDescriptor::Params* ret = new cv::line_descriptor::BinaryDescriptor::Params();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeX(void* instance, void* fn) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->read(*reinterpret_cast<const cv::FileNode*>(fn));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageX(void* instance, void* fs) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptor::Params*>(instance)->write(*reinterpret_cast<cv::FileStorage*>(fs));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_BinaryDescriptorMatcher_delete(cv::line_descriptor::BinaryDescriptorMatcher* instance) {
		delete instance;
	}
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatX_const_MatX_vector_DMatch_X_const_MatX(void* instance, void* queryDescriptors, void* trainDescriptors, void* matches, void* mask) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->match(*reinterpret_cast<const cv::Mat*>(queryDescriptors), *reinterpret_cast<const cv::Mat*>(trainDescriptors), *reinterpret_cast<std::vector<cv::DMatch>*>(matches), *reinterpret_cast<const cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatX_vector_DMatch_X_const_vector_Mat_X(void* instance, void* queryDescriptors, void* matches, void* masks) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->match(*reinterpret_cast<const cv::Mat*>(queryDescriptors), *reinterpret_cast<std::vector<cv::DMatch>*>(matches), *reinterpret_cast<const std::vector<cv::Mat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatX_const_MatX_vector_vector_DMatch__X_int_const_MatX_bool(void* instance, void* queryDescriptors, void* trainDescriptors, void* matches, int k, void* mask, bool compactResult) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->knnMatch(*reinterpret_cast<const cv::Mat*>(queryDescriptors), *reinterpret_cast<const cv::Mat*>(trainDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), k, *reinterpret_cast<const cv::Mat*>(mask), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatX_vector_vector_DMatch__X_int_const_vector_Mat_X_bool(void* instance, void* queryDescriptors, void* matches, int k, void* masks, bool compactResult) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->knnMatch(*reinterpret_cast<const cv::Mat*>(queryDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), k, *reinterpret_cast<const std::vector<cv::Mat>*>(masks), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatX_const_MatX_vector_vector_DMatch__X_float_const_MatX_bool(void* instance, void* queryDescriptors, void* trainDescriptors, void* matches, float maxDistance, void* mask, bool compactResult) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->radiusMatch(*reinterpret_cast<const cv::Mat*>(queryDescriptors), *reinterpret_cast<const cv::Mat*>(trainDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), maxDistance, *reinterpret_cast<const cv::Mat*>(mask), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatX_vector_vector_DMatch__X_float_const_vector_Mat_X_bool(void* instance, void* queryDescriptors, void* matches, float maxDistance, void* masks, bool compactResult) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->radiusMatch(*reinterpret_cast<const cv::Mat*>(queryDescriptors), *reinterpret_cast<std::vector<std::vector<cv::DMatch>>*>(matches), maxDistance, *reinterpret_cast<const std::vector<cv::Mat>*>(masks), compactResult);
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_add_const_vector_Mat_X(void* instance, void* descriptors) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->add(*reinterpret_cast<const std::vector<cv::Mat>*>(descriptors));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_train(void* instance) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->train();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher() {
		try {
			cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher> ret = cv::line_descriptor::BinaryDescriptorMatcher::createBinaryDescriptorMatcher();
			return Ok<void*>(new cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_line_descriptor_BinaryDescriptorMatcher_clear(void* instance) {
		try {
			reinterpret_cast<cv::line_descriptor::BinaryDescriptorMatcher*>(instance)->clear();
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<void*> cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher() {
		try {
			cv::line_descriptor::BinaryDescriptorMatcher* ret = new cv::line_descriptor::BinaryDescriptorMatcher();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_DrawLinesMatchesFlags_delete(cv::line_descriptor::DrawLinesMatchesFlags* instance) {
		delete instance;
	}
	Result<float> cv_line_descriptor_KeyLine_angle_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->angle;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setAngle_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->angle = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_KeyLine_class_id_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->class_id;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setClass_id_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->class_id = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_KeyLine_octave_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->octave;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setOctave_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->octave = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<cv::Point2f> cv_line_descriptor_KeyLine_pt_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->pt;
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setPt_Point2f(void* instance, cv::Point2f val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->pt = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_response_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->response;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setResponse_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->response = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_size_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->size;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setSize_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->size = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_startPointX_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->startPointX;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setStartPointX_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->startPointX = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_startPointY_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->startPointY;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setStartPointY_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->startPointY = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_endPointX_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->endPointX;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setEndPointX_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->endPointX = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_endPointY_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->endPointY;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setEndPointY_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->endPointY = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_sPointInOctaveX_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->sPointInOctaveX;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setSPointInOctaveX_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->sPointInOctaveX = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_sPointInOctaveY_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->sPointInOctaveY;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setSPointInOctaveY_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->sPointInOctaveY = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_ePointInOctaveX_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->ePointInOctaveX;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setEPointInOctaveX_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->ePointInOctaveX = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_ePointInOctaveY_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->ePointInOctaveY;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setEPointInOctaveY_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->ePointInOctaveY = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<float> cv_line_descriptor_KeyLine_lineLength_const(void* instance) {
		try {
			float ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->lineLength;
			return Ok<float>(ret);
		} OCVRS_CATCH(Result<float>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setLineLength_float(void* instance, float val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->lineLength = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result<int> cv_line_descriptor_KeyLine_numOfPixels_const(void* instance) {
		try {
			int ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->numOfPixels;
			return Ok<int>(ret);
		} OCVRS_CATCH(Result<int>)
	}
	
	Result_void cv_line_descriptor_KeyLine_setNumOfPixels_int(void* instance, int val) {
		try {
			reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->numOfPixels = val;
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	void cv_KeyLine_delete(cv::line_descriptor::KeyLine* instance) {
		delete instance;
	}
	Result<cv::Point2f> cv_line_descriptor_KeyLine_getStartPoint_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->getStartPoint();
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Point2f> cv_line_descriptor_KeyLine_getEndPoint_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->getEndPoint();
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Point2f> cv_line_descriptor_KeyLine_getStartPointInOctave_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->getStartPointInOctave();
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<cv::Point2f> cv_line_descriptor_KeyLine_getEndPointInOctave_const(void* instance) {
		try {
			cv::Point2f ret = reinterpret_cast<cv::line_descriptor::KeyLine*>(instance)->getEndPointInOctave();
			return Ok<cv::Point2f>(ret);
		} OCVRS_CATCH(Result<cv::Point2f>)
	}
	
	Result<void*> cv_line_descriptor_KeyLine_KeyLine() {
		try {
			cv::line_descriptor::KeyLine* ret = new cv::line_descriptor::KeyLine();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	void cv_LSDDetector_delete(cv::line_descriptor::LSDDetector* instance) {
		delete instance;
	}
	Result<void*> cv_line_descriptor_LSDDetector_LSDDetector() {
		try {
			cv::line_descriptor::LSDDetector* ret = new cv::line_descriptor::LSDDetector();
			return Ok<void*>(ret);
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result<void*> cv_line_descriptor_LSDDetector_createLSDDetector() {
		try {
			cv::Ptr<cv::line_descriptor::LSDDetector> ret = cv::line_descriptor::LSDDetector::createLSDDetector();
			return Ok<void*>(new cv::Ptr<cv::line_descriptor::LSDDetector>(ret));
		} OCVRS_CATCH(Result<void*>)
	}
	
	Result_void cv_line_descriptor_LSDDetector_detect_const_MatX_vector_KeyLine_X_int_int_const_MatX(void* instance, void* image, void* keypoints, int scale, int numOctaves, void* mask) {
		try {
			reinterpret_cast<cv::line_descriptor::LSDDetector*>(instance)->detect(*reinterpret_cast<const cv::Mat*>(image), *reinterpret_cast<std::vector<cv::line_descriptor::KeyLine>*>(keypoints), scale, numOctaves, *reinterpret_cast<const cv::Mat*>(mask));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
	Result_void cv_line_descriptor_LSDDetector_detect_const_const_vector_Mat_X_vector_vector_KeyLine__X_int_int_const_vector_Mat_X(void* instance, void* images, void* keylines, int scale, int numOctaves, void* masks) {
		try {
			reinterpret_cast<cv::line_descriptor::LSDDetector*>(instance)->detect(*reinterpret_cast<const std::vector<cv::Mat>*>(images), *reinterpret_cast<std::vector<std::vector<cv::line_descriptor::KeyLine>>*>(keylines), scale, numOctaves, *reinterpret_cast<const std::vector<cv::Mat>*>(masks));
			return Ok();
		} OCVRS_CATCH(Result_void)
	}
	
}
