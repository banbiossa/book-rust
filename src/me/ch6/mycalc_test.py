import platform, os
from ctypes import *

pf = platform.system()
print(pf)

# breakpoint()
libfile = "libmycalc.dylib"
libpath = os.path.join(os.path.dirname(__file__), libfile)
print("libpath=", libpath)

mycalc = cdll.LoadLibrary(libpath)
print(mycalc.rust_mul(100, 9))
print(mycalc.rust_mul(9, 1))
