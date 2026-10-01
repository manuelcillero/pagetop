# Date formats.
date_format_short = { $month }/{ $day }/{ $year_short }
date_format_medium = { $month }/{ $day }/{ $year }
date_format_long = { $month } { $day }, { $year }

# Time formats.
time_format_short = { $hour }:{ $minute }
time_format_long = { $hour }:{ $minute }:{ $second }
datetime_join = { $date }, { $time }

# Relative dates.
relative_today = today
relative_years = { $n ->
    [one] { $n } year
   *[other] { $n } years
}
relative_months = { $n ->
    [one] { $n } month
   *[other] { $n } months
}
relative_days = { $n ->
    [one] { $n } day
   *[other] { $n } days
}
relative_join_two = { $a } and { $b }
relative_join_three = { $a }, { $b }, and { $c }
relative_past = { $value } ago
relative_future = in { $value }

# Start/end date precision.
since_short = since { $month }
since_medium = since { $month } { $year }
since_long = since { $month } { $day }, { $year }
until_short = until { $month }
until_medium = until { $month } { $year }
until_long = until { $month } { $day }, { $year }

# Month names.
month_01 = January
month_02 = February
month_03 = March
month_04 = April
month_05 = May
month_06 = June
month_07 = July
month_08 = August
month_09 = September
month_10 = October
month_11 = November
month_12 = December
