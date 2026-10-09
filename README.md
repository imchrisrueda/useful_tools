# useful_tools

Repositorio personal de herramientas, skills y procedimientos para desarrollar con IA.
Cada componente mantiene sus instrucciones, dependencias y comprobaciones en su directorio.

## Componentes y usos

| Directorio | Uso | Estado y documentación |
| --- | --- | --- |
| `herdr_usage/` | Ver modelo, contexto y cuota disponible de Codex y Antigravity en el sidebar de Herdr, agrupados por proyecto | Fork funcional; [guía de uso y límites](herdr_usage/docs/USAGE.es.md) y [README técnico](herdr_usage/README.md) |
| `skills/` | Alojar instrucciones reutilizables para agentes | Estructura preparada; todavía sin skills publicadas. [Convenciones](skills/README.md) |
| `procedures/` | Alojar procedimientos reproducibles de desarrollo | Estructura preparada; todavía sin procedimientos publicados. [Convenciones](procedures/README.md) |
| `.github/` | Ejecutar comprobaciones y mantener plantillas del repositorio | La CI del plugin trabaja desde `herdr_usage/` |

## Herdr Usage: instalación

Fork de [levi-qiao/herdr-agent-usage](https://github.com/levi-qiao/herdr-agent-usage).
El identificador del plugin y su binario siguen siendo `herdr-agent-usage`.
Requiere Herdr 0.9.0+, Codex y/o Antigravity y, para compilar, el toolchain fijado
en `herdr_usage/rust-toolchain.toml` y un compilador C compatible.

```sh
git clone https://github.com/imchrisrueda/useful_tools.git
cd useful_tools/herdr_usage
```

En Windows nativo, desde PowerShell:

```powershell
./install.ps1
# Solo Codex:
./install.ps1 -Agent codex
# Binario previamente compilado, sin descargar dependencias:
./install.ps1 -BinaryPath <ruta-a-herdr-agent-usage.exe>
```

En Linux/macOS, o dentro de WSL si Herdr también corre allí:

```sh
./install.sh
# Solo Codex:
./install.sh --agent codex
```

Los iconos requieren una fuente y un mapeo compatibles con el terminal.
La [guía de uso](herdr_usage/docs/USAGE.es.md) explica actualización,
configuración, desinstalación y diagnóstico.

## Límites de privacidad y funcionamiento offline

- El plugin obtiene métricas de rollouts locales de Codex y del hook StatusLine
  de Agy. Las actualizaciones no consultan APIs remotas ni almacenes de credenciales,
  no leen la salida del terminal y no publican prompts ni resúmenes de sesión.
- Offline significa presentar datos ya producidos en el equipo. Sin nuevas
  observaciones, una cuota puede faltar o conservar su última lectura verificada;
  no representa una consulta actual al proveedor ni una factura.
- El estado local conserva métricas e identificadores técnicos de sesión/pane.
  Los backups de configuración pueden conservar secretos que ya existían en esos
  archivos. Usa directorios privados fuera de sincronización o montajes remotos.
- El recolector Agy suspende la salida del StatusLine anterior; conserva su
  configuración para restaurarla al desinstalar.
- El plugin no impone una sandbox a Codex, Agy, Herdr o al sistema operativo.
  Compilar e instalar puede descargar herramientas y dependencias si no están
  preparadas. No se ofrece una garantía absoluta de seguridad del equipo.

Consulta la [política de seguridad](herdr_usage/SECURITY.md) y la
[auditoría con resultados y comprobaciones pendientes](herdr_usage/docs/PRIVACY_AUDIT.md).

## Añadir contenido

Las herramientas ejecutables nuevas tendrán un directorio propio en la raíz,
con un README, sus dependencias y sus comprobaciones. Las skills se guardan en
`skills/<nombre>/` y los procedimientos en `procedures/`.

Consulta [CONTRIBUTING.md](CONTRIBUTING.md) y las [instrucciones para agentes](AGENTS.md).
La licencia de cada componente se especifica en su directorio; el fork de Herdr
conserva su [licencia MIT](herdr_usage/LICENSE).
