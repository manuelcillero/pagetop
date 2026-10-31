# Breadcrumb component.
breadcrumb_label = Ruta de navegación

# Dialog component.
dialog_close = Cerrar diálogo

# Dropdown component.
dropdown_toggle = Alternar menú desplegable
dropdown_default_title = Menú desplegable

# Form components.
field_required = Este campo es obligatorio

select_language_site_default = Usar el idioma del sitio: { $language }
select_language_placeholder = Elige un idioma...

select_theme_site_default = Usar el tema del sitio: { $theme }
select_theme_placeholder = Elige un tema...

select_timezone_site_default = Usar la zona horaria del sitio: { $timezone }
select_timezone_placeholder = Elige una zona horaria...
select_timezone_current = Zona horaria actual

timezone_region_africa = África
timezone_region_america = América
timezone_region_antarctica = Antártida
timezone_region_arctic = Ártico
timezone_region_asia = Asia
timezone_region_atlantic = Atlántico
timezone_region_australia = Australia
timezone_region_europe = Europa
timezone_region_indian = Índico
timezone_region_pacific = Pacífico
timezone_region_etc = Otras

# Intro component.
intro_default_title = ¡Hola, mundo!
intro_default_slogan = Descubre⚡{ $app }
intro_default_button = Una solución web creada con <strong>PageTop</strong>

intro_pagetop_label = Versión de PageTop en Crates.io
intro_release_label = Publicado
intro_license_label = Licencia

intro_text1 = PageTop es un <strong>entorno de desarrollo web basado en Rust</strong>, pensado para construir soluciones web modulares, extensibles y configurables.
intro_text2 = PageTop reivindica la esencia de la web clásica, renderiza en el servidor (SSR) utilizando componentes <em>HTML-first</em>, CSS y JavaScript, <strong>con el rendimiento y la seguridad de Rust</strong>.

intro_code = Código
intro_have_fun = Programar es crear

# Navbar component.
navbar_toggle = Alternar navegación

# Pager component.
pager_aria_label = Navegación de páginas
pager_previous_label = Anterior
pager_previous_aria_label = Página anterior
pager_next_label = Siguiente
pager_next_aria_label = Página siguiente
pager_goto_label = Saltar a la página
pager_goto_button = Ir
pager_summary = Mostrando { $first }-{ $last } de { $total }

# PoweredBy component.
poweredby_pagetop = Funciona con { $pagetop_link }

# Table component.
table_empty = No hay datos que mostrar

# File uploads (web::Upload and web::Download).
upload_too_large = «{ $file }» supera el tamaño máximo de { $size } por archivo.
upload_extension_not_allowed = «{ $file }» no es de un tipo de archivo permitido ({ $allowed }).
upload_too_many_files = { $limit ->
    [one] Sólo se puede enviar un archivo a la vez.
   *[other] Sólo se pueden enviar { $limit } archivos a la vez.
}
upload_fields_too_large = Los campos del formulario superan el tamaño máximo de { $size }.
upload_body_too_large = El envío supera el tamaño máximo de { $size }.
upload_not_found = El archivo no existe o ya no está disponible.
upload_invalid = El envío del formulario no es válido. Inténtalo de nuevo.
upload_failed = No se ha podido procesar el archivo. Inténtalo más tarde.
