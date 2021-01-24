#ifndef _CLASS_IMAGE_
#define _CLASS_IMAGE_

#include "./vec3.h"
#include <algorithm>
#include <fstream>
#include <string>
#include <tuple>

class image
{
protected:
    unsigned int width;
    unsigned int height;

    float* canvas;
    unsigned int canvasArraySize;

public:
    image(int _width, int _height) {
        width = _width;
        height = _height;

        canvasArraySize = 3 * width * height;
        canvas = new float[canvasArraySize];
    }

    std::tuple<int, int> get_size() const {
        return std::make_tuple(width, height);
    }

    void set_pixel(int x, int y, vec3f& rgb) {
        auto index = 3 * x + 3 * width * y;
        canvas[index] = rgb.get_x();
        canvas[index + 1] = rgb.get_y();
        canvas[index + 2] = rgb.get_z();
    }

    void write_ppm(const std::string& filename) const {
        std::ofstream file(filename);
        if (!file) {
            throw std::exception("failed to open");

            return;
        }

        file << "P3" << std::endl;
        file << width << " " << height << std::endl;
        file << "255" << std::endl;
        for (int j = 0; j < height; ++j) {
            for (int i = 0; i < width; ++i) {
                const int index = 3 * i + 3 * width * j;
                float r = canvas[index];
                float g = canvas[index + 1];
                float b = canvas[index + 2];

                file << static_cast<unsigned int>(std::clamp(255.0f * r, 0.0f, 255.0f)) << " ";
                file << static_cast<unsigned int>(std::clamp(255.0f * g, 0.0f, 255.0f)) << " ";
                file << static_cast<unsigned int>(std::clamp(255.0f * b, 0.0f, 255.0f)) << std::endl;
            }
        }
        file.close();
    }

    void gamma_set() {
        for(int i = 0; i < canvasArraySize; i++) {
            canvas[i] = std::pow(canvas[i], 1 / 2.2f);
        }
    }

    ~image() {

    }
};

#endif // !_CLASS_IMAGE_
