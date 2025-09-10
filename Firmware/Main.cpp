// 📁 firmware/main.cpp (Core del PortaPack-Havoc-Mod)
#include "havoc_engine.hpp"
#include "signal_ninja.hpp"
#include "kev_theme.hpp"

// 🔥 Configuración KevDevSecOps 
#define HAVOC_MODE_ENABLED   true
#define STEALTH_MODE         false  // ¡Actívalo en misiones!

void setup() {
    // 1. Inicializa el hardware y el tema personalizado
    KevTheme::apply();  // Fondo negro, texto verde neón
    Radio::init();

    // 2. Menú interactivo tipo Metasploit
    HavocUI::show_main_menu({
        {"WiFi Havoc", &wifi_havoc_attack},
        {"Signal Ninja", &hide_data_in_fm},
        {"KeyFob-Cloner", &clone_remote},
        {"RF-Capture", &capture_signal}
    });
}

// 🛠️ Funciones principales
void wifi_havoc_attack() {
    if (HAVOC_MODE_ENABLED) {
        HavocEngine::deauth("target_bssid");  // ¡Ataque Deauth sin PC!
        HavocEngine::beacon_flood();          // Spam de redes falsas
    }
}

void hide_data_in_fm() {
    SignalNinja::encode("Secreto KevDevSecOps", "audio.wav");
    Radio::transmitFM("audio.wav");  // Mensaje oculto en música
}

# Clona el repositorio del firmware original
git clone https://github.com/sharebrained/portapack-havoc
cd portapack-havoc

# Añade los módulos de KevDevSecOps
cp -r ~/PortaPack-Havoc-Mod-KevDevSecOps/firmware/* src/

# Compila
make -j4

// 📁 firmware/havoc_engine.cpp
void HavocEngine::beacon_flood() {
    while (true) {
        for (int i = 0; i < 100; i++) {
            String fake_ssid = "FreeWiFi_" + String(i);
            Packet pkt = build_wifi_beacon(fake_ssid);
            Radio::send(pkt);
            delay(100);
        }
    }
}  

#include "ch.h"
#include "hal.h"
#include "portapack.hpp"

// Includes con paths correctos
#include "ui/ui_system.hpp"
#include "ui/ui_navigation.hpp"
#include "ui/ui_receiver.hpp"
#include "ui/ui_transmitter.hpp"
#include "ui/ui_spectrum.hpp"

using namespace portapack;

// Función de inicialización del hardware
void hardware_init() {
    halInit();
    chSysInit();
    
    // Inicialización del PortaPack
    portapack::init();
    
    // Configuración de la pantalla (si existe)
    // display.init();
    // display.set_contrast(40);
}

// Función principal de la aplicación
void application_main() {
    // Crear sistema de UI
    auto system = ui::System::create();
    
    // Crear navegación principal
    auto navigator = std::make_unique<ui::NavigationView>();
    
    // Configurar vista inicial (Receiver por defecto)
    auto receiver_view = ui::ReceiverView::create(navigator.get());
    // navigator->push(receiver_view);
    
    // Ejecutar el sistema
    system->run(navigator.get());
}

// Punto de entrada principal
int main(void) {
    // Inicializar hardware
    hardware_init();
    
    // Iniciar aplicación
    application_main();
    
    // Loop infinito de seguridad
    while (true) {
        chThdSleepMilliseconds(1000);
    }
    
    return 0;
}
