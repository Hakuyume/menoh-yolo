# menoh-yolo

[YOLOv2](https://pjreddie.com/darknet/yolov2/) object detector on [Menoh](https://github.com/pfnet-research/menoh)

## Requirements

- Rust 1.27
- Cargo
- [Menoh](https://github.com/pfnet-research/menoh) 1.0+
- [menoh-rs](https://github.com/Hakuyume/menoh-rs)
    - pkg-config (for [pkg-config](https://crates.io/crates/pkg-config))
    - libclang (for [bindgen](https://crates.io/crates/bindgen))
- [ChainerCV](https://github.com/chainer/chainercv)
- [onnx-chainer](https://github.com/chainer/onnx-chainer) 1.1.1a2

## Demo

```
$ git clone https://github.com/Hakuyume/menoh-yolo.git
$ cd menoh-yolo

$ python3 convert.py

$ curl -LO https://github.com/pjreddie/darknet/raw/master/data/dog.jpg
$ cargo run -- dog.jpg result.jpg
bicycle: 0.6089509
car: 0.73168564
dog: 0.7923419
```
![result.jpg](https://user-images.githubusercontent.com/3014172/42409967-3ef0faec-821d-11e8-8dc3-88cd8b52df26.jpg)
