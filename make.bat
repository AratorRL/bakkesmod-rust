cargo build -p rustplugin
copy "target\debug\rustplugin.dll" "%BAKKESMOD_PLUGINS_PATH%\rustplugin.dll"