#pragma once
#include <vector>
#include <memory>

namespace ui {

class View;

class NavigationView {
public:
    void push(std::unique_ptr<View> view);
    void pop();
    View* top() const;
    
private:
    std::vector<std::unique_ptr<View>> stack_;
};

} // namespace ui