# OBJETIVO GENERAL

Crear un fork privado/personal de:

https://github.com/levi-qiao/herdr-agent-usage

adaptado exclusivamente a mi caso de uso:

- Herdr como gestor de sesiones/agentes.
- OpenAI Codex CLI.
- Google Antigravity CLI / Agy.
- Visualización de:
  - sesiones activas;
  - agente/modelo;
  - contexto utilizado;
  - tokens;
  - límites/cuotas;
  - ventanas temporales de uso;
  - pacing o relación consumo/tiempo;
  - información útil por pane/Space de Herdr.

El resultado debe conservar las mejores capacidades visuales y de integración con Herdr del proyecto original, pero reducir radicalmente la superficie de acceso del plugin.

La regla arquitectónica principal es:

> El plugin no debe necesitar conocer, leer, almacenar ni transmitir credenciales para cumplir su función.

---

# 1. REPOSITORIO BASE

Usar como base principal:

https://github.com/levi-qiao/herdr-agent-usage

El repositorio está escrito principalmente en Rust y tiene una integración con Herdr más completa que otras alternativas evaluadas.

Antes de modificar nada:

1. Clonar el repositorio.
2. Registrar:
   - commit SHA de origen;
   - fecha;
   - versión/tag si corresponde.
3. Crear una rama específica, por ejemplo:

```bash
git checkout -b custom/codex-agy-only
```

No desarrollar directamente sobre `main`.

Como referencia técnica secundaria usar:

https://github.com/senna-lang/herdr-agent-usage

No sustituir el repositorio base por Senna.

La finalidad principal de Senna es estudiar su estrategia de obtención local de información de Codex, especialmente a partir de:

```text
~/.codex/sessions/
```

y sus archivos `rollout*.jsonl`.

---

# 2. PRINCIPIO DE DISEÑO

La versión final debe soportar únicamente:

```text
Herdr
├── Codex
└── Antigravity / Agy
```

Todo proveedor adicional debe eliminarse del código compilado.

No basta con ocultarlo de la configuración.

Debe eliminarse el código correspondiente siempre que sea razonablemente posible.

Eliminar soporte para:

```text
Claude
Cursor
Grok
Muse
Devin
OpenCode
Pi
OMP
y cualquier otro proveedor
```

---

# 3. MODELO DE SEGURIDAD

El plugin debe funcionar siguiendo un principio de mínimo privilegio.

## Accesos permitidos

### Codex

Preferentemente lectura de:

```text
~/.codex/sessions/**
```

para obtener exclusivamente información como:

- session ID;
- timestamps;
- modelo;
- token counts;
- context window;
- rate limits;
- ventanas de consumo;
- otra metadata útil que ya esté registrada localmente.

Debe estudiarse cómo lo hace `senna-lang/herdr-agent-usage` y adaptar esa aproximación a la arquitectura Rust del fork.

### Antigravity

Usar principalmente el mecanismo de `StatusLine` / payload producido por Antigravity.

La información debe convertirse en snapshots locales mínimos consumibles por el plugin.

### Herdr

Permitir:

- lectura de información de pane;
- Space;
- sesión;
- estado del agente;
- eventos necesarios;
- almacenamiento de configuración propia del plugin.

---

# 4. ACCESOS PROHIBIDOS

La versión final NO debe leer:

```text
~/.codex/auth.json
tokens OAuth
API keys
Keychain
Credential Manager
cookies de navegador
credenciales de Cursor
credenciales de Claude
credenciales de Grok
credenciales de Muse
credenciales de Devin
credenciales de OpenCode
```

Eliminar cualquier código relacionado con:

```text
security find-generic-password
browser cookies
accessToken
oauth
Bearer
Authorization
credential extraction
keychain
```

salvo que una referencia aparezca exclusivamente dentro de documentación histórica o tests específicamente diseñados para comprobar que no se utiliza.

---

# 5. RED

El funcionamiento normal del plugin debe ser:

```text
LOCAL ONLY
```

El plugin no debe realizar peticiones HTTP autenticadas.

Idealmente, no debe realizar ninguna petición HTTP durante runtime.

Eliminar dependencias de networking si dejan de ser necesarias.

Investigar particularmente si puede eliminarse:

```text
ureq
```

o cualquier otra librería HTTP del proyecto.

Si existe una razón técnica real para conservar networking, documentarla antes de mantenerla.

No conservar networking "por si acaso".

---

