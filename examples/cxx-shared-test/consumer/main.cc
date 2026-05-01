#include <iostream>
#include <string>

#include "cxx.h"
#include "lib.rs.h"

using namespace cxx_shared_test;

int main() {
    std::cout << "--- Starting DLL test ---" << std::endl;

    // 1. Instantiate a Rust-owned opaque type across the DLL boundary.
    rust::Box<Engine> engine = new_engine();

    // 2. Create and pass shared structs.
    Point p1{10.5, 20.0};
    Point p2{-5.0, 3.14};
    engine->add_point(p1);
    engine->add_point(p2);

    // 3. Retrieve a Vec of shared structs.
    rust::Vec<Point> points = engine->get_points();
    std::cout << "C++: Retrieved " << points.size() << " points from Rust." << std::endl;

    // 4. Retrieve a String return value.
    rust::String report = engine->report();
    std::cout << "C++: " << std::string(report) << std::endl;

    // 5. Round-trip: pass a C++ string slice to Rust and get a String back.
    rust::Str label = "Production";
    rust::String labeled = engine->report_with_label(label);
    std::cout << "C++: " << std::string(labeled) << std::endl;

    std::cout << "--- Test complete ---" << std::endl;
    return 0;
}
