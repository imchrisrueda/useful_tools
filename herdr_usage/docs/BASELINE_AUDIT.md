> Auditoría histórica del punto de partida. Para el estado revisado del fork y
> los límites de verificación, consulta [PRIVACY_AUDIT.md](PRIVACY_AUDIT.md).

# Auditoría de Base (Baseline Audit) — Herdr Agent Usage

> **Registro de Auditoría de Base y Fijación de Contratos (Fase 1 / Paso 0)**  
> Las decisiones arquitectónicas vigentes se rigen por `.agents/IMPLEMENTATION_PLAN.md`, `AGENTS.md` y `context.md`.  
> Esta auditoría documenta el estado observado del repositorio base, las correcciones derivadas de la revisión técnica y la matriz de contratos fuente → campo → alcance → timestamp → ausencia.

**Fecha:** 25 de septiembre de 2026  
**Repositorio Base Principal:** [levi-qiao/herdr-agent-usage](https://github.com/levi-qiao/herdr-agent-usage)  
**Commit Origen:** `00b8dbd078e5e072d24f1dadea994c1c44c9fe3f`  
**Tag / Versión:** `v1.6.2-21-g00b8dbd` (Release base `v1.6.2`)  
**Repositorio Referencia Secundaria:** [senna-lang/herdr-agent-usage](https://github.com/senna-lang/herdr-agent-usage) (`956aa4f3efa5e86f4d5f1665533210bb7eadaa09`, v0.5.16)  
**Rama de Trabajo:** `custom/codex-agy-only`  
**Toolchain Fijado:** Rust `1.95.0` (`rust-toolchain.toml`)  

---

## 1. Resumen Ejecutivo y Diagnóstico Global

El repositorio base `herdr-agent-usage` de Levi Qiao es un plugin para el multiplexor y gestor de agentes [Herdr](https://herdr.dev) escrito en Rust. Proporciona recopilación y visualización de cuotas, límites de contexto y métricas en la barra lateral (*sidebar*), indicadores visuales (*gauges*) y paneles de configuración (*settings* y *dashboard*).

### Hallazgos Críticos Verificados en el Código:

1. **Superficie de Acceso y Extracción de Credenciales**:
   - `src/providers/cursor.rs`: Utiliza `rusqlite` para abrir `state.vscdb` (SQLite de Cursor IDE) y extraer `cursorAuth/accessToken`.
   - `src/providers/muse.rs`: Invoca `/usr/bin/security find-generic-password` para extraer tokens del Keychain de macOS.
   - `src/providers/devin.rs`: Parsea `credentials.toml` para extraer API tokens.
   - `src/providers/opencode_go.rs`: Inyecta cabeceras `Authorization: Bearer <token>`.
   - `src/providers/codex.rs`: Abre `~/.codex/auth.json` y deserializa IDs de cuenta (`account_id`, `chatgpt_account_id`). Aunque la estructura `AuthMetadata` no materializa el token para uso posterior, el mero acceso a archivos de autenticación vulnera el principio de cero credenciales.

2. **Dependencia de Red Externa**:
   - `Cargo.toml` incluye la dependencia cliente HTTP `ureq = { version = "2.12", features = ["json"] }`.
   - `ureq` es utilizado exclusivamente por los proveedores Cursor, Devin, Grok, Muse y OpenCode Go para consultar APIs externas.
   - **Observación verificada:** Ni Codex ni Antigravity requieren `ureq`. Al eliminar los otros 8 proveedores, `ureq` puede ser completamente extirpado.

3. **Incompatibilidad y Asunciones de Plataforma (Windows vs. Unix)**:
   - `src/herdr.rs:427`: Conecta al socket IPC de Herdr mediante `std::os::unix::net::UnixStream::connect(&path)` sin condicional `#[cfg(unix)]`.
   - `src/herdr.rs:700`: Invoca el comando externo `ps -o etimes= -p <pid>`, inexistente en Windows.
   - `src/process.rs:33`: Invoca `Command::new("sh").args(["-c", ...])` y llamadas POSIX (`libc::setpgid`, `libc::killpg`).
   - `src/configure/statusline.rs`: Asume variable de entorno `$HOME` en lugar de resolver de forma portable (`directories::BaseDirs` o `%USERPROFILE%`).
   - `herdr-plugin.toml`: Declara `platforms = ["macos", "linux"]` y empaqueta todas las acciones en `sh -c`. El soporte de Windows debe tratarse como condicionado a pruebas de compatibilidad reales con el runtime de Herdr.

4. **Persistencia de Payloads No Sanitizados en Caché**:
   - En `src/cache.rs`, el método `save_statusline_observation` almacena `observation.clone()` directamente en disco (`StatuslineObservation.payload`), lo cual puede incluir transcripciones o datos sensibles recibidos de `statusLine`. Debe filtrarse rigurosamente a un esquema mínimo permitido.

5. **Atribución de Cuota y Asociación Sesión ↔ Pane**:
   - En Codex, un archivo `rollout*.jsonl` **no identifica de forma verificable la cuenta servidora global**. Atribuir la cuota de un rollout a nivel de cuenta global o compartirla entre sesiones es incorrecto. Debe limitarse estrictamente a la sesión emisora con timestamp y caducidad explícita.
   - El emparejamiento por CWD y «rollout más reciente» es insuficiente: se debe priorizar el ID explícito de Herdr, o la combinación de CWD exacto con tiempo de inicio de proceso y candidato único. Ante candidatos múltiples o ambiguos, **no debe realizarse ninguna asociación**.

---

## 2. Mapa Arquitectónico del Repositorio

```text
                                 HERDR RUNTIME
                                       │
            ┌──────────────────────────┴──────────────────────────┐
            ▼                                                     ▼
     Eventos / Hooks                                       Acciones de Usuario
  (pane.agent_detected,                                  (refresh, configure,
   pane.agent_status_changed,                             dashboard, settings)
   focus, startup)                                                │
            │                                                     │
            └──────────────────────────┬──────────────────────────┘
                                       ▼
                              CLI Router (`src/cli.rs`)
                                       │
            ┌──────────────────────────┴──────────────────────────┐
            ▼                                                     ▼
   Configurador Global                                  Motor de Refresco
 (`src/configure/mod.rs`)                              (`src/refresh.rs`)
            │                                                     │
            ├─ statusline hook setup                              ├─ Detección de pane / agente
            │  (Claude, Agy)                                      ├─ Routing de suscripción (`src/route.rs`)
            └─ herdr.toml settings injection                      └─ Invocación de colectores
                                                                          │
                                     ┌────────────────────────────────────┴───────────────────────────┐
                                     ▼                                                                ▼
                     Proveedores Locales Seguros                                     Proveedores con Red / Credenciales
               ┌─────────────────────┴─────────────────────┐                   ┌──────────────────────┴──────────────────────┐
               ▼                                           ▼                   ▼                                             ▼
          Antigravity                                    Codex              Cursor, Devin, Grok,                       OpenCode, Pi,
        (StatusLine IPC)                          (App-server + Rollouts)   Muse, Claude                                    Omp
  `src/providers/agy.rs`                          `src/providers/codex.rs`  - `ureq` HTTP a APIs públicas              - Lectura SQLite local
  - Captura stdin JSON                            - Lanza `codex app-server`  - Extracción de tokens OAuth               - Hashing Sha256 de claves
  - Snapshot de cuota (5h, 7d, api)               - Lee `auth.json`           - Ejecución de `security` (Keychain)       - Parsing de sesiones .db
  - Contexto y caché de tokens                    - Lee rollouts JSONL        - Almacena cookies de navegador
               │                                           │
               └─────────────────────┬─────────────────────┘
                                     ▼
                        Persistencia Local en Caché
                    (`src/cache.rs` en `$HERDR_PLUGIN_STATE_DIR`)
                                     │
                                     ▼
                          Normalización y Presentación
                 (`src/model.rs`, `src/presentation.rs`, `src/herdr.rs`)
                                     │
                                     ▼
                    Inyección de UI en Herdr (Sidebar / Popup)
```

---

## 3. Inventario y Clasificación de Proveedores

| Proveedor | Archivos Fuente | Mecanismo de Obtención | ¿Lee Credenciales? | ¿Usa Red (`ureq`)? | ¿Lanza Procesos Externos? | Veredicto |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Antigravity (Agy)** | `src/providers/agy.rs`<br>`src/configure/agy.rs`<br>`src/providers/statusline.rs` | Hook `statusLine` de Antigravity (recibe JSON por stdin). | **NO**. Lee métricas de cuota y ventana de contexto directamente del payload. | **NO**. Cero red. | Opcional (encadenamiento de comando previo del usuario si existía). | **CONSERVAR Y REFORZAR** (Filtrar payload bruto persistido). |
| **Codex** | `src/providers/codex.rs` | Híbrido: `codex app-server --stdio` + lectura de rollouts `~/.codex/sessions/**/rollout*.jsonl`. | **SÍ**: Abre `auth.json` y extrae IDs de cuenta. | **Potencial indirecto**: `app-server` puede consultar servidores de OpenAI. | **SÍ**: Lanza `codex app-server`. | **MODIFICAR RADICALMENTE**: Eliminar `app-server` y `auth.json`; convertir en lector exclusivo de rollouts locales; atribuir cuota solo a nivel de sesión. |
| **Claude** | `src/providers/claude.rs`<br>`src/configure/claude.rs` | Hook `statusLine` en `~/.claude/settings.json`. | **SÍ**: Manipula configuración externa del usuario y credenciales OAuth. | **NO**. | **SÍ**: Invocación de shell. | **ELIMINAR** |
| **Cursor** | `src/providers/cursor.rs`<br>`src/configure/cursor.rs` | SQLite `state.vscdb` + HTTP POST a `api2.cursor.sh`. | **SÍ**: Extrae `cursorAuth/accessToken`. | **SÍ**: Peticiones HTTPS autenticadas con `ureq`. | **SÍ**: Comandos de aprobación interactiva. | **ELIMINAR** |
| **Devin** | `src/providers/devin.rs` | Lee `credentials.toml` y `sessions.db` + HTTP POST a `server.codeium.com`. | **SÍ**: Parsea API tokens en `credentials.toml`. | **SÍ**: Peticiones HTTPS autenticadas con `ureq`. | **NO**. | **ELIMINAR** |
| **Grok** | `src/providers/grok.rs`<br>`src/configure/grok.rs` | Lee `credentials.json` + HTTP GET a `cli-chat-proxy.grok.com`. | **SÍ**: Extrae tokens de API. | **SÍ**: Peticiones HTTPS autenticadas con `ureq`. | **NO**. | **ELIMINAR** |
| **Muse** | `src/providers/muse.rs` | Acceso a macOS Keychain vía `security` + HTTP POST a `api.meta.ai`. | **SÍ**: Lee contraseñas genéricas del Keychain (`find-generic-password`). | **SÍ**: Peticiones HTTPS autenticadas con `ureq`. | **SÍ**: Ejecuta `/usr/bin/security`. | **ELIMINAR** |
| **OMP** | `src/omp.rs`<br>`src/providers/omp.rs` | Lectura de SQLite `models.db` y hashing Sha256 de cuentas. | **SÍ**: Hashea identidades de cuentas para enrutamiento. | **NO**. | **NO**. | **ELIMINAR** |
| **OpenCode** | `src/opencode.rs`<br>`src/providers/opencode_go.rs` | Lectura de SQLite `opencode.db` + HTTP GET a `opencode.ai`. | **SÍ**: Lee claves de API en archivos locales. | **SÍ**: Peticiones HTTPS autenticadas con `ureq`. | **NO**. | **ELIMINAR** |
| **Pi** | `src/pi.rs` | Inspección de `~/.pi/agent/auth.json` y sesiones locales. | **SÍ**: Compara tokens y credenciales de cuentas. | **NO**. | **NO**. | **ELIMINAR** |

---

## 4. Auditoría de Networking y Conexiones de Red

Búsqueda en todo el código fuente de primitives de red y librerías cliente:
- `Cargo.toml`: Dependencia `ureq = { version = "2.12", features = ["json"] }`.
- Llamadas `ureq::AgentBuilder::new()`:
  - `src/providers/cursor.rs:154`: `POST https://api2.cursor.sh/aiserver.v1.DashboardService/GetCurrentPeriodUsage`.
  - `src/providers/devin.rs:106`: `POST https://server.codeium.com`.
  - `src/providers/grok.rs:50`: `GET https://cli-chat-proxy.grok.com/v1/billing?format=credits`.
  - `src/providers/muse.rs:169`: `POST https://api.meta.ai/muse-code/key`.
  - `src/providers/opencode_go.rs:43`: `GET https://opencode.ai/zen/go/v1/usage`.
- Sockets Locales (IPC):
  - `src/herdr.rs:427`: `std::os::unix::net::UnixStream::connect(&path)` para el socket IPC local de Herdr (`HERDR_SOCKET_PATH`). Este es un socket local del servidor Herdr en la misma máquina, **no tráfico de red externa**.

**Conclusión de Red:** El funcionamiento final para Codex y Agy será estrictamente **LOCAL ONLY** (cero llamadas de red salientes, cero clientes HTTP).

---

## 5. Auditoría de Acceso a Credenciales y Secretos

Ubicación exacta de las lecturas de credenciales prohibidas en el repositorio actual:

1. **Codex (`src/providers/codex.rs`)**:
   - Línea 938: `pub fn auth_path() -> Result<PathBuf> { Ok(codex_home()?.join("auth.json")) }`
   - Línea 961: `pub fn account_id_from_auth(path: &Path) -> Option<String>` deserializa `tokens.account_id` y `tokens.chatgpt_account_id`.
   - Líneas 1108 y 1555: Pruebas unitarias que manipulan cadenas de prueba `access_token: "secret"`.
2. **Cursor (`src/providers/cursor.rs`)**:
   - Línea 450: Consulta directa mediante `rusqlite` a `ItemTable` en `state.vscdb` buscando la clave `cursorAuth/accessToken`.
3. **Muse (`src/providers/muse.rs`)**:
   - Línea 996: `Command::new("/usr/bin/security").args(["find-generic-password", ...])` para extraer credenciales del Keychain de macOS.
4. **Devin (`src/providers/devin.rs`)**:
   - Deserialización de `credentials.toml` para extraer `api_key`.
5. **OpenCode Go (`src/providers/opencode_go.rs`)**:
   - Línea 52: Cabecera `Authorization: Bearer <key>`.
6. **OMP (`src/omp.rs` / `src/providers/omp.rs`)**:
   - Hashing con `Sha256` (`sha2::Sha256`) de identidades de cuentas para mapeo interno.

---

## 6. Auditoría de Procesos Externos (`Command::new`)

| Ubicación | Comando Ejecutado | Argumentos / Propósito | Riesgos Identificados | Mitigación en Fork |
| :--- | :--- | :--- | :--- | :--- |
| `src/providers/codex.rs:165` | `codex` | `app-server --stdio` | Proceso demonio persistente, comunicación JSON-RPC, networking indirecto potencial, llamadas POSIX `pre_exec`. | **ELIMINAR COMPLETAMENTE**. Sustituir por lectura directa del rollout JSONL. |
| `src/process.rs:33` | `sh` | `-c <command>` para ejecutar comandos de wrapper con timeout. | Asume entorno UNIX; riesgo de shell injection si los argumentos contienen comillas no sanitizadas; falla en Windows. | **REFACTORIZAR**: Aislar ejecución directa sin invocación ciega de shell. |
| `src/herdr.rs:700` | `ps` | `-o etimes= -p <pid>` para obtener tiempo de ejecución. | Binario UNIX exclusivo. Falla de inmediato en Windows. | **REEMPLAZAR** por APIs portables o aislar condicionalmente. |
| `src/herdr.rs` (varios) | `herdr` | Llamadas de control al CLI de Herdr (`herdr plugin ...`). | Legítimo cuando Herdr está en PATH. Debe validarse la existencia del binario y aplicar timeouts. | Mantener llamadas CLI documentadas con timeouts estrictos. |
| `src/providers/muse.rs:996` | `/usr/bin/security` | `find-generic-password` para Keychain. | Invasivo, específico de macOS, prompts interactivos molestos. | **ELIMINAR** junto con el módulo de Muse. |

---

## 7. Esquema Real de `token_count.rate_limits` en Codex

Inspección realizada en `tests/fixtures/codex/` y `src/providers/codex.rs`:

1. **Payload en Rollout JSONL (`~/.codex/sessions/**/rollout*.jsonl`)**:
   ```json
   {
     "timestamp": "2026-09-22T13:01:42Z",
     "type": "event_msg",
     "payload": {
       "type": "token_count",
       "rate_limits": {
         "primary": {
           "used_percent": 80.0,
           "window_minutes": 300,
           "resets_at": 1786795200
         },
         "secondary": {
           "used_percent": 31.0,
           "window_minutes": 10080,
           "resets_at": 1787400000
         }
       },
       "info": {
         "last_token_usage": {
           "input_tokens": 1000,
           "cached_input_tokens": 800,
           "cache_write_input_tokens": 100,
           "total_tokens": 25000
         },
         "model_context_window": 100000
       }
     }
   }
   ```
2. **Propiedades Clave Observadas**:
   - `primary`: Ventana corta (~300 minutos / 5 horas) con `used_percent` y timestamp epoch de reinicio `resets_at`.
   - `secondary`: Ventana larga (~10080 minutos / 7 días) con `used_percent` y timestamp epoch de reinicio `resets_at`.
   - `info.last_token_usage`: Tokens de la última interacción para calcular ocupación de contexto y ahorro de caché.
   - `info.model_context_window`: Límite máximo de la ventana de contexto del modelo.
   - `turn_context.payload.model`: Modelo activo (ej. `gpt-5.6-luna`, `gpt-6-astra`).
   - `session_meta.payload`: Contiene `id`, `session_id`, `cwd`, `timestamp`.

---

## 8. Matriz de Contratos: Fuente → Campo → Alcance → Timestamp → Ausencia

Para cumplir estrictamente la condición de salida de la Fase 1 / Paso 0 (*ningún campo «actual» sin origen y alcance definidos*):

| Proveedor / Fuente | Campo Extraído | Alcance Exacto | Timestamp de Origen | Política de Ausencia (Fallback) | Justificación de Invariante |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Codex**<br>`rollout*.jsonl`<br>(`session_meta`) | `session_id`<br>`cwd` | **Sesión** | `session_meta.timestamp` (o mtime de archivo) | Si falta `session_meta`, no se asocia la sesión ni el pane. | La identidad de sesión es la base de toda atribución técnica. |
| **Codex**<br>`rollout*.jsonl`<br>(`turn_context`) | `model` | **Sesión** | Timestamp del evento `turn_context` más reciente | Ausente (`None`). Nunca asumir un modelo por defecto si no hay evento explícito. | Previene atribuir modelos de sesiones previas en ventanas reutilizadas. |
| **Codex**<br>`rollout*.jsonl`<br>(`token_count.info`) | `context_used`<br>`context_limit`<br>`cache_tokens` | **Sesión** | Timestamp del último `token_count` | Ausente (`None`). Nunca mostrar 0 % ni inventar contexto sin evento. | Solo el último evento refleja el estado real del contexto. |
| **Codex**<br>`rollout*.jsonl`<br>(`rate_limits`) | `window_5h`<br>`window_7d` | **Sesión emisora** (No global de cuenta) | Timestamp del evento `token_count` emisor; `resets_at` fija la caducidad | Ausente (`None`). Si el reset expira o el rollout no tiene `rate_limits`, se omite. | **CRÍTICO:** Un rollout no prueba la cuenta activa actual. Solo aplica a la sesión que lo emitió. |
| **Antigravity**<br>`statusLine` IPC<br>(`quota`) | `gemini_5h`<br>`gemini_7d`<br>`third_party_api` | **Sesión / Pane** (asociado a `HERDR_PANE_ID`) | Unix epoch actual (`now_unix`) al recibir el payload en stdin | Ausente (`None`). Si el pool es ambiguo (ej. modelo desconocido con múltiples pools), se omiten las ventanas. | Evita mezclar cuotas de Gemini con cuotas de terceros. |
| **Antigravity**<br>`statusLine` IPC<br>(`context_window`) | `context_tokens`<br>`context_limit`<br>`cache_tokens` | **Sesión / Pane** | `now_unix` de recepción de hook | Ausente (`None`). Si los contadores de caché son 0 y están inactivos, se limpian para evitar falsos positivos. | Reportar solo métricas activas validadas. |
| **Antigravity**<br>`statusLine` IPC<br>(`model`) | `model_name` | **Sesión / Pane** | `now_unix` de recepción de hook | Ausente (`None`). | Identificación precisa del modelo activo reportado por el motor. |
| **Pacing**<br>(Cálculo derivado) | `pace_delta`<br>`time_remaining` | **Ventana de Cuota** (5h o 7d) | Derivado de `resets_at` del evento y `now_unix` del reloj local | Ausente (`None`) si falta `resets_at`, si el tiempo expiró o si transcurrió menos del 5 % de la ventana. | No proyectar ritmos con información temporal insuficiente. |

---

## 9. Política de Atribución y Asociación Pane ↔ Sesión

1. **Orden de Precedencia Estricto para Asociación**:
   - **Nivel 1 (Máxima certeza):** ID de sesión explícito proporcionado por Herdr (`pane.agent_session`).
   - **Nivel 2 (Heurística acotada):** Coincidencia exacta de `session_meta.cwd` con el directorio de trabajo del pane de Herdr **Y** timestamp de inicio de proceso dentro de una ventana de tolerancia justificada **Y** existencia de un **único candidato**.
   - **Nivel 3 (Ambigüedad):** Si existen múltiples archivos de sesión que coinciden con el CWD en la misma ventana de tiempo, **no se realiza ninguna asociación**. Ausencia es siempre preferible a mostrar métricas de otra sesión.
2. **Aislamiento de Cuentas y Estado de Cuota**:
   - Ninguna cuota leída desde un rollout de Codex se promoverá a "cuota global de la cuenta" sin una prueba criptográfica o de autenticación de cuenta activa (que por diseño no existe al no leer `auth.json`).
   - La cuota de Codex se presenta estrictamente vinculada a la sesión y con etiqueta de última observación y tiempo de reinicio.

---

## 10. Conclusiones y Estado de Cumplimiento de Fase 1

La Fase 1 (Discovery) y la fijación de contratos (Paso 0) quedan formalmente concluidas:
- El repositorio está en la rama aislada `custom/codex-agy-only`.
- El diff inicial está registrado y preservado.
- La matriz de contratos fuente → campo → alcance → timestamp → ausencia está completamente definida y libre de campos ambiguos.
- No se ha modificado código de producción todavía.
- El proyecto está en condiciones de proceder al **Paso 1 (Colector Codex Local)** mediante cortes pequeños, pruebas con fixtures sintéticos y verificación con las puertas del repositorio (`cargo fmt`, `cargo test`, `cargo clippy --release`).
