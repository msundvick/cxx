#include <iostream>
#include <string>

#include "cxx.h"
#include "lib.rs.h"

using namespace cxx_shared_test_v3;

int main() {
    std::cout << "--- Starting DLL test (v3: object-crate exports) ---" << std::endl;

    rust::Box<Engine> engine = new_engine();

    Point p1{10.5, 20.0};
    Point p2{-5.0, 3.14};
    engine->add_point(p1);
    engine->add_point(p2);

    rust::Vec<Point> points = engine->get_points();
    std::cout << "C++: Retrieved " << points.size() << " points from Rust." << std::endl;

    rust::String report = engine->report();
    std::cout << "C++: " << std::string(report) << std::endl;

    rust::Str label = "Production";
    rust::String labeled = engine->report_with_label(label);
    std::cout << "C++: " << std::string(labeled) << std::endl;

    std::cout << "--- Test complete ---" << std::endl;
    return 0;
}
