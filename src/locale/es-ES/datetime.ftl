# Date formats.
date_format_short = { $day }/{ $month }/{ $year_short }
date_format_medium = { $day }/{ $month }/{ $year }
date_format_long = { $day } de { $month } de { $year }

# Time formats.
time_format_short = { $hour }:{ $minute }
time_format_long = { $hour }:{ $minute }:{ $second }
datetime_join = { $date }, { $time }

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
