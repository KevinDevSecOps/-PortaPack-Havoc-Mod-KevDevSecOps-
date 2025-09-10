# Crear inicialización del sistema
cat > firmware/src/system_init.cpp << 'EOF'
#include "ch.h"
#include "hal.h"
#include "portapack.hpp"

extern "C" {
    // Variable para el stack
    extern unsigned long _estack;
    
    // Variables del linker
    extern unsigned long _sdata, _edata, _etext;
    extern unsigned long _sbss, _ebss;
}

namespace portapack {

void PortaPack::init() {
    // Inicializar data section (copiar de Flash a RAM)
    unsigned long *src = &_etext;
    unsigned long *dst = &_sdata;
    while (dst < &_edata) {
        *dst++ = *src++;
    }
    
    // Limpiar bss section
    dst = &_sbss;
    while (dst < &_ebss) {
        *dst++ = 0;
    }
    
    // Configurar clock del sistema a 168MHz (STM32F4)
    rcc_clock_init();
    
    // Inicializar periféricos básicos
    gpio_init();
    usart_init();
    
    // Más inicializaciones específicas del PortaPack...
}

void PortaPack::shutdown() {
    // Apagar periféricos
    // Poner en modo low-power
}

} // namespace portapack
EOF