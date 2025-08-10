package main

import (
	"fmt"
	"os"
	"github.com/go-fft/fft" // Librería FFT para análisis espectral
)

func analyzeIQ(file string) {
	data, _ := os.ReadFile(file)
	spectrum := fft.Compute(data)
	
	for freq, power := range spectrum {
		if power > threshold {
			fmt.Printf("🚨 Señal detectada a %d MHz\n", freq)
		}
	}
}

func main() {
	analyzeIQ("capture.iq")
}
