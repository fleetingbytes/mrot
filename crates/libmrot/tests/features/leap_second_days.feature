Feature: Handle Dates From an Implicit Time Span On Days Where There Was a Leap Second

    two_timer is the crate which parses strings into chrono::NaiveDateTime objects.

    two_timer always returns a tuple of start_NaiveDateTime, end_NaiveDateTime,
    and a bool whether the parsed string was an explicit time span.

    When adding meals, explicit time spans, e.g. "from Monday to Wednesday this week"
    are by design rejected by libmrot (Error::TimeSpanNotSupported).
    (Explicit time spans are only allowed when searching and showing recorded meals.)

    Implicit time spans, e.g. "five minutes before and after midnight" are tolerated
    If the chrono::TimeDelta between the end time is less than one day.
    In such case they are understood as the date of the starting time. 

    This is by design, so we don't get tempted to use an explicit time span

    ```
    mrot add tuna --date "from today until one day after tomorrow"
    ```

    instead of multiple dates

    ```
    mrot add tuna --date "today" --date "tomorrow" --date "one day after tomorrow"
    ```

    The handling of timespans and their mapping to multiple days is hard to
    implement properly because two_timer delivers the same result for both of the
    following strings:

    ```
    one day before and after January 2nd, 2026
    from January 1st, 2026 to January 3rd, 2026
    ```

    In both cases the start and end times are 2026-01-01 00:00:00, 2026-01-03 00:00:00.
    While in this case it would be desirable to record a meal on the date of the
    end date, normally, the end date must be excluded because two timer
    considers the end date as the first point in time which is outside the parsed
    time expression. Thus

    ```
    January 1st, 2026
    ```

    will be parsed as 2026-01-01 00:00:00 --- 2026-01-02 00:00:00.
    Here the end time clearly is to be excluded. The meal must only be added on
    the start date.

    The implicit time span expression

    ```
    12 hours before and after noon on January 1st, 2026
    ```

    is also parsed as 2026-01-01 00:00:00 --- 2026-01-02 00:00:00.

    Libmrot cannot tell whether an implicit time span or a regular date expression
    was parsed by two_timer. Libmrot tries to take just the date of the starting
    time and returns Error::DateSpansMoreThanOneDay if the difference between
    the end time and start time is more than one day.

    This is to avoid confusing the user. Implicit time spans are to be avoided.
    The only way libmrot can tell that an implicit time span might have been used
    is if the TimeDelta between start date and end date is anything else than one
    day.

    If it is shorter than one day, it is no problem. This can happen in expressions
    like "Sunday noon". However, if it is longer than one day, the user may
    or may not want to include the end date and we have no way to tell.
    So we reject to record anything rather than record something unexpected.

    The tests in this feature make sure that the check for the time span between
    the end date and the start date being not more than one day is also true on days
    where there was a leap second inserted.

    Scenario Outline: Add meal on a leap second date, show when meal
        Given an empty in-memory storage
        When I add the meal <meal> on the date <text_date> to the storage
        Then the storage, asked when <meal> was recorded, returns <naive_dates>

        Examples:
            | meal   | text_date                                             | naive_dates |
            | weasel | 12 hours before and after noon on December 31st, 1971 | 1971-12-31  |
            | weasel | 12 hours before and after noon on June 30th, 1972     | 1972-06-30  |
            | weasel | 12 hours before and after noon on December 31st, 1972 | 1972-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1973 | 1973-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1974 | 1974-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1975 | 1975-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1976 | 1976-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1977 | 1977-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1978 | 1978-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1979 | 1979-12-31  |
            | weasel | 12 hours before and after noon on June 30th, 1981     | 1981-06-30  |
            | weasel | 12 hours before and after noon on June 30th, 1982     | 1982-06-30  |
            | weasel | 12 hours before and after noon on June 30th, 1983     | 1983-06-30  |
            | weasel | 12 hours before and after noon on June 30th, 1985     | 1985-06-30  |
            | weasel | 12 hours before and after noon on December 31st, 1987 | 1987-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1989 | 1989-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 1990 | 1990-12-31  |
            | weasel | 12 hours before and after noon on June 30th, 1992     | 1992-06-30  |
            | weasel | 12 hours before and after noon on June 30th, 1993     | 1993-06-30  |
            | weasel | 12 hours before and after noon on June 30th, 1994     | 1994-06-30  |
            | weasel | 12 hours before and after noon on December 31st, 1995 | 1995-12-31  |
            | weasel | 12 hours before and after noon on June 30th, 1997     | 1997-06-30  |
            | weasel | 12 hours before and after noon on December 31st, 1998 | 1998-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 2005 | 2005-12-31  |
            | weasel | 12 hours before and after noon on December 31st, 2008 | 2008-12-31  |
            | weasel | 12 hours before and after noon on June 30th, 2012     | 2012-06-30  |
            | weasel | 12 hours before and after noon on June 30th, 2015     | 2015-06-30  |
            | weasel | 12 hours before and after noon on December 31st, 2016 | 2016-12-31  |

