#pragma once
#include "ui.hpp"
#include "receiver_model.hpp"

namespace ui {

class ScannerModView : public View {
public:
    ScannerModView(NavigationView& nav);
    void on_show() override;
    void on_hide() override;
private:
    ReceiverModel receiver;
    bool is_scanning = false;
    void start_scan();
    void stop_scan();
};
} // namespace ui