# 6. ARQUITECTURA OBJETIVO

La arquitectura deseada es aproximadamente:

```text
                       HERDR
                         │
             ┌───────────┴────────────┐
             │                        │
          Codex pane               Agy pane
             │                        │
             ▼                        ▼
      Herdr integration           StatusLine
             │                        │
             ▼                        ▼
       session identity          local snapshot
             │                        │
             ▼                        │
 ~/.codex/sessions/**/*.jsonl         │
             │                        │
             └──────────┬─────────────┘
                        ▼
                 local collectors
                        │
              ┌─────────┼──────────┐
              ▼         ▼          ▼
            model     context     quota
              │         │          │
              └─────────┼──────────┘
                        ▼
                normalized state
                        │
                        ▼
                  Herdr UI
```

Separar claramente:

```text
collectors
domain/state
Herdr integration
UI/rendering
configuration
security boundaries
```

Evitar mezclar adquisición de datos con presentación.

---

# 7. CODEX: ESTRATEGIA DE DATOS

Estudiar primero el funcionamiento actual de:

```text
src/providers/codex*
```

o su equivalente en la versión actual del repositorio.

Después estudiar cómo Senna obtiene información desde:

```text
~/.codex/sessions/**/rollout*.jsonl
```

La estrategia deseada es:

```text
PRIMARY
    rollout local de Codex

OPTIONAL FALLBACK
    codex app-server
```

El fallback solo debe conservarse si aporta información que no pueda obtenerse de forma robusta localmente.

Antes de conservar `codex app-server`, documentar:

1. qué información aporta;
2. por qué no puede obtenerse del rollout;
3. qué proceso se lanza;
4. qué permisos utiliza;
5. qué información entra y sale;
6. si supone networking indirecto.

Preferencia general:

```text
rollout local > app-server > acceso directo autenticado
```

Acceso directo autenticado desde el plugin debe evitarse.

---

# 8. ANTIGRAVITY: ESTRATEGIA DE DATOS

Mantener la aproximación basada en `StatusLine`.

Determinar exactamente qué datos proporciona actualmente Antigravity:

- session ID;
- model;
- context;
- tokens;
- cuota;
- ventanas;
- pool/model quota;
- 5h;
- 7d;
- otros límites disponibles.

El wrapper de StatusLine debe:

1. preservar cualquier StatusLine existente del usuario cuando sea posible;
2. no modificar configuraciones más allá de lo necesario;
3. poder restaurar completamente la configuración original durante uninstall;
4. almacenar únicamente la información necesaria;
5. no almacenar prompts completos;
6. no almacenar credenciales;
7. realizar escrituras atómicas cuando sea posible.

---

# 9. FUNCIONALIDAD DE LEVI QUE SE DEBE CONSERVAR

Evaluar y conservar, siempre que sea compatible con la nueva arquitectura:

- integración con panes de Herdr;
- asociación pane ↔ agent ↔ session;
- agrupación por Space;
- modelo activo;
- contexto usado/disponible;
- quota/rate limits;
- ventanas temporales;
- gauges;
- layouts útiles;
- configuración desde Herdr;
- actualización automática;
- pacing;
- alertas;
- estados de agente;
- session/topic metadata cuando pueda obtenerse sin leer contenido sensible.

No conservar funcionalidad simplemente porque exista.

Cada característica debe justificar:

```text
utilidad
+
fuente de datos
+
nivel de acceso requerido
```

---

# 10. PACING

Conservar especialmente el concepto de `pacing`.

Debe ser posible comparar:

```text
% cuota consumida
vs.
% tiempo transcurrido de la ventana
```

Ejemplo conceptual:

```text
Ventana: 5 h
Tiempo transcurrido: 40 %
Consumo: 70 %

=> consumo adelantado respecto al ritmo sostenible
```

No hace falta replicar exactamente la implementación original si existe una forma más clara y robusta.

Mantener la lógica separada del rendering.

Crear tests unitarios específicos.

---

# 11. PRIVACIDAD DE SESIONES

El plugin puede conocer:

- session ID;
- modelo;
- timestamps;
- uso;
- contexto;
- tokens;
- estado;
- identificadores técnicos necesarios.

Debe evitar almacenar:

- prompt completo;
- respuesta completa;
- transcript completo;
- secrets;
- tokens OAuth;
- API keys.

Si para obtener un "topic" actualmente se inspecciona texto de prompts, evaluar si realmente merece la pena.

Preferencia:

