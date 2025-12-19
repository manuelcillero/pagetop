# MAINTAINERS.md

## Guía para mantenedores de PageTop

Este documento describe **el flujo técnico interno de revisión e integración de contribuciones** en
**PageTop**.

Está dirigido a **mantenedores del proyecto** y **no forma parte de la guía de contribución para
usuarios externos**. Su objetivo es servir como **referencia operativa**, garantizando coherencia,
trazabilidad y preservación de la autoría en un entorno con repositorios espejo.


## 1. Repositorios y roles

PageTop mantiene **un único repositorio oficial**:

  * **Repositorio oficial:** https://git.cillero.es/manuelcillero/pagetop
  * **Repositorio espejo:** https://github.com/manuelcillero/pagetop

El repositorio de GitHub actúa como espejo y punto de entrada para:

  * dar mayor visibilidad al proyecto,
  * facilitar la participación de la comunidad,
  * centralizar *issues* y *pull requests* externas.

### Principios clave

  * El repositorio oficial **es la única fuente de verdad** del historial.
  * **Nunca se realizan *merges* en GitHub**.


## 2. Configuración local recomendada

Configuración típica de *remotes*:

```bash
git remote -v
```

Ejemplo esperado:

```text
origin   git@git.cillero.es:manuelcillero/pagetop.git (fetch)
origin   git@git.cillero.es:manuelcillero/pagetop.git (push)
github   git@github.com:manuelcillero/pagetop.git    (fetch)
github   git@github.com:manuelcillero/pagetop.git    (push)
```

Convenciones usadas en este documento:

* `origin` -> Oficial
* `github` -> GitHub (espejo)


## 3. Recepción de Pull Requests desde GitHub

Las contribuciones externas llegan como *pull requests* en GitHub, normalmente contra `main`.

### 3.1 Obtener la PR en local

Opción habitual (ejemplo con PR #123):

```bash
git fetch github pull/123/head:pr-123
git checkout pr-123
```

Alternativamente, si la rama del contribuidor es accesible directamente como referencia remota:

```bash
git fetch github
git checkout nombre-de-la-rama
```


## 4. Revisión local

Antes de integrar cualquier cambio:

* Revisar el código manualmente.
* Verificar compilación y pruebas:

```bash
cargo build
cargo test
```

* Comprobar impacto en documentación.
* Evaluar coherencia con la arquitectura y el estilo del proyecto.

Si se requieren cambios:

* comentar en la PR,
* solicitar ajustes,
* o realizar modificaciones locales explicadas claramente.


## 5. Estrategia de integración

La integración **se realiza siempre en el repositorio oficial**.

### 5.1 Estrategia por defecto: *squash merge*

Usada cuando:

* la PR tiene varios commits intermedios,
* los commits no siguen el estilo del proyecto,
* se desea un historial compacto.

Procedimiento típico:

```bash
git checkout main
git pull origin main
git merge --squash pr-123
```

Crear el commit final **preservando la autoría** (ver sección 6).

### 5.2 Cherry-pick selectivo

Usado cuando:

* uno o varios commits son claros y autocontenidos,
* interesa conservar referencias explícitas.

Ejemplo:

```bash
git checkout main
git pull origin main
git cherry-pick -x <commit-sha>
```


## 6. Preservación de la autoría

La autoría original **debe conservarse siempre**.

### 6.1 Commit con autor explícito

Ejemplo:

```bash
git commit --author="Nombre Apellido <email@ejemplo.com>"
```

El mantenedor figura como *committer*; el contribuidor como *author*.

### 6.2 Co-authored-by

Cuando procede, puede añadirse al mensaje del commit:

```text
Co-authored-by: Nombre Apellido <email@ejemplo.com>
```


## 7. Push al repositorio oficial

Una vez integrado:

```bash
git push origin main
```

Este push representa **la integración definitiva**.


## 8. Cierre de la Pull Request en GitHub

Tras integrar el cambio en el repositorio oficial:

* **No se mergea la PR en GitHub**.
* Se cierra manualmente con un mensaje estándar.

Ejemplo recomendado:

```text
Este cambio ha sido integrado en el repositorio oficial.
GitHub actúa como repositorio espejo, por lo que la PR se cierra sin merge.
Gracias por tu contribución.
```


## 9. Sincronización del repositorio oficial a GitHub

El repositorio de GitHub se mantiene como **espejo automático** del repositorio oficial
mediante un **push mirror configurado**.

No se realizan sincronizaciones manuales desde clones locales.

### Consideraciones

  * El repositorio oficial es siempre la **fuente de verdad**.
  * El historial de GitHub puede **reescribirse automáticamente** para reflejar el estado del
    repositorio oficial.
  * Todas las ramas que deban preservarse en GitHub **deben existir también en el repositorio
    oficial**.
  * GitHub no debe usarse como referencia del historial real.


## 10. Principios de mantenimiento

* Priorizar **claridad y trazabilidad** frente a rapidez.
* Mantener un historial legible y significativo.
* Documentar cambios estructurales o públicos.
* Tratar las contribuciones externas con respeto y transparencia.

---

Este documento puede evolucionar con el proyecto.
Su objetivo no es imponer rigidez, sino **capturar el conocimiento operativo real** de PageTop.
