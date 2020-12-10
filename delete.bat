taskkill /F /IM "java.exe"
DEL /F "D:\Eigene-Dateien\Intellij-Projects\Rust\Wormcord\target\release\wormcord.exe"
start /b "" cmd /c del "%~f0"&exit /b