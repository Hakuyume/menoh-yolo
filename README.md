# menoh-yolo

[YOLOv2](https://pjreddie.com/darknet/yolov2/) object detector on [Menoh](https://github.com/pfnet-research/menoh)

## Requirements

- Rust 1.27+
- [Menoh](https://github.com/pfnet-research/menoh) 1.1.1+
- OpenCV (optional)

## Demo

### build manually

```
$ git clone https://github.com/Hakuyume/menoh-yolo.git --recursive
$ cd menoh-yolo

$ curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_voc0712.onnx
$ curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_voc0712.json

$ curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
$ cargo run --release -- dog.jpg result.jpg
dog, (215.18188, 130.01926, 520.168, 321.96957) 0.794875
car, (66.51419, 472.41486, 180.37459, 687.9617) 0.732398
bicycle, (79.09003, 96.10492, 419.95575, 578.96405) 0.6060025
```

### use Docker

```
$ docker run -it hakuyume/menoh-yolo menoh-yolo dog.jpg result.jpg
dog, (215.18188, 130.01926, 520.168, 321.96957) 0.794875
car, (66.51419, 472.41486, 180.37459, 687.9617) 0.732398
bicycle, (79.09003, 96.10492, 419.95575, 578.96405) 0.6060025
```

## Demo using Camera

This demo requires OpenCV.

```
$ git clone https://github.com/Hakuyume/menoh-yolo.git --recursive
$ cd menoh-yolo

$ curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_voc0712.onnx
$ curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_voc0712.json

$ cargo run --features=opencv --release
(press 'q' to quit)
```

If the FPS is too low, please try YOLOv2 tiny.

```
$ curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_tiny_voc0712.onnx
$ curl -LO https://github.com/Hakuyume/menoh-yolo/releases/download/assets/yolo_v2_tiny_voc0712.json

$ cargo run --features=opencv --release -- --model yolo_v2_tiny_voc0712.onnx --config yolo_v2_tiny_voc0712.json
(press 'q' to quit)
```

![result.jpg](https://user-images.githubusercontent.com/3014172/42957529-efc58ec4-8bbd-11e8-9b00-440924369e2b.jpg)
