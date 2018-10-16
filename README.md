# menoh-yolo

[YOLOv2](https://pjreddie.com/darknet/yolov2/) object detector on [Menoh](https://github.com/pfnet-research/menoh)

## Requirements

- Rust 1.27+
- [Menoh](https://github.com/pfnet-research/menoh) 1.1+
- [ChainerCV](https://github.com/chainer/chainercv)
- [onnx-chainer](https://github.com/chainer/onnx-chainer) 1.2.2a3
- OpenCV (optional)

## Demo

### build manually

```
$ git clone https://github.com/Hakuyume/menoh-yolo.git --recursive
$ cd menoh-yolo

$ python3 convert.py

$ curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
$ cargo run --release -- dog.jpg result.jpg
bicycle: (78.776596, 85.96881, 420.28717, 569.1071) 0.6089509
car: (66.229126, 443.1903, 180.43053, 658.6296) 0.73168564
dog: (215.31303, 114.400635, 519.98016, 306.53442) 0.7923419
```

### use Docker

```
$ docker run -it hakuyume/menoh-yolo menoh-yolo dog.jpg result.jpg
bicycle: (79.09003, 86.08069, 419.95575, 568.9399) 0.6060025
car: (66.51419, 443.09775, 180.37459, 658.64453) 0.732398
dog: (215.18188, 114.50763, 520.168, 306.45792) 0.79487497
```

## Demo using Camera

This demo requires OpenCV.

```
$ git clone https://github.com/Hakuyume/menoh-yolo.git --recursive
$ cd menoh-yolo

$ python3 convert.py

$ cargo run --features=opencv --release
(press 'q' to quit)
```

![result.jpg](https://user-images.githubusercontent.com/3014172/42957529-efc58ec4-8bbd-11e8-9b00-440924369e2b.jpg)
