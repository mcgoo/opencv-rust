template struct Result<bool>;
template struct Result<const char*>;
template struct Result<cv::Rect_<int>>;
template struct Result<double>;
template struct Result<double(*)[2]>;
template struct Result<double(*)[3]>;
template struct Result<float>;
template struct Result<int>;
template struct Result<void*>;
extern "C" void cv_PtrOfERFilter_delete(cv::Ptr<cv::text::ERFilter>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfERFilter_get_inner_ptr(cv::Ptr<cv::text::ERFilter>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfERFilter_Callback_delete(cv::Ptr<cv::text::ERFilter::Callback>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfERFilter_Callback_get_inner_ptr(cv::Ptr<cv::text::ERFilter::Callback>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfOCRBeamSearchDecoder_delete(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete(cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr(cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfOCRHMMDecoder_delete(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfOCRHMMDecoder_get_inner_ptr(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete(cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr(cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfOCRHolisticWordRecognizer_delete(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfOCRTesseract_delete(cv::Ptr<cv::text::OCRTesseract>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfOCRTesseract_get_inner_ptr(cv::Ptr<cv::text::OCRTesseract>* instance) {
	return instance->get();
}

extern "C" void cv_PtrOfTextDetectorCNN_delete(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
	delete instance;
}

extern "C" void* cv_PtrOfTextDetectorCNN_get_inner_ptr(cv::Ptr<cv::text::TextDetectorCNN>* instance) {
	return instance->get();
}

extern "C" {
	void cv_VectorOfERStat_delete(std::vector<cv::text::ERStat>* instance) {
		delete instance;
	}

	void* cv_VectorOfERStat_new() {
		return new std::vector<cv::text::ERStat>();
	}

	size_t cv_VectorOfERStat_len(const std::vector<cv::text::ERStat>* instance) {
		return instance->size();
	}

	bool cv_VectorOfERStat_is_empty(const std::vector<cv::text::ERStat>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfERStat_capacity(const std::vector<cv::text::ERStat>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfERStat_shrink_to_fit(std::vector<cv::text::ERStat>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfERStat_reserve(std::vector<cv::text::ERStat>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfERStat_remove(std::vector<cv::text::ERStat>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfERStat_swap(std::vector<cv::text::ERStat>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfERStat_clear(std::vector<cv::text::ERStat>* instance) {
		instance->clear();
	}

	void cv_VectorOfERStat_push(std::vector<cv::text::ERStat>* instance, cv::text::ERStat* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfERStat_insert(std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfERStat_get(const std::vector<cv::text::ERStat>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::text::ERStat(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfERStat_get_unchecked(const std::vector<cv::text::ERStat>* instance, size_t index) {
		return new cv::text::ERStat((*instance)[index]);
	}
	
	Result_void cv_VectorOfERStat_set(std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfERStat_set_unchecked(std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat* val) {
		(*instance)[index] = *val;
	}
	
}


extern "C" {
	void cv_VectorOfVectorOfERStat_delete(std::vector<std::vector<cv::text::ERStat>>* instance) {
		delete instance;
	}

	void* cv_VectorOfVectorOfERStat_new() {
		return new std::vector<std::vector<cv::text::ERStat>>();
	}

	size_t cv_VectorOfVectorOfERStat_len(const std::vector<std::vector<cv::text::ERStat>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVectorOfERStat_is_empty(const std::vector<std::vector<cv::text::ERStat>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVectorOfERStat_capacity(const std::vector<std::vector<cv::text::ERStat>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVectorOfERStat_shrink_to_fit(std::vector<std::vector<cv::text::ERStat>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVectorOfERStat_reserve(std::vector<std::vector<cv::text::ERStat>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVectorOfERStat_remove(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVectorOfERStat_swap(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVectorOfERStat_clear(std::vector<std::vector<cv::text::ERStat>>* instance) {
		instance->clear();
	}

	void cv_VectorOfVectorOfERStat_push(std::vector<std::vector<cv::text::ERStat>>* instance, std::vector<cv::text::ERStat>* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfVectorOfERStat_insert(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfVectorOfERStat_get(const std::vector<std::vector<cv::text::ERStat>>* instance, size_t index) {
		try {
			return Ok<void*>(new std::vector<cv::text::ERStat>(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfVectorOfERStat_get_unchecked(const std::vector<std::vector<cv::text::ERStat>>* instance, size_t index) {
		return new std::vector<cv::text::ERStat>((*instance)[index]);
	}
	
	Result_void cv_VectorOfVectorOfERStat_set(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfVectorOfERStat_set_unchecked(std::vector<std::vector<cv::text::ERStat>>* instance, size_t index, std::vector<cv::text::ERStat>* val) {
		(*instance)[index] = *val;
	}
	
}


