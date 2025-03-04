//! Types and helpers for handling ORT errors.

use std::{convert::Infallible, io, path::PathBuf, string};

use thiserror::Error;

use super::{char_p_to_string, ortsys, tensor::TensorElementDataType};

/// Type alias for the Result type returned by ORT functions.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An enum of all errors returned by ORT functions.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
	/// Workaround to get [`crate::inputs`] to accept `Value`s, since it will attempt to `Value::try_from` on provided
	/// values, and the implementation `TryFrom<T> for T` uses `Infallible` as the error type.
	#[error("unreachable")]
	#[doc(hidden)]
	Infallible,
	/// An error occurred when converting an FFI C string to a Rust `String`.
	#[error("Failed to construct Rust String")]
	FfiStringConversion(ErrorInternal),
	/// An error occurred while creating an ONNX environment.
	#[error("Failed to create ONNX Runtime environment: {0}")]
	CreateEnvironment(ErrorInternal),
	/// Error occurred when creating ONNX session options.
	#[error("Failed to create ONNX Runtime session options: {0}")]
	CreateSessionOptions(ErrorInternal),
	/// Error occurred when creating an ONNX session.
	#[error("Failed to create ONNX Runtime session: {0}")]
	CreateSession(ErrorInternal),
	/// Error occurred when creating an IO binding.
	#[error("Failed to create IO binding: {0}")]
	CreateIoBinding(ErrorInternal),
	/// Error occurred when counting ONNX session input/output count.
	#[error("Failed to get input or output count: {0}")]
	GetInOutCount(ErrorInternal),
	/// Error occurred when getting ONNX input name.
	#[error("Failed to get input name: {0}")]
	GetInputName(ErrorInternal),
	/// Error occurred when getting ONNX type information
	#[error("Failed to get type info: {0}")]
	GetTypeInfo(ErrorInternal),
	/// Error occurred when getting ONNX type information
	#[error("Failed to get onnx type from type info: {0}")]
	GetOnnxTypeFromTypeInfo(ErrorInternal),
	/// Error occurred when casting ONNX type information to tensor information
	#[error("Failed to cast type info to tensor info: {0}")]
	CastTypeInfoToTensorInfo(ErrorInternal),
	/// Error occurred when casting ONNX type information to sequence type info
	#[error("Failed to cast type info to sequence type info: {0}")]
	CastTypeInfoToSequenceTypeInfo(ErrorInternal),
	/// Error occurred when casting ONNX type information to map type info
	#[error("Failed to cast type info to map typ info: {0}")]
	CastTypeInfoToMapTypeInfo(ErrorInternal),
	/// Error occurred when getting map key type
	#[error("Failed to get map key type: {0}")]
	GetMapKeyType(ErrorInternal),
	/// Error occurred when getting map value type
	#[error("Failed to get map value type: {0}")]
	GetMapValueType(ErrorInternal),
	/// Error occurred when getting sequence element type
	#[error("Failed to get sequence element type: {0}")]
	GetSequenceElementType(ErrorInternal),
	/// Error occurred when getting tensor elements type
	#[error("Failed to get tensor element type: {0}")]
	GetTensorElementType(ErrorInternal),
	/// Error occurred when getting ONNX dimensions count
	#[error("Failed to get dimensions count: {0}")]
	GetDimensionsCount(ErrorInternal),
	/// Error occurred when getting ONNX dimensions
	#[error("Failed to get dimensions: {0}")]
	GetDimensions(ErrorInternal),
	/// Error occurred when getting string length
	#[error("Failed to get string tensor length: {0}")]
	GetStringTensorDataLength(ErrorInternal),
	/// Error occurred when getting tensor element count
	#[error("Failed to get tensor element count: {0}")]
	GetTensorShapeElementCount(ErrorInternal),
	/// Error occurred when creating ONNX tensor
	#[error("Failed to create tensor: {0}")]
	CreateTensor(ErrorInternal),
	/// Error occurred when creating ONNX tensor with specific data
	#[error("Failed to create tensor with data: {0}")]
	CreateTensorWithData(ErrorInternal),
	/// Error occurred when filling a tensor with string data
	#[error("Failed to fill string tensor: {0}")]
	FillStringTensor(ErrorInternal),
	/// Error occurred when checking if ONNX tensor was properly initialized
	#[error("Failed to check if tensor is a tensor or was properly initialized: {0}")]
	FailedTensorCheck(ErrorInternal),
	/// Error occurred when getting tensor type and shape
	#[error("Failed to get tensor type and shape: {0}")]
	GetTensorTypeAndShape(ErrorInternal),
	/// Error occurred when ONNX inference operation was called
	#[error("Failed to run inference on model: {0}")]
	SessionRun(ErrorInternal),
	/// Error occurred when ONNX inference operation was called using `IoBinding`.
	#[error("Failed to run inference on model with IoBinding: {0}")]
	SessionRunWithIoBinding(ErrorInternal),
	/// Error occurred when extracting data from an ONNX tensor into an C array to be used as an `ndarray::ArrayView`.
	#[error("Failed to get tensor data: {0}")]
	GetTensorMutableData(ErrorInternal),
	/// Error occurred when extracting string data from an ONNX tensor
	#[error("Failed to get tensor string data: {0}")]
	GetStringTensorContent(ErrorInternal),
	/// Error occurred when converting data to a String
	#[error("Data was not UTF-8: {0}")]
	StringFromUtf8Error(#[from] string::FromUtf8Error),
	/// Error occurred when downloading a pre-trained ONNX model from the [ONNX Model Zoo](https://github.com/onnx/models).
	#[error("Failed to download ONNX model: {0}")]
	DownloadError(#[from] FetchModelError),
	/// Type of input data and the ONNX model do not match.
	#[error("Data types do not match: expected {model:?}, got {input:?}")]
	NonMatchingDataTypes { input: TensorElementDataType, model: TensorElementDataType },
	/// Dimensions of input data and the ONNX model do not match.
	#[error("Dimensions do not match: {0:?}")]
	NonMatchingDimensions(NonMatchingDimensionsError),
	/// File does not exist
	#[error("File `{filename:?}` does not exist")]
	FileDoesNotExist {
		/// Path which does not exists
		filename: PathBuf
	},
	/// Path is invalid UTF-8
	#[error("Path `{path:?}` cannot be converted to UTF-8")]
	NonUtf8Path {
		/// Path with invalid UTF-8
		path: PathBuf
	},
	/// Attempt to build a Rust `CString` when the original string contains a null character.
	#[error("Failed to build CString when original contains null: {0}")]
	FfiStringNull(#[from] std::ffi::NulError),
	/// Attempt to build a `WideCString` when the original string contains a null character.
	#[cfg(all(windows, feature = "profiling"))]
	#[error("Failed to build CString when original contains null: {0}")]
	WideFfiStringNull(#[from] widestring::error::ContainsNul<u16>),
	#[error("`{0}` should be a null pointer")]
	/// ORT pointer should have been null
	PointerShouldBeNull(String),
	/// ORT pointer should not have been null
	#[error("`{0}` should not be a null pointer")]
	PointerShouldNotBeNull(String),
	/// The runtime type was undefined.
	#[error("Undefined tensor element type")]
	UndefinedTensorElementType,
	/// Could not retrieve model metadata.
	#[error("Failed to retrieve model metadata: {0}")]
	GetModelMetadata(ErrorInternal),
	/// The user tried to extract the wrong type of tensor from the underlying data
	#[error("Data type mismatch: was {actual:?}, tried to convert to {requested:?}")]
	DataTypeMismatch {
		/// The actual type of the ort output
		actual: TensorElementDataType,
		/// The type corresponding to the attempted conversion into a Rust type, not equal to `actual`
		requested: TensorElementDataType
	},
	#[error("Error trying to load symbol `{symbol}` from dynamic library: {error}")]
	DlLoad { symbol: &'static str, error: String },
	#[error("{0}")]
	ExecutionProvider(ErrorInternal),
	#[error("Execution provider `{0}` was not registered because its corresponding Cargo feature is disabled.")]
	ExecutionProviderNotRegistered(&'static str),
	#[error("Expected tensor to be on CPU in order to get data, but had allocation device `{0}`.")]
	TensorNotOnCpu(&'static str),
	#[error("String tensors require the session's allocator to be provided through `Value::from_array`.")]
	StringTensorRequiresAllocator,
	#[error("Failed to create memory info: {0}")]
	CreateMemoryInfo(ErrorInternal),
	#[error("Could not get allocation device from `MemoryInfo`: {0}")]
	GetAllocationDevice(ErrorInternal),
	#[error("Failed to get available execution providers: {0}")]
	GetAvailableProviders(ErrorInternal),
	#[error("Unknown allocation device `{0}`")]
	UnknownAllocationDevice(String),
	#[error("Error when binding input: {0}")]
	BindInput(ErrorInternal),
	#[error("Error when binding output: {0}")]
	BindOutput(ErrorInternal),
	#[error("Failed to clear IO binding: {0}")]
	ClearBinding(ErrorInternal),
	#[error("Error when retrieving session outputs from `IoBinding`: {0}")]
	GetBoundOutputs(ErrorInternal)
}

impl From<Infallible> for Error {
	fn from(_: Infallible) -> Self {
		Error::Infallible
	}
}

/// Error used when the input dimensions defined in the model and passed from an inference call do not match.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum NonMatchingDimensionsError {
	/// Number of inputs from model does not match the number of inputs from inference call.
	#[error(
		"Non-matching number of inputs: {inference_input_count:?} provided vs {model_input_count:?} for model (inputs: {inference_input:?}, model: {model_input:?})"
	)]
	InputsCount {
		/// Number of input dimensions used by inference call
		inference_input_count: usize,
		/// Number of input dimensions defined in model
		model_input_count: usize,
		/// Input dimensions used by inference call
		inference_input: Vec<Vec<usize>>,
		/// Input dimensions defined in model
		model_input: Vec<Vec<Option<u32>>>
	},
	/// Inputs length from model does not match the expected input from inference call
	#[error("Different input lengths; expected input: {model_input:?}, received input: {inference_input:?}")]
	InputsLength {
		/// Input dimensions used by inference call
		inference_input: Vec<Vec<usize>>,
		/// Input dimensions defined in model
		model_input: Vec<Vec<Option<u32>>>
	}
}

/// Error details when ONNX C API returns an error.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ErrorInternal {
	/// Details about the error.
	#[error("{0}")]
	Msg(String),
	/// Converting the ONNX error message to UTF-8 failed.
	#[error("an error occurred, but ort failed to convert the error message to UTF-8")]
	IntoStringError(std::ffi::IntoStringError)
}

/// Error from downloading pre-trained model from the [ONNX Model Zoo](https://github.com/onnx/models).
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum FetchModelError {
	/// Generic input/output error
	#[error("Error reading file: {0}")]
	IoError(#[from] io::Error),
	/// Download error by ureq
	#[cfg(feature = "fetch-models")]
	#[cfg_attr(docsrs, doc(cfg(feature = "fetch-models")))]
	#[error("Error downloading to file: {0}")]
	FetchError(#[from] Box<ureq::Error>),
	/// Error getting Content-Length from HTTP GET request.
	#[error("Error getting Content-Length from HTTP GET")]
	ContentLengthError,
	/// Mismatch between amount of downloaded and expected bytes.
	#[error("Error copying data to file: expected {expected} length, but got {io}")]
	CopyError {
		/// Expected amount of bytes to download
		expected: u64,
		/// Number of bytes read from network and written to file
		io: u64
	}
}

/// Wrapper type around ONNX's `OrtStatus` pointer.
///
/// This wrapper exists to facilitate conversion from C raw pointers to Rust error types.
pub struct OrtStatusWrapper(*mut ort_sys::OrtStatus);

impl From<*mut ort_sys::OrtStatus> for OrtStatusWrapper {
	fn from(status: *mut ort_sys::OrtStatus) -> Self {
		OrtStatusWrapper(status)
	}
}

pub(crate) fn assert_null_pointer<T>(ptr: *const T, name: &str) -> Result<()> {
	ptr.is_null().then_some(()).ok_or_else(|| Error::PointerShouldBeNull(name.to_owned()))
}

pub(crate) fn assert_non_null_pointer<T>(ptr: *const T, name: &str) -> Result<()> {
	(!ptr.is_null())
		.then_some(())
		.ok_or_else(|| Error::PointerShouldNotBeNull(name.to_owned()))
}

impl From<OrtStatusWrapper> for Result<(), ErrorInternal> {
	fn from(status: OrtStatusWrapper) -> Self {
		if status.0.is_null() {
			Ok(())
		} else {
			let raw: *const std::os::raw::c_char = ortsys![unsafe GetErrorMessage(status.0)];
			match char_p_to_string(raw) {
				Ok(msg) => Err(ErrorInternal::Msg(msg)),
				Err(err) => match err {
					Error::FfiStringConversion(ErrorInternal::IntoStringError(e)) => Err(ErrorInternal::IntoStringError(e)),
					_ => unreachable!()
				}
			}
		}
	}
}

impl Drop for OrtStatusWrapper {
	fn drop(&mut self) {
		ortsys![unsafe ReleaseStatus(self.0)];
	}
}

pub(crate) fn status_to_result(status: *mut ort_sys::OrtStatus) -> Result<(), ErrorInternal> {
	let status_wrapper: OrtStatusWrapper = status.into();
	status_wrapper.into()
}
