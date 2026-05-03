# CHANGELOG

Este archivo documenta los cambios más relevantes realizados en cada versión. El formato está basado
en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/), y las versiones se numeran siguiendo
las reglas del [Versionado Semántico](https://semver.org/lang/es/).

Resume la evolución del proyecto para usuarios y colaboradores, destacando nuevas funcionalidades,
correcciones, mejoras durante el desarrollo o cambios en la documentación. Cambios menores o
internos pueden omitirse si no afectan al uso del proyecto.

## 0.5.0 (2026-05-03)

PageTop 0.5.0 es la versión más ambiciosa hasta la fecha; concentra un largo periodo de trabajo en
refactorizaciones, nuevas abstracciones y mejoras que sientan las bases para una API estable y
robusta.

Algunos cambios pueden romper la compatibilidad con versiones anteriores. Se recomienda consultar la
[documentación de PageTop](https://docs.rs/pagetop) para actualizar el código a un entorno más
expresivo y mejor preparado para crecer hacia la versión 1.0. Entre estos cambios destacan:

- **Respuestas web completas**: soporte para páginas HTML, redirecciones HTTP, respuestas JSON,
  cookies, y página de bienvenida integrada.
- **API de componentes consolidada**: ciclo de renderizado definitivo con `is_renderable`, manejo de
  errores con `ComponentError` o mensajes de estado con `StatusMessage`/`MessageLevel`.
- **Temas hijo y macros de renderizado**: los temas pueden extenderse entre sí para sobrescribir el
  renderizado de cualquier componente con `render_component!` y `setup_component!`.
- **Nueva acción `AlterMarkup`**: permite a extensiones y temas interceptar y transformar el HTML
  final de cualquier componente antes de entregarlo.
- **Regiones y plantillas en temas**: los componentes `Region` y `Template` formalizan la gestión de
  regiones, respaldados por una API de `Children` e `InRegion` completamente revisada.
- **Sistema de localización refactorizado**: nueva arquitectura interna con API más clara, mejor
  integración en el contexto y soporte robusto para múltiples idiomas.
- **Tipos HTML consolidados**: unidades CSS, clase `Classes`, atributos HTML refactorizados y
  cadenas internas optimizadas con `CowStr`.
- **Nuevas macros y utilidades de API pública**: macro `Getters` para exponer campos de componentes.
- **Configuración tipada**: nuevas opciones de configuración enumeradas para el log y otros
  parámetros del sistema, con una gestión más expresiva y segura.
- **Recursos estáticos y trazabilidad**: gestión de recursos estáticos integrada en el núcleo de
  PageTop y soporte para trazas y registro de eventos desde la propia librería.

## 0.4.0 (2025-09-20)

### Añadido

- (app) Añade manejo de rutas no encontradas
- (context) Añade métodos auxiliares de parámetros
- (util) Añade `indoc` para indentar código bien
- Añade componente `PoweredBy` para copyright

### Cambiado

- (html) Cambia tipos `Option...` por `Attr...`
- (html) Implementa `Default` en `Context`
- (welcome) Crea página de bienvenida desde intro
- (context) Generaliza los parámetros de contexto
- (context) Define un `trait` común de contexto
- Modifica tipos para atributos HTML a minúsculas
- Renombra `with_component` por `add_child`

### Corregido

- (welcome) Corrige giro botón con ancho estrecho
- (welcome) Corrige centrado del pie de página
- Corrige nombre de función en prueba de `Html`
- Corrige doc y código por cambios en Page

### Dependencias

- Actualiza dependencias para 0.4.0

### Documentado

- (component) Amplía documentación de preparación
- Normaliza referencias al nombre PageTop
- Simplifica documentación de obsoletos
- Mejora la documentación de recursos y contexto

### Otros cambios

- (theme) Mejora gestión de regiones en páginas
- (tests) Amplía pruebas para `PrepareMarkup'
- (locale) Mejora el uso de `lookup` / `using`
- (tools) Fuerza pulsar intro para confirmar input
- Unifica conversiones a String con `to_string()`
- Elimina `Render` para usar siempre el contexto

## 0.3.0 (2025-08-16)

### Cambiado

- Redefine función para directorios absolutos
- Mejora la integración de archivos estáticos

### Documentado

- Cambia el formato para la documentación

## 0.2.0 (2025-08-09)

### Añadido

- Añade librería para gestionar recursos estáticos
- Añade soporte a changelog de `pagetop-statics`

### Documentado

- Corrige enlace del botón de licencia en la documentación

### Otros cambios

- Afina Cargo.toml para buscar la mejor categoría

## 0.1.0 (2025-08-06)

- Versión inicial
