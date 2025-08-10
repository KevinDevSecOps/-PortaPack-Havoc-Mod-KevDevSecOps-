func cloneSignal(sourceFile, outputFile string) {
	rawData := readRawSignal(sourceFile)
	portapackFormat := convertToPortaPackFormat(rawData)
	os.WriteFile(outputFile, portapackFormat, 0644)
	fmt.Println("✅ Señal clonada para PortaPack!")
}
