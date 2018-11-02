import argparse
import json
import numpy as np
from unittest import mock

import chainer
import chainer.functions as F
import chainer.links as L
import chainercv
import onnx
import onnx_chainer


# YOLOv2 with some hacks
# FIXME: Menoh cannot caluculate reorg layer.
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
# FIXME: onnx-chainer cannot treat maxpool layer (darknet version).
# Temporarily, we modify onnx model directly.
class YOLOv2Tiny(chainercv.experimental.links.YOLOv2Tiny):

    def __call__(self, x):
        with mock.patch(
                'chainercv.experimental.links.model.yolo.yolo_v2_tiny._maxpool',
                F.max_pooling_2d):
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
    elif args.model == 'yolo_v2_tiny':
        model = YOLOv2Tiny(pretrained_model='voc0712')

    x = np.empty((1, 3, model.insize, model.insize), dtype=np.float32)
    with chainer.using_config('train', False):
        onnx_model = onnx_chainer.export(model, x, opset_version=7)

    # Fix MaxPool
    for op in onnx_model.graph.node:
        if op.op_type == 'MaxPool':
            attrs = {attr.name: attr for attr in op.attribute}
            for i in {0, 1}:
                attrs['pads'].ints[2 + i] += \
                    attrs['kernel_shape'].ints[i] - attrs['strides'].ints[i]

    onnx.save(onnx_model, args.model_out)

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
