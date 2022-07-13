#define RESULT_NO  0
#define RESULT_UNK 1
#define RESULT_YES 2

int aslr_active(void);
int detect_aslr(int);
int detect_vdso(void);