```text
metadata técnica > contenido conversacional
```

Si no puede obtenerse topic sin leer contenido de conversaciones, desactivar la característica inicialmente.

---

# 12. REDUCCIÓN DEL CÓDIGO

Después de eliminar providers innecesarios:

1. eliminar módulos muertos;
2. eliminar imports;
3. eliminar configuración obsoleta;
4. eliminar dependencias Cargo innecesarias;
5. eliminar tests específicos de providers eliminados;
6. eliminar documentación no aplicable;
7. simplificar enums;
8. simplificar routing;
9. simplificar CLI;
10. simplificar configuración.

Ejecutar:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
```

El objetivo no es mantener compatibilidad con providers eliminados.

El objetivo es obtener un fork pequeño y comprensible.

---

# 13. TESTS DE SEGURIDAD

Crear tests o verificaciones automáticas que detecten regresiones.

Como mínimo comprobar que el source/runtime no contiene mecanismos activos relacionados con:

```text
auth.json
accessToken
Authorization
Bearer
find-generic-password
Keychain
browser cookies
cursor-access-token
API key extraction
OAuth token extraction
```

Puede implementarse mediante:

- tests Rust;
- script de auditoría;
- CI;
- combinación de ambos.

Un ejemplo conceptual:

```text
security-audit
├── forbidden_strings
├── forbidden_paths
├── forbidden_network_hosts
├── dependency_check
└── runtime network check
```

---

# 14. AUDITORÍA DE NETWORKING

Realizar búsqueda completa del código para identificar:

```text
http://
https://
ureq
reqwest
TcpStream
UdpSocket
curl
wget
Command::new(...)
sh -c
bash -c
```

Clasificar cada aparición como:

```text
REQUIRED
SAFE/LOCAL
REMOVE
REVIEW
```

El resultado esperado debe ser que el runtime del plugin no necesite acceso de red.

---

# 15. AUDITORÍA DE EJECUCIÓN DE PROCESOS

Buscar todas las ejecuciones de procesos externos.

Especial atención a:

```rust
Command::new(...)
```

Documentar cada proceso ejecutado.

Ejemplos potencialmente legítimos:

```text
codex app-server
herdr
shell para preservar un StatusLine previo
```

Pero no asumir que son necesarios.

Para cada ejecución indicar:

- comando;
- argumentos;
- origen de los argumentos;
- posibilidad de shell injection;
- timeout;
- stdout/stderr;
- datos sensibles potenciales.

Evitar `sh -c` siempre que sea posible.

---

# 16. ESTADO LOCAL

Definir exactamente qué escribe el plugin en disco.

El estado persistido debería contener únicamente algo equivalente a:

```json
{
  "provider": "codex",
  "session_id": "...",
  "model": "...",
  "context_used": 12345,
  "context_limit": 200000,
  "quota": {},
  "updated_at": "..."
}
```

No almacenar:

```text
credentials
prompts completos
responses completas
API tokens
cookies
OAuth tokens
```

Documentar las rutas utilizadas.

---

# 17. UNINSTALL LIMPIO

El fork debe proporcionar uninstall reversible.

Debe eliminar únicamente los elementos creados por el plugin.

Debe restaurar:

- StatusLine previo;
- configuración Herdr modificada;
- hooks propios.

Nunca borrar configuraciones ajenas.

Añadir tests cuando sea viable.

---

# 18. THREAT MODEL

Crear:

```text
docs/THREAT_MODEL.md
```

Debe incluir:

## Assets

- Codex credentials.
- Antigravity credentials.
- sesiones.
- prompts.
- código del usuario.
- archivos locales.
- metadata de uso.

## Trust boundaries

```text
Herdr
plugin
Codex session storage
Antigravity StatusLine
filesystem
external network
```

## Threats principales

- credential exfiltration;
- transcript leakage;
- arbitrary command execution;
- malicious StatusLine input;
- path traversal;
- symlink attacks;
- unsafe JSON parsing;
- shell injection;
- malicious session file;
- dependency compromise;
- accidental telemetry.

## Mitigaciones

Documentar las implementadas.

---

# 19. POLÍTICA DE DEPENDENCIAS

Revisar:

```text
Cargo.toml
Cargo.lock
```

Eliminar dependencias asociadas exclusivamente a providers eliminados.

Ejecutar:

```bash
cargo audit
```

Si está disponible:

```bash
cargo deny check
```

No actualizar todas las dependencias arbitrariamente durante esta primera refactorización.

Primero minimizar.

Después actualizar en un cambio independiente.

---

# 20. CI

Crear o adaptar CI para ejecutar:

```text
cargo fmt --check
cargo clippy
cargo test
cargo build --release
cargo audit
security regression tests
```

Opcionalmente:

```text
cargo deny
```

CI debe fallar si reaparecen mecanismos prohibidos de autenticación/networking.

---

# 21. LICENCIA

El proyecto original utiliza licencia MIT.

Mantener correctamente:

- LICENSE;
- avisos correspondientes;
- atribución necesaria.

Documentar que se trata de un fork/modificación.

---

# 22. NO INSTALAR TODAVÍA EN EL ENTORNO PRINCIPAL

Durante el desarrollo:

NO ejecutar directamente una instalación permanente sobre mi configuración principal de Herdr.

Trabajar primero mediante:

- directorio temporal;
- HOME temporal cuando sea posible;
- fixtures;
- mocks;
- tests;
- entorno de prueba.

Antes de instalar en mi sistema real, presentar una auditoría.

---

# 23. PLAN DE TRABAJO ESPERADO

Seguir estas fases.

## FASE 1 — Discovery

Sin modificar código todavía:

1. inspeccionar repo;
2. mapear arquitectura;
3. identificar providers;
4. identificar networking;
5. identificar credential access;
6. identificar filesystem access;
7. identificar procesos externos;
8. identificar UI;
9. identificar watchers;
10. identificar integración Herdr.

Entregar:

```text
docs/BASELINE_AUDIT.md
```

---

## FASE 2 — Plan

Crear:

```text
docs/REFACTOR_PLAN.md
```

Debe clasificar módulos como:

```text
KEEP
MODIFY
REPLACE
DELETE
```

No empezar refactor grande sin este mapa.

---

## FASE 3 — Provider reduction

Eliminar todos los providers excepto:

```text
Codex
Agy
```

Compilar y ejecutar tests.

---

## FASE 4 — Credential removal

Eliminar:

```text
credential readers
Keychain
cookies
auth files
authenticated HTTP
```

Compilar y ejecutar tests.

---

## FASE 5 — Codex local collector

Adaptar la estrategia inspirada en Senna:

```text
~/.codex/sessions/**/rollout*.jsonl
```

Crear fixtures sintéticos para tests.

No utilizar sesiones reales en tests automáticos.

---

## FASE 6 — Agy StatusLine

Reducir y endurecer la integración StatusLine.

Crear fixtures sintéticos.

---

## FASE 7 — UI

Reconectar:

```text
Codex collector
Agy collector
        ↓
