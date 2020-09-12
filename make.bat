cargo build -p rustplugin
copy "target\debug\rustplugin.dll" "%BAKKESMOD_PLUGINS_PATH%\rustplugin.dll"

cargo build -p tinyplugin
copy "target\debug\tinyplugin.dll" "%BAKKESMOD_PLUGINS_PATH%\tinyplugin.dll"

cargo build -p canvastest
copy "target\debug\canvastest.dll" "%BAKKESMOD_PLUGINS_PATH%\canvastest.dll"