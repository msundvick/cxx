#include <iostream>
#include <string>

// No CXX_API annotations — the generated header has plain declarations.
#include "cxx.h"
#include "lib.rs.h"

using namespace cxx_noexport_test;

int main() {
    std::cout << "--- Legacy shared-lib (no-export) test ---" << std::endl;

    rust::Box<Engine> engine = new_engine();

    Point p1{1.0, 2.0};
    engine->add_point(p1);

    rust::String report = engine->report();
    std::cout << std::string(report) << std::endl;

    std::cout << "--- Legacy test complete ---" << std::endl;
    return 0;
}
