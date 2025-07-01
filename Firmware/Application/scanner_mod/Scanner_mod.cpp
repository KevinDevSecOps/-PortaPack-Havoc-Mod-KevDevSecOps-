#include "scanner_mod.hpp"
#include "audio.hpp"
#include "portapack.hpp"

using namespace portapack;

namespace ui {

ScannerModView::ScannerModView(NavigationView& nav) {
    add_children({
        &labels,
        &field_frequency,
        &field_bw,
        &button_start
    });

    field_frequency.set_value(initial_frequency / 1000000);
    field_bw.set_by_value(scan_bandwidth);
}

ScannerModView::~ScannerModView() {
    stop_scan();
}

void ScannerModView::focus() {
    button_start.focus();
}

void ScannerModView::start_scan() {
    if (is_scanning) return;
    
    receiver.set_sampling_rate(field_bw.selected_value());
    receiver.set_baseband_bandwidth(field_bw.selected_value());
    receiver.set_frequency(field_frequency.value() * 1000000);
    receiver.enable();
    
    SpectrumCollector::start();  // Usa el recolector de espectro del firmware base
    is_scanning = true;
    button_start.set_text("DETENER");
}

void ScannerModView::stop_scan() {
    if (!is_scanning) return;
    
    SpectrumCollector::stop();
    receiver.disable();
    is_scanning = false;
    button_start.set_text("ESCANEAR");
}

void ScannerModView::update_start_frequency() {
    if (is_scanning) {
        receiver.set_frequency(field_frequency.value() * 1000000);
    }
}

} // namespace ui
