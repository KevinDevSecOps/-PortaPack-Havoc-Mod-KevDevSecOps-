#pragma once
#include "ui.hpp"
#include "receiver_model.hpp"
#include "spectrum_collector.hpp"

namespace ui {

class ScannerModView : public View {
public:
    ScannerModView(NavigationView& nav);
    ~ScannerModView();
    
    void focus() override;
    void on_show() override;
    void on_hide() override;

private:
    // Configuración
    static constexpr uint32_t initial_frequency = 2400000000; // 2.4GHz (WiFi/BLE)
    static constexpr uint32_t step = 1000000;                // Paso de 1MHz
    static constexpr uint32_t scan_bandwidth = 80000000;     // 80MHz de ancho de banda

    // Componentes UI
    Labels labels {
        { { 7 * 8, 2 * 16 }, "Frec. Inicial (MHz):", Color::light_grey() },
        { { 7 * 8, 4 * 16 }, "Ancho de banda:", Color::light_grey() }
    };

    NumberField field_frequency {
        { 7 * 8, 3 * 16 },
        5,
        { 100, 6000 },  // Rango: 100MHz a 6GHz
        step / 1000000,
        ' ',
        [this](int32_t) { update_start_frequency(); }
    };

    OptionsField field_bw {
        { 7 * 8, 5 * 16 },
        6,
        {
            { "20M", 20000000 },
            { "40M", 40000000 },
            { "80M", 80000000 }
        }
    };

    Button button_start {
        { 7 * 8, 8 * 16, 96, 32 },
        "ESCANEAR",
        [this]() { start_scan(); }
    };

    // Modelos
    ReceiverModel receiver{};
    bool is_scanning{false};

    // Métodos
    void start_scan();
    void stop_scan();
    void update_start_frequency();
};

} // namespace ui

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
