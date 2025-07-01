// Añade esto en el vector 'app_settings':
{ "Jammer Mod", &ui::Bitmap::jammer_icon, [](NavigationView& nav) { nav.push<JammerModView>(); } },
{ "RF Scanner", &ui::Bitmap::scan_icon, [](NavigationView& nav) { nav.push<ScannerModView>(); } },
{ "Replay Tool", &ui::Bitmap::replay_icon, [](NavigationView& nav) { nav.push<ReplayAttackView>(); } },
// Añade esto al vector 'app_settings':
{ "Jammer Mod", &ui::Bitmap::jammer_icon, [](NavigationView& nav) { nav.push<JammerModView>(); } },
{ "Scanner Mod", &ui::Bitmap::scan_icon, [](NavigationView& nav) { nav.push<ScannerModView>(); } },
