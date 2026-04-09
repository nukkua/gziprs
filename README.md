# the gzip wrapper 

gzip itselft is just a thin wrapper around the ==DEFLATE algorithm== (the same one used by .zip files and .png files)

the file starts with a magic number (0x1F 0x8B), (31, 139) followed by some flags and metadata, and then the compressed data. parsing it is straightforward:

------------------- gzip file -------------------------
0x1F 0x8B magic --- flags & metadata --- DEFLATE stream
0x1F 0x8B.. 8 bytes more... FNAME ... /0(null terminator).. DEFLATE stream

the only flag we care about is == FNAME, which contains the name of the file that was compressed. == we skip it by looking for the next null byte as is standard in C

the inflate function that processes the DEFLATE stream. 
theres many other features in gzip
- such as the operating system that was used to create the file
- the original file size

but we don't need any of them to decompress the data, so we can ignore them.

## DEFLATE blocks

DEFLATE data is organized into blocks. each block starts with a bit indicating whether it's the last block, followed by a 2-bit type:

- type 0: stored (uncompressed) data
- type 1: compressed with fixed huffman codes
- type 2: compressed with dynamic huffman codes

