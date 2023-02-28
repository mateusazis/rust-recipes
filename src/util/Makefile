# CC=clang
DEFAULT_FLAGS=-nostdlib --shared

libmylib.so: mylib.c
	@echo "CFLAGS are: " $(CFLAGS)
	$(CC) $(CFLAGS) $(DEFAULT_FLAGS) $^ -o$@
