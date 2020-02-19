template struct Result<bool>;
template struct Result<const char*>;
template struct Result<cv::Point_<float>>;
template struct Result<cv::Point_<int>>;
template struct Result<cv::Rect_<int>>;
template struct Result<cv::Size_<int>>;
template struct Result<cv::Stitcher::Status>;
template struct Result<cv::detail::DpSeamFinder::CostFunction>;
template struct Result<cv::detail::WaveCorrectKind>;
template struct Result<double>;
template struct Result<float>;
template struct Result<float(*)[3]>;
template struct Result<float(*)[9]>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfDetail_Blender_delete(cv::Ptr<cv::detail::Blender>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_Blender_get_inner_ptr(cv::Ptr<cv::detail::Blender>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_BundleAdjusterBase_delete(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_ExposureCompensator_delete(cv::Ptr<cv::detail::ExposureCompensator>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_ExposureCompensator_get_inner_ptr(cv::Ptr<cv::detail::ExposureCompensator>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_FeaturesFinder_delete(cv::Ptr<cv::detail::FeaturesFinder>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_FeaturesFinder_get_inner_ptr(cv::Ptr<cv::detail::FeaturesFinder>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_FeaturesMatcher_delete(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_FeaturesMatcher_get_inner_ptr(cv::Ptr<cv::detail::FeaturesMatcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_RotationWarper_delete(cv::Ptr<cv::detail::RotationWarper>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_RotationWarper_get_inner_ptr(cv::Ptr<cv::detail::RotationWarper>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfDetail_SeamFinder_delete(cv::Ptr<cv::detail::SeamFinder>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfDetail_SeamFinder_get_inner_ptr(cv::Ptr<cv::detail::SeamFinder>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfStitcher_delete(cv::Ptr<cv::Stitcher>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfStitcher_get_inner_ptr(cv::Ptr<cv::Stitcher>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfWarperCreator_delete(cv::Ptr<cv::WarperCreator>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfWarperCreator_get_inner_ptr(cv::Ptr<cv::WarperCreator>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfDetail_CameraParams_delete(std::vector<cv::detail::CameraParams>* instance) {
		delete instance;
	}

	void* cv_VectorOfDetail_CameraParams_new() {
		return new std::vector<cv::detail::CameraParams>();
	}

	size_t cv_VectorOfDetail_CameraParams_len(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_CameraParams_is_empty(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_CameraParams_capacity(const std::vector<cv::detail::CameraParams>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetail_CameraParams_shrink_to_fit(std::vector<cv::detail::CameraParams>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_CameraParams_reserve(std::vector<cv::detail::CameraParams>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_CameraParams_remove(std::vector<cv::detail::CameraParams>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_CameraParams_swap(std::vector<cv::detail::CameraParams>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_CameraParams_clear(std::vector<cv::detail::CameraParams>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_CameraParams_push(std::vector<cv::detail::CameraParams>* instance, cv::detail::CameraParams* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfDetail_CameraParams_insert(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfDetail_CameraParams_get(const std::vector<cv::detail::CameraParams>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::detail::CameraParams(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfDetail_CameraParams_get_unchecked(const std::vector<cv::detail::CameraParams>* instance, size_t index) {
		return new cv::detail::CameraParams((*instance)[index]);
	}
	
	Result_void cv_VectorOfDetail_CameraParams_set(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfDetail_CameraParams_set_unchecked(std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams* val) {
		(*instance)[index] = *val;
	}
	
}


extern "C" {
	void cv_VectorOfDetail_ImageFeatures_delete(std::vector<cv::detail::ImageFeatures>* instance) {
		delete instance;
	}

	void* cv_VectorOfDetail_ImageFeatures_new() {
		return new std::vector<cv::detail::ImageFeatures>();
	}

	size_t cv_VectorOfDetail_ImageFeatures_len(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_ImageFeatures_is_empty(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_ImageFeatures_capacity(const std::vector<cv::detail::ImageFeatures>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetail_ImageFeatures_shrink_to_fit(std::vector<cv::detail::ImageFeatures>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_ImageFeatures_reserve(std::vector<cv::detail::ImageFeatures>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_ImageFeatures_remove(std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_ImageFeatures_swap(std::vector<cv::detail::ImageFeatures>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_ImageFeatures_clear(std::vector<cv::detail::ImageFeatures>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_ImageFeatures_push(std::vector<cv::detail::ImageFeatures>* instance, cv::detail::ImageFeatures* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfDetail_ImageFeatures_insert(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfDetail_ImageFeatures_get(const std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::detail::ImageFeatures(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfDetail_ImageFeatures_get_unchecked(const std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
		return new cv::detail::ImageFeatures((*instance)[index]);
	}
	
	Result_void cv_VectorOfDetail_ImageFeatures_set(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfDetail_ImageFeatures_set_unchecked(std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures* val) {
		(*instance)[index] = *val;
	}
	
}


extern "C" {
	void cv_VectorOfDetail_MatchesInfo_delete(std::vector<cv::detail::MatchesInfo>* instance) {
		delete instance;
	}

	void* cv_VectorOfDetail_MatchesInfo_new() {
		return new std::vector<cv::detail::MatchesInfo>();
	}

	size_t cv_VectorOfDetail_MatchesInfo_len(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->size();
	}

	bool cv_VectorOfDetail_MatchesInfo_is_empty(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfDetail_MatchesInfo_capacity(const std::vector<cv::detail::MatchesInfo>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfDetail_MatchesInfo_shrink_to_fit(std::vector<cv::detail::MatchesInfo>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfDetail_MatchesInfo_reserve(std::vector<cv::detail::MatchesInfo>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfDetail_MatchesInfo_remove(std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfDetail_MatchesInfo_swap(std::vector<cv::detail::MatchesInfo>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfDetail_MatchesInfo_clear(std::vector<cv::detail::MatchesInfo>* instance) {
		instance->clear();
	}

	void cv_VectorOfDetail_MatchesInfo_push(std::vector<cv::detail::MatchesInfo>* instance, cv::detail::MatchesInfo* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfDetail_MatchesInfo_insert(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfDetail_MatchesInfo_get(const std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::detail::MatchesInfo(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfDetail_MatchesInfo_get_unchecked(const std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
		return new cv::detail::MatchesInfo((*instance)[index]);
	}
	
	Result_void cv_VectorOfDetail_MatchesInfo_set(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfDetail_MatchesInfo_set_unchecked(std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo* val) {
		(*instance)[index] = *val;
	}
	
}


