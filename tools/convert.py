from argparse import ArgumentParser
from pathlib import Path
from PIL import Image

import numpy as np


def convert(input: Path, width: int, height: int) -> Image.Image:
    words = np.fromfile(input, dtype="<u4")
    shifts = np.array([0, 10, 20], dtype=np.uint32)
    samples = ((words[:, None] >> shifts) & 0x3FF).astype(np.uint16)
    samples = samples.ravel()

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
