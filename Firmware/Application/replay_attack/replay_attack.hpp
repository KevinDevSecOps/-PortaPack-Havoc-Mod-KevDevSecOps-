#pragma once
#include "ui.hpp"
#include "transmitter_model.hpp"
#include "sd_card.hpp"

namespace ui {

class ReplayAttackView : public View {
public:
    ReplayAttackView(NavigationView& nav);
    void on_save(const std::string& filename);
private:
    TransmitterModel transmitter;
    void load_signal_from_sd();
};
} // namespace ui
