void toUpper(char* input) {
  char c;
  while (*input) {
    c = *input;
    if (c < 'A' || c > 'Z') {
      *input = 'A' + (c - 'a');
    }
    input++;
  }
}
