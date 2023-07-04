#include "mylib.h"
#include <stdlib.h>
#include <string.h>

static char toUpperChar(char c)
{
  return c >= 'a' && c <= 'z' ? 'A' + (c - 'a') : c;
}

int __attribute__((pure)) toUpper(char *input)
{
  char *start = input;
  char c;
  while (*input)
  {
    c = *input;
    *input = toUpperChar(c);
    input++;
  }
  return input - start;
}

void __attribute__((pure)) toUpper2(const char *input, ResultString *result)
{
  memset(result->str, '\0', sizeof(result->str) / sizeof(char));
  int i = 0;
  char c;
  while ((c = input[i]) != '\0')
  {
    char c = input[i];
    result->str[i] = toUpperChar(c);
    i++;
  }
}
