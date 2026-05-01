#include <iostream>
#include <string>

// No CXX_API_SHARED needed — linking the static archive directly.
#include "cxx.h"
#include "lib.rs.h"

using namespace cxx_static_test;

int main() {
    std::cout << "--- Static library test ---" << std::endl;

    rust::Box<Engine> engine = new_engine();

    Point p1{1.0, 2.0};
    Point p2{3.0, 4.0};
    engine->add_point(p1);
    engine->add_point(p2);

    rust::Vec<Point> points = engine->get_points();
    std::cout << "Retrieved " << points.size() << " points." << std::endl;

    rust::String report = engine->report();
    std::cout << std::string(report) << std::endl;

    std::cout << "--- Static library test complete ---" << std::endl;
    return 0;
}
