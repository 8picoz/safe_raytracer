#include "image.h"
#include "pinhole-camera.h"
#include "sphere.h"

int main() {
  constexpr int width = 512;
  constexpr int height = 512;
  Image img(512, 512);

  PinholeCamera camera(Vec3f(0, 0, 3), Vec3f(0, 0, -1));

  for (int j = 0; j < height; ++j) {
    for (int i = 0; i < width; ++i) {
      // (u, v)の計算
      float u = (2.0f * i - width) / height;
      float v = (2.0f * j - height) / height;

      // レイの生成
      const Ray ray = camera.sampleRay(u, v);

      // 生成されたレイの可視化
      img.setPixel(i, j, 0.5f * ray.direction + Vec3f(0.5f));
    }
  }

  // PPM画像の生成
  img.writePPM("output.ppm");

  return 0;
}