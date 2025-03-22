Feature: Suggest Meals

    Scenario Outline: No meal from empty storage
        Given an in-memory storage with the records
            | date       | meal               |
        When I ask for a random meal
        Then I get no meal record

    Scenario Outline: Only one option
        Given an in-memory storage with the records
            | date       | meal               |
            | 1741824000 | rinderbraten       |
        When I ask for a random meal
        Then I get the meal <meal>

        Examples:
            | meal         |
            | rinderbraten |

    Scenario Outline: More than one option
        Given an in-memory storage with the records
            | date       | meal               |
            | 1741824000 | rinderbraten       |
            | 1741737600 | bolognese          |
        When I ask for a random meal
        Then the meal is one of <possibilities>

        Examples:
            | possibilities           |
            | rinderbraten, bolognese |

