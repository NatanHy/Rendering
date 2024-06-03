from PIL import Image
import numpy as np
import os
import fileinput
import shutil

class ImageMapper:
    def __init__(self, img : Image.Image, maps):
        self.img = img
        self.maps = maps

    def translate_tex_coord(self, coord, img_name):
        f = self.maps[img_name]
        return f(coord, self.img)

    def save(self, file_path):
        self.img.save(file_path)

def convert_tex_coord(coord, img : Image.Image, pos1, pos2):
    x, y = coord
    i0, j0 = pos1
    i1, j1 = pos2

    delta_i = i1 - i0
    delta_j = j1 - j0

    i_offset = y * delta_i
    j_offset = x * delta_j

    new_i = i0 + i_offset
    new_j = j0 + j_offset

    new_x = new_j / img.size[0]
    new_y = new_i / img.size[1]

    return (new_x, new_y)

def tex_coordinate_mapper(pos1, pos2):
    def f(coord, img):
        return convert_tex_coord(coord, img, pos1, pos2)
    return f

def merge_images(paths : list[str]) -> ImageMapper:
    img_arrs : list[np.ndarray] = []

    for filename in paths:
        with Image.open(filename) as img:
            if img.mode != "RGB":
                img = img.convert("RGB")
            img_arr = np.array(img)
            img_arrs.append(img_arr)

    img_bounds = [x.shape for x in img_arrs]
    num_rows = max(img_bounds, key=lambda x: x[0])[0]
    num_cols = sum([x[1] for x in img_bounds])
    merged_img_arr = np.zeros([num_rows, num_cols, 3], dtype=np.uint8)

    maps = {}

    j_ptr = 0

    for i, (img, bounds) in enumerate(zip(img_arrs, img_bounds)):
        
        if not paths[i] in maps:
            maps[paths[i]] = tex_coordinate_mapper((0, j_ptr), (bounds[0], j_ptr + bounds[1]))

        for i in range(bounds[0]):
            for j in range(bounds[1]):
                merged_img_arr[i][j + j_ptr] = img[i][j][0:3]

        j_ptr += j

    img = Image.fromarray(merged_img_arr)
    return ImageMapper(img, maps)

def convert_textures(textures_path) -> ImageMapper:
    im_mapper = merge_images([textures_path + "/" + str(p) for p in os.listdir(textures_path)])
    im_mapper.save("converted/textures/test.png")
    return im_mapper

# def convert_obj_file(obj_file_path, im_mapper):
#     tag = ""
#     tags = set()

#     filename = obj_file_path.split("/")[-1]
#     new_file_path = f"converted/objects/{filename}"
#     shutil.copyfile(obj_file_path, new_file_path)

#     for line in fileinput.input(new_file_path, inplace=True):
#         elms = line.split(" ")
#         if elms[0] != "vt":
#             print(line, end="")
#             if elms[0] == "o":
#                 tag = elms[1]
#                 tags.add(tag)
#         else:
#             tex_coord = [float(elms[1]), float(elms[2])]
#             tex_coord = im_mapper.translate_tex_coord(tex_coord, tag_to_img(tag))

#             print(f"vt {tex_coord[0]} {tex_coord[1]}")

#     print(tags)

im_mapper = convert_textures("textures/car_textures")

