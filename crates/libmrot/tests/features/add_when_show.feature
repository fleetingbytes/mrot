Feature: Add meal

    Scenario: Add meal
        Given a storage
        Given some dates
        Given a meal
        When I add the meal on those dates to the storage
        Then the storage contains these records
