<div align="center">

<h1>Drust</h1>

<p>Un Sistema de Gestión de Contenidos (CMS) basado en <strong>PageTop</strong> para compartir tu mundo.</p>

[![Licencia](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?label=Licencia&style=for-the-badge)](#-license)
[![Crates.io](https://img.shields.io/crates/v/drust.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/drust)
[![Descargas](https://img.shields.io/crates/d/drust.svg?label=Descargas&style=for-the-badge&logo=transmission)](https://crates.io/crates/drust)

</div>

`Drust` exprime `PageTop` para desarrollar un *Sistema de Gestión de Contenidos* (CMS) básico,
modestamente inspirado en [Drupal](https://www.drupal.org), que permita construir sitios web
dinámicos, manejables y personalizables; y facilite a los usuarios la gestión de una variedad de
contenidos de manera sencilla.


# 📌 Sobre PageTop

[PageTop](https://docs.rs/pagetop) es un entorno de desarrollo que reivindica la esencia de la web
clásica para crear soluciones web SSR (*renderizadas en el servidor*) modulares, extensibles y
configurables, basadas en HTML, CSS y JavaScript.


# ⚡️ Guía rápida

`Drust` requiere una base de datos para funcionar. La aplicación se encarga de ejecutar las
migraciones y cargar los datos mínimos necesarios, pero para crear o borrar la base de datos puedes
usar los scripts `db-create.sh` y `db-delete.sh` que se encuentran en el directorio `tools` del
*workspace*.

## Configuración de `.env`

Para simplificar la configuración, en el directorio `tools` puedes crear un archivo `.env` para
definir las variables de entorno que requieren los scripts para gestionar la base de datos, aunque
su presencia es **opcional**. Si no se encuentra `.env` o carece de ciertos valores, los scripts
solicitarán las variables necesarias para su ejecución.

> **Nota**: Evita usar caracteres especiales como `@`, `#`, `?`, `:` en `DB_PASS` para prevenir
> posibles problemas de interpretación de `DATABASE_URL` en el código.

### Ejemplo de `.env`

```bash
# Sistema de base de datos
DB_SYSTEM="psql"

# Nombre del host
DB_HOST="localhost"

# Puerto de conexión
DB_PORT="5432"

# Nombre de la base de datos
DB_NAME="drust"

# Usuario de la base de datos
DB_USER="drust"

# Contraseña para el usuario de la base de datos
# Evita usar caracteres especiales como '@', '#', '?', ':', ';' o espacios
DB_PASS="password"

# Usuario administrador
DB_ADMIN="postgres"

# Contraseña del usuario administrador
DB_ADMIN_PASS="adminpassword"
```

## Ejecución de los scripts

Asegúrate de que los scripts tienen permisos de ejecución:

```bash
chmod +x db-create.sh db-delete.sh
```

Y ejecuta el script deseado:

```bash
./db-create.sh
```

o

```bash
./db-delete.sh
```


# 🚧 Advertencia

`PageTop` es un proyecto personal que hago por diversión para aprender cosas nuevas. Su API es
inestable y está sujeta a cambios frecuentes. No recomiendo su uso en producción, al menos mientras
no se libere una versión **1.0.0**.


# 📜 Licencia

El código está disponible bajo una doble licencia:

  * **Licencia MIT**
    ([LICENSE-MIT](LICENSE-MIT) o también https://opensource.org/licenses/MIT)

  * **Licencia Apache, Versión 2.0**
    ([LICENSE-APACHE](LICENSE-APACHE) o también https://www.apache.org/licenses/LICENSE-2.0)

Puedes elegir la licencia que prefieras. Este enfoque de doble licencia es el estándar de facto en
el ecosistema Rust.
