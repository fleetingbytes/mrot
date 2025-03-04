Feature: Parse Date

    For the user to check and practise understanding how strings will be parsed into dates
    the library shall provide a function which would convert a single date or a date range
    from a string into a collection of dates.


    Scenario Outline: Normal date is the day
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date  | intermediate                                      | naive_dates |
            | 2025-03-04 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-04  |

    Scenario Outline: An two_timer interval of one second is always interpreted as 
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date           | intermediate | naive_dates |
            | 23:59:59 2025-03-04 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-04  |
            | 2025-03-04 23:59:59 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-04  |

    Scenario Outline: Implicit time span within the boundaries of one day is the day
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                                  | intermediate                                      | naive_dates |
            | 5 minutes before and after noon 2025-03-01 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-01  |

    Scenario Outline: Implicit time span across one whole day is the day
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                                 | intermediate                                      | naive_dates |
            | 12 hours before and after noon 2025-03-01 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-01  |

    Scenario Outline: Implicit time span of less than a day across the boundaries of a day is the day where the span starts
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                            | intermediate                                      | naive_dates |
            | 1 second before and after 2025-03-01 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-02-28  |

    Scenario Outline: Implicit time span of one whole day across the boundaries of a day is the day where the span starts
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                            | intermediate                                      | naive_dates |
            | 12 hours before and after 2025-03-01 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-02-28  |

    Scenario Outline: Implicit time span of more than one day but less than two days crossing one day boundary is the day where it starts
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                                  | intermediate                                      | naive_dates |
            | 13 hours before and after 2025-03-02 14:00 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-02  |
            | 13 hours before and after 2025-03-02 13:00 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-02  |

    Scenario Outline: Implicit time span of more than one day but less than two days crossing two day boundaries is the day where it starts and the two following days
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                                      | intermediate | naive_dates                        |
            # 43201 s is just one second more than one half of a day (86400 s)
            | 43201 seconds before and after noon 2025-03-01 | intermediate | 2025-02-28, 2025-03-01, 2025-03-02 |

    Scenario Outline: Implicit time span of two full days crossing only one day boundary is the starting day and the following day
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                           | intermediate                                      | naive_dates            |
            | one day before and after 2025-03-02 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-01, 2025-03-02 |

    Scenario Outline: Implicit time span of two full days crossing two boundaries of days is the starting day and the following two days
        When I parse the date "<text_date>"
        Then two_timer's intermediate result is <intermediate>
        Then our own parse result is <naive_dates>

        Examples:
            | text_date                                | intermediate                                      | naive_dates                        |
            | one day before and after noon 2025-03-02 | (2025-03-04T00:00:00, 2025-03-05T00:00:00, false) | 2025-03-01, 2025-03-02, 2025-03-01 |

            #| 86399 seconds before and after 2025-03-01      | 2025-02-28                         |
            #| from 2025-02-28 until 2025-03-01               | 2025-02-28, 2025-03-01             |
            #| from 2025-02-28 until 2025-03-02               | 2025-02-28, 2025-03-01, 2025-03-02 |
