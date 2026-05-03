#include <iostream>
#include <string>

// Include cxx and our generated header
#include "target/cxxbridge/rust/cxx.h"
#include "target/cxxbridge/cxx-shared-test/src/lib.rs.h"

int main() {
    std::cout << "--- Starting Complex DLL Test ---" << std::endl;

    // 1. Call the constructor. This instantiates a Rust Box across the DLL boundary.
    rust::Box<Engine> engine = new_engine();

    // 2. Create instances of the shared struct
    Point p1 { 10.5, 20.0 };
    Point p2 { -5.0, 3.14 };

    // 3. Call mutating methods on the opaque Rust object
    engine->add_point(p1);
    engine->add_point(p2);

    // 4. Retrieve a Vector of shared structs
    rust::Vec<Point> points = engine->get_points();
    std::cout << "C++: Retrieved " << points.size() << " points from Rust." << std::endl;

    // 5. Call a method returning a String
    rust::String report = engine->report();
    std::cout << "C++: " << std::string(report) << std::endl;

    std::cout << "--- Test Complete ---" << std::endl;
    return 0;
}