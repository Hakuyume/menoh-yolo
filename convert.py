import argparse
import json
import numpy as np
from unittest import mock

import chainer
import chainer.functions as F
import chainer.links as L
import chainercv
import onnx_chainer


# YOLOv2 with some hacks
# FIXME: reorg layer cannot be implemented by Menoh.
# Temporarily, we replace it with a dummy convolution.
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


# YOLOv2Tiny with some hacks
# FIXME: maxpool layer with stride=1 cannot be implemented by Menoh.
# Temporarily, we skip it.
class YOLOv2Tiny(chainercv.links.YOLOv2Tiny):

    def __call__(self, x):
        def _maxpool(x, ksize, stride=None):
            if stride is None or ksize == stride:
                return F.max_pooling_2d(x, ksize, stride)
            else:
                return x

        with mock.patch(
                'chainercv.links.model.yolo.yolo_v2._maxpool', _maxpool):
            return self.subnet(self.extractor(x))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--model', choices=('yolo_v2', 'yolo_v2_tiny'), default='yolo_v2')
    parser.add_argument('--model-out', default='model.onnx')
    parser.add_argument('--config-out', default='config.json')
    args = parser.parse_args()

    if args.model == 'yolo_v2':
        model = YOLOv2(pretrained_model='voc0712')
    else:
        model = YOLOv2Tiny(pretrained_model='voc0712')

    x = np.empty((1, 3, model.insize, model.insize), dtype=np.float32)
    with chainer.using_config('train', False):
        onnx_model = onnx_chainer.export(
            model, x, filename=args.model_out, opset_version=7)

    config = {
        'input': onnx_model.graph.node[0].input[0],
        'output': onnx_model.graph.node[-1].output[0],
        'insize': model.insize,
        'anchors': model._anchors,
        'label_names': chainercv.datasets.voc_bbox_label_names,
    }
    with open(args.config_out, mode='w') as f:
        json.dump(config, f)


if __name__ == '__main__':
    main()
