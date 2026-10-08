# Date formats.
date_format_short = { $month }/{ $day }/{ $year_short }
date_format_medium = { $month }/{ $day }/{ $year }
date_format_long = { $month } { $day }, { $year }

# Time formats.
time_format_short = { $hour }:{ $minute }
time_format_long = { $hour }:{ $minute }:{ $second }
datetime_join = { $date }, { $time }

# Date and time input. `date_input_format` and `time_input_format` are strftime patterns (%d day,
# %m month, %Y four-digit year, %H hour, %M minute), not text: they are used both to show a value
# and to read what the user types. Keep the date one in the same order as `date_format_medium`.
date_input_format = %m/%d/%Y
date_input_hint = mm/dd/yyyy
time_input_format = %H:%M
time_input_hint = hh:mm
input_format_help = Format: { $format }
date_input_invalid = Invalid date: use the format { $format }.
time_input_invalid = Invalid time: use the format { $format }.
datetime_input_invalid = Invalid date and time: use the format { $format }.
datetime_input_nonexistent = That time does not exist in your time zone: the clock moves forward for daylight saving time.

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
