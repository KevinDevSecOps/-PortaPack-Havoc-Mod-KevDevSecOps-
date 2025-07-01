#include "scanner_mod.hpp"
#include "spectrum_collector.hpp"

namespace ui {

ScannerModView::ScannerModView(NavigationView& nav) {
    add_children({ /* Añade aquí tus componentes UI */ });

    receiver.set_sampling_rate(4000000);  // 4 Msps
    receiver.set_baseband_bandwidth(3500000);
}

void ScannerModView::start_scan() {
    receiver.set_frequency(frequency);  // Define 'frequency' como variable de clase
    SpectrumCollector::start();  // Usa el recolector de espectro del firmware base
    is_scanning = true;
}

void ScannerModView::stop_scan() {
    SpectrumCollector::stop();
    receiver.disable();
    is_scanning = false;
}

} // namespace ui
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
