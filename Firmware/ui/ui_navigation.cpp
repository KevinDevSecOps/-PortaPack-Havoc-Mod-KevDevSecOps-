// Añade esto al vector 'app_settings':
{ "Jammer Mod", &ui::Bitmap::jammer_icon, [](NavigationView& nav) { nav.push<JammerModView>(); } },
{ "Scanner Mod", &ui::Bitmap::scan_icon, [](NavigationView& nav) { nav.push<ScannerModView>(); } },
