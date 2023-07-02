#ifdef __cplusplus
extern "C" {
  #endif

int toUpper(char* input);

typedef struct {
  char str[1024];
} ResultString;

void toUpper2(const char *input, ResultString *result);

#ifdef __cplusplus
}
#endif
