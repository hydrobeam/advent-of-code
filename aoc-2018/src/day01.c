#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

const int ARRAY_SIZE = 400000;

bool inArray(int array[], int currSum, int currIndex) {
  for (int i = 0; i < ARRAY_SIZE && i < currIndex; ++i) {
    /* printf("hi: %d\n", currIndex); */
    if (array[i] == currSum) {
      return true;
    }
  }
  return false;
}

int main(int argc, char *argv[]) {
  FILE *file = fopen("../inputs/day01.txt", "r");

  // part 1
  int sum = 0;
  int freq = 0;

  while (fscanf(file, "%d", &freq) > 0) {
    sum += freq;
  }
  printf("part1: %d\n", sum);
  rewind(file);

  // part 2

  int array[ARRAY_SIZE];
  int index = 0;
  sum = 0;
  freq = 0;

  while (1) {
    while (fscanf(file, "%d", &freq) > 0) {
      sum += freq;
      if (inArray(array, sum, index)) {
        goto out;
      } else {
        array[index] = sum;
        ++index;
      }
    }
    rewind(file);
  }

out:
  printf("part2: %d\n", sum);
  printf("index: %d\n", index);
  return 0;
}
