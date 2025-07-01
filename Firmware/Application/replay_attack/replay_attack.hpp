#include "replay_attack.hpp"
#include "io_file.hpp"

namespace ui {

ReplayAttackView::ReplayAttackView(NavigationView& nav) {
    add_children({ /* Botones/UI aquí */ });
}

void ReplayAttackView::on_save(const std::string& filename) {
    File file;
    if (file.create(filename)) {  // Usa la API de archivos del PortaPack
        file.write(signal_data);  // 'signal_data' debe contener las muestras IQ
    }
}

} // namespace ui
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
