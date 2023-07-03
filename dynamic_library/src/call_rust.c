#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  char letter;
  int occurrences;
} FindMostCommonLetterResult;

extern int double_of(int);
extern void to_upper(char *);
extern FindMostCommonLetterResult find_most_common_letter(char *);

int main(int argc, char **argv) {
  int v = 42;
  int doubled = double_of(42);
  printf("[C] The double of %d is %d (from Rust)\n", v, doubled);

  char *str = "Hello World!";
  char *str2 = strdup(str);
  to_upper(str2);

  printf("[C] String '%s' was transformed to '%s'\n", str, str2);
  FindMostCommonLetterResult mostCommonLetterResult =
      find_most_common_letter(str2);
  printf("[C] The most common letter is %c, which appears %d times.\n",
         mostCommonLetterResult.letter, mostCommonLetterResult.occurrences);

  free((void *)str2);
  return EXIT_SUCCESS;
}
