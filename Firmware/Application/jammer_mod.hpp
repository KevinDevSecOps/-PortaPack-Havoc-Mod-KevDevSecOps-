#pragma once
#include "ui.hpp"
#include "transmitter_model.hpp"
#include "receiver_model.hpp"
#include "audio.hpp"

namespace ui {

class JammerModView : public View {
public:
    JammerModView(NavigationView& nav);
    ~JammerModView();
    
    void focus() override;
    void on_show() override;
    void on_hide() override;

private:
    // Configuración básica
    static constexpr uint32_t initial_frequency = 433920000;  // 433.92MHz (frecuencia ISM común)
    static constexpr uint32_t min_freq = 100000000;          // 100MHz (límite inferior)
    static constexpr uint32_t max_freq = 6000000000;         // 6GHz (límite superior del HackRF)
    static constexpr uint32_t step = 1000000;                // Paso de 1MHz

    // Estados
    bool is_active = false;
    bool sweep_enabled = false;
    uint32_t current_freq = initial_frequency;

    // Componentes de UI
    Labels labels {
        { { 7 * 8, 2 * 16 }, "Frecuencia:", Color::light_grey() },
        { { 7 * 8, 4 * 16 }, "Ancho de banda:", Color::light_grey() },
        { { 7 * 8, 6 * 16 }, "Modo:", Color::light_grey() }
    };

    NumberField field_frequency {
        { 7 * 8, 3 * 16 },
        9,
        { min_freq / 1000000, max_freq / 1000000 },
        step / 1000000,
        ' ',
        [this](int32_t value) {
            set_frequency(value * 1000000);
        }
    };

    OptionsField field_bw {
        { 7 * 8, 5 * 16 },
        6,
        {
            { "1.75M", 1750000 },
            { "2.5M", 2500000 },
            { "3.5M", 3500000 },
            { "5M", 5000000 }
        }
    };

    OptionsField field_mode {
        { 7 * 8, 7 * 16 },
        8,
        {
            { "Continua", 0 },
            { "Barrido", 1 }
        }
    };

    Button button_toggle {
        { 7 * 8, 9 * 16, 96, 32 },
        "INICIAR",
        [this]() {
            toggle_jammer();
        }
    };

    // Modelos
    TransmitterModel transmitter;
    ReceiverModel receiver;

    // Métodos
    void set_frequency(uint32_t freq);
    void toggle_jammer();
    void start_jammer();
    void stop_jammer();
    void sweep_frequencies();
};

} // namespace ui
