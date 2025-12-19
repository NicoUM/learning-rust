# 📊 Monitor de Sistema en Rust

## 📌 Descripción
Aplicación de línea de comandos desarrollada en **Rust** que muestra métricas del sistema en tiempo real, como uso de CPU, memoria, disco y tiempo de actividad.

---

## 1️⃣ Requerimientos Funcionales (MVP)

- Mostrar **uso de CPU**
  - Porcentaje total de uso
- Mostrar **uso de memoria RAM**
  - Memoria total
  - Memoria usada
  - Memoria libre
- Mostrar **uso de disco**
  - Espacio total
  - Espacio usado
  - Espacio disponible
- Mostrar **tiempo de actividad del sistema (uptime)**
- Actualización automática de métricas
  - Intervalo configurable (por defecto: 1 segundo)
- Ejecución desde **línea de comandos (CLI)**

---

## 2️⃣ Requerimientos Técnicos

- Lenguaje: **Rust**
- Compatibilidad:
  - Linux (obligatorio)
  - Windows / macOS (opcional)
- Crates principales:
  - `sysinfo` → obtención de métricas del sistema
  - `clap` → manejo de argumentos CLI
- Manejo correcto de errores usando `Result` y `?`
- Código modular dividido por responsabilidad:
  - `cpu.rs`
  - `memory.rs`
  - `disk.rs`
  - `uptime.rs`

---

## 3️⃣ Interfaz de Usuario

### CLI Básico
- Salida clara y legible en texto
- Secciones separadas por métrica

Ejemplo de salida:

CPU: 23%
MEM: 5.1 GB / 8.0 GB
DISK: 120 GB / 256 GB
UPTIME: 2h 13m


### (Opcional) TUI
- Interfaz en pantalla completa
- Refresco automático
- Barras de progreso
- Uso de `ratatui` u otro crate similar

---

## 4️⃣ Configuración y Argumentos CLI

- `--interval <segundos>`
  - Define el tiempo de refresco
- `--once`
  - Ejecuta una sola vez y termina
- `--no-disk`
  - Omite la información de disco
- `--json`
  - Salida en formato JSON

---

## 5️⃣ Requerimientos de Rendimiento

- Uso bajo de CPU por parte de la aplicación (≤ 2–3%)
- No bloquear el sistema
- Lectura eficiente de métricas
- Evitar asignaciones innecesarias dentro del loop principal

---

## 6️⃣ Requerimientos de Calidad

- Código documentado usando comentarios `///`
- Proyecto sin warnings al compilar
- README con:
  - Descripción del proyecto
  - Instrucciones de instalación
  - Ejemplos de uso
- Tests básicos:
  - Tests de formato de salida
  - Tests de lógica interna (sin depender del sistema real)

---

## 7️⃣ Extensiones Futuras

- Historial de métricas
- Alertas configurables (ej. CPU > 80%)
- Exportación de datos a CSV
- Soporte multiplataforma completo
- Modo daemon / servicio
- Integración con Prometheus

---

## 8️⃣ Criterios de Finalización

El proyecto se considera completo cuando:

- Compila correctamente sin warnings
- Se ejecuta al menos 10 minutos sin errores
- Maneja correctamente la señal `Ctrl+C`
- Documentación mínima completa y actualizada

---

## 🚀 Notas

Este proyecto está pensado como una herramienta de aprendizaje para profundizar en:
- Manejo de recursos del sistema
- Concurrencia y temporización
- Diseño modular en Rust
- Buenas prácticas de CLI


