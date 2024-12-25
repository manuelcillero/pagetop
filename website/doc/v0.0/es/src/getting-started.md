# Comenzando

Esta sección te ayudará a conocer PageTop de la manera más rápida posible. Te enseñará a preparar un entorno de desarrollo apropiado para crear una aplicación web sencilla usando PageTop.


# Inicio rápido

Si quieres entrar de lleno en el código de PageTop y ya cuentas con un entorno de Rust operativo puedes seguir leyendo este apartado de "inicio rápido".

En otro caso puedes pasar a la siguiente página para preparar un entorno de Rust desde cero y empezar a programar tu primera aplicación web con PageTop.

<!-- Nota: la configuración para "compilaciones rápidas" se encuentra en la próxima página, por lo que podrías querer leer esa sección primero. -->

## Empieza con los ejemplos

1. Clona el [repositorio de PageTop](https://github.com/manuelcillero/pagetop):

   ```bash
   git clone https://github.com/manuelcillero/pagetop
   ```

2. Cambia a la carpeta recién creada "pagetop":

   ```bash
   cd pagetop
   ```

3. Asegurate de que trabajas con la última versión de PageTop (ya que por defecto se descarga la rama principal de git):

   ```bash
   git checkout latest
   ```

4. Prueba los ejemplos de la [carpeta de ejemplos](https://github.com/manuelcillero/pagetop/tree/latest/examples):

   ```bash
   cargo run --example hello-world
   ```

   Recuerda que cada ejecución pone en marcha un servidor web. Tendrás que abrir un navegador y acceder a la dirección `http://localhost:8088` (según configuración predeterminada) para comprobar el funcionamiento de los servicios web ofrecidos por cada ejemplo. Para detener la ejecución del servidor bastará con pulsar `Ctrl-C` en el terminal.
