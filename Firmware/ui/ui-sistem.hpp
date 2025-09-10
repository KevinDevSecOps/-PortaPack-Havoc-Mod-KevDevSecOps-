#pragma once
#include <memory>

namespace ui {

class NavigationView;

class System {
public:
    static std::unique_ptr<System> create();
    void run(NavigationView* navigator);
};

} // namespace ui