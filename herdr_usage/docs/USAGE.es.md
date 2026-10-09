# Herdr Usage: usos, operación y límites

## Para qué sirve

Este fork muestra métricas de **Codex y Antigravity (Agy)** en el panel Agent de
Herdr: proveedor, modelo, permiso de sesión (modo sandbox de Codex y estado del
sandbox de Agy), contexto y ventanas de cuota disponibles. Agrupa panes por Space y permite ordenar por
menor cuota restante, cambiar porcentajes usados/restantes y activar avisos de
cuota baja. Los avisos están apagados por defecto; cache y TTL son opcionales.

Es útil para vigilar varias sesiones y elegir dónde continuar trabajando sin
consultar un panel remoto desde el plugin. No calcula facturación ni proporciona
cuotas que el agente no haya registrado. Los demás proveedores presentes en notas
históricas o fixtures no son agentes soportados por este fork.

## Requisitos e instalación

Ejecuta los comandos desde `herdr_usage/`. Necesitas Herdr 0.9.0+ y al menos un
agente soportado. Para compilar necesitas Rust 1.95.0, los componentes del
`rust-toolchain.toml`, las dependencias de `Cargo.lock` y un compilador C nativo
compatible. La instalación enlaza el plugin a este directorio: mantenlo disponible.

| Plataforma | Instalar | Seleccionar agentes | Desinstalar |
| --- | --- | --- | --- |
| Windows nativo, PowerShell | `./install.ps1` | `./install.ps1 -Agent codex,agy` | `./uninstall.ps1` |
| Linux/macOS, shell | `./install.sh` | `./install.sh --agent codex,agy` | `./uninstall.sh` |
| WSL con Herdr dentro de WSL | Igual que Linux | Igual que Linux | Igual que Linux |

La instalación prepara el sidebar y el hook de Agy cuando está seleccionado.
Verifica `herdr integration status`: Codex necesita su integración de Herdr.
Las sesiones abiertas pueden necesitar reiniciarse para cargar hooks nuevos.
Agy necesita emitir un StatusLine para que aparezcan sus métricas.
Los iconos requieren instalar/mapear la fuente en el terminal; una compilación
correcta no verifica su apariencia. El [README](../README.md) explica las fuentes.

### Preparar una instalación offline

La recolección funciona sin Internet con el binario, Herdr y las fuentes locales
preparados. Cargo/rustup pueden descargar dependencias o herramientas durante la
preparación inicial. Para compilar sin descargas, con todo ya en caché:

```sh
cargo build --release --locked --offline
```

En Windows puedes usar `./install.ps1 -Offline` con el toolchain preparado, o
`./install.ps1 -BinaryPath <ruta-a-herdr-agent-usage.exe>` con un binario local
confiable. WSL y Windows nativo son instalaciones distintas; usa la correspondiente
al servidor Herdr que estás ejecutando.

## Uso cotidiano

Abre ajustes con `prefix+shift+q` o mediante:

```sh
herdr plugin pane open --plugin herdr-agent-usage --entrypoint settings --focus
```

Cambia campos, layout, orden, alertas y el intervalo del watcher
(60 segundos por defecto; configurable entre 30 segundos y una hora).
Las comprobaciones adicionales de foco son solo de metadatos locales.
El campo topic está deshabilitado por privacidad.

```sh
# Confirmar que está habilitado:
herdr plugin list
# Forzar una actualización local:
herdr plugin action invoke refresh --plugin herdr-agent-usage
# Reparar configuración administrada:
herdr plugin action invoke configure --plugin herdr-agent-usage
```

La invocación de una acción puede devolver antes de terminar: revisa su log y
espera `succeeded`. No leas panes para diagnosticar métricas; eso puede repintar
los TUI. Ante datos ausentes, comprueba integración, producción de rollouts o
StatusLine y atribución de sesión antes de interpretar la ausencia como cero.

Para actualizar, usa `git pull --ff-only` y vuelve a ejecutar el instalador de tu
plataforma. Guarda antes tus cambios locales. Se conservan preferencias y se
repara la configuración administrada. Para desinstalar usa el script: deshabilitar
el plugin por sí solo no restaura el StatusLine anterior.

## De dónde salen los datos y qué significan

| Fuente | Uso y límite |
| --- | --- |
| Rollouts locales Codex `.jsonl` / `.jsonl.zst` | Métricas registradas; modelo, contexto, cuota y modo de permisos sandbox (`read-only`, `workspace-write`, `full-access`, `external-sandbox`). Atribución por directorio y tiempo de inicio. No consulta APIs de cuota ni `auth.json`. Descarta rutas o políticas privadas. |
| JSON StatusLine Agy | Lista de campos tipados y entrada acotada a 1 MiB. Conserva solo `sandbox.enabled` para mostrar `>sandbox-on` / `>sandbox-off`; atribuye la observación a la sesión y no adivina pools ambiguos. |
| Herdr local, CLI y socket Unix/named pipe Windows | Inventario, metadatos, foco y presentación. No lee salida de terminal. En Windows, argumentos y rutas para shells como cmd/PowerShell se escapan y entrecomillan adecuadamente solo cuando contienen espacios o metacaracteres. |

Solo aparecen ventanas observadas. Un error de lectura preserva la última
observación verificada; no fabrica cero ni convierte el dato anterior en una
nueva medición del proveedor. `ttl≈` es una estimación, no una garantía de
expiración. La atribución ambigua puede producir campos vacíos.

## Privacidad y límites de la garantía

- Las rutas de actualización no usan clientes HTTP, telemetría ni lectores de
  credenciales. No ejecutan el comando StatusLine anterior ni siguen rutas de
  transcripts indicadas por su payload. Tampoco publican topics o resúmenes.
- Los rollouts contienen conversaciones: la lectura acotada puede cargar
  fragmentos en memoria al extraer métricas. Se retienen métricas, modelos e IDs
  técnicos; no se copia ni sube el transcript completo.
- El estado local contiene identificadores de sesión/pane. No es anonimización.
  Los backups completos de configuración pueden contener secretos previamente
  escritos en ella y sirven para restaurarla. El upgrade no purga estado antiguo.
- La salida visual del StatusLine anterior queda suspendida mientras el
  recolector está instalado. Su comando se conserva únicamente para restauración.
- Confía en los binarios, productores de métricas y rutas configuradas. Un
  productor malicioso puede introducir contenido sensible en un campo permitido.
  Directorios sincronizados/remotos y procesos con tus mismos permisos quedan
  fuera de la protección del plugin.
- Codex, Agy, Herdr y el sistema operativo pueden hacer conexiones propias.
  La ausencia de peticiones remotas en el plugin no bloquea esas conexiones.

La [política de seguridad](../SECURITY.md) detalla permisos y almacenamiento.

## Evidencia y comprobaciones pendientes

La [auditoría del 2026-10-09](PRIVACY_AUDIT.md) registra 532 tests en Linux/WSL,
465 en Windows, formato, Clippy y compilación release. La traza con fixtures en
WSL cubre siete rutas, sin conexiones IPv4/IPv6 ni lectura del archivo señuelo
de credenciales. Es evidencia limitada a esas ejecuciones.

En este equipo se instaló y activó el plugin nativo; se comprobaron métricas en
dos panes Codex y el hook Agy con datos sintéticos. Quedan sin comprobar una
sesión Agy real, la apariencia de iconos por observación humana y una traza de
red completa del runtime Windows mediante ETW/WFP. Estas pruebas no certifican
seguridad absoluta ni todos los entornos posibles.
