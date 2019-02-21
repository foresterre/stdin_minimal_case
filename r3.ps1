# TODO{}:
# * Perhaps -Encoding Bytes needs to be removed 
# * Flag -Raw idem
Get-Content in3.bmp -Encoding Byte -ReadCount 0 -Raw | cargo run --bin s -- out3ps.bmp
