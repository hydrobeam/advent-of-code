#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define BUF_SIZE 27
#define ALPHABET_SIZE 26

int main(int argc, char *argv[]) {
  FILE *file = fopen("../inputs/day02.txt", "r");
  // part 1

  // must be zero otherwise we get old trash data here
  char inputBuf[BUF_SIZE] = {0};
  char charray[ALPHABET_SIZE] = {0};
  int appearTwo = 0;
  int appearThree = 0;

  while (fscanf(file, "%s", inputBuf) > 0) {
    for (int i = 0; i < BUF_SIZE; ++i) {
      int newInd = inputBuf[i] - 'a';
      charray[newInd] += 1;
    }
    bool seenTwo = false;
    bool seenThree = false;

    for (int i = 0; i < ALPHABET_SIZE; ++i) {
      if (charray[i] == 2 && !seenTwo) {
        appearTwo += 1;
        seenTwo = true;
      } else if (charray[i] == 3 && !seenThree) {
        appearThree += 1;
        seenThree = true;
      }
    }

    // reset the charray
    memset(charray, 0, sizeof(charray));
  }

  printf("appearTwo: %d\n", appearTwo);
  printf("appearThree: %d\n", appearThree);
  printf("part1: %d\n", appearTwo * appearThree);

  // part 2

  // we're at eof by end of part 1
  int size = ftell(file);
  size = sizeof(char) * size / BUF_SIZE;
  char **heldArray = malloc(size * sizeof(char *));
  for (int i = 0; i < size; ++i) {
    heldArray[i] = malloc(BUF_SIZE);
  }

  rewind(file);

  for (int i = 0; fscanf(file, "%s", inputBuf) > 0; ++i) {
    memcpy(heldArray[i], inputBuf, BUF_SIZE);
  }

  // we need these beyond the loop
  int i = 0;
  int j = 1;
  for (int diffIndex = 0; i < size || diffIndex == 1; ++i) {
    for (j = i + 1; j < size; ++j) {
      diffIndex = 0;
      for (int cmpInd = 0; cmpInd < BUF_SIZE; ++cmpInd) {
        char chr1 = heldArray[i][cmpInd];
        char chr2 = heldArray[j][cmpInd];
        if (chr1 != chr2) {
          ++diffIndex;
        }
      }
      if (diffIndex == 1) {
        goto out;
      }
    }
  }

out:
  for (int bufInd = 0, cmpInd = 0; cmpInd < BUF_SIZE; ++cmpInd) {
    char chr1 = heldArray[i][cmpInd];
    char chr2 = heldArray[j][cmpInd];
    if (chr1 == chr2) {
      inputBuf[bufInd] = chr1;
      ++bufInd;
    }
  }
  printf("part2: %s\n", inputBuf);

  for (int freeMem = 0; freeMem < size; ++freeMem) {
    free(heldArray[freeMem]);
  }
  free(heldArray);
  fclose(file);

  return 0;
}