normalized state
        ↓
Herdr UI
```

Conservar las capacidades visuales útiles del proyecto original.

---

## FASE 8 — Security hardening

Implementar:

- security regression tests;
- filesystem boundary tests;
- malformed JSON tests;
- symlink/path tests;
- shell/process review;
- network audit.

---

## FASE 9 — Documentation

Crear como mínimo:

```text
README.md
docs/ARCHITECTURE.md
docs/THREAT_MODEL.md
docs/BASELINE_AUDIT.md
docs/SECURITY.md
docs/DATA_FLOW.md
```

---

# 24. DATA FLOW DOCUMENTATION

Crear diagramas explícitos.

Ejemplo Codex:

```text
Codex
  ↓
rollout.jsonl
  ↓
parser read-only
  ↓
normalized metrics
  ↓
Herdr state
  ↓
UI
```

Ejemplo Agy:

```text
Antigravity
  ↓
StatusLine
  ↓
wrapper
  ↓
minimal snapshot
  ↓
normalized metrics
  ↓
Herdr UI
```

Indicar para cada flecha:

```text
READ
WRITE
EXEC
NETWORK
```

La arquitectura final ideal debería tener:

```text
NETWORK = NONE
```

---

# 25. CRITERIOS DE ACEPTACIÓN

No considerar terminado el trabajo hasta cumplir:

### Funcional

- Codex visible correctamente en Herdr.
- Agy visible correctamente en Herdr.
- Pane/session mapping correcto.
- Modelo visible.
- Context usage visible.
- Quota/rate limits visibles cuando existan.
- Pacing operativo.
- No mezclar métricas entre sesiones.
- Datos obsoletos correctamente invalidados.

### Seguridad

- No lectura de credenciales.
- No Keychain.
- No cookies.
- No API tokens.
- No authenticated HTTP.
- Preferentemente cero networking.
- No almacenamiento de prompts completos.
- No almacenamiento de respuestas completas.
- No secrets en logs.
- No providers adicionales compilados.

### Calidad

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --release
cargo audit
```

