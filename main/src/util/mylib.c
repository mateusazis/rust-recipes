int toUpper(char* input) {
  char* start = input;
  char c;
  while (*input) {
    c = *input;
    if (c >= 'a' && c <= 'z') {
      *input = 'A' + (c - 'a');
    }
    input++;
  }
  return input - start;
}
