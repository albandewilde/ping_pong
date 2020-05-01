# ping pong
Ping pong discord bot

## Presentation

You can play ping pong with this discord bot !

## Usage

Send `¨ping` and the bot response with `pong`.
Send `¨pong` and the bot response with `ping`.

Some time, the bot miss the ball and you got a point, but some time you miss the
ball and the bot get the point.

You can know your score against the bot with the `¨score` command.

## Implementation

In the code, each time you send him `¨ping` or `¨pong` a random function is started,
there is 1 in 5 chance someone win.  
To know who is the winner, a second random function is started and in that one,
there is 2 in 3 chances you win.

Scores are saved in the `scores.json` file which is at the root of the project.
