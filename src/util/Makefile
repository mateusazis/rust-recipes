CC=clang
DEFAULT_FLAGS=-nostdlib --shared -fuse-ld=lld

libmylib.so: mylib.c
	@echo "CFLAGS are: " $(CFLAGS)
	$(CC) $(CFLAGS) $(DEFAULT_FLAGS) $^ -o$@
