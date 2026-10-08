# Date formats.
date_format_short = { $day }/{ $month }/{ $year_short }
date_format_medium = { $day }/{ $month }/{ $year }
date_format_long = { $day } de { $month } de { $year }

# Time formats.
time_format_short = { $hour }:{ $minute }
time_format_long = { $hour }:{ $minute }:{ $second }
datetime_join = { $date }, { $time }

# Date and time input. `date_input_format` and `time_input_format` are strftime patterns (%d day,
# %m month, %Y four-digit year, %H hour, %M minute), not text: they are used both to show a value
# and to read what the user types. Keep the date one in the same order as `date_format_medium`.
date_input_format = %d/%m/%Y
date_input_hint = dd/mm/aaaa
time_input_format = %H:%M
time_input_hint = hh:mm
input_format_help = Formato: { $format }
date_input_invalid = Fecha no válida: usa el formato { $format }.
time_input_invalid = Hora no válida: usa el formato { $format }.
datetime_input_invalid = Fecha y hora no válidas: usa el formato { $format }.
datetime_input_nonexistent = Esa hora no existe en tu zona horaria: coincide con el adelanto del reloj al horario de verano.

# Relative dates.
relative_today = hoy
relative_years = { $n ->
    [one] { $n } año
   *[other] { $n } años
}
relative_months = { $n ->
    [one] { $n } mes
   *[other] { $n } meses
}
relative_days = { $n ->
    [one] { $n } día
   *[other] { $n } días
}
relative_join_two = { $a } y { $b }
relative_join_three = { $a }, { $b } y { $c }
relative_past = hace { $value }
relative_future = dentro de { $value }

# Start/end date precision.
since_short = desde { $month }
since_medium = desde { $month } de { $year }
since_long = desde el { $day } de { $month } de { $year }
until_short = hasta { $month }
until_medium = hasta { $month } de { $year }
until_long = hasta el { $day } de { $month } de { $year }

# Month names.
month_01 = enero
month_02 = febrero
month_03 = marzo
month_04 = abril
month_05 = mayo
month_06 = junio
month_07 = julio
month_08 = agosto
month_09 = septiembre
month_10 = octubre
month_11 = noviembre
month_12 = diciembre
