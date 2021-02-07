#include "image.h"
#include "sphere.h"

int main() {
  constexpr int width = 512;
  constexpr int height = 512;
  Image img(512, 512);

  for (int j = 0; j < height; ++j) {
    for (int i = 0; i < width; ++i) {
      img.setPixel(i, j,
                   Vec3f(static_cast<float>(i) / width,
                         static_cast<float>(j) / height, 1.0f));
    }
  }
  img.writePPM("output.ppm");

  return 0;
}