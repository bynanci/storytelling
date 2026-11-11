# SPDX-License-Identifier: MIT
"""Fixture-specific PNG readback, not general object recognition."""

from PIL import Image, __version__


def readback(path):
    with Image.open(path) as image:
        image.verify()
    with Image.open(path) as image:
        image.load()  # Fresh, complete decode rather than just a PNG header read.
        result = {"decoder": f"Pillow {__version__}", "format": image.format,
                  "size": list(image.size), "mode": image.mode, "fills": {}}
        rgb = image.convert("RGB")
        for name, color, side in (
            ("blue_canopy", (67, 116, 131), "left"),
            ("ochre_thread", (206, 147, 62), "right"),
        ):
            points = [(x, y) for y in range(rgb.height) for x in range(rgb.width)
                      if rgb.getpixel((x, y)) == color]
            if not points:
                raise ValueError(f"missing source fill: {name}")
            xs, ys = zip(*points)
            centroid = [sum(xs) / len(xs), sum(ys) / len(ys)]
            expected_side = centroid[0] < 320 if side == "left" else centroid[0] > 320
            result["fills"][name] = {"rgb": list(color), "pixel_count": len(points),
                                     "bbox_inclusive": [min(xs), min(ys), max(xs), max(ys)],
                                     "centroid": centroid, "expected_side": side,
                                     "side_check_passed": expected_side}
        result["passed"] = (result["format"] == "PNG" and result["size"] == [640, 360]
                            and all(fill["side_check_passed"]
                                    for fill in result["fills"].values()))
        if not result["passed"]:
            raise ValueError("PNG format, size, or fixture position check failed")
        return result
