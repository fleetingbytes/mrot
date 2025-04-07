Feature: Meal Record

    Scenario Outline: New MealRecord from meal and date string
        When I create a meal record from meal <meal> and date <date>
        Then I get the meal record <meal_record>
        Then the meal record has meal <meal>
        Then the meal record has naive date <naive_date>
        Then the meal record has timestamp <timestamp>

        Examples:
            | meal     | date            | meal_record          | naive_date | timestamp  |
            | broccoli | April 6th, 2025 | 1743897600, broccoli | 2025-04-06 | 1743897600 |

    Scenario Outline: New MealRecord from meal and naive date
        When I create a meal record from meal <meal> and naive date <naive_date>
        Then I get the meal record <meal_record>
        Then the meal record has meal <meal>
        Then the meal record has naive date <naive_date>
        Then the meal record has timestamp <timestamp>

        Examples:
            | meal     | meal_record          | naive_date | timestamp  |
            | broccoli | 1743897600, broccoli | 2025-04-06 | 1743897600 |

    Scenario Outline: New MealRecord from meal and timestamp
        When I create a meal record from meal <meal> and timestamp <unquantized_timestamp>
        Then I get the meal record <meal_record>
        Then the meal record has meal <meal>
        Then the meal record has naive date <naive_date>
        Then the meal record has timestamp <quantized_timestamp>

        Examples:
            | meal     | unquantized_timestamp | meal_record          | naive_date | quantized_timestamp |
            | broccoli | 1743897600            | 1743897600, broccoli | 2025-04-06 | 1743897600          |
            | broccoli | 1743912345            | 1743897600, broccoli | 2025-04-06 | 1743897600          |
