​​import ctypes
def call_main():
   rust_library = ctypes.CDLL('./libmain.so')
   rust_function = rust_library.main
   rust_function.argtype = []
   rust_function.restype = ctypes.c_int
   result = rust_function()
   print(result)

call_main()
