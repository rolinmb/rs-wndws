#include <iostream>

extern "C" __declspec(dllexport) int add(int a, int b) {
  return a + b;
}
