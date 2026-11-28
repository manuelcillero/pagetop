extension-name = Catálogos
extension-description = Administración de los catálogos de la aplicación: listado, alta, edición, desactivación, borrado e historial.

# **< Permisos >************************************************************************************
permission-group = Catálogos
permission-view = Consultar { $catalog }
permission-manage = Gestionar { $catalog }

# **< Campos >**************************************************************************************
field-code = Código
field-name = Nombre
field-weight = Orden
field-active = Activo
field-translations = Traducciones del nombre

help-code = Obligatorio. Único en el catálogo y sin espacios. No se puede cambiar una vez creado.
help-code-fixed = El código no se puede cambiar una vez creado.
help-name = Obligatorio. El nombre que ven los usuarios.
help-weight = Opcional. Posición en las listas: primero los números menores. Vacío equivale a 0.
help-translations = Opcional. El nombre en otros idiomas; donde esté vacío, se usa el nombre de arriba.

value-yes = Sí
value-no = No

# **< Listado >*************************************************************************************
filter-search = Buscar
placeholder-search = Código o nombre (* comodín)
placeholder-search-name = Nombre (* comodín)
filter-status = Estado
status-active = Activos
status-inactive = Inactivos
filter-all = Todos
status-all = Todos

col-actions = Acciones
empty-list = No hay registros.

btn-new = Nuevo
btn-view = Ver
btn-deactivate = Desactivar
btn-reactivate = Reactivar
btn-history = Historial

confirm-delete = ¿Borrar «{ $name }»? No se puede deshacer.

# **< Páginas >*************************************************************************************
title-new = Nuevo registro: { $catalog }
title-edit = Editar registro: { $catalog }
title-view = Registro: { $catalog }
title-history = Historial de { $record }

# **< Resultados >**********************************************************************************
done-created = Se ha dado de alta el registro.
done-updated = Se han guardado los cambios.
done-deactivated = Se ha desactivado el registro.
done-reactivated = Se ha reactivado el registro.
done-deleted = Se ha borrado el registro.

error-internal = No se ha podido completar la operación. Inténtelo más tarde.
error-not-found = El registro no existe.
error-duplicate = Ya hay otro registro con el mismo valor en { $field1 }.
error-duplicate-combination = Ya hay otro registro con la misma combinación de { $count ->
    [2] { $field1 } y { $field2 }
   *[other] { $field1 }, { $field2 } y { $field3 }
}.
error-duplicate-any = Ya hay otro registro con alguno de estos valores.
error-not-a-decimal = { $field } debe ser un número.
error-invalid-characters = { $field } tiene caracteres que no se pueden guardar.
error-conflict = Otra persona ha cambiado este registro desde que lo abrió. Estos son los valores actuales; vuelva a hacer sus cambios.
error-in-use = El registro está en uso y no se puede borrar. Puede desactivarlo.
error-required = { $field } es obligatorio.
error-too-long = { $field } admite como mucho { $max } caracteres.
error-code-spaces = El código no puede tener espacios.
error-not-a-number = { $field } debe ser un número entero.
error-out-of-range = { $field } está fuera del intervalo permitido.
error-invalid-option = { $field } tiene un valor no permitido.

# **< Historial >***********************************************************************************
col-when = Cuándo
col-who = Quién
col-action = Acción
col-changes = Cambios
empty-history = No hay cambios registrados.

action-create = Alta
action-update = Modificación
action-deactivate = Desactivación
action-reactivate = Reactivación
action-delete = Borrado
action-import = Importación

# **< CSV >*****************************************************************************************
btn-export = Exportar CSV
btn-import = Importar CSV
btn-preview = Previsualizar
btn-other-file = Otro archivo

title-import = Importar CSV: { $catalog }
field-import-file = Archivo CSV
help-import-file = Obligatorio. Una fila de cabecera y una fila por registro, separadas por comas o por puntos y comas, en UTF-8. Columnas: { $columns }. Obligatorias: { $required }; una columna que falte conserva los valores actuales. Columnas que identifican cada registro: { $keys }; los registros nuevos se dan de alta y los existentes se modifican. { $limits } Exporte el catálogo desde su listado para obtener un archivo con las columnas correctas.
help-import-limits = Como mucho { $rows } filas y { $size } por archivo.
help-import-limits-rows = Como mucho { $rows } filas por archivo.

col-record = Registro
col-line = Línea
col-result = Resultado
import-create = Se dará de alta.
import-update = Cambiará: { $columns }.
import-unchanged = Sin cambios.
import-summary = { $created } para dar de alta, { $updated } para modificar, { $unchanged } sin cambios y { $invalid } con errores.{ $invalid ->
    [0] {""}
   *[other] {" "}Corrija el archivo y vuelva a subirlo: no se importa nada mientras alguna fila tenga errores.
}
done-imported = Importación terminada: { $created } altas y { $updated } modificaciones.

import-error-encoding = El archivo no está en UTF-8. Vuelva a guardarlo como «CSV UTF-8».
import-error-empty = El archivo está vacío.
import-error-quotes = El archivo tiene unas comillas sin cerrar, desde la línea { $line }.
import-error-too-many = El archivo tiene demasiadas filas: se pueden importar como mucho { $max } de una vez.
import-error-too-large = El archivo es demasiado grande: se pueden importar como mucho { $max } de una vez.
import-error-unknown-column = Columna desconocida «{ $column }».
import-error-missing-column = Falta la columna «{ $column }».
import-error-repeated = El código está repetido en la línea { $line }.
import-error-changed = Los datos han cambiado desde la previsualización. Vuelva a subir el archivo.
