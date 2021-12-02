#include <stdio.h>

#define cursed_solution _ZN6cursed8solution17hea6b5805cda52171E

struct out {
  char buf[128];
};

extern struct out cursed_solution();

int main() {
  struct out sol = cursed_solution();
  printf("Solution: %s\n", sol.buf);
  return 0;
}
