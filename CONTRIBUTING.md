# Guía de contribución a PageTop

Gracias por tu interés en contribuir a **PageTop** 🎉

Este documento describe **cómo participar en el desarrollo del proyecto**, el flujo de trabajo y las
normas que permitan garantizar un historial limpio, trazable y sostenible a largo plazo.

Por favor, léelo completo antes de abrir una *issue* o una *pull request*.


## 1. Repositorios

PageTop mantiene **un único repositorio oficial**:

  * **Repositorio oficial:** https://git.cillero.es/manuelcillero/pagetop
  * **Repositorio espejo:** https://github.com/manuelcillero/pagetop

> ⚠️ **Importante**
> Aunque GitHub permite abrir *issues* y *pull requests*, **la integración del código se realiza
> únicamente en el repositorio oficial**. GitHub actúa como repositorio espejo que se sincroniza
> automáticamente para reflejar el mismo estado.


## 2. Issues (incidencias, propuestas, preguntas)

Antes de abrir una *issue* **en GitHub**:

  * comprueba que no exista ya una similar,
  * describe claramente el problema o propuesta,
  * incluye pasos de reproducción si se trata de un *bug*,
  * indica versión, entorno y contexto cuando sea relevante.

Las *issues* se usan para:

  * informar de errores,
  * propuestas de mejora,
  * discusión técnica previa a cambios relevantes.


## 3. Pull Requests (PRs)

### 3.1 Dónde abrirlas

Las *pull requests* se abren **en GitHub**, normalmente contra la rama `main`. GitHub es el punto de
entrada recomendado para contribuciones externas.

### 3.2 Reglas generales para PRs

  * Cada PR debe abordar **un único objetivo claro**.
  * Mantén el alcance lo más acotado posible.
  * Incluye descripción clara del cambio.
  * Si el PR corrige una *issue*, enlázala explícitamente.
  * Asegúrate de que el código compila y pasa las pruebas.

### 3.3 Revisión y aceptación

Todas las PRs son **revisadas manualmente** y pueden recibir comentarios o solicitudes de cambios.

Las PRs aceptadas se integran en el repositorio oficial, nunca directamente en GitHub, preservando
siempre la **autoría original** del contribuidor.


### 3.4. Cierre de Pull Requests y sincronización

Una vez que el cambio ha sido integrado en el repositorio oficial:

  * La PR en GitHub se **cierra manualmente**.
  * Se añade un **mensaje estándar de cierre** indicando que el cambio ha sido integrado.
  * El repositorio de GitHub **se sincroniza automáticamente** como espejo.


## 4. Estilo de código y calidad

  * Sigue el estilo existente del proyecto.
  * Mantén los comentarios claros y precisos.
  * La documentación es parte del código: actualízala cuando sea necesario.
  * Cambios públicos o estructurales deben ir acompañados de documentación.


## 5. Commits

PageTop usa la especificación **gitmoji** para los mensajes de *commit*. El formato recomendado es:

  `<propósito> (ámbito opcional): <mensaje>`

Ejemplos:

  * 📝 Actualiza la guía de contribución
  * ✨ (locale): Refactoriza sistema de localización
  * ♻️ (bootsier): Simplifica asignación de clases

El emoji puede usarse en formato Unicode o como *shortcode*, por ejemplo `:sparkles:` en vez de ✨.

Consulta la especificación oficial en https://gitmoji.dev/specification

Durante la integración, los *commits* pueden ajustarse para adaptarse al historial del proyecto.

Un *commit* debe representar una unidad lógica de cambio. Usa mensajes claros y descriptivos.


## 6. Comunicación y respeto

PageTop sigue un enfoque profesional y colaborativo:

  * Sé respetuoso en revisiones y discusiones.
  * Acepta sugerencias técnicas como parte del proceso.
  * Recuerda que todas las contribuciones son revisadas con el objetivo de mejorar el proyecto.

Si tienes dudas sobre el proceso, abre una *issue* de tipo pregunta para tratar la cuestión en
comunidad.

---

Gracias por contribuir a **PageTop** 🚀 Cada aportación contribuye a mejorar el proyecto.
