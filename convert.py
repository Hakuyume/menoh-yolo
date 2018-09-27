import argparse
import numpy as np
from unittest import mock

import chainer
import chainer.links as L
import chainercv
import onnx_chainer


# YOLOv2 with some hacks
# FIXME: Reorg layer cannot be implemented by Menoh. Temporarily, we replace it with a dummy convolution.
class YOLOv2(chainercv.links.YOLOv2):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        with self.init_scope():
            self.reorg = L.Convolution2D(64, 256, 2, stride=2, nobias=True)
        self.reorg.W.array[:] = 0

    def __call__(self, x):
        with mock.patch(
                'chainercv.links.model.yolo.yolo_v2._reorg', self.reorg):
            return self.subnet(self.extractor(x))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', default='YOLOv2.onnx')
    args = parser.parse_args()

    model = YOLOv2(pretrained_model='voc0712')
    x = np.empty((1, 3, model.insize, model.insize), dtype=np.float32)
    with chainer.using_config('train', False):
        onnx_chainer.export(model, x, filename=args.out, opset_version=7)


if __name__ == '__main__':
    main()
