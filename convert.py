import argparse
import numpy as np
from unittest import mock

import chainer
import chainer.links as L
import chainercv
import onnx_chainer


# YOLOv2 with some hacks
class YOLOv2(chainercv.links.YOLOv2):

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        # FIXME
        with self.init_scope():
            self.reorg = L.Convolution2D(64, 256, 2, stride=2, nobias=True)
        self.reorg.W.array[:] = 0

    def __call__(self, x):
        x.node._onnx_name = 'input'
        with mock.patch(
                'chainercv.links.model.yolo.yolo_v2._reorg', self.reorg):
            y = self.subnet(self.extractor(x))
        y.node._onnx_name = 'output'
        return y


class IDGenerator(object):

    def __init__(self):
        # keep original
        self._id = id

    def __call__(self, obj):
        return getattr(obj, '_onnx_name', self._id(obj))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', default='YOLOv2.onnx')
    args = parser.parse_args()

    model = YOLOv2(pretrained_model='voc0712')
    x = np.empty((1, 3, model.insize, model.insize), dtype=np.float32)
    with chainer.using_config('train', False), \
            mock.patch('builtins.id', IDGenerator()):
        onnx_chainer.export(model, x, filename=args.out)


if __name__ == '__main__':
    main()
