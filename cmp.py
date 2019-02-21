# requires Pillow
# diff from: https://stackoverflow.com/questions/1927660/compare-two-images-the-python-linux-way#1927681
# diffchops from: http://effbot.org/zone/pil-comparing-images.htm

from PIL import Image
from PIL import ImageChops
import operator
import math
from functools import reduce

IMAGE_IN = 'in3.bmp'
IMAGE_OUT = 'out3.bmp'

# code to add custom paths will be added eventually (maybe :))

cmp1 = Image.open(IMAGE_IN)
cmp2 = Image.open(IMAGE_OUT)

h1 = cmp1.histogram()
h2 = cmp2.histogram()

diff = math.sqrt(reduce(operator.add, map(lambda a,b: (a-b)**2, h1, h2)) / len(h1))
# lets just hope with the floating point comparison :)
print("diff | val =", diff, "| equal =", diff == 0.0)


diffchops = ImageChops.difference(cmp1, cmp2).getbbox()
print("diffchops | eq_exact = ", diffchops is None)