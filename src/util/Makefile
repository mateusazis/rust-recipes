CC=clang

libmylib.so: mylib.c
	echo "CFLAGS are: " $(CFLAGS)
	$(CC) $(CFLAGS) --shared $^ -o$@
	# clang --target=arm64v7a-linux-gnueabi -fuse-ld=lld --shared $^ -o$@
