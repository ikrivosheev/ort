#![allow(clippy::upper_case_acronyms)]

use crate::download::ModelUrl;

/// Object Detection & Image Segmentation
///
/// > Object detection models detect the presence of multiple objects in an image and segment out areas of the
/// > image where the objects are detected. Semantic segmentation models partition an input image by labeling each pixel
/// > into a set of pre-defined categories.
#[derive(Debug, Clone)]
pub enum ObjectDetectionImageSegmentation {
	/// A real-time CNN for object detection that detects 20 different classes. A smaller version of the
	/// more complex full YOLOv2 network.
	TinyYoloV2,
	/// Single Stage Detector: real-time CNN for object detection that detects 80 different classes.
	Ssd,
	/// A variant of MobileNet that uses the Single Shot Detector (SSD) model framework. The model detects 80
	/// different object classes and locates up to 10 objects in an image.
	SSDMobileNetV1,
	/// Increases efficiency from R-CNN by connecting a RPN with a CNN to create a single, unified network for
	/// object detection that detects 80 different classes.
	FasterRcnn,
	/// A real-time neural network for object instance segmentation that detects 80 different classes. Extends
	/// Faster R-CNN as each of the 300 elected ROIs go through 3 parallel branches of the network: label
	/// prediction, bounding box prediction and mask prediction.
	MaskRcnn,
	/// A real-time dense detector network for object detection that addresses class imbalance through Focal Loss.
	/// RetinaNet is able to match the speed of previous one-stage detectors and defines the state-of-the-art in
	/// two-stage detectors (surpassing R-CNN).
	RetinaNet,
	/// A CNN model for real-time object detection system that can detect over 9000 object categories. It uses a
	/// single network evaluation, enabling it to be more than 1000x faster than R-CNN and 100x faster than
	/// Faster R-CNN.
	YoloV2,
	/// A CNN model for real-time object detection system that can detect over 9000 object categories. It uses
	/// a single network evaluation, enabling it to be more than 1000x faster than R-CNN and 100x faster than
	/// Faster R-CNN. This model is trained with COCO dataset and contains 80 classes.
	YoloV2Coco,
	/// A deep CNN model for real-time object detection that detects 80 different classes. A little bigger than
	/// YOLOv2 but still very fast. As accurate as SSD but 3 times faster.
	YoloV3,
	/// A smaller version of YOLOv3 model.
	TinyYoloV3,
	/// Optimizes the speed and accuracy of object detection. Two times faster than EfficientDet. It improves
	/// YOLOv3's AP and FPS by 10% and 12%, respectively, with mAP50 of 52.32 on the COCO 2017 dataset and
	/// FPS of 41.7 on Tesla 100.
	YoloV4,
	/// Deep CNN based pixel-wise semantic segmentation model with >80% mIOU (mean Intersection Over Union).
	/// Trained on cityscapes dataset, which can be effectively implemented in self driving vehicle systems.
	Duc
}

impl ModelUrl for ObjectDetectionImageSegmentation {
	fn model_url(&self) -> &'static str {
		match self {
			ObjectDetectionImageSegmentation::TinyYoloV2 => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/tiny-yolov2/model/tinyyolov2-8.onnx"
			}
			ObjectDetectionImageSegmentation::Ssd => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/ssd/model/ssd-10.onnx"
			}
			ObjectDetectionImageSegmentation::SSDMobileNetV1 => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/ssd-mobilenetv1/model/ssd_mobilenet_v1_10.onnx"
			}
			ObjectDetectionImageSegmentation::FasterRcnn => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/faster-rcnn/model/FasterRCNN-10.onnx"
			}
			ObjectDetectionImageSegmentation::MaskRcnn => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/mask-rcnn/model/MaskRCNN-10.onnx"
			}
			ObjectDetectionImageSegmentation::RetinaNet => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/retinanet/model/retinanet-9.onnx"
			}
			ObjectDetectionImageSegmentation::YoloV2 => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/yolov2/model/yolov2-voc-8.onnx"
			}
			ObjectDetectionImageSegmentation::YoloV2Coco => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/yolov2-coco/model/yolov2-coco-9.onnx"
			}
			ObjectDetectionImageSegmentation::YoloV3 => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/yolov3/model/yolov3-10.onnx"
			}
			ObjectDetectionImageSegmentation::TinyYoloV3 => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/tiny-yolov3/model/tiny-yolov3-11.onnx"
			}
			ObjectDetectionImageSegmentation::YoloV4 => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/yolov4/model/yolov4.onnx"
			}
			ObjectDetectionImageSegmentation::Duc => {
				"https://github.com/onnx/models/raw/5faef4c33eba0395177850e1e31c4a6a9e634c82/vision/object_detection_segmentation/duc/model/ResNet101-DUC-7.onnx"
			}
		}
	}
}
