# CHANGELOG

Este archivo documenta los cambios más relevantes realizados en cada versión. El formato está basado
en [Keep a Changelog](https://keepachangelog.com/es-ES/1.0.0/), y las versiones se numeran siguiendo
las reglas del [Versionado Semántico](https://semver.org/lang/es/).

Resume la evolución del proyecto para usuarios y colaboradores, destacando nuevas funcionalidades,
correcciones, mejoras durante el desarrollo o cambios en la documentación. Cambios menores o
internos pueden omitirse si no afectan al uso del proyecto.

## 0.4.0 (2025-09-20)

### Añadido

- [app] Añade manejo de rutas no encontradas
- [context] Añade métodos auxiliares de parámetros
- [util] Añade `indoc` para indentar código bien
- Añade componente `PoweredBy` para copyright

### Cambiado

- [html] Cambia tipos `Option...` por `Attr...`
- [html] Implementa `Default` en `Context`
- [welcome] Crea página de bienvenida desde intro
- [context] Generaliza los parámetros de contexto
- [context] Define un `trait` común de contexto
- Modifica tipos para atributos HTML a minúsculas
- Renombra `with_component` por `add_component`

### Corregido

- [welcome] Corrige giro botón con ancho estrecho
- [welcome] Corrige centrado del pie de página
- Corrige nombre de función en prueba de `Html`
- Corrige doc y código por cambios en Page

### Dependencias

- Actualiza dependencias para 0.4.0

### Documentado

- [component] Amplía documentación de preparación
- Normaliza referencias al nombre PageTop
- Simplifica documentación de obsoletos
- Mejora la documentación de recursos y contexto

### Otros cambios

- 🎨 [theme] Mejora gestión de regiones en páginas
- ✅ [tests] Amplía pruebas para `PrepareMarkup'
- 🎨 [locale] Mejora el uso de `lookup` / `using`
- 🔨 [tools] Fuerza pulsar intro para confirmar input
- 💄 Aplica BEM a estilos de bienvenida y componente
- 🎨 Unifica conversiones a String con `to_string()`
- 🔥 Elimina `Render` para usar siempre el contexto

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
