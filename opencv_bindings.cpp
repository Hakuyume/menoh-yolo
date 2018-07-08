#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>

extern "C" {
void cv_imshow(const char *winname, int rows, int cols, const uint8_t *data)
{
  cv::Mat mat(rows, cols, CV_8UC3, const_cast<uint8_t *>(data));
  cv::imshow(winname, mat);
}

int cv_waitKey(int delay)
{
  return cv::waitKey(delay);
}
}
