#pragma once
#include "ui_navigation.hpp"

namespace ui {

class NavigationView;

class ReceiverView : public View {
public:
    explicit ReceiverView(NavigationView* navigator);
    ~ReceiverView() override;
    
    void on_show() override;
    void on_hide() override;
    void focus() override;
    
    static std::unique_ptr<ReceiverView> create(NavigationView* navigator);
};

} // namespace ui