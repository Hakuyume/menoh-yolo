import argparse
import numpy as np
from unittest import mock

import chainer
import chainer.functions as F
import chainercv
import onnx_chainer


# FIXME
def reorg(x):
    n, c, h, w = x.shape
    return F.reshape(x, (n, c * 4, h // 2, w // 2))


chainercv.links.model.yolo.yolo_v2._reorg = reorg


class IDGenerator(object):

    def __init__(self):
        # keep original function
        self._id = id
        self._count = 0
        self._map = {}

    def __call__(self, obj):
        id_ = self._id(obj)
        if id_ not in self._map:
            self._map[id_] = 'v{:04d}'.format(self._count)
            self._count += 1
        return self._map[id_]


def main():
    chainer.config.train = False

    parser = argparse.ArgumentParser()
    parser.add_argument('--out', default='YOLOv2.onnx')
    args = parser.parse_args()

    model = chainercv.links.YOLOv2(pretrained_model='voc0712')

    x = np.empty((1, 3, model.insize, model.insize), dtype=np.float32)
    with mock.patch('builtins.id', IDGenerator()):
        model = onnx_chainer.export(
            chainer.Sequential(model.extractor, model.subnet),
            x, filename=args.out)


if __name__ == '__main__':
    main()
