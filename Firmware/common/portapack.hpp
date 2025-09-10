#pragma once

namespace portapack {

class PortaPack {
public:
    static void init();
    static void shutdown();
};

// Alias para fácil acceso
using portapack = PortaPack;

} // namespace portapack