use crate::{mod_prelude::*, core, types, sys};

mod core_types {
    use super::*;

    pub struct PtrOfConjGradSolver {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfConjGradSolver {
        #[inline(always)] pub fn as_raw_PtrOfConjGradSolver(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfConjGradSolver {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ConjGradSolver>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfConjGradSolver {}
    
    impl PtrOfConjGradSolver {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ConjGradSolver>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> ConjGradSolverRef {
            let inner = core::ConjGradSolver { ptr: self.get_inner() };
            ConjGradSolverRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> ConjGradSolverRefMut {
            let inner = core::ConjGradSolver { ptr: self.get_inner() };
            ConjGradSolverRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct ConjGradSolverRef<'o> {
        inner: std::mem::ManuallyDrop<core::ConjGradSolver>,
        owner: std::marker::PhantomData<&'o types::PtrOfConjGradSolver>,
    }
    
    impl std::ops::Deref for ConjGradSolverRef<'_> {
        type Target = core::ConjGradSolver;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct ConjGradSolverRefMut<'o> {
        inner: std::mem::ManuallyDrop<core::ConjGradSolver>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfConjGradSolver>,
    }
    
    impl std::ops::Deref for ConjGradSolverRefMut<'_> {
        type Target = core::ConjGradSolver;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for ConjGradSolverRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfDownhillSolver {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDownhillSolver {
        #[inline(always)] pub fn as_raw_PtrOfDownhillSolver(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDownhillSolver {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::DownhillSolver>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDownhillSolver {}
    
    impl core::AlgorithmTrait for PtrOfDownhillSolver {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl core::DownhillSolver for PtrOfDownhillSolver {
        #[inline(always)] fn as_raw_DownhillSolver(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::DownhillSolver>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl core::MinProblemSolver for PtrOfDownhillSolver {
        #[inline(always)] fn as_raw_MinProblemSolver(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MinProblemSolver>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFormatted {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFormatted {
        #[inline(always)] pub fn as_raw_PtrOfFormatted(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFormatted {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::Formatted>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFormatted {}
    
    impl core::Formatted for PtrOfFormatted {
        #[inline(always)] fn as_raw_Formatted(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Formatted>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFormatter {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFormatter {
        #[inline(always)] pub fn as_raw_PtrOfFormatter(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFormatter {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::Formatter>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFormatter {}
    
    impl core::Formatter for PtrOfFormatter {
        #[inline(always)] fn as_raw_Formatter(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Formatter>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFunction {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFunction {
        #[inline(always)] pub fn as_raw_PtrOfFunction(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFunction {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::MinProblemSolver::Function>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFunction {}
    
    impl core::MinProblemSolver_Function for PtrOfFunction {
        #[inline(always)] fn as_raw_MinProblemSolver_Function(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MinProblemSolver::Function>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct VectorOfDMatch {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfDMatch {
        #[inline(always)] pub fn as_raw_VectorOfDMatch(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::DMatch] {
            unsafe {
                let vec = self.as_raw_VectorOfDMatch();
                let data = cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] -> *const core::DMatch as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfDMatch {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfDMatch {
        type Item = core::DMatch;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfDMatch {
        type Item = core::DMatch;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDMatch>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfDMatch {
        type Storage = core::DMatch;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::DMatch>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<cv::DMatch>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<cv::DMatch>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<cv::DMatch>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfDMatch();
                cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::DMatch;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*", val as "cv::DMatch"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t", val as "cv::DMatch"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<cv::DMatch>*", index as "size_t"] -> crate::sys::cv_return_value_DMatch as "cv_return_value_DMatch" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_DMatch)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<cv::DMatch>*", index as "size_t"] -> core::DMatch as "cv::DMatch" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t", val as "cv::DMatch"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<cv::DMatch>*", index as "size_t", val as "cv::DMatch"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfDMatch {}
    
    pub struct VectorOfKeyPoint {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfKeyPoint {
        #[inline(always)] pub fn as_raw_VectorOfKeyPoint(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::KeyPoint] {
            unsafe {
                let vec = self.as_raw_VectorOfKeyPoint();
                let data = cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] -> *const core::KeyPoint as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfKeyPoint {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfKeyPoint {
        type Item = core::KeyPoint;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfKeyPoint {
        type Item = core::KeyPoint;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfKeyPoint>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfKeyPoint {
        type Storage = core::KeyPoint;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::KeyPoint>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfKeyPoint();
                cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::KeyPoint;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", val as "cv::KeyPoint"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t", val as "cv::KeyPoint"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*", index as "size_t"] -> crate::sys::cv_return_value_KeyPoint as "cv_return_value_KeyPoint" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_KeyPoint)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<cv::KeyPoint>*", index as "size_t"] -> core::KeyPoint as "cv::KeyPoint" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t", val as "cv::KeyPoint"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<cv::KeyPoint>*", index as "size_t", val as "cv::KeyPoint"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfKeyPoint {}
    
    pub struct VectorOfMat {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfMat {
        #[inline(always)] pub fn as_raw_VectorOfMat(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfMat {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfMat {
        type Item = core::Mat;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfMat {
        type Item = core::Mat;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfMat>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfMat {
        type Storage = core::Mat;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Mat>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "const std::vector<cv::Mat>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "const std::vector<cv::Mat>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "const std::vector<cv::Mat>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfMat();
                cpp!(unsafe [vec as "std::vector<cv::Mat>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Mat;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfMat();
            let val = val.as_raw_Mat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*", val as "cv::Mat*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfMat();
            let val = val.as_raw_Mat();
            cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t", val as "cv::Mat*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfMat();
            cpp!(unsafe [vec as "const std::vector<cv::Mat>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::Mat(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| core::Mat { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfMat();
            core::Mat { ptr: cpp!(unsafe [vec as "const std::vector<cv::Mat>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::Mat((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfMat();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t", val as "cv::Mat*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfMat();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::Mat>*", index as "size_t", val as "cv::Mat*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfMat {}
    
    pub struct VectorOfPlatformInfo {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPlatformInfo {
        #[inline(always)] pub fn as_raw_VectorOfPlatformInfo(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfPlatformInfo {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPlatformInfo {
        type Item = core::PlatformInfo;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPlatformInfo {
        type Item = core::PlatformInfo;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPlatformInfo>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPlatformInfo {
        type Storage = core::PlatformInfo;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::ocl::PlatformInfo>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "const std::vector<cv::ocl::PlatformInfo>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "const std::vector<cv::ocl::PlatformInfo>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "const std::vector<cv::ocl::PlatformInfo>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPlatformInfo();
                cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::PlatformInfo;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPlatformInfo();
            let val = val.as_raw_PlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", val as "cv::ocl::PlatformInfo*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPlatformInfo();
            let val = val.as_raw_PlatformInfo();
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", index as "size_t", val as "cv::ocl::PlatformInfo*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPlatformInfo();
            cpp!(unsafe [vec as "const std::vector<cv::ocl::PlatformInfo>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::ocl::PlatformInfo(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| core::PlatformInfo { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPlatformInfo();
            core::PlatformInfo { ptr: cpp!(unsafe [vec as "const std::vector<cv::ocl::PlatformInfo>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::ocl::PlatformInfo((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPlatformInfo();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", index as "size_t", val as "cv::ocl::PlatformInfo*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPlatformInfo();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::ocl::PlatformInfo>*", index as "size_t", val as "cv::ocl::PlatformInfo*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfPlatformInfo {}
    
    pub struct VectorOfPoint {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPoint {
        #[inline(always)] pub fn as_raw_VectorOfPoint(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Point] {
            unsafe {
                let vec = self.as_raw_VectorOfPoint();
                let data = cpp!(unsafe [vec as "std::vector<cv::Point>*"] -> *const core::Point as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfPoint {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPoint {
        type Item = core::Point;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPoint {
        type Item = core::Point;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPoint {
        type Storage = core::Point;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Point>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<cv::Point>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<cv::Point>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<cv::Point>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPoint();
                cpp!(unsafe [vec as "std::vector<cv::Point>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Point;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*", val as "cv::Point"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t", val as "cv::Point"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<PointWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_PointWrapper as "cv_return_value_PointWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_PointWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<PointWrapper>*", index as "size_t"] -> core::Point as "PointWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t", val as "cv::Point"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<cv::Point>*", index as "size_t", val as "cv::Point"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfPoint {}
    
    impl core::ToInputArray for VectorOfPoint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfPoint();
            cpp!(unsafe [me as "std::vector<cv::Point>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfPoint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfPoint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfPoint();
            cpp!(unsafe [me as "std::vector<cv::Point>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfPoint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfPoint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfPoint();
            cpp!(unsafe [me as "std::vector<cv::Point>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfPoint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfPoint2d {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPoint2d {
        #[inline(always)] pub fn as_raw_VectorOfPoint2d(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Point2d] {
            unsafe {
                let vec = self.as_raw_VectorOfPoint2d();
                let data = cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] -> *const core::Point2d as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfPoint2d {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPoint2d {
        type Item = core::Point2d;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPoint2d {
        type Item = core::Point2d;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint2d>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPoint2d {
        type Storage = core::Point2d;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Point2d>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "const std::vector<cv::Point2d>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "const std::vector<cv::Point2d>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "const std::vector<cv::Point2d>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPoint2d();
                cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Point2d;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*", val as "cv::Point2d"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t", val as "cv::Point2d"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "const std::vector<Point2dWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point2dWrapper as "cv_return_value_Point2dWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Point2dWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "const std::vector<Point2dWrapper>*", index as "size_t"] -> core::Point2d as "Point2dWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t", val as "cv::Point2d"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [vec as "std::vector<cv::Point2d>*", index as "size_t", val as "cv::Point2d"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfPoint2d {}
    
    impl core::ToInputArray for VectorOfPoint2d {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [me as "std::vector<cv::Point2d>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfPoint2d {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfPoint2d {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [me as "std::vector<cv::Point2d>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfPoint2d {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfPoint2d {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfPoint2d();
            cpp!(unsafe [me as "std::vector<cv::Point2d>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfPoint2d {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfPoint2f {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPoint2f {
        #[inline(always)] pub fn as_raw_VectorOfPoint2f(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Point2f] {
            unsafe {
                let vec = self.as_raw_VectorOfPoint2f();
                let data = cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] -> *const core::Point2f as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfPoint2f {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPoint2f {
        type Item = core::Point2f;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPoint2f {
        type Item = core::Point2f;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint2f>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPoint2f {
        type Storage = core::Point2f;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Point2f>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<cv::Point2f>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<cv::Point2f>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<cv::Point2f>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPoint2f();
                cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Point2f;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*", val as "cv::Point2f"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t", val as "cv::Point2f"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<Point2fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point2fWrapper as "cv_return_value_Point2fWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Point2fWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<Point2fWrapper>*", index as "size_t"] -> core::Point2f as "Point2fWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t", val as "cv::Point2f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<cv::Point2f>*", index as "size_t", val as "cv::Point2f"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfPoint2f {}
    
    impl core::ToInputArray for VectorOfPoint2f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [me as "std::vector<cv::Point2f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfPoint2f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfPoint2f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [me as "std::vector<cv::Point2f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfPoint2f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfPoint2f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfPoint2f();
            cpp!(unsafe [me as "std::vector<cv::Point2f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfPoint2f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfPoint3d {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPoint3d {
        #[inline(always)] pub fn as_raw_VectorOfPoint3d(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Point3d] {
            unsafe {
                let vec = self.as_raw_VectorOfPoint3d();
                let data = cpp!(unsafe [vec as "std::vector<cv::Point3d>*"] -> *const core::Point3d as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfPoint3d {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPoint3d {
        type Item = core::Point3d;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPoint3d {
        type Item = core::Point3d;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint3d>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPoint3d {
        type Storage = core::Point3d;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Point3d>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<cv::Point3d>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<cv::Point3d>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<cv::Point3d>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPoint3d();
                cpp!(unsafe [vec as "std::vector<cv::Point3d>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Point3d;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*", val as "cv::Point3d"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*", index as "size_t", val as "cv::Point3d"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<Point3dWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point3dWrapper as "cv_return_value_Point3dWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Point3dWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<Point3dWrapper>*", index as "size_t"] -> core::Point3d as "Point3dWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*", index as "size_t", val as "cv::Point3d"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<cv::Point3d>*", index as "size_t", val as "cv::Point3d"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfPoint3d {}
    
    impl core::ToInputArray for VectorOfPoint3d {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [me as "std::vector<cv::Point3d>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfPoint3d {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfPoint3d {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [me as "std::vector<cv::Point3d>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfPoint3d {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfPoint3d {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfPoint3d();
            cpp!(unsafe [me as "std::vector<cv::Point3d>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfPoint3d {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfPoint3f {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPoint3f {
        #[inline(always)] pub fn as_raw_VectorOfPoint3f(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Point3f] {
            unsafe {
                let vec = self.as_raw_VectorOfPoint3f();
                let data = cpp!(unsafe [vec as "std::vector<cv::Point3f>*"] -> *const core::Point3f as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfPoint3f {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPoint3f {
        type Item = core::Point3f;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPoint3f {
        type Item = core::Point3f;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint3f>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPoint3f {
        type Storage = core::Point3f;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Point3f>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<cv::Point3f>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<cv::Point3f>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<cv::Point3f>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPoint3f();
                cpp!(unsafe [vec as "std::vector<cv::Point3f>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Point3f;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*", val as "cv::Point3f"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*", index as "size_t", val as "cv::Point3f"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<Point3fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point3fWrapper as "cv_return_value_Point3fWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Point3fWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<Point3fWrapper>*", index as "size_t"] -> core::Point3f as "Point3fWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*", index as "size_t", val as "cv::Point3f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<cv::Point3f>*", index as "size_t", val as "cv::Point3f"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfPoint3f {}
    
    impl core::ToInputArray for VectorOfPoint3f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [me as "std::vector<cv::Point3f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfPoint3f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfPoint3f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [me as "std::vector<cv::Point3f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfPoint3f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfPoint3f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfPoint3f();
            cpp!(unsafe [me as "std::vector<cv::Point3f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfPoint3f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfPoint3i {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfPoint3i {
        #[inline(always)] pub fn as_raw_VectorOfPoint3i(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Point3i] {
            unsafe {
                let vec = self.as_raw_VectorOfPoint3i();
                let data = cpp!(unsafe [vec as "std::vector<cv::Point3i>*"] -> *const core::Point3i as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfPoint3i {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfPoint3i {
        type Item = core::Point3i;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfPoint3i {
        type Item = core::Point3i;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfPoint3i>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfPoint3i {
        type Storage = core::Point3i;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Point3i>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<cv::Point3i>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<cv::Point3i>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<cv::Point3i>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfPoint3i();
                cpp!(unsafe [vec as "std::vector<cv::Point3i>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Point3i;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*", val as "cv::Point3i"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*", index as "size_t", val as "cv::Point3i"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<Point3iWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Point3iWrapper as "cv_return_value_Point3iWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Point3iWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<Point3iWrapper>*", index as "size_t"] -> core::Point3i as "Point3iWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*", index as "size_t", val as "cv::Point3i"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<cv::Point3i>*", index as "size_t", val as "cv::Point3i"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfPoint3i {}
    
    impl core::ToInputArray for VectorOfPoint3i {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [me as "std::vector<cv::Point3i>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfPoint3i {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfPoint3i {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [me as "std::vector<cv::Point3i>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfPoint3i {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfPoint3i {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfPoint3i();
            cpp!(unsafe [me as "std::vector<cv::Point3i>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfPoint3i {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfRange {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfRange {
        #[inline(always)] pub fn as_raw_VectorOfRange(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfRange {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "std::vector<cv::Range>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfRange {
        type Item = core::Range;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfRange {
        type Item = core::Range;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRange>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfRange {
        type Storage = core::Range;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Range>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "const std::vector<cv::Range>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "const std::vector<cv::Range>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "const std::vector<cv::Range>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "std::vector<cv::Range>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "std::vector<cv::Range>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfRange();
                cpp!(unsafe [vec as "std::vector<cv::Range>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "std::vector<cv::Range>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Range;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfRange();
            let val = val.as_raw_Range();
            cpp!(unsafe [vec as "std::vector<cv::Range>*", val as "cv::Range*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfRange();
            let val = val.as_raw_Range();
            cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t", val as "cv::Range*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfRange();
            cpp!(unsafe [vec as "const std::vector<cv::Range>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::Range(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| core::Range { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfRange();
            core::Range { ptr: cpp!(unsafe [vec as "const std::vector<cv::Range>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::Range((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfRange();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t", val as "cv::Range*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfRange();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::Range>*", index as "size_t", val as "cv::Range*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfRange {}
    
    pub struct VectorOfRect {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfRect {
        #[inline(always)] pub fn as_raw_VectorOfRect(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Rect] {
            unsafe {
                let vec = self.as_raw_VectorOfRect();
                let data = cpp!(unsafe [vec as "std::vector<cv::Rect>*"] -> *const core::Rect as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfRect {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfRect {
        type Item = core::Rect;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfRect {
        type Item = core::Rect;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfRect>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfRect {
        type Storage = core::Rect;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Rect>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "const std::vector<cv::Rect>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "const std::vector<cv::Rect>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "const std::vector<cv::Rect>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfRect();
                cpp!(unsafe [vec as "std::vector<cv::Rect>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Rect;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*", val as "cv::Rect"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t", val as "cv::Rect"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "const std::vector<RectWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_RectWrapper as "cv_return_value_RectWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_RectWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "const std::vector<RectWrapper>*", index as "size_t"] -> core::Rect as "RectWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t", val as "cv::Rect"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<cv::Rect>*", index as "size_t", val as "cv::Rect"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfRect {}
    
    impl core::ToInputArray for VectorOfRect {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfRect();
            cpp!(unsafe [me as "std::vector<cv::Rect>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfRect {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfRect {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfRect();
            cpp!(unsafe [me as "std::vector<cv::Rect>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfRect {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfRect {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfRect();
            cpp!(unsafe [me as "std::vector<cv::Rect>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfRect {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfString {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfString {
        #[inline(always)] pub fn as_raw_VectorOfString(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfString {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "std::vector<String>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfString {
        type Item = String;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfString {
        type Item = String;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfString>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfString {
        type Storage = String;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<String>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "const std::vector<String>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "const std::vector<String>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "const std::vector<String>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "std::vector<String>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "std::vector<String>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfString();
                cpp!(unsafe [vec as "std::vector<String>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "std::vector<String>*"] {
                vec->clear();
            })
        }
    
        type Arg = &'i str;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfString();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", val as "const char*"] {
                vec->push_back(String(val));
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfString();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] {
                vec->insert(vec->begin() + index, String(val));
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfString();
            cpp!(unsafe [vec as "const std::vector<String>*", index as "size_t"] -> crate::sys::cv_return_value_char_X as "cv_return_value_char_X" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index).c_str() };
                } VEC_CATCH(cv_return_value_char_X)
            }).into_result().map(|x| unsafe { ::std::ffi::CStr::from_ptr(x) }.to_string_lossy().into_owned())
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfString();
            ::std::ffi::CStr::from_ptr(cpp!(unsafe [vec as "const std::vector<String>*", index as "size_t"] -> *mut c_char as "const char*" {
                return (*vec)[index].c_str();
            })).to_string_lossy().into_owned()
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfString();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = String(val);
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfString();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] {
                (*vec)[index] = String(val);
            })
        }
    }
    
    unsafe impl Send for VectorOfString {}
    
    pub struct VectorOfUMat {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfUMat {
        #[inline(always)] pub fn as_raw_VectorOfUMat(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfUMat {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfUMat {
        type Item = core::UMat;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfUMat {
        type Item = core::UMat;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfUMat>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfUMat {
        type Storage = core::UMat;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::UMat>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "const std::vector<cv::UMat>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "const std::vector<cv::UMat>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "const std::vector<cv::UMat>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfUMat();
                cpp!(unsafe [vec as "std::vector<cv::UMat>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::UMat;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfUMat();
            let val = val.as_raw_UMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*", val as "cv::UMat*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfUMat();
            let val = val.as_raw_UMat();
            cpp!(unsafe [vec as "std::vector<cv::UMat>*", index as "size_t", val as "cv::UMat*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfUMat();
            cpp!(unsafe [vec as "const std::vector<cv::UMat>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::UMat(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| core::UMat { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfUMat();
            core::UMat { ptr: cpp!(unsafe [vec as "const std::vector<cv::UMat>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::UMat((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfUMat();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::UMat>*", index as "size_t", val as "cv::UMat*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfUMat();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::UMat>*", index as "size_t", val as "cv::UMat*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfUMat {}
    
    pub struct VectorOfVec4f {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVec4f {
        #[inline(always)] pub fn as_raw_VectorOfVec4f(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Vec4f] {
            unsafe {
                let vec = self.as_raw_VectorOfVec4f();
                let data = cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] -> *const core::Vec4f as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfVec4f {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVec4f {
        type Item = core::Vec4f;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVec4f {
        type Item = core::Vec4f;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec4f>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVec4f {
        type Storage = core::Vec4f;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Vec4f>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "const std::vector<cv::Vec4f>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "const std::vector<cv::Vec4f>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "const std::vector<cv::Vec4f>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVec4f();
                cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Vec4f;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", val as "cv::Vec4f"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t", val as "cv::Vec4f"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "const std::vector<Vec4fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Vec4fWrapper as "cv_return_value_Vec4fWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Vec4fWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "const std::vector<Vec4fWrapper>*", index as "size_t"] -> core::Vec4f as "Vec4fWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t", val as "cv::Vec4f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [vec as "std::vector<cv::Vec4f>*", index as "size_t", val as "cv::Vec4f"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfVec4f {}
    
    impl core::ToInputArray for VectorOfVec4f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [me as "std::vector<cv::Vec4f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVec4f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVec4f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [me as "std::vector<cv::Vec4f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVec4f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVec4f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVec4f();
            cpp!(unsafe [me as "std::vector<cv::Vec4f>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVec4f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVec4i {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVec4i {
        #[inline(always)] pub fn as_raw_VectorOfVec4i(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Vec4i] {
            unsafe {
                let vec = self.as_raw_VectorOfVec4i();
                let data = cpp!(unsafe [vec as "std::vector<cv::Vec4i>*"] -> *const core::Vec4i as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfVec4i {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVec4i {
        type Item = core::Vec4i;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVec4i {
        type Item = core::Vec4i;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec4i>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVec4i {
        type Storage = core::Vec4i;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Vec4i>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "const std::vector<cv::Vec4i>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "const std::vector<cv::Vec4i>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "const std::vector<cv::Vec4i>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVec4i();
                cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Vec4i;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", val as "cv::Vec4i"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", index as "size_t", val as "cv::Vec4i"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "const std::vector<Vec4iWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Vec4iWrapper as "cv_return_value_Vec4iWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Vec4iWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "const std::vector<Vec4iWrapper>*", index as "size_t"] -> core::Vec4i as "Vec4iWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", index as "size_t", val as "cv::Vec4i"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [vec as "std::vector<cv::Vec4i>*", index as "size_t", val as "cv::Vec4i"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfVec4i {}
    
    impl core::ToInputArray for VectorOfVec4i {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [me as "std::vector<cv::Vec4i>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVec4i {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVec4i {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [me as "std::vector<cv::Vec4i>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVec4i {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVec4i {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVec4i();
            cpp!(unsafe [me as "std::vector<cv::Vec4i>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVec4i {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVec6f {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVec6f {
        #[inline(always)] pub fn as_raw_VectorOfVec6f(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[core::Vec6f] {
            unsafe {
                let vec = self.as_raw_VectorOfVec6f();
                let data = cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] -> *const core::Vec6f as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfVec6f {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVec6f {
        type Item = core::Vec6f;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVec6f {
        type Item = core::Vec6f;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVec6f>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVec6f {
        type Storage = core::Vec6f;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::Vec6f>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "const std::vector<cv::Vec6f>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "const std::vector<cv::Vec6f>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "const std::vector<cv::Vec6f>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVec6f();
                cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*"] {
                vec->clear();
            })
        }
    
        type Arg = core::Vec6f;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", val as "cv::Vec6f"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t", val as "cv::Vec6f"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "const std::vector<Vec6fWrapper>*", index as "size_t"] -> crate::sys::cv_return_value_Vec6fWrapper as "cv_return_value_Vec6fWrapper" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_Vec6fWrapper)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "const std::vector<Vec6fWrapper>*", index as "size_t"] -> core::Vec6f as "Vec6fWrapper" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t", val as "cv::Vec6f"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVec6f();
            cpp!(unsafe [vec as "std::vector<cv::Vec6f>*", index as "size_t", val as "cv::Vec6f"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfVec6f {}
    
    pub struct VectorOfVectorOfDMatch {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfDMatch {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfDMatch(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfDMatch {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfDMatch {
        type Item = types::VectorOfDMatch;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfDMatch {
        type Item = types::VectorOfDMatch;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfDMatch>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfDMatch {
        type Storage = types::VectorOfDMatch;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::DMatch>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfDMatch();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfDMatch;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            let val = val.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", val as "std::vector<cv::DMatch>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            let val = val.as_raw_VectorOfDMatch();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t", val as "std::vector<cv::DMatch>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::DMatch>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfDMatch { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            types::VectorOfDMatch { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::DMatch>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::DMatch>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t", val as "std::vector<cv::DMatch>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfDMatch();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::DMatch>>*", index as "size_t", val as "std::vector<cv::DMatch>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfDMatch {}
    
    pub struct VectorOfVectorOfKeyPoint {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfKeyPoint {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfKeyPoint(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfKeyPoint {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfKeyPoint {
        type Item = types::VectorOfKeyPoint;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfKeyPoint {
        type Item = types::VectorOfKeyPoint;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfKeyPoint>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfKeyPoint {
        type Storage = types::VectorOfKeyPoint;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::KeyPoint>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfKeyPoint();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfKeyPoint;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            let val = val.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", val as "std::vector<cv::KeyPoint>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            let val = val.as_raw_VectorOfKeyPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t", val as "std::vector<cv::KeyPoint>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::KeyPoint>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfKeyPoint { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            types::VectorOfKeyPoint { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::KeyPoint>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::KeyPoint>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t", val as "std::vector<cv::KeyPoint>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfKeyPoint();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::KeyPoint>>*", index as "size_t", val as "std::vector<cv::KeyPoint>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfKeyPoint {}
    
    pub struct VectorOfVectorOfPoint {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfPoint {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfPoint {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfPoint {
        type Item = types::VectorOfPoint;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfPoint {
        type Item = types::VectorOfPoint;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint {
        type Storage = types::VectorOfPoint;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::Point>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfPoint();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfPoint;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            let val = val.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", val as "std::vector<cv::Point>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfPoint();
            let val = val.as_raw_VectorOfPoint();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t", val as "std::vector<cv::Point>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::Point>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfPoint { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            types::VectorOfPoint { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::Point>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t", val as "std::vector<cv::Point>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point>>*", index as "size_t", val as "std::vector<cv::Point>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfPoint {}
    
    impl core::ToInputArray for VectorOfVectorOfPoint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfPoint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfPoint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfPoint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfPoint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfPoint2f {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfPoint2f {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfPoint2f {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfPoint2f {
        type Item = types::VectorOfPoint2f;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfPoint2f {
        type Item = types::VectorOfPoint2f;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint2f>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint2f {
        type Storage = types::VectorOfPoint2f;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::Point2f>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfPoint2f();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfPoint2f;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            let val = val.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", val as "std::vector<cv::Point2f>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            let val = val.as_raw_VectorOfPoint2f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t", val as "std::vector<cv::Point2f>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::Point2f>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfPoint2f { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            types::VectorOfPoint2f { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point2f>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::Point2f>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t", val as "std::vector<cv::Point2f>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint2f();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point2f>>*", index as "size_t", val as "std::vector<cv::Point2f>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfPoint2f {}
    
    impl core::ToInputArray for VectorOfVectorOfPoint2f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point2f>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfPoint2f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfPoint2f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point2f>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfPoint2f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfPoint2f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint2f();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point2f>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint2f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfPoint3d {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfPoint3d {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint3d(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfPoint3d {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfPoint3d {
        type Item = types::VectorOfPoint3d;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfPoint3d {
        type Item = types::VectorOfPoint3d;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint3d>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint3d {
        type Storage = types::VectorOfPoint3d;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::Point3d>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3d>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3d>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3d>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfPoint3d();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfPoint3d;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            let val = val.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", val as "std::vector<cv::Point3d>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            let val = val.as_raw_VectorOfPoint3d();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", index as "size_t", val as "std::vector<cv::Point3d>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3d>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::Point3d>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfPoint3d { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            types::VectorOfPoint3d { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3d>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::Point3d>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", index as "size_t", val as "std::vector<cv::Point3d>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint3d();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3d>>*", index as "size_t", val as "std::vector<cv::Point3d>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfPoint3d {}
    
    impl core::ToInputArray for VectorOfVectorOfPoint3d {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3d>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfPoint3d {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfPoint3d {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3d>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfPoint3d {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfPoint3d {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3d();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3d>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3d {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfPoint3f {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfPoint3f {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint3f(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfPoint3f {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfPoint3f {
        type Item = types::VectorOfPoint3f;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfPoint3f {
        type Item = types::VectorOfPoint3f;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint3f>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint3f {
        type Storage = types::VectorOfPoint3f;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::Point3f>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3f>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3f>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3f>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfPoint3f();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfPoint3f;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            let val = val.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", val as "std::vector<cv::Point3f>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            let val = val.as_raw_VectorOfPoint3f();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", index as "size_t", val as "std::vector<cv::Point3f>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3f>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::Point3f>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfPoint3f { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            types::VectorOfPoint3f { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3f>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::Point3f>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", index as "size_t", val as "std::vector<cv::Point3f>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint3f();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3f>>*", index as "size_t", val as "std::vector<cv::Point3f>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfPoint3f {}
    
    impl core::ToInputArray for VectorOfVectorOfPoint3f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3f>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfPoint3f {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfPoint3f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3f>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfPoint3f {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfPoint3f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3f();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3f>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3f {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfPoint3i {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfPoint3i {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfPoint3i {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfPoint3i {
        type Item = types::VectorOfPoint3i;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfPoint3i {
        type Item = types::VectorOfPoint3i;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfPoint3i>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfPoint3i {
        type Storage = types::VectorOfPoint3i;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::Point3i>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3i>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3i>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3i>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfPoint3i();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfPoint3i;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            let val = val.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", val as "std::vector<cv::Point3i>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            let val = val.as_raw_VectorOfPoint3i();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", index as "size_t", val as "std::vector<cv::Point3i>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3i>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::Point3i>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfPoint3i { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            types::VectorOfPoint3i { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::Point3i>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::Point3i>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", index as "size_t", val as "std::vector<cv::Point3i>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfPoint3i();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Point3i>>*", index as "size_t", val as "std::vector<cv::Point3i>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfPoint3i {}
    
    impl core::ToInputArray for VectorOfVectorOfPoint3i {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3i>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfPoint3i {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfPoint3i {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3i>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfPoint3i {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfPoint3i {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfPoint3i();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Point3i>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfPoint3i {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfRect {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfRect {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfRect(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfRect {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfRect {
        type Item = types::VectorOfRect;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfRect {
        type Item = types::VectorOfRect;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfRect>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfRect {
        type Storage = types::VectorOfRect;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::Rect>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfRect();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfRect;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfRect();
            let val = val.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", val as "std::vector<cv::Rect>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfRect();
            let val = val.as_raw_VectorOfRect();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t", val as "std::vector<cv::Rect>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::Rect>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfRect { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfRect();
            types::VectorOfRect { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::Rect>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::Rect>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfRect();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t", val as "std::vector<cv::Rect>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfRect();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::Rect>>*", index as "size_t", val as "std::vector<cv::Rect>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfRect {}
    
    impl core::ToInputArray for VectorOfVectorOfRect {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Rect>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfRect {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfRect {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Rect>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfRect {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfRect {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfRect();
            cpp!(unsafe [me as "std::vector<std::vector<cv::Rect>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfRect {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfchar {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfchar {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfchar(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfchar {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfchar {
        type Item = types::VectorOfchar;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfchar {
        type Item = types::VectorOfchar;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfchar>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfchar {
        type Storage = types::VectorOfchar;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<char>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<char>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<char>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<char>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfchar();
                cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfchar;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfchar();
            let val = val.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*", val as "std::vector<char>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfchar();
            let val = val.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t", val as "std::vector<char>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<char>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<char>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfchar { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfchar();
            types::VectorOfchar { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<char>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<char>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfchar();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t", val as "std::vector<char>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfchar();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<char>>*", index as "size_t", val as "std::vector<char>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfchar {}
    
    impl core::ToInputArray for VectorOfVectorOfchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [me as "std::vector<std::vector<char>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [me as "std::vector<std::vector<char>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfchar();
            cpp!(unsafe [me as "std::vector<std::vector<char>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfint {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfint {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfint(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfint {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfint {
        type Item = types::VectorOfint;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfint {
        type Item = types::VectorOfint;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfint>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfint {
        type Storage = types::VectorOfint;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<int>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "const std::vector<std::vector<int>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "const std::vector<std::vector<int>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "const std::vector<std::vector<int>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfint();
                cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfint;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfint();
            let val = val.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*", val as "std::vector<int>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfint();
            let val = val.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t", val as "std::vector<int>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [vec as "const std::vector<std::vector<int>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<int>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfint { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfint();
            types::VectorOfint { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<int>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<int>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfint();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t", val as "std::vector<int>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfint();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<int>>*", index as "size_t", val as "std::vector<int>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfint {}
    
    impl core::ToInputArray for VectorOfVectorOfint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [me as "std::vector<std::vector<int>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [me as "std::vector<std::vector<int>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfint();
            cpp!(unsafe [me as "std::vector<std::vector<int>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfVectorOfuchar {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfuchar {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfuchar(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfuchar {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfuchar {
        type Item = types::VectorOfuchar;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfuchar {
        type Item = types::VectorOfuchar;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfuchar>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfuchar {
        type Storage = types::VectorOfuchar;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<uchar>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfuchar();
                cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfuchar;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            let val = val.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", val as "std::vector<uchar>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfuchar();
            let val = val.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t", val as "std::vector<uchar>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<uchar>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfuchar { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            types::VectorOfuchar { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<uchar>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<uchar>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t", val as "std::vector<uchar>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfuchar();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<uchar>>*", index as "size_t", val as "std::vector<uchar>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfuchar {}
    
    impl core::ToInputArray for VectorOfVectorOfuchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [me as "std::vector<std::vector<uchar>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfVectorOfuchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfVectorOfuchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [me as "std::vector<std::vector<uchar>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfVectorOfuchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfVectorOfuchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfVectorOfuchar();
            cpp!(unsafe [me as "std::vector<std::vector<uchar>>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfVectorOfuchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfbool {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfbool {
        #[inline(always)] pub fn as_raw_VectorOfbool(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfbool {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfbool {
        type Item = bool;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfbool {
        type Item = bool;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfbool>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfbool {
        type Storage = bool;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<bool>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "const std::vector<bool>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "const std::vector<bool>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "const std::vector<bool>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfbool();
                cpp!(unsafe [vec as "std::vector<bool>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*"] {
                vec->clear();
            })
        }
    
        type Arg = bool;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*", val as "bool"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t", val as "bool"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "const std::vector<bool>*", index as "size_t"] -> crate::sys::cv_return_value_bool as "cv_return_value_bool" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_bool)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "const std::vector<bool>*", index as "size_t"] -> bool as "bool" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t", val as "bool"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfbool();
            cpp!(unsafe [vec as "std::vector<bool>*", index as "size_t", val as "bool"] {
                (*vec)[index] = val;
            })
        }
    }
    
    unsafe impl Send for VectorOfbool {}
    
    pub struct VectorOfchar {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfchar {
        #[inline(always)] pub fn as_raw_VectorOfchar(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[i8] {
            unsafe {
                let vec = self.as_raw_VectorOfchar();
                let data = cpp!(unsafe [vec as "std::vector<char>*"] -> *const i8 as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfchar {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfchar {
        type Item = i8;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfchar {
        type Item = i8;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfchar>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfchar {
        type Storage = i8;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<char>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "const std::vector<char>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "const std::vector<char>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "const std::vector<char>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfchar();
                cpp!(unsafe [vec as "std::vector<char>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*"] {
                vec->clear();
            })
        }
    
        type Arg = i8;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*", val as "char"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*", index as "size_t", val as "char"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "const std::vector<char>*", index as "size_t"] -> crate::sys::cv_return_value_char as "cv_return_value_char" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_char)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "const std::vector<char>*", index as "size_t"] -> i8 as "char" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*", index as "size_t", val as "char"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfchar();
            cpp!(unsafe [vec as "std::vector<char>*", index as "size_t", val as "char"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfchar {}
    
    impl core::ToInputArray for VectorOfchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfchar();
            cpp!(unsafe [me as "std::vector<char>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfchar();
            cpp!(unsafe [me as "std::vector<char>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfchar();
            cpp!(unsafe [me as "std::vector<char>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfdouble {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfdouble {
        #[inline(always)] pub fn as_raw_VectorOfdouble(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[f64] {
            unsafe {
                let vec = self.as_raw_VectorOfdouble();
                let data = cpp!(unsafe [vec as "std::vector<double>*"] -> *const f64 as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfdouble {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfdouble {
        type Item = f64;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfdouble {
        type Item = f64;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfdouble>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfdouble {
        type Storage = f64;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<double>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "const std::vector<double>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "const std::vector<double>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "const std::vector<double>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfdouble();
                cpp!(unsafe [vec as "std::vector<double>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*"] {
                vec->clear();
            })
        }
    
        type Arg = f64;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*", val as "double"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*", index as "size_t", val as "double"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "const std::vector<double>*", index as "size_t"] -> crate::sys::cv_return_value_double as "cv_return_value_double" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_double)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "const std::vector<double>*", index as "size_t"] -> f64 as "double" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*", index as "size_t", val as "double"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfdouble();
            cpp!(unsafe [vec as "std::vector<double>*", index as "size_t", val as "double"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfdouble {}
    
    impl core::ToInputArray for VectorOfdouble {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfdouble();
            cpp!(unsafe [me as "std::vector<double>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfdouble {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfdouble {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfdouble();
            cpp!(unsafe [me as "std::vector<double>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfdouble {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfdouble {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfdouble();
            cpp!(unsafe [me as "std::vector<double>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfdouble {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOffloat {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOffloat {
        #[inline(always)] pub fn as_raw_VectorOffloat(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[f32] {
            unsafe {
                let vec = self.as_raw_VectorOffloat();
                let data = cpp!(unsafe [vec as "std::vector<float>*"] -> *const f32 as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOffloat {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOffloat {
        type Item = f32;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOffloat {
        type Item = f32;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOffloat>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOffloat {
        type Storage = f32;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<float>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "const std::vector<float>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "const std::vector<float>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "const std::vector<float>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOffloat();
                cpp!(unsafe [vec as "std::vector<float>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*"] {
                vec->clear();
            })
        }
    
        type Arg = f32;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*", val as "float"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*", index as "size_t", val as "float"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "const std::vector<float>*", index as "size_t"] -> crate::sys::cv_return_value_float as "cv_return_value_float" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_float)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "const std::vector<float>*", index as "size_t"] -> f32 as "float" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*", index as "size_t", val as "float"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOffloat();
            cpp!(unsafe [vec as "std::vector<float>*", index as "size_t", val as "float"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOffloat {}
    
    impl core::ToInputArray for VectorOffloat {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOffloat();
            cpp!(unsafe [me as "std::vector<float>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOffloat {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOffloat {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOffloat();
            cpp!(unsafe [me as "std::vector<float>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOffloat {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOffloat {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOffloat();
            cpp!(unsafe [me as "std::vector<float>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOffloat {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfint {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfint {
        #[inline(always)] pub fn as_raw_VectorOfint(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[i32] {
            unsafe {
                let vec = self.as_raw_VectorOfint();
                let data = cpp!(unsafe [vec as "std::vector<int>*"] -> *const i32 as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfint {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfint {
        type Item = i32;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfint {
        type Item = i32;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfint>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfint {
        type Storage = i32;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<int>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "const std::vector<int>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "const std::vector<int>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "const std::vector<int>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfint();
                cpp!(unsafe [vec as "std::vector<int>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*"] {
                vec->clear();
            })
        }
    
        type Arg = i32;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*", val as "int"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*", index as "size_t", val as "int"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "const std::vector<int>*", index as "size_t"] -> crate::sys::cv_return_value_int as "cv_return_value_int" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_int)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "const std::vector<int>*", index as "size_t"] -> i32 as "int" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*", index as "size_t", val as "int"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfint();
            cpp!(unsafe [vec as "std::vector<int>*", index as "size_t", val as "int"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfint {}
    
    impl core::ToInputArray for VectorOfint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfint();
            cpp!(unsafe [me as "std::vector<int>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfint {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfint();
            cpp!(unsafe [me as "std::vector<int>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfint {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfint();
            cpp!(unsafe [me as "std::vector<int>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfint {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
    pub struct VectorOfstd_string {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfstd_string {
        #[inline(always)] pub fn as_raw_VectorOfstd_string(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfstd_string {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "std::vector<String>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfstd_string {
        type Item = String;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfstd_string {
        type Item = String;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfstd_string>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfstd_string {
        type Storage = String;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<String>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "const std::vector<String>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "const std::vector<String>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "const std::vector<String>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "std::vector<String>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "std::vector<String>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfstd_string();
                cpp!(unsafe [vec as "std::vector<String>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "std::vector<String>*"] {
                vec->clear();
            })
        }
    
        type Arg = &'i str;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfstd_string();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", val as "const char*"] {
                vec->push_back(String(val));
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfstd_string();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] {
                vec->insert(vec->begin() + index, String(val));
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfstd_string();
            cpp!(unsafe [vec as "const std::vector<String>*", index as "size_t"] -> crate::sys::cv_return_value_char_X as "cv_return_value_char_X" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index).c_str() };
                } VEC_CATCH(cv_return_value_char_X)
            }).into_result().map(|x| unsafe { ::std::ffi::CStr::from_ptr(x) }.to_string_lossy().into_owned())
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfstd_string();
            ::std::ffi::CStr::from_ptr(cpp!(unsafe [vec as "const std::vector<String>*", index as "size_t"] -> *mut c_char as "const char*" {
                return (*vec)[index].c_str();
            })).to_string_lossy().into_owned()
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfstd_string();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = String(val);
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfstd_string();
            let val = ::std::ffi::CString::new(val).unwrap();
            let val = val.as_ptr();
            cpp!(unsafe [vec as "std::vector<String>*", index as "size_t", val as "const char*"] {
                (*vec)[index] = String(val);
            })
        }
    }
    
    unsafe impl Send for VectorOfstd_string {}
    
    pub struct VectorOfuchar {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfuchar {
        #[inline(always)] pub fn as_raw_VectorOfuchar(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
        
        pub fn to_slice(&self) -> &[u8] {
            unsafe {
                let vec = self.as_raw_VectorOfuchar();
                let data = cpp!(unsafe [vec as "std::vector<uchar>*"] -> *const u8 as "void**" {
                    return reinterpret_cast<void**>(vec->data());
                });
                ::std::slice::from_raw_parts(data, crate::templ::Vector::len(self))
            }
        }
    }
    
    impl Drop for VectorOfuchar {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfuchar {
        type Item = u8;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfuchar {
        type Item = u8;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfuchar>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfuchar {
        type Storage = u8;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<uchar>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<uchar>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<uchar>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<uchar>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfuchar();
                cpp!(unsafe [vec as "std::vector<uchar>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*"] {
                vec->clear();
            })
        }
    
        type Arg = u8;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*", val as "uchar"] {
                vec->push_back(val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t", val as "uchar"] {
                vec->insert(vec->begin() + index, val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<unsigned char>*", index as "size_t"] -> crate::sys::cv_return_value_unsigned_char as "cv_return_value_unsigned_char" {
                try {
                    return { Error::Code::StsOk, NULL, vec->at(index) };
                } VEC_CATCH(cv_return_value_unsigned_char)
            }).into_result()
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "const std::vector<unsigned char>*", index as "size_t"] -> u8 as "unsigned char" {
                return (*vec)[index];
            })
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t", val as "uchar"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfuchar();
            cpp!(unsafe [vec as "std::vector<uchar>*", index as "size_t", val as "uchar"] {
                (*vec)[index] = val;
            })
        }
        
        #[inline]
        fn to_vec(&self) -> Vec<Self::Storage> {
            self.to_slice().to_vec()
        }
    }
    
    unsafe impl Send for VectorOfuchar {}
    
    impl core::ToInputArray for VectorOfuchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            let me = self.as_raw_VectorOfuchar();
            cpp!(unsafe [me as "std::vector<uchar>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputArray { ptr })
        }
    }
    
    impl core::ToInputArray for &VectorOfuchar {
        #[inline]
        fn input_array(&self) -> Result<core::_InputArray> {
            (*self).input_array()
        }
    }
    
    impl core::ToOutputArray for VectorOfuchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            let me = self.as_raw_VectorOfuchar();
            cpp!(unsafe [me as "std::vector<uchar>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _OutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_OutputArray { ptr })
        }
    }
    
    impl core::ToOutputArray for &mut VectorOfuchar {
        #[inline]
        fn output_array(&mut self) -> Result<core::_OutputArray> {
            (*self).output_array()
        }
    }
    
    impl core::ToInputOutputArray for VectorOfuchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            let me = self.as_raw_VectorOfuchar();
            cpp!(unsafe [me as "std::vector<uchar>*"] -> sys::cv_return_value_const_void_X as "cv_return_value_const_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new _InputOutputArray(*me) };
                } CVRS_CATCH(cv_return_value_const_void_X)
            }).into_result()
                .map(|ptr| core::_InputOutputArray { ptr })
        }
    }
    
    impl core::ToInputOutputArray for &mut VectorOfuchar {
        #[inline]
        fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
            (*self).input_output_array()
        }
    }
    
}
pub use core_types::*;

#[cfg(feature = "contrib")]
mod aruco_types {
    use super::*;

    pub struct PtrOfBoard {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBoard {
        #[inline(always)] pub fn as_raw_PtrOfBoard(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBoard {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::aruco::Board>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBoard {}
    
    impl crate::aruco::BoardTrait for PtrOfBoard {
        #[inline(always)] fn as_raw_Board(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::aruco::Board>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfCharucoBoard {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfCharucoBoard {
        #[inline(always)] pub fn as_raw_PtrOfCharucoBoard(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfCharucoBoard {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::aruco::CharucoBoard>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfCharucoBoard {}
    
    impl PtrOfCharucoBoard {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::aruco::CharucoBoard>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> CharucoBoardRef {
            let inner = crate::aruco::CharucoBoard { ptr: self.get_inner() };
            CharucoBoardRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> CharucoBoardRefMut {
            let inner = crate::aruco::CharucoBoard { ptr: self.get_inner() };
            CharucoBoardRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct CharucoBoardRef<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::CharucoBoard>,
        owner: std::marker::PhantomData<&'o types::PtrOfCharucoBoard>,
    }
    
    impl std::ops::Deref for CharucoBoardRef<'_> {
        type Target = crate::aruco::CharucoBoard;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct CharucoBoardRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::CharucoBoard>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfCharucoBoard>,
    }
    
    impl std::ops::Deref for CharucoBoardRefMut<'_> {
        type Target = crate::aruco::CharucoBoard;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for CharucoBoardRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfDetectorParameters {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDetectorParameters {
        #[inline(always)] pub fn as_raw_PtrOfDetectorParameters(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDetectorParameters {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::aruco::DetectorParameters>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDetectorParameters {}
    
    impl PtrOfDetectorParameters {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::aruco::DetectorParameters>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> DetectorParametersRef {
            let inner = crate::aruco::DetectorParameters { ptr: self.get_inner() };
            DetectorParametersRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> DetectorParametersRefMut {
            let inner = crate::aruco::DetectorParameters { ptr: self.get_inner() };
            DetectorParametersRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct DetectorParametersRef<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::DetectorParameters>,
        owner: std::marker::PhantomData<&'o types::PtrOfDetectorParameters>,
    }
    
    impl std::ops::Deref for DetectorParametersRef<'_> {
        type Target = crate::aruco::DetectorParameters;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct DetectorParametersRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::DetectorParameters>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfDetectorParameters>,
    }
    
    impl std::ops::Deref for DetectorParametersRefMut<'_> {
        type Target = crate::aruco::DetectorParameters;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for DetectorParametersRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfDictionary {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDictionary {
        #[inline(always)] pub fn as_raw_PtrOfDictionary(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDictionary {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::aruco::Dictionary>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDictionary {}
    
    impl PtrOfDictionary {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::aruco::Dictionary>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> DictionaryRef {
            let inner = crate::aruco::Dictionary { ptr: self.get_inner() };
            DictionaryRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> DictionaryRefMut {
            let inner = crate::aruco::Dictionary { ptr: self.get_inner() };
            DictionaryRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct DictionaryRef<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::Dictionary>,
        owner: std::marker::PhantomData<&'o types::PtrOfDictionary>,
    }
    
    impl std::ops::Deref for DictionaryRef<'_> {
        type Target = crate::aruco::Dictionary;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct DictionaryRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::Dictionary>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfDictionary>,
    }
    
    impl std::ops::Deref for DictionaryRefMut<'_> {
        type Target = crate::aruco::Dictionary;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for DictionaryRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfGridBoard {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfGridBoard {
        #[inline(always)] pub fn as_raw_PtrOfGridBoard(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfGridBoard {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::aruco::GridBoard>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfGridBoard {}
    
    impl PtrOfGridBoard {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::aruco::GridBoard>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> GridBoardRef {
            let inner = crate::aruco::GridBoard { ptr: self.get_inner() };
            GridBoardRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> GridBoardRefMut {
            let inner = crate::aruco::GridBoard { ptr: self.get_inner() };
            GridBoardRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct GridBoardRef<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::GridBoard>,
        owner: std::marker::PhantomData<&'o types::PtrOfGridBoard>,
    }
    
    impl std::ops::Deref for GridBoardRef<'_> {
        type Target = crate::aruco::GridBoard;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct GridBoardRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::aruco::GridBoard>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfGridBoard>,
    }
    
    impl std::ops::Deref for GridBoardRefMut<'_> {
        type Target = crate::aruco::GridBoard;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for GridBoardRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
}
#[cfg(feature = "contrib")]
pub use aruco_types::*;

#[cfg(feature = "contrib")]
mod bgsegm_types {
    use super::*;

    pub struct PtrOfBackgroundSubtractorGMG {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBackgroundSubtractorGMG {
        #[inline(always)] pub fn as_raw_PtrOfBackgroundSubtractorGMG(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBackgroundSubtractorGMG {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::bgsegm::BackgroundSubtractorGMG>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBackgroundSubtractorGMG {}
    
    impl crate::bgsegm::BackgroundSubtractorGMG for PtrOfBackgroundSubtractorGMG {
        #[inline(always)] fn as_raw_BackgroundSubtractorGMG(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::bgsegm::BackgroundSubtractorGMG>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfBackgroundSubtractorMOG {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBackgroundSubtractorMOG {
        #[inline(always)] pub fn as_raw_PtrOfBackgroundSubtractorMOG(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBackgroundSubtractorMOG {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::bgsegm::BackgroundSubtractorMOG>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBackgroundSubtractorMOG {}
    
    impl crate::bgsegm::BackgroundSubtractorMOG for PtrOfBackgroundSubtractorMOG {
        #[inline(always)] fn as_raw_BackgroundSubtractorMOG(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::bgsegm::BackgroundSubtractorMOG>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use bgsegm_types::*;

#[cfg(feature = "contrib")]
mod bioinspired_types {
    use super::*;

    pub struct PtrOfRetina {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfRetina {
        #[inline(always)] pub fn as_raw_PtrOfRetina(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfRetina {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::bioinspired::Retina>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfRetina {}
    
    impl core::AlgorithmTrait for PtrOfRetina {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::bioinspired::Retina for PtrOfRetina {
        #[inline(always)] fn as_raw_Retina(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::bioinspired::Retina>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfRetinaFastToneMapping {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfRetinaFastToneMapping {
        #[inline(always)] pub fn as_raw_PtrOfRetinaFastToneMapping(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfRetinaFastToneMapping {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::bioinspired::RetinaFastToneMapping>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfRetinaFastToneMapping {}
    
    impl core::AlgorithmTrait for PtrOfRetinaFastToneMapping {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::bioinspired::RetinaFastToneMapping for PtrOfRetinaFastToneMapping {
        #[inline(always)] fn as_raw_RetinaFastToneMapping(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::bioinspired::RetinaFastToneMapping>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTransientAreasSegmentationModule {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTransientAreasSegmentationModule {
        #[inline(always)] pub fn as_raw_PtrOfTransientAreasSegmentationModule(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTransientAreasSegmentationModule {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::bioinspired::TransientAreasSegmentationModule>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTransientAreasSegmentationModule {}
    
    impl core::AlgorithmTrait for PtrOfTransientAreasSegmentationModule {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::bioinspired::TransientAreasSegmentationModule for PtrOfTransientAreasSegmentationModule {
        #[inline(always)] fn as_raw_TransientAreasSegmentationModule(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use bioinspired_types::*;

mod calib3d_types {
    use super::*;

    pub struct PtrOfStereoBM {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfStereoBM {
        #[inline(always)] pub fn as_raw_PtrOfStereoBM(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfStereoBM {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::StereoBM>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfStereoBM {}
    
    impl core::AlgorithmTrait for PtrOfStereoBM {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::calib3d::StereoBM for PtrOfStereoBM {
        #[inline(always)] fn as_raw_StereoBM(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::StereoBM>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::calib3d::StereoMatcher for PtrOfStereoBM {
        #[inline(always)] fn as_raw_StereoMatcher(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::StereoMatcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfStereoSGBM {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfStereoSGBM {
        #[inline(always)] pub fn as_raw_PtrOfStereoSGBM(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfStereoSGBM {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::StereoSGBM>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfStereoSGBM {}
    
    impl core::AlgorithmTrait for PtrOfStereoSGBM {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::calib3d::StereoMatcher for PtrOfStereoSGBM {
        #[inline(always)] fn as_raw_StereoMatcher(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::StereoMatcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::calib3d::StereoSGBM for PtrOfStereoSGBM {
        #[inline(always)] fn as_raw_StereoSGBM(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::StereoSGBM>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use calib3d_types::*;

#[cfg(feature = "contrib")]
mod dnn_types {
    use super::*;

    pub struct PtrOfAbsLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfAbsLayer {
        #[inline(always)] pub fn as_raw_PtrOfAbsLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfAbsLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::AbsLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfAbsLayer {}
    
    impl PtrOfAbsLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::AbsLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> AbsLayerRef {
            let inner = crate::dnn::AbsLayer { ptr: self.get_inner() };
            AbsLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> AbsLayerRefMut {
            let inner = crate::dnn::AbsLayer { ptr: self.get_inner() };
            AbsLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct AbsLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::AbsLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfAbsLayer>,
    }
    
    impl std::ops::Deref for AbsLayerRef<'_> {
        type Target = crate::dnn::AbsLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct AbsLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::AbsLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfAbsLayer>,
    }
    
    impl std::ops::Deref for AbsLayerRefMut<'_> {
        type Target = crate::dnn::AbsLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for AbsLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfBNLLLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBNLLLayer {
        #[inline(always)] pub fn as_raw_PtrOfBNLLLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBNLLLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::BNLLLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBNLLLayer {}
    
    impl PtrOfBNLLLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::BNLLLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BNLLLayerRef {
            let inner = crate::dnn::BNLLLayer { ptr: self.get_inner() };
            BNLLLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BNLLLayerRefMut {
            let inner = crate::dnn::BNLLLayer { ptr: self.get_inner() };
            BNLLLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BNLLLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::BNLLLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfBNLLLayer>,
    }
    
    impl std::ops::Deref for BNLLLayerRef<'_> {
        type Target = crate::dnn::BNLLLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BNLLLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::BNLLLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBNLLLayer>,
    }
    
    impl std::ops::Deref for BNLLLayerRefMut<'_> {
        type Target = crate::dnn::BNLLLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BNLLLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfBaseConvolutionLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBaseConvolutionLayer {
        #[inline(always)] pub fn as_raw_PtrOfBaseConvolutionLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBaseConvolutionLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::BaseConvolutionLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBaseConvolutionLayer {}
    
    impl crate::dnn::BaseConvolutionLayerTrait for PtrOfBaseConvolutionLayer {
        #[inline(always)] fn as_raw_BaseConvolutionLayer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::BaseConvolutionLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::dnn::Layer for PtrOfBaseConvolutionLayer {
        #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfConcatLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfConcatLayer {
        #[inline(always)] pub fn as_raw_PtrOfConcatLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfConcatLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::ConcatLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfConcatLayer {}
    
    impl PtrOfConcatLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::ConcatLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> ConcatLayerRef {
            let inner = crate::dnn::ConcatLayer { ptr: self.get_inner() };
            ConcatLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> ConcatLayerRefMut {
            let inner = crate::dnn::ConcatLayer { ptr: self.get_inner() };
            ConcatLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct ConcatLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::ConcatLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfConcatLayer>,
    }
    
    impl std::ops::Deref for ConcatLayerRef<'_> {
        type Target = crate::dnn::ConcatLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct ConcatLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::ConcatLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfConcatLayer>,
    }
    
    impl std::ops::Deref for ConcatLayerRefMut<'_> {
        type Target = crate::dnn::ConcatLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for ConcatLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfCropLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfCropLayer {
        #[inline(always)] pub fn as_raw_PtrOfCropLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfCropLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::CropLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfCropLayer {}
    
    impl PtrOfCropLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::CropLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> CropLayerRef {
            let inner = crate::dnn::CropLayer { ptr: self.get_inner() };
            CropLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> CropLayerRefMut {
            let inner = crate::dnn::CropLayer { ptr: self.get_inner() };
            CropLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct CropLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::CropLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfCropLayer>,
    }
    
    impl std::ops::Deref for CropLayerRef<'_> {
        type Target = crate::dnn::CropLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct CropLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::CropLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfCropLayer>,
    }
    
    impl std::ops::Deref for CropLayerRefMut<'_> {
        type Target = crate::dnn::CropLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for CropLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfEltwiseLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfEltwiseLayer {
        #[inline(always)] pub fn as_raw_PtrOfEltwiseLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfEltwiseLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::EltwiseLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfEltwiseLayer {}
    
    impl PtrOfEltwiseLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::EltwiseLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> EltwiseLayerRef {
            let inner = crate::dnn::EltwiseLayer { ptr: self.get_inner() };
            EltwiseLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> EltwiseLayerRefMut {
            let inner = crate::dnn::EltwiseLayer { ptr: self.get_inner() };
            EltwiseLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct EltwiseLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::EltwiseLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfEltwiseLayer>,
    }
    
    impl std::ops::Deref for EltwiseLayerRef<'_> {
        type Target = crate::dnn::EltwiseLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct EltwiseLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::EltwiseLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfEltwiseLayer>,
    }
    
    impl std::ops::Deref for EltwiseLayerRefMut<'_> {
        type Target = crate::dnn::EltwiseLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for EltwiseLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfImporter {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfImporter {
        #[inline(always)] pub fn as_raw_PtrOfImporter(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfImporter {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::Importer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfImporter {}
    
    impl crate::dnn::Importer for PtrOfImporter {
        #[inline(always)] fn as_raw_Importer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::Importer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfInnerProductLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfInnerProductLayer {
        #[inline(always)] pub fn as_raw_PtrOfInnerProductLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfInnerProductLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::InnerProductLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfInnerProductLayer {}
    
    impl PtrOfInnerProductLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::InnerProductLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> InnerProductLayerRef {
            let inner = crate::dnn::InnerProductLayer { ptr: self.get_inner() };
            InnerProductLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> InnerProductLayerRefMut {
            let inner = crate::dnn::InnerProductLayer { ptr: self.get_inner() };
            InnerProductLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct InnerProductLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::InnerProductLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfInnerProductLayer>,
    }
    
    impl std::ops::Deref for InnerProductLayerRef<'_> {
        type Target = crate::dnn::InnerProductLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct InnerProductLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::InnerProductLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfInnerProductLayer>,
    }
    
    impl std::ops::Deref for InnerProductLayerRefMut<'_> {
        type Target = crate::dnn::InnerProductLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for InnerProductLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfLRNLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLRNLayer {
        #[inline(always)] pub fn as_raw_PtrOfLRNLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLRNLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::LRNLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLRNLayer {}
    
    impl PtrOfLRNLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::LRNLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> LRNLayerRef {
            let inner = crate::dnn::LRNLayer { ptr: self.get_inner() };
            LRNLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> LRNLayerRefMut {
            let inner = crate::dnn::LRNLayer { ptr: self.get_inner() };
            LRNLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct LRNLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::LRNLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfLRNLayer>,
    }
    
    impl std::ops::Deref for LRNLayerRef<'_> {
        type Target = crate::dnn::LRNLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct LRNLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::LRNLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfLRNLayer>,
    }
    
    impl std::ops::Deref for LRNLayerRefMut<'_> {
        type Target = crate::dnn::LRNLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for LRNLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfLSTMLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLSTMLayer {
        #[inline(always)] pub fn as_raw_PtrOfLSTMLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLSTMLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::LSTMLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLSTMLayer {}
    
    impl crate::dnn::LSTMLayer for PtrOfLSTMLayer {
        #[inline(always)] fn as_raw_LSTMLayer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::LSTMLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::dnn::Layer for PtrOfLSTMLayer {
        #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLayer {
        #[inline(always)] pub fn as_raw_PtrOfLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::Layer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLayer {}
    
    impl crate::dnn::Layer for PtrOfLayer {
        #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMVNLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMVNLayer {
        #[inline(always)] pub fn as_raw_PtrOfMVNLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMVNLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::MVNLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMVNLayer {}
    
    impl PtrOfMVNLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::MVNLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> MVNLayerRef {
            let inner = crate::dnn::MVNLayer { ptr: self.get_inner() };
            MVNLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> MVNLayerRefMut {
            let inner = crate::dnn::MVNLayer { ptr: self.get_inner() };
            MVNLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct MVNLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::MVNLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfMVNLayer>,
    }
    
    impl std::ops::Deref for MVNLayerRef<'_> {
        type Target = crate::dnn::MVNLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct MVNLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::MVNLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfMVNLayer>,
    }
    
    impl std::ops::Deref for MVNLayerRefMut<'_> {
        type Target = crate::dnn::MVNLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for MVNLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfPoolingLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfPoolingLayer {
        #[inline(always)] pub fn as_raw_PtrOfPoolingLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfPoolingLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::PoolingLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfPoolingLayer {}
    
    impl PtrOfPoolingLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::PoolingLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> PoolingLayerRef {
            let inner = crate::dnn::PoolingLayer { ptr: self.get_inner() };
            PoolingLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> PoolingLayerRefMut {
            let inner = crate::dnn::PoolingLayer { ptr: self.get_inner() };
            PoolingLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct PoolingLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::PoolingLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfPoolingLayer>,
    }
    
    impl std::ops::Deref for PoolingLayerRef<'_> {
        type Target = crate::dnn::PoolingLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct PoolingLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::PoolingLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfPoolingLayer>,
    }
    
    impl std::ops::Deref for PoolingLayerRefMut<'_> {
        type Target = crate::dnn::PoolingLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for PoolingLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfPowerLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfPowerLayer {
        #[inline(always)] pub fn as_raw_PtrOfPowerLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfPowerLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::PowerLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfPowerLayer {}
    
    impl PtrOfPowerLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::PowerLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> PowerLayerRef {
            let inner = crate::dnn::PowerLayer { ptr: self.get_inner() };
            PowerLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> PowerLayerRefMut {
            let inner = crate::dnn::PowerLayer { ptr: self.get_inner() };
            PowerLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct PowerLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::PowerLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfPowerLayer>,
    }
    
    impl std::ops::Deref for PowerLayerRef<'_> {
        type Target = crate::dnn::PowerLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct PowerLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::PowerLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfPowerLayer>,
    }
    
    impl std::ops::Deref for PowerLayerRefMut<'_> {
        type Target = crate::dnn::PowerLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for PowerLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfRNNLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfRNNLayer {
        #[inline(always)] pub fn as_raw_PtrOfRNNLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfRNNLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::RNNLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfRNNLayer {}
    
    impl crate::dnn::Layer for PtrOfRNNLayer {
        #[inline(always)] fn as_raw_Layer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::Layer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::dnn::RNNLayer for PtrOfRNNLayer {
        #[inline(always)] fn as_raw_RNNLayer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::RNNLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfReLULayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfReLULayer {
        #[inline(always)] pub fn as_raw_PtrOfReLULayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfReLULayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::ReLULayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfReLULayer {}
    
    impl PtrOfReLULayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::ReLULayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> ReLULayerRef {
            let inner = crate::dnn::ReLULayer { ptr: self.get_inner() };
            ReLULayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> ReLULayerRefMut {
            let inner = crate::dnn::ReLULayer { ptr: self.get_inner() };
            ReLULayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct ReLULayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::ReLULayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfReLULayer>,
    }
    
    impl std::ops::Deref for ReLULayerRef<'_> {
        type Target = crate::dnn::ReLULayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct ReLULayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::ReLULayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfReLULayer>,
    }
    
    impl std::ops::Deref for ReLULayerRefMut<'_> {
        type Target = crate::dnn::ReLULayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for ReLULayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfReshapeLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfReshapeLayer {
        #[inline(always)] pub fn as_raw_PtrOfReshapeLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfReshapeLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::ReshapeLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfReshapeLayer {}
    
    impl PtrOfReshapeLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::ReshapeLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> ReshapeLayerRef {
            let inner = crate::dnn::ReshapeLayer { ptr: self.get_inner() };
            ReshapeLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> ReshapeLayerRefMut {
            let inner = crate::dnn::ReshapeLayer { ptr: self.get_inner() };
            ReshapeLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct ReshapeLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::ReshapeLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfReshapeLayer>,
    }
    
    impl std::ops::Deref for ReshapeLayerRef<'_> {
        type Target = crate::dnn::ReshapeLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct ReshapeLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::ReshapeLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfReshapeLayer>,
    }
    
    impl std::ops::Deref for ReshapeLayerRefMut<'_> {
        type Target = crate::dnn::ReshapeLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for ReshapeLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfSigmoidLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSigmoidLayer {
        #[inline(always)] pub fn as_raw_PtrOfSigmoidLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSigmoidLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::SigmoidLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSigmoidLayer {}
    
    impl PtrOfSigmoidLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::SigmoidLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> SigmoidLayerRef {
            let inner = crate::dnn::SigmoidLayer { ptr: self.get_inner() };
            SigmoidLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> SigmoidLayerRefMut {
            let inner = crate::dnn::SigmoidLayer { ptr: self.get_inner() };
            SigmoidLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct SigmoidLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SigmoidLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfSigmoidLayer>,
    }
    
    impl std::ops::Deref for SigmoidLayerRef<'_> {
        type Target = crate::dnn::SigmoidLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct SigmoidLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SigmoidLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfSigmoidLayer>,
    }
    
    impl std::ops::Deref for SigmoidLayerRefMut<'_> {
        type Target = crate::dnn::SigmoidLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for SigmoidLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfSliceLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSliceLayer {
        #[inline(always)] pub fn as_raw_PtrOfSliceLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSliceLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::SliceLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSliceLayer {}
    
    impl PtrOfSliceLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::SliceLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> SliceLayerRef {
            let inner = crate::dnn::SliceLayer { ptr: self.get_inner() };
            SliceLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> SliceLayerRefMut {
            let inner = crate::dnn::SliceLayer { ptr: self.get_inner() };
            SliceLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct SliceLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SliceLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfSliceLayer>,
    }
    
    impl std::ops::Deref for SliceLayerRef<'_> {
        type Target = crate::dnn::SliceLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct SliceLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SliceLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfSliceLayer>,
    }
    
    impl std::ops::Deref for SliceLayerRefMut<'_> {
        type Target = crate::dnn::SliceLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for SliceLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfSoftmaxLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSoftmaxLayer {
        #[inline(always)] pub fn as_raw_PtrOfSoftmaxLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSoftmaxLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::SoftmaxLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSoftmaxLayer {}
    
    impl PtrOfSoftmaxLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::SoftmaxLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> SoftmaxLayerRef {
            let inner = crate::dnn::SoftmaxLayer { ptr: self.get_inner() };
            SoftmaxLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> SoftmaxLayerRefMut {
            let inner = crate::dnn::SoftmaxLayer { ptr: self.get_inner() };
            SoftmaxLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct SoftmaxLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SoftmaxLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfSoftmaxLayer>,
    }
    
    impl std::ops::Deref for SoftmaxLayerRef<'_> {
        type Target = crate::dnn::SoftmaxLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct SoftmaxLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SoftmaxLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfSoftmaxLayer>,
    }
    
    impl std::ops::Deref for SoftmaxLayerRefMut<'_> {
        type Target = crate::dnn::SoftmaxLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for SoftmaxLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfSplitLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSplitLayer {
        #[inline(always)] pub fn as_raw_PtrOfSplitLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSplitLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::SplitLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSplitLayer {}
    
    impl PtrOfSplitLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::SplitLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> SplitLayerRef {
            let inner = crate::dnn::SplitLayer { ptr: self.get_inner() };
            SplitLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> SplitLayerRefMut {
            let inner = crate::dnn::SplitLayer { ptr: self.get_inner() };
            SplitLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct SplitLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SplitLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfSplitLayer>,
    }
    
    impl std::ops::Deref for SplitLayerRef<'_> {
        type Target = crate::dnn::SplitLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct SplitLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::SplitLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfSplitLayer>,
    }
    
    impl std::ops::Deref for SplitLayerRefMut<'_> {
        type Target = crate::dnn::SplitLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for SplitLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfTanHLayer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTanHLayer {
        #[inline(always)] pub fn as_raw_PtrOfTanHLayer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTanHLayer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::dnn::TanHLayer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTanHLayer {}
    
    impl PtrOfTanHLayer {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::dnn::TanHLayer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> TanHLayerRef {
            let inner = crate::dnn::TanHLayer { ptr: self.get_inner() };
            TanHLayerRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> TanHLayerRefMut {
            let inner = crate::dnn::TanHLayer { ptr: self.get_inner() };
            TanHLayerRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct TanHLayerRef<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::TanHLayer>,
        owner: std::marker::PhantomData<&'o types::PtrOfTanHLayer>,
    }
    
    impl std::ops::Deref for TanHLayerRef<'_> {
        type Target = crate::dnn::TanHLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct TanHLayerRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::dnn::TanHLayer>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfTanHLayer>,
    }
    
    impl std::ops::Deref for TanHLayerRefMut<'_> {
        type Target = crate::dnn::TanHLayer;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for TanHLayerRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct VectorOfBlob {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfBlob {
        #[inline(always)] pub fn as_raw_VectorOfBlob(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfBlob {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfBlob {
        type Item = crate::dnn::Blob;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfBlob {
        type Item = crate::dnn::Blob;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfBlob>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfBlob {
        type Storage = crate::dnn::Blob;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::dnn::Blob>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::Blob>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::Blob>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::Blob>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfBlob();
                cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::dnn::Blob;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfBlob();
            let val = val.as_raw_Blob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", val as "cv::dnn::Blob*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfBlob();
            let val = val.as_raw_Blob();
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", index as "size_t", val as "cv::dnn::Blob*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfBlob();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::Blob>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::dnn::Blob(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::dnn::Blob { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfBlob();
            crate::dnn::Blob { ptr: cpp!(unsafe [vec as "const std::vector<cv::dnn::Blob>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::dnn::Blob((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfBlob();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", index as "size_t", val as "cv::dnn::Blob*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfBlob();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::dnn::Blob>*", index as "size_t", val as "cv::dnn::Blob*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfBlob {}
    
    pub struct VectorOfDictValue {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfDictValue {
        #[inline(always)] pub fn as_raw_VectorOfDictValue(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfDictValue {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfDictValue {
        type Item = crate::dnn::DictValue;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfDictValue {
        type Item = crate::dnn::DictValue;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDictValue>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfDictValue {
        type Storage = crate::dnn::DictValue;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::dnn::DictValue>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::DictValue>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::DictValue>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::DictValue>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfDictValue();
                cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::dnn::DictValue;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfDictValue();
            let val = val.as_raw_DictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", val as "cv::dnn::DictValue*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfDictValue();
            let val = val.as_raw_DictValue();
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", index as "size_t", val as "cv::dnn::DictValue*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfDictValue();
            cpp!(unsafe [vec as "const std::vector<cv::dnn::DictValue>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::dnn::DictValue(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::dnn::DictValue { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfDictValue();
            crate::dnn::DictValue { ptr: cpp!(unsafe [vec as "const std::vector<cv::dnn::DictValue>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::dnn::DictValue((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfDictValue();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", index as "size_t", val as "cv::dnn::DictValue*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfDictValue();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::dnn::DictValue>*", index as "size_t", val as "cv::dnn::DictValue*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfDictValue {}
    
}
#[cfg(feature = "contrib")]
pub use dnn_types::*;

#[cfg(feature = "contrib")]
mod dpm_types {
    use super::*;

    pub struct VectorOfObjectDetection {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfObjectDetection {
        #[inline(always)] pub fn as_raw_VectorOfObjectDetection(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfObjectDetection {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfObjectDetection {
        type Item = crate::dpm::DPMDetector_ObjectDetection;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfObjectDetection {
        type Item = crate::dpm::DPMDetector_ObjectDetection;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfObjectDetection>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfObjectDetection {
        type Storage = crate::dpm::DPMDetector_ObjectDetection;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::dpm::DPMDetector::ObjectDetection>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfObjectDetection();
                cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::dpm::DPMDetector_ObjectDetection;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfObjectDetection();
            let val = val.as_raw_DPMDetector_ObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", val as "cv::dpm::DPMDetector::ObjectDetection*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfObjectDetection();
            let val = val.as_raw_DPMDetector_ObjectDetection();
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t", val as "cv::dpm::DPMDetector::ObjectDetection*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfObjectDetection();
            cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::dpm::DPMDetector::ObjectDetection(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::dpm::DPMDetector_ObjectDetection { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfObjectDetection();
            crate::dpm::DPMDetector_ObjectDetection { ptr: cpp!(unsafe [vec as "const std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::dpm::DPMDetector::ObjectDetection((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfObjectDetection();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t", val as "cv::dpm::DPMDetector::ObjectDetection*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfObjectDetection();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::dpm::DPMDetector::ObjectDetection>*", index as "size_t", val as "cv::dpm::DPMDetector::ObjectDetection*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfObjectDetection {}
    
}
#[cfg(feature = "contrib")]
pub use dpm_types::*;

#[cfg(feature = "contrib")]
mod face_types {
    use super::*;

    pub struct PtrOfBasicFaceRecognizer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBasicFaceRecognizer {
        #[inline(always)] pub fn as_raw_PtrOfBasicFaceRecognizer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBasicFaceRecognizer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::face::BasicFaceRecognizer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBasicFaceRecognizer {}
    
    impl core::AlgorithmTrait for PtrOfBasicFaceRecognizer {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::face::BasicFaceRecognizer for PtrOfBasicFaceRecognizer {
        #[inline(always)] fn as_raw_BasicFaceRecognizer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::face::BasicFaceRecognizer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::face::FaceRecognizer for PtrOfBasicFaceRecognizer {
        #[inline(always)] fn as_raw_FaceRecognizer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::face::FaceRecognizer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfLBPHFaceRecognizer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLBPHFaceRecognizer {
        #[inline(always)] pub fn as_raw_PtrOfLBPHFaceRecognizer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLBPHFaceRecognizer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::face::LBPHFaceRecognizer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLBPHFaceRecognizer {}
    
    impl core::AlgorithmTrait for PtrOfLBPHFaceRecognizer {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::face::FaceRecognizer for PtrOfLBPHFaceRecognizer {
        #[inline(always)] fn as_raw_FaceRecognizer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::face::FaceRecognizer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::face::LBPHFaceRecognizer for PtrOfLBPHFaceRecognizer {
        #[inline(always)] fn as_raw_LBPHFaceRecognizer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::face::LBPHFaceRecognizer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfStandardCollector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfStandardCollector {
        #[inline(always)] pub fn as_raw_PtrOfStandardCollector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfStandardCollector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::face::StandardCollector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfStandardCollector {}
    
    impl PtrOfStandardCollector {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::face::StandardCollector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> StandardCollectorRef {
            let inner = crate::face::StandardCollector { ptr: self.get_inner() };
            StandardCollectorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> StandardCollectorRefMut {
            let inner = crate::face::StandardCollector { ptr: self.get_inner() };
            StandardCollectorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct StandardCollectorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::face::StandardCollector>,
        owner: std::marker::PhantomData<&'o types::PtrOfStandardCollector>,
    }
    
    impl std::ops::Deref for StandardCollectorRef<'_> {
        type Target = crate::face::StandardCollector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct StandardCollectorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::face::StandardCollector>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfStandardCollector>,
    }
    
    impl std::ops::Deref for StandardCollectorRefMut<'_> {
        type Target = crate::face::StandardCollector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for StandardCollectorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
}
#[cfg(feature = "contrib")]
pub use face_types::*;

mod features2d_types {
    use super::*;

    pub struct PtrOfAKAZE {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfAKAZE {
        #[inline(always)] pub fn as_raw_PtrOfAKAZE(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfAKAZE {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::AKAZE>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfAKAZE {}
    
    impl crate::features2d::AKAZE for PtrOfAKAZE {
        #[inline(always)] fn as_raw_AKAZE(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::AKAZE>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl core::AlgorithmTrait for PtrOfAKAZE {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfAKAZE {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfAgastFeatureDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfAgastFeatureDetector {
        #[inline(always)] pub fn as_raw_PtrOfAgastFeatureDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfAgastFeatureDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::AgastFeatureDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfAgastFeatureDetector {}
    
    impl crate::features2d::AgastFeatureDetector for PtrOfAgastFeatureDetector {
        #[inline(always)] fn as_raw_AgastFeatureDetector(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::AgastFeatureDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl core::AlgorithmTrait for PtrOfAgastFeatureDetector {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfAgastFeatureDetector {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfBFMatcher {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBFMatcher {
        #[inline(always)] pub fn as_raw_PtrOfBFMatcher(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBFMatcher {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::BFMatcher>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBFMatcher {}
    
    impl PtrOfBFMatcher {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BFMatcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BFMatcherRef {
            let inner = crate::features2d::BFMatcher { ptr: self.get_inner() };
            BFMatcherRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BFMatcherRefMut {
            let inner = crate::features2d::BFMatcher { ptr: self.get_inner() };
            BFMatcherRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BFMatcherRef<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::BFMatcher>,
        owner: std::marker::PhantomData<&'o types::PtrOfBFMatcher>,
    }
    
    impl std::ops::Deref for BFMatcherRef<'_> {
        type Target = crate::features2d::BFMatcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BFMatcherRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::BFMatcher>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBFMatcher>,
    }
    
    impl std::ops::Deref for BFMatcherRefMut<'_> {
        type Target = crate::features2d::BFMatcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BFMatcherRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfBRISK {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBRISK {
        #[inline(always)] pub fn as_raw_PtrOfBRISK(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBRISK {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::BRISK>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBRISK {}
    
    impl PtrOfBRISK {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BRISK>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BRISKRef {
            let inner = crate::features2d::BRISK { ptr: self.get_inner() };
            BRISKRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BRISKRefMut {
            let inner = crate::features2d::BRISK { ptr: self.get_inner() };
            BRISKRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BRISKRef<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::BRISK>,
        owner: std::marker::PhantomData<&'o types::PtrOfBRISK>,
    }
    
    impl std::ops::Deref for BRISKRef<'_> {
        type Target = crate::features2d::BRISK;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BRISKRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::BRISK>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBRISK>,
    }
    
    impl std::ops::Deref for BRISKRefMut<'_> {
        type Target = crate::features2d::BRISK;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BRISKRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfDescriptorMatcher {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDescriptorMatcher {
        #[inline(always)] pub fn as_raw_PtrOfDescriptorMatcher(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDescriptorMatcher {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::DescriptorMatcher>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDescriptorMatcher {}
    
    impl core::AlgorithmTrait for PtrOfDescriptorMatcher {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::DescriptorMatcher for PtrOfDescriptorMatcher {
        #[inline(always)] fn as_raw_DescriptorMatcher(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::DescriptorMatcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFastFeatureDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFastFeatureDetector {
        #[inline(always)] pub fn as_raw_PtrOfFastFeatureDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFastFeatureDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::FastFeatureDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFastFeatureDetector {}
    
    impl core::AlgorithmTrait for PtrOfFastFeatureDetector {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::FastFeatureDetector for PtrOfFastFeatureDetector {
        #[inline(always)] fn as_raw_FastFeatureDetector(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::FastFeatureDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfFastFeatureDetector {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFeature2D {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFeature2D {
        #[inline(always)] pub fn as_raw_PtrOfFeature2D(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFeature2D {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::Feature2D>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFeature2D {}
    
    impl core::AlgorithmTrait for PtrOfFeature2D {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfFeature2D {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFlannBasedMatcher {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFlannBasedMatcher {
        #[inline(always)] pub fn as_raw_PtrOfFlannBasedMatcher(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFlannBasedMatcher {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::FlannBasedMatcher>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFlannBasedMatcher {}
    
    impl PtrOfFlannBasedMatcher {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::FlannBasedMatcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> FlannBasedMatcherRef {
            let inner = crate::features2d::FlannBasedMatcher { ptr: self.get_inner() };
            FlannBasedMatcherRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> FlannBasedMatcherRefMut {
            let inner = crate::features2d::FlannBasedMatcher { ptr: self.get_inner() };
            FlannBasedMatcherRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct FlannBasedMatcherRef<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::FlannBasedMatcher>,
        owner: std::marker::PhantomData<&'o types::PtrOfFlannBasedMatcher>,
    }
    
    impl std::ops::Deref for FlannBasedMatcherRef<'_> {
        type Target = crate::features2d::FlannBasedMatcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct FlannBasedMatcherRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::FlannBasedMatcher>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfFlannBasedMatcher>,
    }
    
    impl std::ops::Deref for FlannBasedMatcherRefMut<'_> {
        type Target = crate::features2d::FlannBasedMatcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for FlannBasedMatcherRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfGFTTDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfGFTTDetector {
        #[inline(always)] pub fn as_raw_PtrOfGFTTDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfGFTTDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::GFTTDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfGFTTDetector {}
    
    impl core::AlgorithmTrait for PtrOfGFTTDetector {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfGFTTDetector {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::GFTTDetector for PtrOfGFTTDetector {
        #[inline(always)] fn as_raw_GFTTDetector(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::GFTTDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfKAZE {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfKAZE {
        #[inline(always)] pub fn as_raw_PtrOfKAZE(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfKAZE {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::KAZE>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfKAZE {}
    
    impl core::AlgorithmTrait for PtrOfKAZE {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfKAZE {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::KAZE for PtrOfKAZE {
        #[inline(always)] fn as_raw_KAZE(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::KAZE>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMSER {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMSER {
        #[inline(always)] pub fn as_raw_PtrOfMSER(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMSER {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::MSER>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMSER {}
    
    impl core::AlgorithmTrait for PtrOfMSER {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfMSER {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::MSER for PtrOfMSER {
        #[inline(always)] fn as_raw_MSER(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MSER>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfORB {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfORB {
        #[inline(always)] pub fn as_raw_PtrOfORB(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfORB {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ORB>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfORB {}
    
    impl core::AlgorithmTrait for PtrOfORB {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfORB {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::ORB for PtrOfORB {
        #[inline(always)] fn as_raw_ORB(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ORB>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSimpleBlobDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSimpleBlobDetector {
        #[inline(always)] pub fn as_raw_PtrOfSimpleBlobDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSimpleBlobDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::SimpleBlobDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSimpleBlobDetector {}
    
    impl PtrOfSimpleBlobDetector {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::SimpleBlobDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> SimpleBlobDetectorRef {
            let inner = crate::features2d::SimpleBlobDetector { ptr: self.get_inner() };
            SimpleBlobDetectorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> SimpleBlobDetectorRefMut {
            let inner = crate::features2d::SimpleBlobDetector { ptr: self.get_inner() };
            SimpleBlobDetectorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct SimpleBlobDetectorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::SimpleBlobDetector>,
        owner: std::marker::PhantomData<&'o types::PtrOfSimpleBlobDetector>,
    }
    
    impl std::ops::Deref for SimpleBlobDetectorRef<'_> {
        type Target = crate::features2d::SimpleBlobDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct SimpleBlobDetectorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::features2d::SimpleBlobDetector>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfSimpleBlobDetector>,
    }
    
    impl std::ops::Deref for SimpleBlobDetectorRefMut<'_> {
        type Target = crate::features2d::SimpleBlobDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for SimpleBlobDetectorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
}
pub use features2d_types::*;

#[cfg(feature = "contrib")]
mod freetype_types {
    use super::*;

    pub struct PtrOfFreeType2 {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFreeType2 {
        #[inline(always)] pub fn as_raw_PtrOfFreeType2(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFreeType2 {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::freetype::FreeType2>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFreeType2 {}
    
    impl core::AlgorithmTrait for PtrOfFreeType2 {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::freetype::FreeType2 for PtrOfFreeType2 {
        #[inline(always)] fn as_raw_FreeType2(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::freetype::FreeType2>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use freetype_types::*;

#[cfg(feature = "contrib")]
mod hdf_types {
    use super::*;

    pub struct PtrOfHDF5 {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfHDF5 {
        #[inline(always)] pub fn as_raw_PtrOfHDF5(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfHDF5 {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::hdf::HDF5>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfHDF5 {}
    
    impl crate::hdf::HDF5 for PtrOfHDF5 {
        #[inline(always)] fn as_raw_HDF5(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::hdf::HDF5>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use hdf_types::*;

mod imgproc_types {
    use super::*;

    pub struct PtrOfCLAHE {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfCLAHE {
        #[inline(always)] pub fn as_raw_PtrOfCLAHE(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfCLAHE {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::CLAHE>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfCLAHE {}
    
    impl core::AlgorithmTrait for PtrOfCLAHE {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::imgproc::CLAHE for PtrOfCLAHE {
        #[inline(always)] fn as_raw_CLAHE(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::CLAHE>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfGeneralizedHoughBallard {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfGeneralizedHoughBallard {
        #[inline(always)] pub fn as_raw_PtrOfGeneralizedHoughBallard(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfGeneralizedHoughBallard {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::GeneralizedHoughBallard>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfGeneralizedHoughBallard {}
    
    impl core::AlgorithmTrait for PtrOfGeneralizedHoughBallard {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughBallard {
        #[inline(always)] fn as_raw_GeneralizedHough(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::GeneralizedHough>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::imgproc::GeneralizedHoughBallard for PtrOfGeneralizedHoughBallard {
        #[inline(always)] fn as_raw_GeneralizedHoughBallard(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::GeneralizedHoughBallard>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfGeneralizedHoughGuil {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfGeneralizedHoughGuil {
        #[inline(always)] pub fn as_raw_PtrOfGeneralizedHoughGuil(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfGeneralizedHoughGuil {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::GeneralizedHoughGuil>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfGeneralizedHoughGuil {}
    
    impl core::AlgorithmTrait for PtrOfGeneralizedHoughGuil {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::imgproc::GeneralizedHough for PtrOfGeneralizedHoughGuil {
        #[inline(always)] fn as_raw_GeneralizedHough(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::GeneralizedHough>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::imgproc::GeneralizedHoughGuil for PtrOfGeneralizedHoughGuil {
        #[inline(always)] fn as_raw_GeneralizedHoughGuil(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::GeneralizedHoughGuil>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfLineSegmentDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLineSegmentDetector {
        #[inline(always)] pub fn as_raw_PtrOfLineSegmentDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLineSegmentDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::LineSegmentDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLineSegmentDetector {}
    
    impl core::AlgorithmTrait for PtrOfLineSegmentDetector {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::imgproc::LineSegmentDetector for PtrOfLineSegmentDetector {
        #[inline(always)] fn as_raw_LineSegmentDetector(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::LineSegmentDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use imgproc_types::*;

#[cfg(feature = "contrib")]
mod line_descriptor_types {
    use super::*;

    pub struct PtrOfBinaryDescriptor {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBinaryDescriptor {
        #[inline(always)] pub fn as_raw_PtrOfBinaryDescriptor(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBinaryDescriptor {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::line_descriptor::BinaryDescriptor>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBinaryDescriptor {}
    
    impl PtrOfBinaryDescriptor {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::line_descriptor::BinaryDescriptor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BinaryDescriptorRef {
            let inner = crate::line_descriptor::BinaryDescriptor { ptr: self.get_inner() };
            BinaryDescriptorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BinaryDescriptorRefMut {
            let inner = crate::line_descriptor::BinaryDescriptor { ptr: self.get_inner() };
            BinaryDescriptorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BinaryDescriptorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::line_descriptor::BinaryDescriptor>,
        owner: std::marker::PhantomData<&'o types::PtrOfBinaryDescriptor>,
    }
    
    impl std::ops::Deref for BinaryDescriptorRef<'_> {
        type Target = crate::line_descriptor::BinaryDescriptor;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BinaryDescriptorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::line_descriptor::BinaryDescriptor>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBinaryDescriptor>,
    }
    
    impl std::ops::Deref for BinaryDescriptorRefMut<'_> {
        type Target = crate::line_descriptor::BinaryDescriptor;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BinaryDescriptorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfBinaryDescriptorMatcher {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBinaryDescriptorMatcher {
        #[inline(always)] pub fn as_raw_PtrOfBinaryDescriptorMatcher(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBinaryDescriptorMatcher {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::line_descriptor::BinaryDescriptorMatcher>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBinaryDescriptorMatcher {}
    
    impl PtrOfBinaryDescriptorMatcher {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::line_descriptor::BinaryDescriptorMatcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BinaryDescriptorMatcherRef {
            let inner = crate::line_descriptor::BinaryDescriptorMatcher { ptr: self.get_inner() };
            BinaryDescriptorMatcherRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BinaryDescriptorMatcherRefMut {
            let inner = crate::line_descriptor::BinaryDescriptorMatcher { ptr: self.get_inner() };
            BinaryDescriptorMatcherRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BinaryDescriptorMatcherRef<'o> {
        inner: std::mem::ManuallyDrop<crate::line_descriptor::BinaryDescriptorMatcher>,
        owner: std::marker::PhantomData<&'o types::PtrOfBinaryDescriptorMatcher>,
    }
    
    impl std::ops::Deref for BinaryDescriptorMatcherRef<'_> {
        type Target = crate::line_descriptor::BinaryDescriptorMatcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BinaryDescriptorMatcherRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::line_descriptor::BinaryDescriptorMatcher>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBinaryDescriptorMatcher>,
    }
    
    impl std::ops::Deref for BinaryDescriptorMatcherRefMut<'_> {
        type Target = crate::line_descriptor::BinaryDescriptorMatcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BinaryDescriptorMatcherRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfLSDDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLSDDetector {
        #[inline(always)] pub fn as_raw_PtrOfLSDDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLSDDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::line_descriptor::LSDDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLSDDetector {}
    
    impl PtrOfLSDDetector {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::line_descriptor::LSDDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> LSDDetectorRef {
            let inner = crate::line_descriptor::LSDDetector { ptr: self.get_inner() };
            LSDDetectorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> LSDDetectorRefMut {
            let inner = crate::line_descriptor::LSDDetector { ptr: self.get_inner() };
            LSDDetectorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct LSDDetectorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::line_descriptor::LSDDetector>,
        owner: std::marker::PhantomData<&'o types::PtrOfLSDDetector>,
    }
    
    impl std::ops::Deref for LSDDetectorRef<'_> {
        type Target = crate::line_descriptor::LSDDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct LSDDetectorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::line_descriptor::LSDDetector>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfLSDDetector>,
    }
    
    impl std::ops::Deref for LSDDetectorRefMut<'_> {
        type Target = crate::line_descriptor::LSDDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for LSDDetectorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct VectorOfKeyLine {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfKeyLine {
        #[inline(always)] pub fn as_raw_VectorOfKeyLine(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfKeyLine {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfKeyLine {
        type Item = crate::line_descriptor::KeyLine;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfKeyLine {
        type Item = crate::line_descriptor::KeyLine;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfKeyLine>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfKeyLine {
        type Storage = crate::line_descriptor::KeyLine;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::line_descriptor::KeyLine>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<cv::line_descriptor::KeyLine>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<cv::line_descriptor::KeyLine>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<cv::line_descriptor::KeyLine>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfKeyLine();
                cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::line_descriptor::KeyLine;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfKeyLine();
            let val = val.as_raw_KeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", val as "cv::line_descriptor::KeyLine*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfKeyLine();
            let val = val.as_raw_KeyLine();
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", index as "size_t", val as "cv::line_descriptor::KeyLine*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<cv::line_descriptor::KeyLine>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::line_descriptor::KeyLine(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::line_descriptor::KeyLine { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfKeyLine();
            crate::line_descriptor::KeyLine { ptr: cpp!(unsafe [vec as "const std::vector<cv::line_descriptor::KeyLine>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::line_descriptor::KeyLine((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfKeyLine();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", index as "size_t", val as "cv::line_descriptor::KeyLine*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfKeyLine();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::line_descriptor::KeyLine>*", index as "size_t", val as "cv::line_descriptor::KeyLine*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfKeyLine {}
    
    pub struct VectorOfVectorOfKeyLine {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfVectorOfKeyLine {
        #[inline(always)] pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfVectorOfKeyLine {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfVectorOfKeyLine {
        type Item = types::VectorOfKeyLine;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfVectorOfKeyLine {
        type Item = types::VectorOfKeyLine;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfVectorOfKeyLine>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfVectorOfKeyLine {
        type Storage = types::VectorOfKeyLine;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<std::vector<cv::line_descriptor::KeyLine>>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::line_descriptor::KeyLine>>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::line_descriptor::KeyLine>>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::line_descriptor::KeyLine>>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfVectorOfKeyLine();
                cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*"] {
                vec->clear();
            })
        }
    
        type Arg = types::VectorOfKeyLine;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            let val = val.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", val as "std::vector<cv::line_descriptor::KeyLine>*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            let val = val.as_raw_VectorOfKeyLine();
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index as "size_t", val as "std::vector<cv::line_descriptor::KeyLine>*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            cpp!(unsafe [vec as "const std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new std::vector<cv::line_descriptor::KeyLine>(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| types::VectorOfKeyLine { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            types::VectorOfKeyLine { ptr: cpp!(unsafe [vec as "const std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index as "size_t"] -> *mut c_void as "void*" {
                return new std::vector<cv::line_descriptor::KeyLine>((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index as "size_t", val as "std::vector<cv::line_descriptor::KeyLine>*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfVectorOfKeyLine();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<std::vector<cv::line_descriptor::KeyLine>>*", index as "size_t", val as "std::vector<cv::line_descriptor::KeyLine>*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfVectorOfKeyLine {}
    
}
#[cfg(feature = "contrib")]
pub use line_descriptor_types::*;

mod ml_types {
    use super::*;

    pub struct PtrOfANN_MLP {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfANN_MLP {
        #[inline(always)] pub fn as_raw_PtrOfANN_MLP(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfANN_MLP {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::ANN_MLP>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfANN_MLP {}
    
    impl core::AlgorithmTrait for PtrOfANN_MLP {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::ANN_MLP for PtrOfANN_MLP {
        #[inline(always)] fn as_raw_ANN_MLP(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::ANN_MLP>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfANN_MLP {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfBoost {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBoost {
        #[inline(always)] pub fn as_raw_PtrOfBoost(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBoost {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::Boost>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBoost {}
    
    impl core::AlgorithmTrait for PtrOfBoost {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::Boost for PtrOfBoost {
        #[inline(always)] fn as_raw_Boost(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::Boost>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::DTrees for PtrOfBoost {
        #[inline(always)] fn as_raw_DTrees(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::DTrees>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfBoost {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfDTrees {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDTrees {
        #[inline(always)] pub fn as_raw_PtrOfDTrees(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDTrees {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::DTrees>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDTrees {}
    
    impl core::AlgorithmTrait for PtrOfDTrees {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::DTrees for PtrOfDTrees {
        #[inline(always)] fn as_raw_DTrees(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::DTrees>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfDTrees {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfEM {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfEM {
        #[inline(always)] pub fn as_raw_PtrOfEM(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfEM {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::EM>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfEM {}
    
    impl core::AlgorithmTrait for PtrOfEM {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::EM for PtrOfEM {
        #[inline(always)] fn as_raw_EM(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::EM>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfEM {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfKNearest {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfKNearest {
        #[inline(always)] pub fn as_raw_PtrOfKNearest(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfKNearest {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::KNearest>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfKNearest {}
    
    impl core::AlgorithmTrait for PtrOfKNearest {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::KNearest for PtrOfKNearest {
        #[inline(always)] fn as_raw_KNearest(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::KNearest>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfKNearest {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfKernel {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfKernel {
        #[inline(always)] pub fn as_raw_PtrOfKernel(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfKernel {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::SVM::Kernel>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfKernel {}
    
    impl core::AlgorithmTrait for PtrOfKernel {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::SVM_Kernel for PtrOfKernel {
        #[inline(always)] fn as_raw_SVM_Kernel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::SVM::Kernel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfLogisticRegression {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLogisticRegression {
        #[inline(always)] pub fn as_raw_PtrOfLogisticRegression(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLogisticRegression {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::LogisticRegression>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLogisticRegression {}
    
    impl core::AlgorithmTrait for PtrOfLogisticRegression {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::LogisticRegression for PtrOfLogisticRegression {
        #[inline(always)] fn as_raw_LogisticRegression(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::LogisticRegression>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfLogisticRegression {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfNormalBayesClassifier {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfNormalBayesClassifier {
        #[inline(always)] pub fn as_raw_PtrOfNormalBayesClassifier(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfNormalBayesClassifier {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::NormalBayesClassifier>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfNormalBayesClassifier {}
    
    impl core::AlgorithmTrait for PtrOfNormalBayesClassifier {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::NormalBayesClassifier for PtrOfNormalBayesClassifier {
        #[inline(always)] fn as_raw_NormalBayesClassifier(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::NormalBayesClassifier>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfNormalBayesClassifier {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfRTrees {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfRTrees {
        #[inline(always)] pub fn as_raw_PtrOfRTrees(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfRTrees {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::RTrees>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfRTrees {}
    
    impl core::AlgorithmTrait for PtrOfRTrees {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::DTrees for PtrOfRTrees {
        #[inline(always)] fn as_raw_DTrees(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::DTrees>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::RTrees for PtrOfRTrees {
        #[inline(always)] fn as_raw_RTrees(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::RTrees>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfRTrees {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSVM {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSVM {
        #[inline(always)] pub fn as_raw_PtrOfSVM(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSVM {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::SVM>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSVM {}
    
    impl core::AlgorithmTrait for PtrOfSVM {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::SVM for PtrOfSVM {
        #[inline(always)] fn as_raw_SVM(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::SVM>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfSVM {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSVMSGD {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSVMSGD {
        #[inline(always)] pub fn as_raw_PtrOfSVMSGD(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSVMSGD {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::SVMSGD>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSVMSGD {}
    
    impl core::AlgorithmTrait for PtrOfSVMSGD {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::SVMSGD for PtrOfSVMSGD {
        #[inline(always)] fn as_raw_SVMSGD(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::SVMSGD>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::ml::StatModel for PtrOfSVMSGD {
        #[inline(always)] fn as_raw_StatModel(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::StatModel>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTrainData {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTrainData {
        #[inline(always)] pub fn as_raw_PtrOfTrainData(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTrainData {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ml::TrainData>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTrainData {}
    
    impl crate::ml::TrainData for PtrOfTrainData {
        #[inline(always)] fn as_raw_TrainData(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ml::TrainData>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct VectorOfNode {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfNode {
        #[inline(always)] pub fn as_raw_VectorOfNode(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfNode {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfNode {
        type Item = crate::ml::DTrees_Node;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfNode {
        type Item = crate::ml::DTrees_Node;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfNode>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfNode {
        type Storage = crate::ml::DTrees_Node;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::ml::DTrees::Node>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfNode();
                cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::ml::DTrees_Node;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfNode();
            let val = val.as_raw_DTrees_Node();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", val as "cv::ml::DTrees::Node*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfNode();
            let val = val.as_raw_DTrees_Node();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t", val as "cv::ml::DTrees::Node*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfNode();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::ml::DTrees::Node(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::ml::DTrees_Node { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfNode();
            crate::ml::DTrees_Node { ptr: cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Node>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::ml::DTrees::Node((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfNode();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t", val as "cv::ml::DTrees::Node*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfNode();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Node>*", index as "size_t", val as "cv::ml::DTrees::Node*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfNode {}
    
    pub struct VectorOfSplit {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfSplit {
        #[inline(always)] pub fn as_raw_VectorOfSplit(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfSplit {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfSplit {
        type Item = crate::ml::DTrees_Split;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfSplit {
        type Item = crate::ml::DTrees_Split;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfSplit>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfSplit {
        type Storage = crate::ml::DTrees_Split;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::ml::DTrees::Split>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfSplit();
                cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::ml::DTrees_Split;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfSplit();
            let val = val.as_raw_DTrees_Split();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", val as "cv::ml::DTrees::Split*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfSplit();
            let val = val.as_raw_DTrees_Split();
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t", val as "cv::ml::DTrees::Split*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfSplit();
            cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::ml::DTrees::Split(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::ml::DTrees_Split { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfSplit();
            crate::ml::DTrees_Split { ptr: cpp!(unsafe [vec as "const std::vector<cv::ml::DTrees::Split>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::ml::DTrees::Split((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfSplit();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t", val as "cv::ml::DTrees::Split*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfSplit();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::ml::DTrees::Split>*", index as "size_t", val as "cv::ml::DTrees::Split*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfSplit {}
    
}
pub use ml_types::*;

mod objdetect_types {
    use super::*;

    pub struct PtrOfMaskGenerator {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMaskGenerator {
        #[inline(always)] pub fn as_raw_PtrOfMaskGenerator(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMaskGenerator {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMaskGenerator {}
    
    impl crate::objdetect::BaseCascadeClassifier_MaskGenerator for PtrOfMaskGenerator {
        #[inline(always)] fn as_raw_BaseCascadeClassifier_MaskGenerator(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct VectorOfDetectionROI {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfDetectionROI {
        #[inline(always)] pub fn as_raw_VectorOfDetectionROI(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfDetectionROI {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfDetectionROI {
        type Item = crate::objdetect::DetectionROI;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfDetectionROI {
        type Item = crate::objdetect::DetectionROI;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfDetectionROI>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfDetectionROI {
        type Storage = crate::objdetect::DetectionROI;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::DetectionROI>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfDetectionROI();
                cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::objdetect::DetectionROI;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfDetectionROI();
            let val = val.as_raw_DetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", val as "cv::DetectionROI*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfDetectionROI();
            let val = val.as_raw_DetectionROI();
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t", val as "cv::DetectionROI*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfDetectionROI();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::DetectionROI(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::objdetect::DetectionROI { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfDetectionROI();
            crate::objdetect::DetectionROI { ptr: cpp!(unsafe [vec as "const std::vector<cv::DetectionROI>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::DetectionROI((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfDetectionROI();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t", val as "cv::DetectionROI*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfDetectionROI();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::DetectionROI>*", index as "size_t", val as "cv::DetectionROI*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfDetectionROI {}
    
    pub struct VectorOfExtObject {
        pub(crate) ptr: *mut c_void
    }
    
    impl VectorOfExtObject {
        #[inline(always)] pub fn as_raw_VectorOfExtObject(&self) -> *mut c_void { self.ptr }
    
        #[inline]
        pub fn iter(&self) -> crate::templ::VectorRefIterator<Self> {
            crate::templ::VectorRefIterator::new(self)
        }
    }
    
    impl Drop for VectorOfExtObject {
        #[inline]
        fn drop(&mut self) {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*"] {
                delete vec;
            })
        }
    }
    
    impl IntoIterator for VectorOfExtObject {
        type Item = crate::objdetect::DetectionBasedTracker_ExtObject;
        type IntoIter = crate::templ::VectorIterator<Self>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            Self::IntoIter::new(self)
        }
    }
    
    impl<'i> IntoIterator for &'i VectorOfExtObject {
        type Item = crate::objdetect::DetectionBasedTracker_ExtObject;
        type IntoIter = crate::templ::VectorRefIterator<'i, VectorOfExtObject>;
    
        #[inline]
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }
    
    impl<'i> crate::templ::Vector<'i> for VectorOfExtObject {
        type Storage = crate::objdetect::DetectionBasedTracker_ExtObject;
    
        #[inline]
        fn new() -> Self {
            Self { ptr: cpp!(unsafe [] -> *mut c_void as "void*" {
                return new std::vector<cv::DetectionBasedTracker::ExtObject>();
            })}
        }
    
        #[inline]
        fn len(&self) -> size_t {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*"] -> size_t as "size_t" {
                return vec->size();
            })
        }
    
        #[inline]
        fn is_empty(&self) -> bool {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*"] -> bool as "bool" {
                return vec->empty();
            })
        }
    
        #[inline]
        fn capacity(&self) -> size_t {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*"] -> size_t as "size_t" {
                return vec->capacity();
            })
        }
    
        #[inline]
        fn shrink_to_fit(&mut self) {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*"] {
                vec->shrink_to_fit();
            })
        }                
    
        #[inline]
        fn reserve(&mut self, additional: size_t) {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", additional as "size_t"] {
                vec->reserve(vec->size() + additional);
            })
        }
    
        #[inline]
        fn remove(&mut self, index: size_t) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len())?;
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t"] {
                vec->erase(vec->begin() + index);
            });
            Ok(())
        }
    
        #[inline]
        fn swap(&mut self, index1: size_t, index2: size_t) -> Result<()> {
            let len = self.len();
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index1, len)?;
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index2, len)?;
            if index1 != index2 {
                let vec = self.as_raw_VectorOfExtObject();
                cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index1 as "size_t", index2 as "size_t"] {
                    swap((*vec)[index1], (*vec)[index2]);
                });
            }
            Ok(())
        }
    
        #[inline]
        fn clear(&mut self) {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*"] {
                vec->clear();
            })
        }
    
        type Arg = crate::objdetect::DetectionBasedTracker_ExtObject;
        
        #[inline]
        fn push(&mut self, val: Self::Arg) {
            let vec = self.as_raw_VectorOfExtObject();
            let val = val.as_raw_DetectionBasedTracker_ExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", val as "cv::DetectionBasedTracker::ExtObject*"] {
                vec->push_back(*val);
            })
        }
        
        #[inline]
        fn insert(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            crate::templ::Vector::<Storage=Self::Storage, Arg=Self::Arg>::index_check(index, self.len() + 1)?;
            let vec = self.as_raw_VectorOfExtObject();
            let val = val.as_raw_DetectionBasedTracker_ExtObject();
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t", val as "cv::DetectionBasedTracker::ExtObject*"] {
                vec->insert(vec->begin() + index, *val);
            });
            Ok(())
        }
        
        #[inline]
        fn get(&self, index: size_t) -> Result<Self::Storage> {
            let vec = self.as_raw_VectorOfExtObject();
            cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t"] -> crate::sys::cv_return_value_void_X as "cv_return_value_void_X" {
                try {
                    return { Error::Code::StsOk, NULL, new cv::DetectionBasedTracker::ExtObject(vec->at(index)) };
                } VEC_CATCH(cv_return_value_void_X)
            }).into_result().map(|ptr| crate::objdetect::DetectionBasedTracker_ExtObject { ptr })
        }
        
        #[inline]
        unsafe fn get_unchecked(&self, index: size_t) -> Self::Storage {
            let vec = self.as_raw_VectorOfExtObject();
            crate::objdetect::DetectionBasedTracker_ExtObject { ptr: cpp!(unsafe [vec as "const std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t"] -> *mut c_void as "void*" {
                return new cv::DetectionBasedTracker::ExtObject((*vec)[index]);
            })}
        }
        
        #[inline]
        fn set(&mut self, index: size_t, val: Self::Arg) -> Result<()> {
            let vec = self.as_raw_VectorOfExtObject();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t", val as "cv::DetectionBasedTracker::ExtObject*"] -> crate::sys::cv_return_value_void as "cv_return_value_void" {
                try {
                    vec->at(index) = *val;
                    return { Error::Code::StsOk, NULL };
                } VEC_CATCH(cv_return_value_void)
            }).into_result()
        }
        
        #[inline]
        unsafe fn set_unchecked(&mut self, index: size_t, val: Self::Arg) {
            let vec = self.as_raw_VectorOfExtObject();
            let val = val.ptr;
            cpp!(unsafe [vec as "std::vector<cv::DetectionBasedTracker::ExtObject>*", index as "size_t", val as "cv::DetectionBasedTracker::ExtObject*"] {
                (*vec)[index] = *val;
            })
        }
    }
    
    unsafe impl Send for VectorOfExtObject {}
    
}
pub use objdetect_types::*;

#[cfg(feature = "contrib")]
mod phase_unwrapping_types {
    use super::*;

    pub struct PtrOfHistogramPhaseUnwrapping {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfHistogramPhaseUnwrapping {
        #[inline(always)] pub fn as_raw_PtrOfHistogramPhaseUnwrapping(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfHistogramPhaseUnwrapping {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfHistogramPhaseUnwrapping {}
    
    impl core::AlgorithmTrait for PtrOfHistogramPhaseUnwrapping {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::phase_unwrapping::HistogramPhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
        #[inline(always)] fn as_raw_HistogramPhaseUnwrapping(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::phase_unwrapping::HistogramPhaseUnwrapping>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::phase_unwrapping::PhaseUnwrapping for PtrOfHistogramPhaseUnwrapping {
        #[inline(always)] fn as_raw_PhaseUnwrapping(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::phase_unwrapping::PhaseUnwrapping>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use phase_unwrapping_types::*;

mod photo_types {
    use super::*;

    pub struct PtrOfAlignMTB {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfAlignMTB {
        #[inline(always)] pub fn as_raw_PtrOfAlignMTB(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfAlignMTB {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::AlignMTB>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfAlignMTB {}
    
    impl core::AlgorithmTrait for PtrOfAlignMTB {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::AlignExposures for PtrOfAlignMTB {
        #[inline(always)] fn as_raw_AlignExposures(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::AlignExposures>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::AlignMTB for PtrOfAlignMTB {
        #[inline(always)] fn as_raw_AlignMTB(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::AlignMTB>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfCalibrateDebevec {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfCalibrateDebevec {
        #[inline(always)] pub fn as_raw_PtrOfCalibrateDebevec(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfCalibrateDebevec {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::CalibrateDebevec>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfCalibrateDebevec {}
    
    impl core::AlgorithmTrait for PtrOfCalibrateDebevec {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::CalibrateCRF for PtrOfCalibrateDebevec {
        #[inline(always)] fn as_raw_CalibrateCRF(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::CalibrateCRF>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::CalibrateDebevec for PtrOfCalibrateDebevec {
        #[inline(always)] fn as_raw_CalibrateDebevec(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::CalibrateDebevec>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfCalibrateRobertson {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfCalibrateRobertson {
        #[inline(always)] pub fn as_raw_PtrOfCalibrateRobertson(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfCalibrateRobertson {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::CalibrateRobertson>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfCalibrateRobertson {}
    
    impl core::AlgorithmTrait for PtrOfCalibrateRobertson {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::CalibrateCRF for PtrOfCalibrateRobertson {
        #[inline(always)] fn as_raw_CalibrateCRF(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::CalibrateCRF>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::CalibrateRobertson for PtrOfCalibrateRobertson {
        #[inline(always)] fn as_raw_CalibrateRobertson(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::CalibrateRobertson>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMergeDebevec {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMergeDebevec {
        #[inline(always)] pub fn as_raw_PtrOfMergeDebevec(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMergeDebevec {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::MergeDebevec>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMergeDebevec {}
    
    impl core::AlgorithmTrait for PtrOfMergeDebevec {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::MergeDebevec for PtrOfMergeDebevec {
        #[inline(always)] fn as_raw_MergeDebevec(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MergeDebevec>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::MergeExposures for PtrOfMergeDebevec {
        #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MergeExposures>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMergeMertens {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMergeMertens {
        #[inline(always)] pub fn as_raw_PtrOfMergeMertens(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMergeMertens {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::MergeMertens>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMergeMertens {}
    
    impl core::AlgorithmTrait for PtrOfMergeMertens {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::MergeExposures for PtrOfMergeMertens {
        #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MergeExposures>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::MergeMertens for PtrOfMergeMertens {
        #[inline(always)] fn as_raw_MergeMertens(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MergeMertens>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMergeRobertson {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMergeRobertson {
        #[inline(always)] pub fn as_raw_PtrOfMergeRobertson(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMergeRobertson {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::MergeRobertson>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMergeRobertson {}
    
    impl core::AlgorithmTrait for PtrOfMergeRobertson {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::MergeExposures for PtrOfMergeRobertson {
        #[inline(always)] fn as_raw_MergeExposures(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MergeExposures>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::MergeRobertson for PtrOfMergeRobertson {
        #[inline(always)] fn as_raw_MergeRobertson(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::MergeRobertson>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTonemap {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTonemap {
        #[inline(always)] pub fn as_raw_PtrOfTonemap(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTonemap {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::Tonemap>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTonemap {}
    
    impl core::AlgorithmTrait for PtrOfTonemap {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::Tonemap for PtrOfTonemap {
        #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTonemapDrago {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTonemapDrago {
        #[inline(always)] pub fn as_raw_PtrOfTonemapDrago(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTonemapDrago {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::TonemapDrago>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTonemapDrago {}
    
    impl core::AlgorithmTrait for PtrOfTonemapDrago {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::Tonemap for PtrOfTonemapDrago {
        #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::TonemapDrago for PtrOfTonemapDrago {
        #[inline(always)] fn as_raw_TonemapDrago(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::TonemapDrago>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTonemapDurand {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTonemapDurand {
        #[inline(always)] pub fn as_raw_PtrOfTonemapDurand(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTonemapDurand {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::TonemapDurand>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTonemapDurand {}
    
    impl core::AlgorithmTrait for PtrOfTonemapDurand {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::Tonemap for PtrOfTonemapDurand {
        #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::TonemapDurand for PtrOfTonemapDurand {
        #[inline(always)] fn as_raw_TonemapDurand(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::TonemapDurand>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTonemapMantiuk {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTonemapMantiuk {
        #[inline(always)] pub fn as_raw_PtrOfTonemapMantiuk(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTonemapMantiuk {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::TonemapMantiuk>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTonemapMantiuk {}
    
    impl core::AlgorithmTrait for PtrOfTonemapMantiuk {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::Tonemap for PtrOfTonemapMantiuk {
        #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::TonemapMantiuk for PtrOfTonemapMantiuk {
        #[inline(always)] fn as_raw_TonemapMantiuk(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::TonemapMantiuk>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfTonemapReinhard {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfTonemapReinhard {
        #[inline(always)] pub fn as_raw_PtrOfTonemapReinhard(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfTonemapReinhard {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::TonemapReinhard>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfTonemapReinhard {}
    
    impl core::AlgorithmTrait for PtrOfTonemapReinhard {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::Tonemap for PtrOfTonemapReinhard {
        #[inline(always)] fn as_raw_Tonemap(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Tonemap>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::photo::TonemapReinhard for PtrOfTonemapReinhard {
        #[inline(always)] fn as_raw_TonemapReinhard(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::TonemapReinhard>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use photo_types::*;

#[cfg(feature = "contrib")]
mod plot_types {
    use super::*;

    pub struct PtrOfPlot2d {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfPlot2d {
        #[inline(always)] pub fn as_raw_PtrOfPlot2d(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfPlot2d {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::plot::Plot2d>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfPlot2d {}
    
    impl core::AlgorithmTrait for PtrOfPlot2d {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::plot::Plot2d for PtrOfPlot2d {
        #[inline(always)] fn as_raw_Plot2d(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::plot::Plot2d>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use plot_types::*;

#[cfg(feature = "contrib")]
mod sfm_types {
    use super::*;

    pub struct PtrOfSFMLibmvEuclideanReconstruction {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSFMLibmvEuclideanReconstruction {
        #[inline(always)] pub fn as_raw_PtrOfSFMLibmvEuclideanReconstruction(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSFMLibmvEuclideanReconstruction {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSFMLibmvEuclideanReconstruction {}
    
    impl crate::sfm::BaseSFM for PtrOfSFMLibmvEuclideanReconstruction {
        #[inline(always)] fn as_raw_BaseSFM(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::sfm::BaseSFM>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::sfm::SFMLibmvEuclideanReconstruction for PtrOfSFMLibmvEuclideanReconstruction {
        #[inline(always)] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::sfm::SFMLibmvEuclideanReconstruction>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use sfm_types::*;

mod shape_types {
    use super::*;

    pub struct PtrOfAffineTransformer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfAffineTransformer {
        #[inline(always)] pub fn as_raw_PtrOfAffineTransformer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfAffineTransformer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::AffineTransformer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfAffineTransformer {}
    
    impl crate::shape::AffineTransformer for PtrOfAffineTransformer {
        #[inline(always)] fn as_raw_AffineTransformer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::AffineTransformer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl core::AlgorithmTrait for PtrOfAffineTransformer {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::ShapeTransformer for PtrOfAffineTransformer {
        #[inline(always)] fn as_raw_ShapeTransformer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ShapeTransformer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfHausdorffDistanceExtractor {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfHausdorffDistanceExtractor {
        #[inline(always)] pub fn as_raw_PtrOfHausdorffDistanceExtractor(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfHausdorffDistanceExtractor {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::HausdorffDistanceExtractor>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfHausdorffDistanceExtractor {}
    
    impl core::AlgorithmTrait for PtrOfHausdorffDistanceExtractor {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::HausdorffDistanceExtractor for PtrOfHausdorffDistanceExtractor {
        #[inline(always)] fn as_raw_HausdorffDistanceExtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::HausdorffDistanceExtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::ShapeDistanceExtractor for PtrOfHausdorffDistanceExtractor {
        #[inline(always)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ShapeDistanceExtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfHistogramCostExtractor {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfHistogramCostExtractor {
        #[inline(always)] pub fn as_raw_PtrOfHistogramCostExtractor(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfHistogramCostExtractor {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::HistogramCostExtractor>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfHistogramCostExtractor {}
    
    impl core::AlgorithmTrait for PtrOfHistogramCostExtractor {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::HistogramCostExtractor for PtrOfHistogramCostExtractor {
        #[inline(always)] fn as_raw_HistogramCostExtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::HistogramCostExtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfShapeContextDistanceExtractor {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfShapeContextDistanceExtractor {
        #[inline(always)] pub fn as_raw_PtrOfShapeContextDistanceExtractor(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfShapeContextDistanceExtractor {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ShapeContextDistanceExtractor>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfShapeContextDistanceExtractor {}
    
    impl core::AlgorithmTrait for PtrOfShapeContextDistanceExtractor {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::ShapeContextDistanceExtractor for PtrOfShapeContextDistanceExtractor {
        #[inline(always)] fn as_raw_ShapeContextDistanceExtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ShapeContextDistanceExtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::ShapeDistanceExtractor for PtrOfShapeContextDistanceExtractor {
        #[inline(always)] fn as_raw_ShapeDistanceExtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ShapeDistanceExtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfThinPlateSplineShapeTransformer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfThinPlateSplineShapeTransformer {
        #[inline(always)] pub fn as_raw_PtrOfThinPlateSplineShapeTransformer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfThinPlateSplineShapeTransformer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::ThinPlateSplineShapeTransformer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfThinPlateSplineShapeTransformer {}
    
    impl core::AlgorithmTrait for PtrOfThinPlateSplineShapeTransformer {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::ShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
        #[inline(always)] fn as_raw_ShapeTransformer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ShapeTransformer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::shape::ThinPlateSplineShapeTransformer for PtrOfThinPlateSplineShapeTransformer {
        #[inline(always)] fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::ThinPlateSplineShapeTransformer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use shape_types::*;

mod stitching_types {
    use super::*;

    pub struct PtrOfStitcher {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfStitcher {
        #[inline(always)] pub fn as_raw_PtrOfStitcher(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfStitcher {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::Stitcher>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfStitcher {}
    
    impl PtrOfStitcher {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Stitcher>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> StitcherRef {
            let inner = crate::stitching::Stitcher { ptr: self.get_inner() };
            StitcherRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> StitcherRefMut {
            let inner = crate::stitching::Stitcher { ptr: self.get_inner() };
            StitcherRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct StitcherRef<'o> {
        inner: std::mem::ManuallyDrop<crate::stitching::Stitcher>,
        owner: std::marker::PhantomData<&'o types::PtrOfStitcher>,
    }
    
    impl std::ops::Deref for StitcherRef<'_> {
        type Target = crate::stitching::Stitcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct StitcherRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::stitching::Stitcher>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfStitcher>,
    }
    
    impl std::ops::Deref for StitcherRefMut<'_> {
        type Target = crate::stitching::Stitcher;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for StitcherRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
}
pub use stitching_types::*;

#[cfg(feature = "contrib")]
mod structured_light_types {
    use super::*;

    pub struct PtrOfGrayCodePattern {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfGrayCodePattern {
        #[inline(always)] pub fn as_raw_PtrOfGrayCodePattern(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfGrayCodePattern {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::structured_light::GrayCodePattern>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfGrayCodePattern {}
    
    impl core::AlgorithmTrait for PtrOfGrayCodePattern {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::structured_light::GrayCodePattern for PtrOfGrayCodePattern {
        #[inline(always)] fn as_raw_GrayCodePattern(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::structured_light::GrayCodePattern>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::structured_light::StructuredLightPattern for PtrOfGrayCodePattern {
        #[inline(always)] fn as_raw_StructuredLightPattern(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::structured_light::StructuredLightPattern>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSinusoidalPattern {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSinusoidalPattern {
        #[inline(always)] pub fn as_raw_PtrOfSinusoidalPattern(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSinusoidalPattern {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::structured_light::SinusoidalPattern>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSinusoidalPattern {}
    
    impl core::AlgorithmTrait for PtrOfSinusoidalPattern {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::structured_light::SinusoidalPattern for PtrOfSinusoidalPattern {
        #[inline(always)] fn as_raw_SinusoidalPattern(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::structured_light::SinusoidalPattern>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::structured_light::StructuredLightPattern for PtrOfSinusoidalPattern {
        #[inline(always)] fn as_raw_StructuredLightPattern(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::structured_light::StructuredLightPattern>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use structured_light_types::*;

mod superres_types {
    use super::*;

    pub struct PtrOfFrameSource {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFrameSource {
        #[inline(always)] pub fn as_raw_PtrOfFrameSource(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFrameSource {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::superres::FrameSource>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFrameSource {}
    
    impl crate::superres::FrameSource for PtrOfFrameSource {
        #[inline(always)] fn as_raw_FrameSource(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::superres::FrameSource>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSuperResolution {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSuperResolution {
        #[inline(always)] pub fn as_raw_PtrOfSuperResolution(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSuperResolution {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::superres::SuperResolution>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSuperResolution {}
    
    impl core::AlgorithmTrait for PtrOfSuperResolution {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::superres::FrameSource for PtrOfSuperResolution {
        #[inline(always)] fn as_raw_FrameSource(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::superres::FrameSource>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::superres::SuperResolution for PtrOfSuperResolution {
        #[inline(always)] fn as_raw_SuperResolution(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::superres::SuperResolution>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use superres_types::*;

mod video_types {
    use super::*;

    pub struct PtrOfBackgroundSubtractorKNN {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBackgroundSubtractorKNN {
        #[inline(always)] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBackgroundSubtractorKNN {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::BackgroundSubtractorKNN>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBackgroundSubtractorKNN {}
    
    impl core::AlgorithmTrait for PtrOfBackgroundSubtractorKNN {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorKNN {
        #[inline(always)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BackgroundSubtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::BackgroundSubtractorKNN for PtrOfBackgroundSubtractorKNN {
        #[inline(always)] fn as_raw_BackgroundSubtractorKNN(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BackgroundSubtractorKNN>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfBackgroundSubtractorMOG2 {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBackgroundSubtractorMOG2 {
        #[inline(always)] pub fn as_raw_PtrOfBackgroundSubtractorMOG2(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBackgroundSubtractorMOG2 {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::BackgroundSubtractorMOG2>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBackgroundSubtractorMOG2 {}
    
    impl core::AlgorithmTrait for PtrOfBackgroundSubtractorMOG2 {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::BackgroundSubtractor for PtrOfBackgroundSubtractorMOG2 {
        #[inline(always)] fn as_raw_BackgroundSubtractor(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BackgroundSubtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::BackgroundSubtractorMOG2 for PtrOfBackgroundSubtractorMOG2 {
        #[inline(always)] fn as_raw_BackgroundSubtractorMOG2(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::BackgroundSubtractorMOG2>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfDualTVL1OpticalFlow {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDualTVL1OpticalFlow {
        #[inline(always)] pub fn as_raw_PtrOfDualTVL1OpticalFlow(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDualTVL1OpticalFlow {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::DualTVL1OpticalFlow>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDualTVL1OpticalFlow {}
    
    impl core::AlgorithmTrait for PtrOfDualTVL1OpticalFlow {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::DenseOpticalFlow for PtrOfDualTVL1OpticalFlow {
        #[inline(always)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::DenseOpticalFlow>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::DualTVL1OpticalFlow for PtrOfDualTVL1OpticalFlow {
        #[inline(always)] fn as_raw_DualTVL1OpticalFlow(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::DualTVL1OpticalFlow>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFarnebackOpticalFlow {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFarnebackOpticalFlow {
        #[inline(always)] pub fn as_raw_PtrOfFarnebackOpticalFlow(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFarnebackOpticalFlow {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::FarnebackOpticalFlow>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFarnebackOpticalFlow {}
    
    impl core::AlgorithmTrait for PtrOfFarnebackOpticalFlow {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::DenseOpticalFlow for PtrOfFarnebackOpticalFlow {
        #[inline(always)] fn as_raw_DenseOpticalFlow(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::DenseOpticalFlow>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::FarnebackOpticalFlow for PtrOfFarnebackOpticalFlow {
        #[inline(always)] fn as_raw_FarnebackOpticalFlow(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::FarnebackOpticalFlow>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSparsePyrLKOpticalFlow {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSparsePyrLKOpticalFlow {
        #[inline(always)] pub fn as_raw_PtrOfSparsePyrLKOpticalFlow(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSparsePyrLKOpticalFlow {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::SparsePyrLKOpticalFlow>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSparsePyrLKOpticalFlow {}
    
    impl core::AlgorithmTrait for PtrOfSparsePyrLKOpticalFlow {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::SparseOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
        #[inline(always)] fn as_raw_SparseOpticalFlow(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::SparseOpticalFlow>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::video::SparsePyrLKOpticalFlow for PtrOfSparsePyrLKOpticalFlow {
        #[inline(always)] fn as_raw_SparsePyrLKOpticalFlow(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::SparsePyrLKOpticalFlow>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use video_types::*;

mod videostab_types {
    use super::*;

    pub struct PtrOfDeblurerBase {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDeblurerBase {
        #[inline(always)] pub fn as_raw_PtrOfDeblurerBase(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDeblurerBase {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::DeblurerBase>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDeblurerBase {}
    
    impl crate::videostab::DeblurerBase for PtrOfDeblurerBase {
        #[inline(always)] fn as_raw_DeblurerBase(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::DeblurerBase>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfIFrameSource {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfIFrameSource {
        #[inline(always)] pub fn as_raw_PtrOfIFrameSource(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfIFrameSource {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::IFrameSource>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfIFrameSource {}
    
    impl crate::videostab::IFrameSource for PtrOfIFrameSource {
        #[inline(always)] fn as_raw_IFrameSource(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::IFrameSource>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfILog {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfILog {
        #[inline(always)] pub fn as_raw_PtrOfILog(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfILog {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::ILog>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfILog {}
    
    impl crate::videostab::ILog for PtrOfILog {
        #[inline(always)] fn as_raw_ILog(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::ILog>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfIMotionStabilizer {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfIMotionStabilizer {
        #[inline(always)] pub fn as_raw_PtrOfIMotionStabilizer(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfIMotionStabilizer {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::IMotionStabilizer>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfIMotionStabilizer {}
    
    impl crate::videostab::IMotionStabilizer for PtrOfIMotionStabilizer {
        #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::IMotionStabilizer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfImageMotionEstimatorBase {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfImageMotionEstimatorBase {
        #[inline(always)] pub fn as_raw_PtrOfImageMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfImageMotionEstimatorBase {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::ImageMotionEstimatorBase>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfImageMotionEstimatorBase {}
    
    impl crate::videostab::ImageMotionEstimatorBase for PtrOfImageMotionEstimatorBase {
        #[inline(always)] fn as_raw_ImageMotionEstimatorBase(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::ImageMotionEstimatorBase>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfInpainterBase {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfInpainterBase {
        #[inline(always)] pub fn as_raw_PtrOfInpainterBase(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfInpainterBase {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::InpainterBase>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfInpainterBase {}
    
    impl crate::videostab::InpainterBase for PtrOfInpainterBase {
        #[inline(always)] fn as_raw_InpainterBase(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::InpainterBase>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMotionEstimatorBase {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMotionEstimatorBase {
        #[inline(always)] pub fn as_raw_PtrOfMotionEstimatorBase(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMotionEstimatorBase {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::MotionEstimatorBase>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMotionEstimatorBase {}
    
    impl crate::videostab::MotionEstimatorBase for PtrOfMotionEstimatorBase {
        #[inline(always)] fn as_raw_MotionEstimatorBase(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::MotionEstimatorBase>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfMotionFilterBase {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMotionFilterBase {
        #[inline(always)] pub fn as_raw_PtrOfMotionFilterBase(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMotionFilterBase {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::videostab::MotionFilterBase>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMotionFilterBase {}
    
    impl crate::videostab::IMotionStabilizer for PtrOfMotionFilterBase {
        #[inline(always)] fn as_raw_IMotionStabilizer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::IMotionStabilizer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::videostab::MotionFilterBase for PtrOfMotionFilterBase {
        #[inline(always)] fn as_raw_MotionFilterBase(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::videostab::MotionFilterBase>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
pub use videostab_types::*;

#[cfg(feature = "contrib")]
mod xfeatures2d_types {
    use super::*;

    pub struct PtrOfBoostDesc {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBoostDesc {
        #[inline(always)] pub fn as_raw_PtrOfBoostDesc(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBoostDesc {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::BoostDesc>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBoostDesc {}
    
    impl PtrOfBoostDesc {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::BoostDesc>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BoostDescRef {
            let inner = crate::xfeatures2d::BoostDesc { ptr: self.get_inner() };
            BoostDescRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BoostDescRefMut {
            let inner = crate::xfeatures2d::BoostDesc { ptr: self.get_inner() };
            BoostDescRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BoostDescRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::BoostDesc>,
        owner: std::marker::PhantomData<&'o types::PtrOfBoostDesc>,
    }
    
    impl std::ops::Deref for BoostDescRef<'_> {
        type Target = crate::xfeatures2d::BoostDesc;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BoostDescRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::BoostDesc>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBoostDesc>,
    }
    
    impl std::ops::Deref for BoostDescRefMut<'_> {
        type Target = crate::xfeatures2d::BoostDesc;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BoostDescRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfBriefDescriptorExtractor {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfBriefDescriptorExtractor {
        #[inline(always)] pub fn as_raw_PtrOfBriefDescriptorExtractor(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfBriefDescriptorExtractor {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfBriefDescriptorExtractor {}
    
    impl PtrOfBriefDescriptorExtractor {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::BriefDescriptorExtractor>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> BriefDescriptorExtractorRef {
            let inner = crate::xfeatures2d::BriefDescriptorExtractor { ptr: self.get_inner() };
            BriefDescriptorExtractorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> BriefDescriptorExtractorRefMut {
            let inner = crate::xfeatures2d::BriefDescriptorExtractor { ptr: self.get_inner() };
            BriefDescriptorExtractorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct BriefDescriptorExtractorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::BriefDescriptorExtractor>,
        owner: std::marker::PhantomData<&'o types::PtrOfBriefDescriptorExtractor>,
    }
    
    impl std::ops::Deref for BriefDescriptorExtractorRef<'_> {
        type Target = crate::xfeatures2d::BriefDescriptorExtractor;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct BriefDescriptorExtractorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::BriefDescriptorExtractor>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfBriefDescriptorExtractor>,
    }
    
    impl std::ops::Deref for BriefDescriptorExtractorRefMut<'_> {
        type Target = crate::xfeatures2d::BriefDescriptorExtractor;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for BriefDescriptorExtractorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfDAISY {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfDAISY {
        #[inline(always)] pub fn as_raw_PtrOfDAISY(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfDAISY {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::DAISY>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfDAISY {}
    
    impl core::AlgorithmTrait for PtrOfDAISY {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xfeatures2d::DAISY for PtrOfDAISY {
        #[inline(always)] fn as_raw_DAISY(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::DAISY>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfDAISY {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfFREAK {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfFREAK {
        #[inline(always)] pub fn as_raw_PtrOfFREAK(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfFREAK {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::FREAK>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfFREAK {}
    
    impl PtrOfFREAK {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::FREAK>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> FREAKRef {
            let inner = crate::xfeatures2d::FREAK { ptr: self.get_inner() };
            FREAKRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> FREAKRefMut {
            let inner = crate::xfeatures2d::FREAK { ptr: self.get_inner() };
            FREAKRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct FREAKRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::FREAK>,
        owner: std::marker::PhantomData<&'o types::PtrOfFREAK>,
    }
    
    impl std::ops::Deref for FREAKRef<'_> {
        type Target = crate::xfeatures2d::FREAK;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct FREAKRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::FREAK>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfFREAK>,
    }
    
    impl std::ops::Deref for FREAKRefMut<'_> {
        type Target = crate::xfeatures2d::FREAK;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for FREAKRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfLATCH {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLATCH {
        #[inline(always)] pub fn as_raw_PtrOfLATCH(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLATCH {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::LATCH>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLATCH {}
    
    impl PtrOfLATCH {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::LATCH>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> LATCHRef {
            let inner = crate::xfeatures2d::LATCH { ptr: self.get_inner() };
            LATCHRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> LATCHRefMut {
            let inner = crate::xfeatures2d::LATCH { ptr: self.get_inner() };
            LATCHRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct LATCHRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::LATCH>,
        owner: std::marker::PhantomData<&'o types::PtrOfLATCH>,
    }
    
    impl std::ops::Deref for LATCHRef<'_> {
        type Target = crate::xfeatures2d::LATCH;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct LATCHRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::LATCH>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfLATCH>,
    }
    
    impl std::ops::Deref for LATCHRefMut<'_> {
        type Target = crate::xfeatures2d::LATCH;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for LATCHRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfLUCID {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLUCID {
        #[inline(always)] pub fn as_raw_PtrOfLUCID(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLUCID {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::LUCID>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLUCID {}
    
    impl PtrOfLUCID {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::LUCID>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> LUCIDRef {
            let inner = crate::xfeatures2d::LUCID { ptr: self.get_inner() };
            LUCIDRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> LUCIDRefMut {
            let inner = crate::xfeatures2d::LUCID { ptr: self.get_inner() };
            LUCIDRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct LUCIDRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::LUCID>,
        owner: std::marker::PhantomData<&'o types::PtrOfLUCID>,
    }
    
    impl std::ops::Deref for LUCIDRef<'_> {
        type Target = crate::xfeatures2d::LUCID;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct LUCIDRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::LUCID>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfLUCID>,
    }
    
    impl std::ops::Deref for LUCIDRefMut<'_> {
        type Target = crate::xfeatures2d::LUCID;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for LUCIDRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfMSDDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfMSDDetector {
        #[inline(always)] pub fn as_raw_PtrOfMSDDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfMSDDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::MSDDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfMSDDetector {}
    
    impl PtrOfMSDDetector {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::MSDDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> MSDDetectorRef {
            let inner = crate::xfeatures2d::MSDDetector { ptr: self.get_inner() };
            MSDDetectorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> MSDDetectorRefMut {
            let inner = crate::xfeatures2d::MSDDetector { ptr: self.get_inner() };
            MSDDetectorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct MSDDetectorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::MSDDetector>,
        owner: std::marker::PhantomData<&'o types::PtrOfMSDDetector>,
    }
    
    impl std::ops::Deref for MSDDetectorRef<'_> {
        type Target = crate::xfeatures2d::MSDDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct MSDDetectorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::MSDDetector>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfMSDDetector>,
    }
    
    impl std::ops::Deref for MSDDetectorRefMut<'_> {
        type Target = crate::xfeatures2d::MSDDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for MSDDetectorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfPCTSignatures {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfPCTSignatures {
        #[inline(always)] pub fn as_raw_PtrOfPCTSignatures(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfPCTSignatures {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::PCTSignatures>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfPCTSignatures {}
    
    impl core::AlgorithmTrait for PtrOfPCTSignatures {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xfeatures2d::PCTSignatures for PtrOfPCTSignatures {
        #[inline(always)] fn as_raw_PCTSignatures(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::PCTSignatures>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfPCTSignaturesSQFD {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfPCTSignaturesSQFD {
        #[inline(always)] pub fn as_raw_PtrOfPCTSignaturesSQFD(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfPCTSignaturesSQFD {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfPCTSignaturesSQFD {}
    
    impl core::AlgorithmTrait for PtrOfPCTSignaturesSQFD {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xfeatures2d::PCTSignaturesSQFD for PtrOfPCTSignaturesSQFD {
        #[inline(always)] fn as_raw_PCTSignaturesSQFD(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSIFT {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSIFT {
        #[inline(always)] pub fn as_raw_PtrOfSIFT(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSIFT {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::SIFT>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSIFT {}
    
    impl PtrOfSIFT {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::SIFT>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> SIFTRef {
            let inner = crate::xfeatures2d::SIFT { ptr: self.get_inner() };
            SIFTRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> SIFTRefMut {
            let inner = crate::xfeatures2d::SIFT { ptr: self.get_inner() };
            SIFTRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct SIFTRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::SIFT>,
        owner: std::marker::PhantomData<&'o types::PtrOfSIFT>,
    }
    
    impl std::ops::Deref for SIFTRef<'_> {
        type Target = crate::xfeatures2d::SIFT;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct SIFTRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::SIFT>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfSIFT>,
    }
    
    impl std::ops::Deref for SIFTRefMut<'_> {
        type Target = crate::xfeatures2d::SIFT;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for SIFTRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfSURF {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSURF {
        #[inline(always)] pub fn as_raw_PtrOfSURF(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSURF {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::SURF>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSURF {}
    
    impl core::AlgorithmTrait for PtrOfSURF {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfSURF {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xfeatures2d::SURF for PtrOfSURF {
        #[inline(always)] fn as_raw_SURF(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::SURF>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfStarDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfStarDetector {
        #[inline(always)] pub fn as_raw_PtrOfStarDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfStarDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::StarDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfStarDetector {}
    
    impl PtrOfStarDetector {
        #[inline(always)] fn get_inner(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::StarDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    
        pub fn get(&self) -> StarDetectorRef {
            let inner = crate::xfeatures2d::StarDetector { ptr: self.get_inner() };
            StarDetectorRef {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    
        pub fn get_mut(&mut self) -> StarDetectorRefMut {
            let inner = crate::xfeatures2d::StarDetector { ptr: self.get_inner() };
            StarDetectorRefMut {
                inner: std::mem::ManuallyDrop::new(inner),
                owner: std::marker::PhantomData,
            }
        }
    }
    
    pub struct StarDetectorRef<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::StarDetector>,
        owner: std::marker::PhantomData<&'o types::PtrOfStarDetector>,
    }
    
    impl std::ops::Deref for StarDetectorRef<'_> {
        type Target = crate::xfeatures2d::StarDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    pub struct StarDetectorRefMut<'o> {
        inner: std::mem::ManuallyDrop<crate::xfeatures2d::StarDetector>,
        owner: std::marker::PhantomData<&'o mut types::PtrOfStarDetector>,
    }
    
    impl std::ops::Deref for StarDetectorRefMut<'_> {
        type Target = crate::xfeatures2d::StarDetector;
    
        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }
    
    impl std::ops::DerefMut for StarDetectorRefMut<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut *self.inner
        }
    }
    pub struct PtrOfVGG {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfVGG {
        #[inline(always)] pub fn as_raw_PtrOfVGG(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfVGG {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xfeatures2d::VGG>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfVGG {}
    
    impl core::AlgorithmTrait for PtrOfVGG {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::features2d::Feature2DTrait for PtrOfVGG {
        #[inline(always)] fn as_raw_Feature2D(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Feature2D>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xfeatures2d::VGG for PtrOfVGG {
        #[inline(always)] fn as_raw_VGG(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xfeatures2d::VGG>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use xfeatures2d_types::*;

#[cfg(feature = "contrib")]
mod xobjdetect_types {
    use super::*;

    pub struct PtrOfWBDetector {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfWBDetector {
        #[inline(always)] pub fn as_raw_PtrOfWBDetector(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfWBDetector {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xobjdetect::WBDetector>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfWBDetector {}
    
    impl crate::xobjdetect::WBDetector for PtrOfWBDetector {
        #[inline(always)] fn as_raw_WBDetector(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xobjdetect::WBDetector>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use xobjdetect_types::*;

#[cfg(feature = "contrib")]
mod xphoto_types {
    use super::*;

    pub struct PtrOfGrayworldWB {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfGrayworldWB {
        #[inline(always)] pub fn as_raw_PtrOfGrayworldWB(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfGrayworldWB {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xphoto::GrayworldWB>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfGrayworldWB {}
    
    impl core::AlgorithmTrait for PtrOfGrayworldWB {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xphoto::GrayworldWB for PtrOfGrayworldWB {
        #[inline(always)] fn as_raw_GrayworldWB(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xphoto::GrayworldWB>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xphoto::WhiteBalancer for PtrOfGrayworldWB {
        #[inline(always)] fn as_raw_WhiteBalancer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xphoto::WhiteBalancer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfLearningBasedWB {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfLearningBasedWB {
        #[inline(always)] pub fn as_raw_PtrOfLearningBasedWB(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfLearningBasedWB {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xphoto::LearningBasedWB>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfLearningBasedWB {}
    
    impl core::AlgorithmTrait for PtrOfLearningBasedWB {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xphoto::LearningBasedWB for PtrOfLearningBasedWB {
        #[inline(always)] fn as_raw_LearningBasedWB(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xphoto::LearningBasedWB>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xphoto::WhiteBalancer for PtrOfLearningBasedWB {
        #[inline(always)] fn as_raw_WhiteBalancer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xphoto::WhiteBalancer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    pub struct PtrOfSimpleWB {
        pub(crate) ptr: *mut c_void
    }
    
    impl PtrOfSimpleWB {
        #[inline(always)] pub fn as_raw_PtrOfSimpleWB(&self) -> *mut c_void { self.ptr }
    
        pub unsafe fn from_raw_ptr(ptr: *mut c_void) -> Self {
            Self { ptr }
        }
    }
    
    impl Drop for PtrOfSimpleWB {
        fn drop(&mut self) {
            let me = self.ptr;
            cpp!(unsafe [me as "Ptr<cv::xphoto::SimpleWB>*"] {
                delete me;
            })
        }
    }
    
    unsafe impl Send for PtrOfSimpleWB {}
    
    impl core::AlgorithmTrait for PtrOfSimpleWB {
        #[inline(always)] fn as_raw_Algorithm(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::Algorithm>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xphoto::SimpleWB for PtrOfSimpleWB {
        #[inline(always)] fn as_raw_SimpleWB(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xphoto::SimpleWB>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
    impl crate::xphoto::WhiteBalancer for PtrOfSimpleWB {
        #[inline(always)] fn as_raw_WhiteBalancer(&self) -> *mut c_void {
            let me = self.ptr;
            cpp!(unsafe [me as "cv::Ptr<cv::xphoto::WhiteBalancer>*"] -> *mut c_void as "void*" {
                return me->get();
            })
        }
    }
    
}
#[cfg(feature = "contrib")]
pub use xphoto_types::*;

pub use crate::manual::types::*;
