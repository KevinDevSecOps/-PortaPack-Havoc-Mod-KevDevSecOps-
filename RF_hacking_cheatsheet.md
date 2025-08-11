**Contenido técnico real** (para auditorías éticas):  
```markdown
# PortaPack-Havoc Mod - Cheatsheet de RF Hacking (Ético)

## 🔍 **Reconocimiento con HackRF**
```bash
# Escanear frecuencias cercanas (ej: 433.92MHz para IoT)
hackrf_sweep -f 433M:434M -w 100000 -r scan.csv
```

## 📡 **Inyección de Paquetes RF (Ejemplo: Replay Attack)**
```python
# Requiere: Python + GNU Radio
from gnuradio import blocks, gr
flowgraph = gr.top_block()
sdr = blocks.hackrf_source(freq=315e6, samp_rate=2e6)
sink = blocks.file_sink(gr.sizeof_gr_complex, "captura.raw")
flowgraph.connect(sdr, sink)
flowgraph.run()  # Captura señales RAW
```

## 🛡️ **Protección Contra Ataques RF**
- **Encripción AES-128** en dispositivos IoT.
- **Rolling Codes** en sistemas de apertura (garajes, coches).
- **Validación de checksum** en tramas RF.

## ⚠️ **Aviso Legal**
- Este contenido es **solo para investigación autorizada**.
- Nunca atacar sistemas sin permiso por escrito (Art. 197 CP).
```

---

### 📥 **Cómo Añadirlo a tu Fork (Desde Móvil)**  
1. **Abre tu fork** en GitHub (app o navegador).  
2. Toca **"Add file" > "Create new file"**.  
3. Pega el contenido arriba y nómbralo: `RF_hacking_cheatsheet.md`.  
4. Guarda (**Commit directly to the `main` branch**).  

---

### 🎯 **Bonus: Script para PortaPack**  
Si prefieres un **archivo ejecutable** (.py), aquí tienes uno que analiza señales:  
```python
# save as: `RF_analyzer.py`
import numpy as np
from hackrf import HackRF

def scan_frequency(freq):
    with HackRF() as sdr:
        sdr.sample_rate = 20e6
        sdr.center_freq = freq
        samples = sdr.read_samples(1e6)
        power = np.mean(np.abs(samples)**2)
        return f"Freq: {freq/1e6}MHz | Power: {power:.2f} dBm"

print(scan_frequency(433.92e6))  # Ejemplo: frecuencia de sensores IoT
```

---

### 💥 **Por qué esto es "Ciberseguridad Real"**  
- **Herramientas usadas**: HackRF, GNU Radio (estándar en pentesting RF).  
- **Aplicaciones legales**:  
  - Testear tus propios dispositivos.  
  - Auditorías de red inalámbrica (con contrato).  
  - Investigación académica en comunicaciones.  

---