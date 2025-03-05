Feature: Parse Date

    For the user to check and practise understanding how strings will be parsed into dates
    the library shall provide a function which would convert a single date or a date range
    from a string into a collection of dates.

    Rule: A two_timer interval of one day or less is always interpreted as the start date

        Scenario Outline: 1 s interval, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | noon 2025-03-04                                      | (2025-03-04T12:00:00, 2025-03-04T12:00:01, false) | 2025-03-04  |
                | from noon 2025-03-04 through noon 2025-03-04         | (2025-03-04T12:00:00, 2025-03-04T12:00:01, true)  | 2025-03-04  |

        Scenario Outline: 1 s interval, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | midnight 2025-03-04                                  | (2025-03-05T00:00:00, 2025-03-05T00:00:01, false) | 2025-03-05  |
                | from midnight 2025-03-04 through midnight 2025-03-04 | (2025-03-05T00:00:00, 2025-03-05T00:00:01, true)  | 2025-03-05  |

        Scenario Outline: 1 s interval, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 2025-03-04 23:59:59                                  | (2025-03-04T23:59:59, 2025-03-05T00:00:00, false) | 2025-03-04  |
                | from 2025-03-04 23:59:59 through 2025-03-04 23:59:59 | (2025-03-04T23:59:59, 2025-03-05T00:00:00, true)  | 2025-03-04  |

        Scenario Outline: 2 s interval, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 1 second before and after noon 2025-03-05            | (2025-03-05T11:59:59, 2025-03-05T12:00:01, false) | 2025-03-05  |
                | from 11:59:59 2025-03-05 through noon 2025-03-05     | (2025-03-05T11:59:59, 2025-03-05T12:00:01, true)  | 2025-03-05  |

        Scenario Outline: 2 s interval, across day boundary
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | one second before and after 2025-03-04               | (2025-03-03T23:59:59, 2025-03-04T00:00:01, false) | 2025-03-03  |
                | from 23:59:59 2025-03-03 through midnight 2025-03-03 | (2025-03-03T23:59:59, 2025-03-04T00:00:01, true)  | 2025-03-03  |

        Scenario Outline: 2 s interval, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 1 second before and after 12:00:01pm 2025-03-05      | (2025-03-05T00:00:00, 2025-03-05T00:00:02, false) | 2025-03-05  |
                | from 12pm 2025-03-05 until 12:00:02pm 2025-03-05     | (2025-03-05T00:00:00, 2025-03-05T00:00:02, true)  | 2025-03-05  |

        Scenario Outline: 2 s interval, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 1 second before and after 23:59:59 2025-03-05        | (2025-03-05T23:59:58, 2025-03-06T00:00:00, false) | 2025-03-05  |
                | from 23:59:58 2025-03-05 until midnight 2025-03-05   | (2025-03-05T23:59:58, 2025-03-06T00:00:00, true)  | 2025-03-05  |

        Scenario Outline: almost 1 day, mid
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 43199 seconds before and after noon 2025-03-05       | (2025-03-05T00:00:01, 2025-03-05T23:59:59, false) | 2025-03-05  |
                | from 12:00:01pm 2025-03-05 until 23:59:59 2025-03-05 | (2025-03-05T00:00:01, 2025-03-05T23:59:59, true)  | 2025-03-05  |

        Scenario Outline: almost 1 day, across day boundary
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates |
                | 43199 seconds before and after midnight 2025-03-05 | (2025-03-05T12:00:01, 2025-03-06T11:59:59, false) | 2025-03-05  |
                | from 12:00:01 2025-03-05 until 11:59:59 2025-03-06 | (2025-03-05T12:00:01, 2025-03-06T11:59:59, true)  | 2025-03-05  |

        Scenario Outline: almost 1 day, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 43199 seconds before and after 11:59:59 2025-03-05   | (2025-03-05T00:00:00, 2025-03-05T23:59:58, false) | 2025-03-05  |
                | from 12:00:00pm 2025-03-05 until 23:59:59 2025-03-05 | (2025-03-05T00:00:00, 2025-03-05T23:59:59, true)  | 2025-03-05  |

        Scenario Outline: almost 1 day, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates |
                | 43199 seconds before and after 12:00:01 2025-03-05   | (2025-03-05T00:00:02, 2025-03-06T00:00:00, false) | 2025-03-05  |
                | from 12:00:01pm 2025-03-05 until midnight 2025-03-05 | (2025-03-05T00:00:01, 2025-03-06T00:00:00, true)  | 2025-03-05  |

        Scenario Outline: One day mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates |
                | 2025-03-01                                         | (2025-03-01T00:00:00, 2025-03-02T00:00:00, false) | 2025-03-01  |
                | 12 hours before and after noon 2025-03-01          | (2025-03-01T00:00:00, 2025-03-02T00:00:00, false) | 2025-03-01  |
                | from midnight 2025-02-28 until midnight 2025-03-01 | (2025-03-01T00:00:00, 2025-03-02T00:00:00, true)  | 2025-03-01  |

        Scenario Outline: One day across the day boundary
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates |
                | 12 hours before and after midnight 2025-03-04      | (2025-03-04T12:00:00, 2025-03-05T12:00:00, false) | 2025-03-04  |
                | noon 2025-03-04 until noon 2025-03-05              | (2025-03-04T12:00:00, 2025-03-05T12:00:00, true)  | 2025-03-04  |

    Rule: A two_timer interval of more than one day but not exactly N days is interpreted as the start date plus the end dates of every included full day period

        Scenario Outline: 1 day 1 second, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                    | intermediate                                     | naive_dates            |
                | from noon 2025-03-05 through noon 2025-03-06 | (2025-03-05T12:00:00, 2025-03-06T12:00:01, true) | 2025-03-05, 2025-03-06 |

        Scenario Outline: 1 day 1 second, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                    | intermediate                                     | naive_dates            |
                | from 12pm 2025-03-05 through 12pm 2025-03-06 | (2025-03-05T00:00:00, 2025-03-06T00:00:01, true) | 2025-03-05, 2025-03-06 |

        Scenario Outline: 1 day 1 second, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                      | intermediate                                     | naive_dates            |
                | from 23:59:59 2025-03-05 until 12pm 2025-03-07 | (2025-03-05T23:59:59, 2025-03-07T00:00:00, true) | 2025-03-05, 2025-03-06 |

        Scenario Outline: 1 day 2 seconds, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates            |
                | 43201 seconds before and after noon 2025-03-06       | (2025-03-05T23:59:59, 2025-03-07T00:00:01, false) | 2025-03-05, 2025-03-06 |
                | from 23:59:59 2025-03-05 until 12:00:01pm 2025-03-07 | (2025-03-05T23:59:59, 2025-03-07T00:00:01, true)  | 2025-03-05, 2025-03-06 |

        Scenario Outline: 1 day 2 seconds, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates            |
                | 43201 seconds before and after 12:00:01 2025-03-06 | (2025-03-06T00:00:00, 2025-03-07T00:00:02, false) | 2025-03-06, 2025-03-07 |
                | from 12pm 2025-03-06 until 12:00:02pm 2025-03-07   | (2025-03-06T00:00:00, 2025-03-07T00:00:02, true)  | 2025-03-06, 2025-03-07 |

        Scenario Outline: 1 day 2 seconds, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates            |
                | 43201 seconds before and after 11:59:59 2025-03-06 | (2025-03-05T23:59:58, 2025-03-07T00:00:00, false) | 2025-03-05, 2025-03-06 |
                | from 23:59:58 2025-03-05 until 2025-03-07          | (2025-03-05T23:59:58, 2025-03-07T00:00:00, true)  | 2025-03-05, 2025-03-06 |

        Scenario Outline: 2 days minus 2 seconds, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates            |
                | 86399 seconds before and after noon 2025-03-05     | (2025-03-04T12:00:01, 2025-03-06T11:59:59, false) | 2025-03-04, 2025-03-05 |
                | from 12:00:01 2025-03-04 until 11:59:59 2025-03-06 | (2025-03-04T12:00:01, 2025-03-06T11:59:59, true)  | 2025-03-04, 2025-03-05 |

        Scenario Outline: 2 days minus 2 seconds, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates            |
                | 86399 seconds before and after 23:59:59 2025-03-05 | (2025-03-05T00:00:00, 2025-03-06T23:59:58, false) | 2025-03-05, 2025-03-06 |
                | from 2025-03-05 until 23:59:58 2025-03-06          | (2025-03-05T00:00:00, 2025-03-06T23:59:58, true)  | 2025-03-05, 2025-03-06 |

        Scenario Outline: 2 days minus 2 seconds, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates            |
                | 86399 seconds before and after 12:00:01pm 2025-03-06 | (2025-03-05T00:00:02, 2025-03-07T00:00:00, false) | 2025-03-05, 2025-03-06 |
                | from 12:00:02pm 2025-03-05 until 2025-03-07          | (2025-03-05T00:00:02, 2025-03-07T00:00:00, true)  | 2025-03-05, 2025-03-06 |

        Scenario Outline: 2 days plus 2 seconds, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates                        |
                | 86401 seconds before and after noon 2025-03-05     | (2025-03-04T11:59:59, 2025-03-06T12:00:01, false) | 2025-03-04, 2025-03-05, 2025-03-06 |
                | from 11:59:59 2025-03-04 until 12:00:01 2025-03-06 | (2025-03-04T11:59:59, 2025-03-06T12:00:01, true)  | 2025-03-04, 2025-03-05, 2025-03-06 |

        Scenario Outline: 2 days plus 2 seconds, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                            | intermediate                                      | naive_dates                        |
                | 86401 seconds before and after 12:00:01pm 2025-03-05 | (2025-03-04T00:00:00, 2025-03-06T00:00:02, false) | 2025-03-04, 2025-03-05, 2025-03-06 |
                | from 2025-03-04 until 12:00:02pm 2025-03-06          | (2025-03-04T00:00:00, 2025-03-06T00:00:02, true)  | 2025-03-04, 2025-03-05, 2025-03-06 |

        Scenario Outline: 2 days plus 2 seconds, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                          | intermediate                                      | naive_dates                        |
                | 86401 seconds before and after 23:59:59 2025-03-05 | (2025-03-04T23:59:58, 2025-03-07T00:00:00, false) | 2025-03-04, 2025-03-05, 2025-03-06 |
                | from 23:59:58 2025-03-04 until 2025-03-07          | (2025-03-04T23:59:58, 2025-03-07T00:00:00, true)  | 2025-03-04, 2025-03-05, 2025-03-06 |

        Scenario Outline: 3 days plus 2 seconds, mid-day
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                        | intermediate                                      | naive_dates                                    |
                | 129601 seconds before and after noon 2025-03-05  | (2025-03-03T23:59:59, 2025-03-07T00:00:01, false) | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |
                | 23:59:59 2025-03-03 until 12:00:01 pm 2025-03-07 | (2025-03-03T23:59:59, 2025-03-07T00:00:01, true)  | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |

        Scenario Outline: 3 days plus 2 seconds, start aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                           | intermediate                                      | naive_dates                                    |
                | 129601 seconds before and after 12:00:01 2025-03-05 | (2025-03-04T00:00:00, 2025-03-07T00:00:02, false) | 2025-03-04, 2025-03-05, 2025-03-06, 2025-03-07 |
                | 2025-03-04 until 12:00:02 pm 2025-03-07             | (2025-03-04T00:00:00, 2025-03-07T00:00:02, true)  | 2025-03-04, 2025-03-05, 2025-03-06, 2025-03-07 |

        Scenario Outline: 3 days plus 2 seconds, end aligned
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                           | intermediate                                      | naive_dates                                    |
                | 129601 seconds before and after 11:59:59 2025-03-05 | (2025-03-03T23:59:58, 2025-03-07T00:00:00, false) | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |
                | 23:59:58 2025-03-03 until 2025-03-07                | (2025-03-03T23:59:58, 2025-03-07T00:00:00, true)  | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |

    Rule: A two_timer interval of exactly N days (N > 1) depends on the explicitness of the time interval

        Scenario Outline: 2 days implicit
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                      | intermediate                                      | naive_dates                        |
                | one day before and after 2025-03-04            | (2025-03-03T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-03, 2025-03-04, 2025-03-05 |
                | one day before and after noon 2025-03-04       | (2025-03-03T12:00:00, 2025-03-05T12:00:00, false) | 2025-03-03, 2025-03-04, 2025-03-05 |
                | one day before and after 12:00:01pm 2025-03-04 | (2025-03-03T00:00:01, 2025-03-05T00:00:01, false) | 2025-03-03, 2025-03-04, 2025-03-05 |
                | one day before and after 23:59:59 2025-03-04   | (2025-03-03T23:59:59, 2025-03-05T23:59:59, false) | 2025-03-03, 2025-03-04, 2025-03-05 |

        Scenario Outline: 2 days explicit
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                              | intermediate                                     | naive_dates            |
                | from 2025-03-03 until 2025-03-05                       | (2025-03-03T00:00:00, 2025-03-05T00:00:00, true) | 2025-03-03, 2025-03-04 |
                | from noon 2025-03-03 until noon 2025-03-05             | (2025-03-03T12:00:00, 2025-03-05T12:00:00, true) | 2025-03-03, 2025-03-04 |
                | from 12:00:01pm 2025-03-03 until 12:00:01pm 2025-03-05 | (2025-03-03T00:00:01, 2025-03-05T00:00:01, true) | 2025-03-03, 2025-03-04 |
                | from 23:59:59 2025-03-03 until 23:59:59 2025-03-05     | (2025-03-03T23:59:59, 2025-03-05T23:59:59, true) | 2025-03-03, 2025-03-04 |

        Scenario Outline: 3 days implicit
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                     | intermediate                                      | naive_dates                                    |
                | 36 hours before and after 2025-03-05          | (2025-03-03T12:00:00, 2025-03-06T12:00:00, false) | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |
                | 36 hours before and after noon 2025-03-04     | (2025-03-03T00:00:00, 2025-03-06T00:00:00, false) | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |
                | 36 hours before and after 12:00:01 2025-03-04 | (2025-03-03T00:00:01, 2025-03-06T00:00:01, false) | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |
                | 36 hours before and after 11:59:59 2025-03-05 | (2025-03-03T23:59:59, 2025-03-06T23:59:59, false) | 2025-03-03, 2025-03-04, 2025-03-05, 2025-03-06 |

        Scenario Outline: 3 days explicit
            When I parse the date "<text_date>"
            Then two_timer's intermediate parse result is <intermediate>
            Then our own parse result is <naive_dates>

            Examples:
                | text_date                                              | intermediate                                     | naive_dates                        |
                | from 2025-03-03 through 2025-03-05                     | (2025-03-03T00:00:00, 2025-03-06T00:00:00, true) | 2025-03-03, 2025-03-04, 2025-03-05 |
                | from noon 2025-03-03 until noon 2025-03-06             | (2025-03-03T12:00:00, 2025-03-06T12:00:00, true) | 2025-03-03, 2025-03-04, 2025-03-05 |
                | from 12:00:01pm 2025-03-03 until 12:00:01pm 2025-03-06 | (2025-03-03T00:00:01, 2025-03-06T00:00:01, true) | 2025-03-03, 2025-03-04, 2025-03-05 |
                | from 23:59:59 2025-03-03 until 23:59:59 2025-03-06     | (2025-03-03T23:59:59, 2025-03-06T23:59:59, true) | 2025-03-03, 2025-03-04, 2025-03-05 |
