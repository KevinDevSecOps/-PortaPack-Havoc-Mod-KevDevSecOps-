# Crear configuración de hardware
cat > firmware/hal/hardware_config.hpp << 'EOF'
#pragma once

// Configuración del hardware específico del PortaPack
#define PORTPACK_SCREEN_WIDTH 400
#define PORTPACK_SCREEN_HEIGHT 240
#define PORTPACK_SAMPLE_RATE 4000000

// Pines importantes
#define LCD_CS_PIN GPIO_PIN_2
#define LCD_DC_PIN GPIO_PIN_3
#define LCD_RESET_PIN GPIO_PIN_4

// Configuración de radio
#define MAX_RF_FREQUENCY 4400000000
#define MIN_RF_FREQUENCY 100000000
EOF