Todo debe finalizar correctamente o quedar claramente documentada cualquier excepción.

---

# 26. ENTREGABLE FINAL PARA REVISIÓN HUMANA

Antes de pedirme que instale el plugin, entregar un informe:

```text
FINAL_AUDIT.md
```

con:

## 1. Qué se eliminó

Listado de módulos/providers/dependencias.

## 2. Qué permanece

Listado de componentes.

## 3. Filesystem

Todas las rutas que el binario puede leer/escribir.

## 4. Network

Todas las posibles conexiones de red.

Ideal:

```text
None.
```

## 5. External commands

Todos los procesos externos que puede ejecutar.

## 6. Credentials

Confirmación verificable de que no lee credenciales.

## 7. Persistent state

Qué guarda y dónde.

## 8. Herdr modifications

Qué configuración modifica.

## 9. Antigravity modifications

Qué StatusLine/hooks modifica.

## 10. Uninstall

Qué revierte.

## 11. Tests

Resultado completo.

## 12. Dependency audit

Resultado.

## 13. Riesgos residuales

No afirmar simplemente:

```text
"es seguro"
```

Identificar los riesgos que sigan existiendo.

---

# 27. FORMA DE TRABAJAR

Durante el trabajo:

- No hacer grandes refactors simultáneos sin necesidad.
- Mantener commits pequeños y reversibles.
- Un objetivo conceptual por commit.
- Ejecutar tests después de cada fase.
- No introducir features nuevas fuera de alcance.
- No ampliar a otros proveedores.
- No modificar configuraciones reales del usuario sin necesidad.
- No aceptar silenciosamente comportamientos inseguros del proyecto original.
- Verificar el código, no confiar únicamente en README o SECURITY.md.

Cuando haya discrepancias entre documentación y comportamiento real:

```text
EL CÓDIGO ES LA FUENTE DE VERDAD.
```

---

# 28. COMMITS SUGERIDOS

Una secuencia deseable sería aproximadamente:

```text
chore: establish baseline audit
refactor: remove unsupported providers
refactor: simplify provider routing to codex and agy
security: remove credential access paths
security: remove authenticated network clients
refactor: implement local codex rollout collector
refactor: harden agy statusline collector
refactor: simplify normalized usage state
feat: reconnect herdr usage UI
test: add security regression suite
docs: add threat model and data flow
docs: add final security audit
```

No es obligatorio usar exactamente estos nombres.

---

# 29. DECISIONES QUE NO DEBE TOMAR AUTOMÁTICAMENTE

Antes de introducir cualquiera de estos comportamientos, detenerse y documentar el motivo:

- volver a leer credenciales;
- introducir API keys;
- consultar APIs externas;
- añadir telemetría;
- subir información;
- reinstaurar otro provider;
- enviar prompts/transcripts;
- modificar configuración global ajena;
- introducir servicios persistentes adicionales;
- añadir dependencias grandes.

---

# 30. RESULTADO ESPERADO

Quiero terminar con un plugin conceptualmente similar a:

```text
herdr-usage-personal
│
├── codex
│   └── local rollout reader
│
├── agy
│   └── statusline collector
│
├── herdr
│   ├── panes
│   ├── sessions
│   ├── spaces
│   └── events
│
├── metrics
│   ├── context
│   ├── quota
│   └── pacing
│
├── ui
│   ├── sidebar
│   ├── gauges
│   └── settings
│
└── security
    ├── no credentials
    ├── no telemetry
    ├── no authenticated HTTP
    └── regression tests
```

Debe ser más pequeño, más auditable y específico para mi flujo de trabajo que cualquiera de los dos repositorios originales.

La prioridad es:

```text
1. Seguridad y control
2. Corrección de asociación session ↔ pane
3. Fiabilidad de métricas
4. Buena integración visual con Herdr
5. Bajo mantenimiento
6. Nuevas funcionalidades
```

Empieza realizando únicamente la FASE 1 y FASE 2.

No implementes todavía los cambios grandes.

Primero entrégame:

```text
BASELINE_AUDIT.md
REFACTOR_PLAN.md
```

junto con un resumen conciso de las decisiones arquitectónicas que propones y los riesgos que hayas encontrado.