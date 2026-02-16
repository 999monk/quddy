## Quddy

Herramienta de uso personal para capturar texto de juegos (o cualquier aplicación), extraerlo mediante OCR, traducirlo al español usando Google Translate y guardar el resultado en un archivo de texto. Escrita en Rust.

### Estructura del proyecto

```
src/
├── main.rs              # CLI con subcomandos (daemon, capture, status, stop)
├── config.rs            # Configuración TOML
├── capture/
│   ├── mod.rs           # Interfaz de captura
│   └── screenshot.rs    # Implementación con maim
├── ocr/
│   ├── mod.rs           # Orquestador OCR
│   ├── preprocessor.rs  # Pre-procesamiento de imagen
│   └── tesseract.rs     # Wrapper de Tesseract
├── translate/
│   ├── mod.rs           # Interfaz de traducción
│   └── google.rs        # Cliente Google Translate
├── daemon/
│   ├── mod.rs           # Loop principal del daemon
│   └── ipc.rs           # Servidor Unix socket
└── client/
    └── mod.rs           # Cliente Unix socket
```
### Arquitectura/diseño

- **Daemon**: Corre en background, escucha comandos via Unix socket
- **Cliente**: Envía comandos (`capture`) al daemon
- **Output**: Archivo de texto plano en `~/.local/share/quddy/output.txt`
- **Visualización**: Vim/nvim con `:set autoread` (recomendado).

Flujo: `quddy capture` → daemon captura pantalla → OCR → traducción → archivo de texto.

### Instalación

```bash
# Dependencias del sistema (Arch, CachyOS)
sudo pacman -S tesseract tesseract-data-eng maim

# Compilar e instalar
cargo build --release
sudo cp target/release/quddy_ocrtranslator /usr/local/bin/quddy
```

### Comandos disponibles

```
quddy start     # Inicia el daemon
quddy capture   # Captura y traduce
quddy status    # Verifica estado del daemon
quddy stop      # Detiene el daemon
quddy --help    # Ayuda
```

### Cómo lo uso

#### 1. Iniciar el daemon

```bash
quddy start
```

#### 2. Visualizar traducciones (método recomendado)

```bash
vim -R ~/.local/share/quddy/output.txt
```

Dentro de vim:
```vim
:set autoread
```

#### 3. Capturar texto

```bash
quddy capture
```

Seleccionar el área con el mouse. La traducción aparecerá instantáneamente en vim.

#### 4. Detener el daemon

```bash
quddy stop
```

### Configuración que uso en i3wm

Agregar a `~/.config/i3/config`:

```
# Hotkey para captura
bindsym $mod+Shift+t exec quddy capture
```

### Configuración de idiomas

Editar `~/.config/quddy/config.toml`:

**Inglés → Español** (default):
```toml
[ocr]
language = "eng"

[translation]
source_lang = "en"
target_lang = "es"
```

Códigos OCR (Tesseract): `eng`, `spa`, `jpn`, `chi_sim`, `chi_tra`, `fra`, `deu`...

Códigos traducción (Google): `en`, `es`, `ja`, `zh`, `fr`, `de`...

### Archivos

- `~/.config/quddy/config.toml` - Configuración
- `~/.local/share/quddy/output.txt` - Última traducción
- `~/.local/share/quddy/quddy.sock` - Socket IPC
- `~/.local/share/quddy/quddy.log` - Logs del daemon

### Licencia
MIT
