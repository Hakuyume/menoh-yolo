import argparse
import numpy as np

import chainer
import chainer.functions as F

import chainercv
import onnx_chainer


# FIXME
def reorg(x):
    n, c, h, w = x.shape
    return F.reshape(x, (n, c * 4, h // 2, w // 2))


chainercv.links.model.yolo.yolo_v2._reorg = reorg


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out', default='YOLOv2.onnx')
    args = parser.parse_args()

    model = chainercv.links.YOLOv2(pretrained_model='voc0712')

    x = np.empty((1, 3, model.insize, model.insize), dtype=np.float32)
    with chainer.using_config('train', False):
        onnx_chainer.export(
            chainer.Sequential(model.extractor, model.subnet),
            x, filename=args.out)


if __name__ == '__main__':
    main()
