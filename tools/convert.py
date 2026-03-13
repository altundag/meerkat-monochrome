from argparse import ArgumentParser
from pathlib import Path
from PIL import Image

import numpy as np


def convert(input: Path, width: int, height: int) -> Image.Image:
    data = np.fromfile(input, dtype=np.uint8)

    # 5 bytes (4 pixels)
    blocks = data.reshape(-1, 5).astype(np.uint16)

    out = np.empty((blocks.shape[0], 4), dtype=np.uint16)

    out[:, 0] = blocks[:, 0] | ((blocks[:, 1] & 0x03) << 8)
    out[:, 1] = (blocks[:, 1] >> 2) | ((blocks[:, 2] & 0x0F) << 6)
    out[:, 2] = (blocks[:, 2] >> 4) | ((blocks[:, 3] & 0x3F) << 4)
    out[:, 3] = (blocks[:, 3] >> 6) | (blocks[:, 4] << 2)

    samples = out.reshape(-1)

    samples = np.flip(
        np.packbits(np.flip(np.unpackbits(samples.view(np.uint8)))).view(np.uint16)
    )

    image = samples.reshape((height, width))

    return Image.fromarray(image, "I;16")


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("input", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--height", type=int, default=1048)
    parser.add_argument("--width", type=int, default=1312 + 2)

    args = parser.parse_args()

    if args.input.is_dir():
        for raw in args.input.glob("*.RAW"):
            print(raw)
            im = convert(raw, args.width, args.height)
            im.save(raw.with_suffix(".png").name)
    else:
        im = convert(args.input, args.width, args.height)
        im.save(
            args.input.with_suffix(".png").name if args.output is None else args.output
        )
