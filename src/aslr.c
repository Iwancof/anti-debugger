//  Copyright (c) 2018 Julian Kirsch

#include <stdio.h>
#include <string.h>
#include <sys/auxv.h>
#include <sys/utsname.h>
#include <unistd.h>

#include "common.h"

int aslr_active(void) {
  char aslr_state[2] = {0};
  int res = 0;
  FILE *fp = fopen("/proc/sys/kernel/randomize_va_space", "r");

  if (!fp)
    return RESULT_UNK;

  if (fread((void *)aslr_state, 1, sizeof(aslr_state) - 1, fp) !=
      sizeof(aslr_state) - 1) {
    fclose(fp);
    return RESULT_UNK;
  }

  if (aslr_state[0] != '0')
    res = RESULT_YES;
  else
    res = RESULT_NO;

  fclose(fp);
  return res;
}

static const unsigned long long elf_bits = 0x0000555555000000ULL;
static const unsigned long long elf_mask = 0x0000ffffff000000ULL;
static const unsigned long long lib_bits = 0x00007ffff7000000ULL;
static const unsigned long long lib_mask = 0x0000ffffff000000ULL;

static const int DYNAMIC = 0;
static const int STATIC = 1;

int detect_aslr(int is_static) {
  unsigned long elf = getauxval(AT_PHDR) & ~((unsigned long)getpagesize() - 1);
  unsigned long ld = getauxval(AT_BASE) & ~((unsigned long)getpagesize() - 1);
  struct utsname utsname;

  if (!elf || ((is_static == DYNAMIC) && !ld)) {
    return RESULT_UNK;
  }

  if (!(aslr_active() == RESULT_YES)) {
    return RESULT_UNK;
  }

  if (uname(&utsname) == -1) {
    return RESULT_UNK;
  }

  if (((elf & elf_mask) == elf_bits) &&
      ((is_static == DYNAMIC) && ((ld & lib_mask) == lib_bits)))
    return RESULT_YES;
  return RESULT_NO;
}
