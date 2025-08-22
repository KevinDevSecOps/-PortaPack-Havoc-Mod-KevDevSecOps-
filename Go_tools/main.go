package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"

	"go.bug.st/serial"
)

type PortaPackControl struct {
	port serial.Port
}

func NewPortaPackControl(portName string) (*PortaPackControl, error) {
	mode := &serial.Mode{
		BaudRate: 115200,
		DataBits: 8,
		Parity:   serial.NoParity,
		StopBits: serial.OneStopBit,
	}

	port, err := serial.Open(portName, mode)
	if err != nil {
		return nil, err
	}

	return &PortaPackControl{port: port}, nil
}

func (p *PortaPackControl) SendCommand(cmd string) error {
	_, err := p.port.Write([]byte(cmd + "\n"))
	return err
}

func (p *PortaPackControl) SetFrequency(freq uint64) error {
	return p.SendCommand(fmt.Sprintf("FREQ %d", freq))
}

func (p *PortaPackControl) SetModulation(mod string) error {
	return p.SendCommand(fmt.Sprintf("MOD %s", mod))
}

func (p *PortaPackControl) StartTX() error {
	return p.SendCommand("TX START")
}

func (p *PortaPackControl) StartRX() error {
	return p.SendCommand("RX START")
}

func (p *PortaPackControl) CaptureToFile(duration time.Duration, filename string) error {
	p.SendCommand(fmt.Sprintf("CAPTURE %d", duration.Milliseconds()))
	
	// Leer datos capturados
	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	reader := bufio.NewReader(p.port)
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			return err
		}
		
		if strings.HasPrefix(line, "CAPTURE_END") {
			break
		}
		
		if strings.HasPrefix(line, "DATA:") {
			file.WriteString(line[5:])
		}
	}

	return nil
}

func main() {
	if len(os.Args) < 2 {
		log.Fatal("Usage: portapack-control <serial-port>")
	}

	control, err := NewPortaPackControl(os.Args[1])
	if err != nil {
		log.Fatal("Error opening serial port: ", err)
	}
	defer control.port.Close()

	// Ejemplo de uso
	control.SetFrequency(433920000)
	control.SetModulation("BPSK")
	control.StartRX()

	// Capturar datos por 10 segundos
	control.CaptureToFile(10*time.Second, "capture.iq")

	fmt.Println("Operación completada")
}