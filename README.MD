## What Is This

Haunted Thief the game :)

It is a sokoban puzzle game, where you need to steal treasures form a haunted mansion.
However, when you pick a treasure, a blind monster starts chasing you, altering all the tiles you have
been on.

## How To Run

### DISCLAIMER

It is encouraged that you do not use the code from `main` to experience the game as that branch is
highly unstable. Either build whatever is in the `gold` branch or download a ready build from the
release.

### Prerequisites

* You will need Java runtime that supports Java 20
* For building the game, the minimal gradle version is 8.5 

### If you have JetBrains IDEA

Just open this repostiry as a gradle project and run the `desktop:run` gradle task.

### Otherwise...

Cd into the project directory. And do the following:

* On Windows run `gradlew.bat desktop:run`
* On Linux/MacOs run `./gradlew desktop:run`

## Running Unit Tests

If you have JetBrains IDEA -- you can easily run the tests. Otherwise, use the following
command:

* On Windows run `gradlew.bat test`
* On Linux/MacOs run `./gradlew test`.
