#include <iostream>
#include "./vec3.h"

void assert_eq(bool eq) {
  if(eq) {
    std::cout << "ok" << std::endl;
  } else {
    std::cout << "bad" << std::endl;
  }
}

bool vec3_and_scalar_multi_test() {
  auto result = vec3f(1, 1, 1) * 2;

  return result.eqValue(vec3f(2, 2, 2));
}

bool vec3_hadamard_test() {
  auto result = vec3f(1, 2, 3) * vec3f(1, 2, 3);

  return result.eqValue(vec3f(1, 4, 9));
}

bool vec3_add_test() {
  auto result = vec3f(1, 1, 1) + vec3f(1, 1, 1);

  return result.eqValue(vec3f(2, 2, 2));
}

bool vec3_sub_test() {
  auto result = vec3f(1, 1, 1) - vec3f(1, 1, 1);
  
  return result.eqValue(vec3f(0, 0, 0));
}

int main() {
  
  assert_eq(vec3_and_scalar_multi_test());
  assert_eq(vec3_hadamard_test());
  assert_eq(vec3_add_test());
  assert_eq(vec3_sub_test());

  return 0;
}