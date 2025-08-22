#pragma once

#include "ui.hpp"
#include "ui_navigation.hpp"
#include "receiver_model.hpp"
#include "transmitter_model.hpp"

class TXRXApp : public View {
public:
    TXRXApp(NavigationView& nav);
    ~TXRXApp();
    
    void focus() override;
    
    std::string title() const override { return "TX/RX Havoc"; };

private:
    NavigationView& nav_;
    
    // Parámetros de configuración
    rf::Frequency frequency_{ 433920000 };
    rf::Frequency frequency_step_{ 100000 };
    uint32_t bandwidth_{ 25000 };
    uint32_t sampling_rate_{ 40000 };
    uint32_t modulation_{ 0 };
    
    // Modelos
    ReceiverModel receiver_model_;
    TransmitterModel transmitter_model_;
    
    // UI Components
    Labels labels_;
    Button button_tx_;
    Button button_rx_;
    Button button_scan_;
    FrequencyField field_frequency_;
    NumberField field_frequency_step_;
    OptionsField option_modulation_;
    NumberField field_bw_;
    NumberField field_sr_;
    
    void on_tx_progress(uint32_t progress);
    void on_rx_data(int16_t *data, size_t count);
    void start_tx();
    void start_rx();
    void stop();
    void update_status();
};