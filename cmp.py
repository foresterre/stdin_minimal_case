# requires Pillow
# from: https://stackoverflow.com/questions/1927660/compare-two-images-the-python-linux-way#1927681

from PIL import Image
import operator
import math
from functools import reduce

h1 = Image.open('in3.bmp').histogram()
h2 = Image.open('out3.bmp').histogram()

diff = math.sqrt(reduce(operator.add, map(lambda a,b: (a-b)**2, h1, h2)) / len(h1))
# lets just hope with the floating point comparison :P
print("diff | val =", diff, "| equal =", diff == 0.0)