# Guía de contribución a PageTop

Gracias por tu interés en contribuir a **PageTop** 🎉

Este documento describe **cómo participar en el desarrollo del proyecto**, el flujo de trabajo y las
normas que permitan garantizar un historial limpio, trazable y sostenible a largo plazo.

Por favor, léelo completo antes de abrir una *issue* o una *pull request*.


## 1. Repositorios

PageTop mantiene **un único repositorio oficial**:

  * **Repositorio oficial:** https://git.cillero.es/manuelcillero/pagetop
  * **Repositorio espejo:** https://github.com/manuelcillero/pagetop

El repositorio de GitHub actúa como espejo y punto de entrada para:

  * dar mayor visibilidad al proyecto,
  * facilitar la participación de la comunidad,
  * centralizar *issues* y *pull requests* externas.

> ⚠️ **Importante**
> Aunque GitHub permite abrir *pull requests*, **la integración del código se realiza únicamente en
> el repositorio oficial**. El repositorio de GitHub se sincroniza posteriormente para reflejar el
> mismo estado.

En todos los casos, se respeta la **autoría original** de las contribuciones integradas, tanto en el
historial como en la documentación asociada al cambio.


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

Las *pull requests* se abren **en GitHub**, contra la rama `main`. GitHub es el punto de entrada
recomendado para contribuciones externas.

### 3.2 Reglas generales para PRs

  * Cada PR debe abordar **un único objetivo claro**.
  * Mantén el alcance lo más acotado posible.
  * Incluye descripción clara del cambio.
  * Si el PR corrige una *issue*, enlázala explícitamente.
  * Asegúrate de que el código compila y pasa las pruebas.

### 3.3 Revisión y aceptación

Todas las PRs:

  * serán **revisadas manualmente**,
  * pueden recibir comentarios o solicitudes de cambios,
  * **no se integran directamente en GitHub**, ya que la integración se realiza en el repositorio
    oficial para mantener coherencia y trazabilidad.

Una PR aceptada:

  * se integra en el repositorio oficial (Forgejo),
  * respetando **la autoría original del contribuidor**,
  * normalmente mediante **squash merge** para mantener un historial limpio.


## 4. Autoría y atribución

PageTop cuida especialmente la atribución de contribuciones.

  * El **autor original del código se conserva** en el commit final integrado en Forgejo.
  * Aunque el autor no tenga cuenta en Forgejo, su nombre y email quedarán reflejados.
  * En GitHub, cuando es posible, la contribución quedará asociada al usuario original.

Adicionalmente, el mensaje del commit puede incluir líneas `Co-authored-by` cuando proceda.


## 5. Cierre de Pull Requests en GitHub

Una vez que el cambio ha sido integrado en Forgejo:

  * La PR en GitHub se **cerrará manualmente** (no se mergea).
  * Se añadirá un **mensaje estándar de cierre**, indicando:
    * que el cambio ha sido integrado,
    * la referencia al commit o versión,
    * que GitHub es un repositorio espejo.

Ejemplo de mensaje de cierre:

> Este cambio ha sido integrado en el repositorio oficial (Forgejo).
> GitHub actúa como repositorio espejo, por lo que la PR se cierra sin merge.
> Gracias por tu contribución.

Esto garantiza:

  * transparencia,
  * trazabilidad,
  * coherencia entre repositorios.


## 6. Sincronización entre Forgejo y GitHub

Tras integrar cambios en Forgejo:

 * el repositorio de GitHub se **actualiza para reflejar el estado de Forgejo**,
 * el historial de GitHub puede reescribirse para mantener coherencia.

Por este motivo:

  * **no se deben hacer merges “definitivos” en GitHub**,
  * GitHub no debe considerarse fuente de verdad del historial.


## 7. Estilo de código y calidad

  * Sigue el estilo existente del proyecto.
  * Mantén los comentarios claros y precisos.
  * La documentación es parte del código: actualízala cuando sea necesario.
  * Cambios públicos o estructurales deben ir acompañados de documentación.


## 8. Commits

Recomendaciones generales:

  * Mensajes claros y descriptivos.
  * Un commit debe representar una unidad lógica de cambio.
  * En contribuciones externas, el formato exacto del commit puede ajustarse durante la integración.

Durante la integración, los commits pueden ajustarse (rebase, squash o edición de mensajes) para
adaptarse al historial del proyecto.

## 9. Comunicación y respeto

PageTop sigue un enfoque profesional y colaborativo:

  * Sé respetuoso en revisiones y discusiones.
  * Acepta sugerencias técnicas como parte del proceso.
  * Recuerda que todas las contribuciones son revisadas con el objetivo de mejorar el proyecto.


## 10. Dudas

Si tienes dudas sobre el proceso:

  * abre una *issue* de tipo pregunta,
  * o inicia una discusión (si está habilitada).

Gracias por contribuir a **PageTop** 🚀 Cada aportación, grande o pequeña, ayuda a que el proyecto
mejore.
