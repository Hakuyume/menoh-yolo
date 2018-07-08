#include <opencv2/highgui.hpp>
#include <opencv2/imgproc.hpp>

extern "C" {
bool cv_imshow()
{
  cv::Mat img(320, 320, CV_8UC3);
  cv::imshow("hoge", img);
  return true;
}

int cv_waitKey(int delay)
{
  return cv::waitKey(delay);
}
}
