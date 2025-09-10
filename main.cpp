# Crear el archivo main.cpp con el contenido básico
cat > main.cpp << 'EOF'
#include "ch.h"
#include "hal.h"
#include "portapack.hpp"

#include "ui_system.hpp"
#include "ui_navigation.hpp"
#include "ui_receiver.hpp"
#include "ui_transmitter.hpp"
#include "ui_spectrum.hpp"

using namespace portapack;

// Función de inicialización del hardware
void hardware_init() {
    halInit();
    chSysInit();
    
    // Inicialización del PortaPack
    portapack::init();
    
    // Configuración de la pantalla
    display.init();
    display.set_contrast(40);
}

// Función principal de la aplicación
void application_main() {
    // Crear sistema de UI
    auto system = ui::System::create();
    
    // Crear navegación principal
    auto navigator = std::make_unique<ui::NavigationView>();
    
    // Configurar vista inicial (Receiver por defecto)
    auto receiver_view = ui::ReceiverView::create(navigator.get());
    navigator->push(receiver_view);
    
    // Ejecutar el sistema
    system->run(navigator);
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
EOF