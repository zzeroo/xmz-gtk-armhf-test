# Testprojekt für die xMZ-Mod-Touch
## Gtk3 GUI

Die Grafische Benutzeroberfläche der xMZ-Mod-Touch. Erstellt mit dem gtk3 Toolkit.


# Cross Compile Umgebung
## Dependencies

  apt-get install libgtk-3-dev:armhf


## Building

    PKG_CONFIG_ALLOW_CROSS=1 cargo build --target=armv7-unknown-linux-gnueabihf



# Software direkt auf der armhf Hardware erstellen
## Dependencies

  apt-get install libgtk-3-dev


## Building

    cargo build --release


