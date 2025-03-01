# Meal Rotator

Helps you rotate through the meals you eat and decide what to cook next.

## Why mrot exists

We regularly cook meals from a small repertoir of recipes. Yet, when deciding what to cook next, we struggle to remember which meals we didn't have in a while. The Meal Rotator, or *mrot* for short, helps you keep track of the days passed since you last cooked spaghetti, pizza, or had a steak. The meals with the longest time passed are suggested as candidates what to cook next.

## CLI Concept

* `mrot add spaghetti` records that you've had spaghetti today
* `mrot add pizza --date yesterday` records that you've had a pizza yesterday
* `mrot add steak --date 2024-02-10` records that you've had a steak on February 10 2024
* `mrot add carp --date "tomorrow" --date "day after tomorrow"` records that you plan to have carp tomorrow and on the day after tomorrow

* `mrot what` will show you some meals which you haven't had for the longest time
* `mrot what --number 5` will show you five meals which you haven't had for the longest time
* `mrot what --ignore liver --ignore salad` will show you will show you some meals which you haven't had for the longest time, ignoring liver and salad

* `mrot show` will show the past and next planned meals according to the configuration
* `mrot show "from last Tuesday to next Monday"` will show the recorded or planned meals in the given time range
* `mrot show "this week"` will show the past and future meals in this week
* `mrot when "spaghetti"` will show the past and future dates where spaghetti were recorded
* `mrot remove "from last week to next week"` will remove all meals in the specified time range
* `mrot remove "from last month to the end of this month" --meal "tomato soup"` will remove the specified meal in the specified time range

* `mrot random` will show you one random meal from your past or planned meals

* `mrot config set what number 5` will configure mrot to suggest five oldest meals (default: 3)
* `mrot config set show "from the day before yesterday until tomorrow"` will configure mrot to show the meals planned for the specified range
* `mrot config get what number` will show how many meals is mrot configured to suggest
* `mrot config get show` will show the time in which mrot-show will show meals
* `mrot config ignore add liver` will add liver to the ignore list
* `mrot config ignore remove salad` will remove salad from the ignore list
* `mrot config ignore show` will list the ignored meals
* `mrot config ignore clear` will remove everything from the ignore list
* `mrot config path` will show the path to the config file

* `mrot generate zsh` will generate shell completion for zsh

* `mrot parse-date` will parse a date or a date range and display the result

## Non-Goals

Mrot is intended to record only the prime meal of the day (lunch). This is because in my family the breakfasts and dinners are routinely the same and change only occasionally. You can record or plan multiple meals on a single day, but they are all equivalent. Mrot will not distinguish if a meal was a breakfast, lunch, or dinner. In queries limiting the number of meals shown, e.g. `mrot plan show --number 3`, such meals are in competition with each other and mrot will show only the first of the matches on that day.
