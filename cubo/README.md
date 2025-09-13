# 🔥 Cubo con Texturas - Proyecto de Gráficas por Computadora

Un proyecto de renderizado 3D desarrollado en Rust que muestra un cubo texturizado utilizando la API gráfica moderna WGPU. Este proyecto demuestra conceptos fundamentales de gráficas por computadora incluyendo renderizado 3D, mapeo de texturas, shaders y el pipeline gráfico.

![Cubo Texturizado](assets/preview.png)

## 🎯 Características del Proyecto

- **Renderizado 3D en tiempo real** usando WGPU (WebGPU para aplicaciones nativas)
- **Mapeo de texturas** aplicado a geometría 3D
- **Shaders personalizados** escritos en WGSL (WebGPU Shading Language)
- **Pipeline gráfico moderno** con vertex y fragment shaders
- **Manejo de eventos de ventana** para interacción básica
- **Arquitectura modular** fácil de entender y extender

## 🛠️ Tecnologías Utilizadas

- **[Rust](https://www.rust-lang.org/)** - Lenguaje de programación principal
- **[WGPU](https://wgpu.rs/)** - API gráfica moderna y multiplataforma
- **[Winit](https://github.com/rust-windowing/winit)** - Creación y manejo de ventanas
- **[Image](https://github.com/image-rs/image)** - Carga y procesamiento de imágenes
- **[Bytemuck](https://github.com/Lokathor/bytemuck)** - Conversión segura de tipos para GPU
- **[CGMath](https://github.com/rustgd/cgmath)** - Matemáticas para gráficas 3D

## 📋 Prerrequisitos

Antes de ejecutar el proyecto, asegúrate de tener instalado:

1. **Rust** (versión 1.70 o superior)
   ```bash
   # Verificar instalación
   rustc --version
   cargo --version
   ```

2. **Git** (para clonar el repositorio)
   ```bash
   git --version
   ```

3. **Drivers gráficos actualizados** que soporten:
   - DirectX 12 (Windows)
   - Vulkan (Windows/Linux)
   - Metal (macOS)

## 🚀 Instalación y Ejecución

### Paso 1: Clonar el Repositorio
```bash
git clone https://github.com/Josero31/Cubo-con-texturas.git
cd Cubo-con-texturas/cubo
```

### Paso 2: Verificar Dependencias
El archivo `Cargo.toml` incluye todas las dependencias necesarias:
```toml
[dependencies]
wgpu = "0.20"
winit = "0.29"
env_logger = "0.11"
log = "0.4"
cgmath = "0.18"
image = "0.25"
bytemuck = { version = "1.12", features = ["derive"] }
pollster = "0.3"
```

### Paso 3: Compilar y Ejecutar

**⚠️ IMPORTANTE: En PowerShell, usa este comando completo:**
```powershell
# Navegar al directorio del proyecto y ejecutar
Set-Location "C:\graficas x computadora\Cubo-con-texturas\cubo"; cargo run
```

**Alternativamente, puedes usar comandos separados:**
```bash
# Compilar el proyecto (opcional)
cargo build

# Ejecutar en modo debug (recomendado para desarrollo)
cargo run

# Ejecutar en modo release (optimizado)
cargo run --release
```

**Nota:** Asegúrate de estar en el directorio `cubo/` antes de ejecutar `cargo run`.

### Paso 4: Verificar Ejecución Exitosa
Al ejecutar el programa, deberías ver:
```
🔥 INICIANDO PROGRAMA - DEBERÍAS VER UNA VENTANA
🔥 CONFIGURACIÓN COMPLETADA
🔥 TEXTURA CARGADA: 640x427
🔥 PIPELINE CREADO - ¡DEBERÍAS VER EL TRIÁNGULO CON TEXTURA!
🔥 VENTANA REDIMENSIONADA: [dimensiones]
🔥 REDIBUJANDO...
```

## 📁 Estructura del Proyecto

```
cubo/
├── Cargo.toml              # Configuración del proyecto y dependencias
├── README.md               # Este archivo
├── assets/
│   └── texture.jpg         # Textura aplicada al cubo
├── src/
│   ├── main.rs            # Código principal de la aplicación
│   ├── shader.wgsl        # Shaders WGSL (vertex y fragment)
│   ├── main_backup.rs     # Respaldo del código principal
│   ├── main_simple.rs     # Versión simplificada
│   ├── simple_shader.wgsl # Shaders simplificados
│   └── simple_vertex.rs   # Definiciones de vértices simples
└── target/                # Archivos compilados (generado automáticamente)
```

## 🎨 Componentes Técnicos

### Vertex Shader (shader.wgsl)
```wgsl
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}
```

### Fragment Shader (shader.wgsl)
```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
```

### Geometría del Cubo
El cubo está definido con vértices que incluyen:
- **Posiciones 3D** (x, y, z)
- **Coordenadas de textura** (u, v)

## 🔧 Configuración Avanzada

### Variables de Entorno
```bash
# Habilitar logs detallados
set RUST_LOG=debug
cargo run

# Forzar uso de Vulkan (Windows/Linux)
set WGPU_BACKEND=vulkan
cargo run

# Forzar uso de DirectX 12 (Windows)
set WGPU_BACKEND=dx12
cargo run
```

### Modificar la Textura
1. Reemplaza `assets/texture.jpg` con tu imagen
2. Formatos soportados: JPG, PNG, BMP, TIFF
3. Recomendado: resoluciones potencia de 2 (256x256, 512x512, etc.)

## 🐛 Solución de Problemas

### La ventana no aparece
- Verifica que no esté minimizada o detrás de otras ventanas
- Usa **Alt+Tab** para buscar la ventana
- Revisa que los drivers gráficos estén actualizados

### Errores de compilación
```bash
# Limpiar caché y recompilar
cargo clean
cargo build
```

### Errores de textura
- Verifica que `assets/texture.jpg` existe
- Asegúrate de ejecutar desde el directorio `cubo/`
- Confirma que la imagen no esté corrupta

### Rendimiento bajo
- Ejecuta en modo release: `cargo run --release`
- Cierra aplicaciones que consuman GPU
- Verifica la temperatura del sistema

## 📚 Conceptos Aprendidos

Este proyecto demuestra:

1. **Pipeline Gráfico Moderno**
   - Vertex Processing
   - Rasterización
   - Fragment Processing

2. **Mapeo de Texturas**
   - Coordenadas UV
   - Sampling de texturas
   - Filtrado

3. **Shaders Programables**
   - Vertex Shaders
   - Fragment Shaders
   - WGSL Syntax

4. **Matemáticas 3D**
   - Transformaciones
   - Matrices de proyección
   - Coordenadas homogéneas

5. **Arquitectura GPU**
   - Buffers de vértices
   - Bind Groups
   - Command Encoding

## 🎓 Ejercicios Propuestos

1. **Agregar rotación automática al cubo**
2. **Implementar múltiples texturas**
3. **Añadir iluminación básica**
4. **Crear múltiples cubos con diferentes texturas**
5. **Implementar controles de cámara**

## 🤝 Contribuciones

Las contribuciones son bienvenidas. Para contribuir:

1. Fork el repositorio
2. Crea una branch para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la branch (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.

## 👨‍💻 Autor

**Josero31** - [GitHub](https://github.com/Josero31)

## 🙏 Agradecimientos

- Comunidad de Rust por el excelente ecosistema
- Desarrolladores de WGPU por la API moderna
- Recursos educativos de gráficas por computadora

---

**¿Problemas?** Abre un issue en el repositorio o contacta al autor.

**¿Te gustó el proyecto?** ¡Dale una ⭐ en GitHub!