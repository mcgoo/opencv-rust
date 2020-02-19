template struct Result<bool>;
template struct Result<const char*>;
template struct Result<cv::VideoCaptureAPIs>;
template struct Result<double>;
template struct Result<int>;
template struct Result<void*>;
extern "C" {
	void cv_VectorOfVideoCapture_delete(std::vector<cv::VideoCapture>* instance) {
		delete instance;
	}

	void* cv_VectorOfVideoCapture_new() {
		return new std::vector<cv::VideoCapture>();
	}

	size_t cv_VectorOfVideoCapture_len(const std::vector<cv::VideoCapture>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVideoCapture_is_empty(const std::vector<cv::VideoCapture>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVideoCapture_capacity(const std::vector<cv::VideoCapture>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVideoCapture_shrink_to_fit(std::vector<cv::VideoCapture>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVideoCapture_reserve(std::vector<cv::VideoCapture>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVideoCapture_remove(std::vector<cv::VideoCapture>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVideoCapture_swap(std::vector<cv::VideoCapture>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVideoCapture_clear(std::vector<cv::VideoCapture>* instance) {
		instance->clear();
	}

	void cv_VectorOfVideoCapture_push(std::vector<cv::VideoCapture>* instance, cv::VideoCapture* val) {
		instance->push_back(*val);
	}
	
	void cv_VectorOfVideoCapture_insert(std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture* val) {
		instance->insert(instance->begin() + index, *val);
	}
	
	Result<void*> cv_VectorOfVideoCapture_get(const std::vector<cv::VideoCapture>* instance, size_t index) {
		try {
			return Ok<void*>(new cv::VideoCapture(instance->at(index)));
		} VEC_CATCH(Result<void*>)
	}
	
	void* cv_VectorOfVideoCapture_get_unchecked(const std::vector<cv::VideoCapture>* instance, size_t index) {
		return new cv::VideoCapture((*instance)[index]);
	}
	
	Result_void cv_VectorOfVideoCapture_set(std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture* val) {
		try {
			instance->at(index) = *val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfVideoCapture_set_unchecked(std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture* val) {
		(*instance)[index] = *val;
	}
	
}


extern "C" {
	void cv_VectorOfVideoCaptureAPIs_delete(std::vector<cv::VideoCaptureAPIs>* instance) {
		delete instance;
	}

	void* cv_VectorOfVideoCaptureAPIs_new() {
		return new std::vector<cv::VideoCaptureAPIs>();
	}

	size_t cv_VectorOfVideoCaptureAPIs_len(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->size();
	}

	bool cv_VectorOfVideoCaptureAPIs_is_empty(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfVideoCaptureAPIs_capacity(const std::vector<cv::VideoCaptureAPIs>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfVideoCaptureAPIs_shrink_to_fit(std::vector<cv::VideoCaptureAPIs>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfVideoCaptureAPIs_reserve(std::vector<cv::VideoCaptureAPIs>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfVideoCaptureAPIs_remove(std::vector<cv::VideoCaptureAPIs>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfVideoCaptureAPIs_swap(std::vector<cv::VideoCaptureAPIs>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfVideoCaptureAPIs_clear(std::vector<cv::VideoCaptureAPIs>* instance) {
		instance->clear();
	}

	void cv_VectorOfVideoCaptureAPIs_push(std::vector<cv::VideoCaptureAPIs>* instance, cv::VideoCaptureAPIs val) {
		instance->push_back(val);
	}
	
	void cv_VectorOfVideoCaptureAPIs_insert(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs val) {
		instance->insert(instance->begin() + index, val);
	}
	
	Result<cv::VideoCaptureAPIs> cv_VectorOfVideoCaptureAPIs_get(const std::vector<cv::VideoCaptureAPIs>* instance, size_t index) {
		try {
			return Ok<cv::VideoCaptureAPIs>(instance->at(index));
		} VEC_CATCH(Result<cv::VideoCaptureAPIs>)
	}
	
	Result_void cv_VectorOfVideoCaptureAPIs_set(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs val) {
		try {
			instance->at(index) = val;
			return Ok();
		} VEC_CATCH(Result_void)
	}
	
	void cv_VectorOfVideoCaptureAPIs_set_unchecked(std::vector<cv::VideoCaptureAPIs>* instance, size_t index, cv::VideoCaptureAPIs val) {
		(*instance)[index] = val;
	}
	
	cv::VideoCaptureAPIs cv_VectorOfVideoCaptureAPIs_get_unchecked(const std::vector<cv::VideoCaptureAPIs>* instance, size_t index) {
		return (*instance)[index];
	}
	
	const void** cv_VectorOfVideoCaptureAPIs_data(std::vector<cv::VideoCaptureAPIs>* instance) {
		return reinterpret_cast<const void**>(instance->data());
	}
	
}


