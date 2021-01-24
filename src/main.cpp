#include <iostream>
#include "./image.h"
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

void write_image_test() {
  image img(512, 512);
  auto size = img.getSize();

  for(int j = 0; j < std::get<1>(size); ++j) {
    for(int i = 0; i < std::get<0>(size); ++i) {
      img.set_pixel(i, j, vec3f(static_cast<float>(i) / std::get<0>(size), static_cast<float>(j) / std::get<1>(size), 1.0f));
    }
  }
  img.write_ppm("output.ppm");
}

int main() {
  
  assert_eq(vec3_and_scalar_multi_test());
  assert_eq(vec3_hadamard_test());
  assert_eq(vec3_add_test());
  assert_eq(vec3_sub_test());

  writeImage();

  return 0;
}