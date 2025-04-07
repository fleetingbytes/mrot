Feature: Suggest Meals

    Scenario Outline:
        Given an in-memory storage with the records
            | date       | meal               |
            | 1741824000 | rinderbraten       |
            | 1741737600 | bolognese          |
            | 1741651200 | bolognese          |
            | 1741564800 | rougaille saucisse |
            | 1741478400 | rougaille saucisse |
            | 1741392000 | pizza              |
            | 1741305600 | pizza              |
            | 1741219200 | chilli con carne   |
            | 1741132800 | chilli con carne   |
            | 1741046400 | pork filet         |
            | 1740960000 | pork filet         |
            | 1740873600 | lentils            |
            | 1740787200 | lentils            |
            | 1740700800 | confit de canard   |
            | 1740614400 | confit de canard   |
            | 1740528000 | kassler            |
            | 1740441600 | kassler            |
            | 1740355200 | meat balls         |
            | 1740268800 | meat balls         |
            | 1740182400 | hamburgers         |
            | 1740096000 | hamburgers         |
            | 1740009600 | chicken curry      |
            | 1739923200 | chicken curry      |
            | 1739836800 | chicken            |
            | 1739750400 | chicken            |
            | 1739664000 | chinese noodles    |
            | 1739577600 | chinese noodles    |
            | 1739491200 | šunkafleky         |
            | 1739404800 | šunkafleky         |
            | 1739318400 | gratin à la m. o.  |
            | 1739232000 | gratin à la m. o.  |
        When I ask for <number> meal suggestions, ignoring <ignore_list> and ignore period <ignore_period>
        Then I get the meal records <records>

        Examples:
            | number | ignore_list                                    | ignore_period                 | records                                                                            |
            | 0      |                                                | None                          |                                                                                    |
            | 1      |                                                | None                          | 1739318400, gratin à la m. o.                                                      |
            | 2      |                                                | None                          | 1739318400, gratin à la m. o.; 1739491200, šunkafleky                              |
            | 3      |                                                | None                          | 1739318400, gratin à la m. o.; 1739491200, šunkafleky; 1739664000, chinese noodles |
            | 1      | gratin à la m. o.                              | None                          | 1739491200, šunkafleky                                                             |
            | 2      | gratin à la m. o., šunkafleky                  | None                          | 1739664000, chinese noodles; 1739836800, chicken                                   |
            | 3      | gratin à la m. o., šunkafleky, chinese noodles | None                          | 1739836800, chicken; 1740009600, chicken curry; 1740182400, hamburgers             |
            | 0      |                                                | 2025-02-11 through 2025-03-13 |                                                                                    |
            | 1      |                                                | 2025-02-11 through 2025-03-13 |                                                                                    |
            | 2      |                                                | 2025-02-11 through 2025-03-13 |                                                                                    |
            | 3      |                                                | 2025-02-11 through 2025-03-13 |                                                                                    |
            | 1      |                                                | 2025-02-12 through 2025-03-13 |                                                                                    |
            | 1      |                                                | 2025-02-13 through 2025-03-13 | 1739318400, gratin à la m. o.                                                      |
            | 3      |                                                | 2025-02-13 through 2025-03-13 | 1739318400, gratin à la m. o.                                                      |
            | 3      |                                                | 2025-02-13 through 2025-03-12 | 1739318400, gratin à la m. o.; 1741824000, rinderbraten                            |
            | 3      |                                                | 2025-02-14 through 2025-03-12 | 1739318400, gratin à la m. o.; 1741824000, rinderbraten                            |
            | 3      |                                                | 2025-02-15 through 2025-03-12 | 1739318400, gratin à la m. o.; 1739491200, šunkafleky; 1741824000, rinderbraten    |
            | 3      |                                                | 2025-02-16 through 2025-03-12 | 1739318400, gratin à la m. o.; 1739491200, šunkafleky; 1741824000, rinderbraten    |
            | 3      |                                                | 2025-02-17 through 2025-03-12 | 1739318400, gratin à la m. o.; 1739491200, šunkafleky; 1739664000, chinese noodles |
            | 3      |                                                | 2025-02-18 through 2025-03-12 | 1739318400, gratin à la m. o.; 1739491200, šunkafleky; 1739664000, chinese noodles |
            | 3      |                                                | 2025-02-19 through 2025-03-12 | 1739318400, gratin à la m. o.; 1739491200, šunkafleky; 1739664000, chinese noodles |
            | 3      | gratin à la m. o., šunkafleky, chinese noodles | 2025-02-19 through 2025-03-13 | 1739836800, chicken                                                                |
            | 3      | gratin à la m. o., šunkafleky, chicken         | 2025-02-19 through 2025-03-13 | 1739664000, chinese noodles                                                        |
            | 3      | gratin à la m. o., šunkafleky, chicken         | 2025-02-20 through 2025-03-13 | 1739664000, chinese noodles                                                        |
            | 3      | gratin à la m. o., šunkafleky, chicken         | 2025-02-21 through 2025-03-13 | 1739664000, chinese noodles; 1740009600, chicken curry                             |
            | 3      | gratin à la m. o., šunkafleky, chicken curry   | 2025-02-21 through 2025-03-13 | 1739664000, chinese noodles; 1739836800, chicken                                   |
