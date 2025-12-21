# MAINTAINERS.md

## Guía para mantenedores de PageTop

Este documento describe **el flujo técnico interno de revisión e integración de contribuciones** en
**PageTop**.

Está dirigido a **mantenedores del proyecto** y **no forma parte de la guía de contribución para
usuarios externos**. Su objetivo es servir como **referencia operativa**, garantizando coherencia,
trazabilidad y preservación de la autoría en un entorno con repositorios espejo.


## 1. Repositorios y principios

PageTop mantiene **un único repositorio oficial**:

  * **Repositorio oficial:** https://git.cillero.es/manuelcillero/pagetop
  * **Repositorio espejo:** https://github.com/manuelcillero/pagetop

### Principios clave

  * El repositorio oficial **es la única fuente de verdad** del historial.
  * **Nunca se realizan *merges* en GitHub**.
  * Toda integración definitiva se realiza en el repositorio oficial.
  * La autoría original debe preservarse siempre.


## 2. Configuración local recomendada

El remoto `github` debe configurarse únicamente para operaciones de lectura (*fetch*), con la URL de
*push* deshabilitada para evitar publicaciones accidentales en el repositorio espejo.

Estado esperado de `git remote -v`:

```text
origin   git@git.cillero.es:manuelcillero/pagetop.git (fetch)
origin   git@git.cillero.es:manuelcillero/pagetop.git (push)
github   git@github.com:manuelcillero/pagetop.git     (fetch)
github   DISABLED                                     (push)
```

Convenciones usadas en este documento:

  * `origin` -> Repositorio oficial
  * `github` -> Repositorio espejo


## 3. Recepción y revisión de Pull Requests

Las PRs externas llegan por GitHub, normalmente contra la rama `main`.

Se asume que el repositorio local está configurado para recuperar PRs de GitHub como referencias
remotas (`refs/pull/<N>/head`):

```bash
git fetch github --prune
git checkout -b pr-123 github/pr/123
```

Antes de integrar:

  * Revisar el código manualmente.
  * Verificar formato, análisis y pruebas:

    ```bash
    cargo fmt
    cargo clippy
    cargo test
    ```

  * Comprobar impacto en documentación.
  * Evaluar coherencia con la arquitectura y el estilo del proyecto.

Los cambios adicionales se solicitan o se aplican explicando claramente el motivo.


## 4. Estrategia de integración

La integración **se realiza siempre en el repositorio oficial** (`origin`).

### 4.1 Estrategia por defecto: *rebase* + *fast-forward*

Esta es la **estrategia estándar y recomendada** en PageTop. Ventajas:

  * conserva los commits originales,
  * preserva la autoría real de cada cambio,
  * mantiene un historial lineal y trazable,
  * facilita auditoría y depuración.

Procedimiento típico:

```bash
git checkout pr-123
git rebase main

# Resolver conflictos si los hay

git checkout main
git merge --ff-only pr-123
```

Si `merge --ff-only` falla, **no se debe continuar**, indica divergencias que deben resolverse antes
de integrar la PR.

### 4.2 Estrategia excepcional: *Squash*

Sólo debe usarse cuando esté justificado:

  * la PR contiene múltiples commits de prueba o ruido,
  * el historial aportado no es significativo,
  * el cambio es pequeño y autocontenido.

En este caso, se debe **preservar explícitamente la autoría**:

```bash
git checkout main
git merge --squash pr-123
git commit --author="Nombre Apellido <email@ejemplo.com>"
```


### 4.3. Publicación en el repositorio oficial

```bash
git push origin main
```

Este *push* representa la **integración definitiva** del cambio en la rama `main`.


## 5. Cierre de la PR y sincronización

Tras integrar el cambio en el repositorio oficial, se cierra manualmente la PR en GitHub con un
mensaje estándar:

```text
Gracias por tu contribución.

Este cambio ha sido integrado en el repositorio oficial en `main` (`<hash>`).
GitHub actúa como repositorio espejo sincronizado.
```


## 6. Principios de mantenimiento

  * Priorizar **claridad y trazabilidad** frente a rapidez.
  * Mantener un historial legible y significativo.
  * Documentar cambios estructurales o públicos.
  * Tratar las contribuciones externas con respeto y transparencia.

---

Este documento puede evolucionar con el proyecto.

No se trata de imponer rigidez, sino de **capturar el conocimiento operativo real** de PageTop como
guía práctica para el mantenimiento.
