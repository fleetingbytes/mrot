# Meal Rotator

We regularly cook meals from a small repertoir of recipes. Yet, when deciding what to cook next, we struggle to remember which meals we didn't have in a while. The Meal Rotator, or *mrot* for short, helps you keep track of the days passed since you last cooked spaghetti, pizza, or had a steak. The meals with the longest time passed are suggested as candidates what to cook next.

## CLI Concept

* `mrot add spaghetti` records that you've had spaghetti today
* `mrot add pizza --date yesterday` records that you've had a pizza yesterday
* `mrot add steak --date 2024-02-10` records that you've had a steak on February 10 2024

* `mrot what` will show you some meals which you haven't had for the longest time
* `mrot what --number 5` will show you five meals which you haven't had for the longest time
* `mrot what --ignore liver --ignore salad` will show you will show you some meals which you haven't had for the longest time, ignoring liver and salad

* `mrot plan add "lamb chops" "next Sunday"` will plan lamb chops for the next Sunday
* `mrot plan show` will show the next planned meals according to the configuration
* `mrot plan show --number 4` will show the next four planned meals
* `mrot plan show --days 3` will show the planned meals for the next 3 days
* `mrot plan remove meal "tomato soup"` will remove any tomato soups planned in the future
* `mrot plan remove date "next Thursday"` will remove any planned meals on the next Thursday
* `mrot plan remove until "2024-03-01"` will remove any planned meals from today until March 1st 2024

* `mrot random` will show you one random meal from your past or planned meals

* `mrot config set what number 5` will configure mrot to suggest three oldest meals (default: 3)
* `mrot config set plan number 2` will configure mrot to show up to two planned meals (default: 3)
* `mrot config set plan days 7` will configure mrot to show the planned meals up to 7 days in the future (default: 5)
* `mrot config get what number` will show how many meals is mrot configured to suggest
* `mrot config get plan number` will show how many planned meals is mrot configured to show
* `mrot config get plan days` will show how many days of planned meals is mrot configured to show
* `mrot config ignore add liver` will add liver to the ignore list
* `mrot config ignore remove salad` will remove salad from the ignore list
* `mrot config ignore show` will list the ignored meals
* `mrot config ignore clear` will remove everything from the ignore list
* `mrot config path` will show the path to the config file

* `mrot generate zsh` will generate shell completion for zsh

If you add a meal on a day on which one or more meals are planned, these planned meal are removed and replaced by the added meal.

Running `mrot add` or `mrot what` will check for any planned meals with yesterday's date or older. If such meals are found, they will be converted to normal records, as if they had been added with `mrot add` in the past.

## Non-Goals

Mrot is intended to record only the prime meal of the day (lunch). This is because in my family the breakfasts and dinners are routinely the same and change only seasonally, if at all. You can record or plan multiple meals on a single day, but they are all equivalent. Mrot will not distinguish if a meal was a breakfast, lunch, or dinner. In queries limiting the number of meals shown, e.g. `mrot plan show --number 3`, such meals are in competition with each other and mrot will show only the first of the matches until the limit is reached.
