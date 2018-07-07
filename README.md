# menoh-yolo

[YOLOv2](https://pjreddie.com/darknet/yolov2/) object detector on [Menoh](https://github.com/pfnet-research/menoh)

## Requirements

- Rust 1.27
- Cargo
- pkg-config (for [pkg-config](https://crates.io/crates/pkg-config))
- libclang (for [bindgen](https://crates.io/crates/bindgen))
- [Menoh](https://github.com/pfnet-research/menoh) 1.0+
- [ChainerCV](https://github.com/chainer/chainercv)
- [onnx-chainer](https://github.com/chainer/onnx-chainer)

## Demo

```
$ git clone https://github.com/Hakuyume/menoh-yolo.git
$ cd menoh-yolo

$ python3 convert.py

$ curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
$ cargo run -- dog.jpg result.jpg
```
