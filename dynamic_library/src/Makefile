#CC=clang

#LIBRARY_PATH=/home/azis/windows_dev/rustplay/recipes/target/debug
LIBRARY_PATH=../../target/debug
# LIBRARY_PATH=/home/azis/windows_dev/rustplay/recipes2/target/debug

call_rust: call_rust.o
	$(CC) call_rust.o -o$@ -ldynamic_library -L$(LIBRARY_PATH)

# run: call_rust
# 	bash -c "LD_LIBRARY_PATH=$(LIBRARY_PATH) ./call_rust"
