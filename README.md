# ICFPC 2020, Team Spacecat

![Spacecat](/images/spacecat.jpg?raw=true)

## Summary

Spacecat is a team competed in [ICFP Programming Contest 2020],
consisting of 9 members: [@chiro], [@draftcode], [@gusmachine], [@kmyk],
[@nya3jp], [@ogiekako], [@phoenixstarhiro], [@shunsakuraba], and [@tanakh].

This repository contains all the code we wrote for the contest.

We used Rust to build [bots to play the space fighting game] for the final
round, TypeScript to build [a Galaxy Pad implementation],
Python, Go, C++, Ruby, OCaml to build support infrastructure and various
utilities.

Note that we decided the team name after the lightning round. Until then our
team name on the system was `???`.

[ICFP Programming Contest 2020]: https://icfpcontest2020.github.io/
[@chiro]: https://github.com/chiro/
[@draftcode]: https://github.com/draftcode/
[@gusmachine]: https://github.com/gusmachine/
[@kmyk]: https://github.com/kmyk/
[@nya3jp]: https://github.com/nya3jp/
[@ogiekako]: https://github.com/ogiekako/
[@phoenixstarhiro]: https://github.com/phoenixstarhiro/
[@shunsakuraba]: https://github.com/shunsakuraba/
[@tanakh]: https://github.com/tanakh/
[bots to play the space fighting game]: #bots
[a Galaxy Pad implementation]: #galaxy-player

## Bots

### super_bot (submitted solution)

Code: [`./tanakh/super_bot`]


The defender first aims to get into orbit. The commands to get to the orbit are usually within a few commands if the accelerate command's limit is enhanced to 2. However, in order to limit the search time, we use simulated annealing to search the command sequence. The cost minimising by simulated annealing is calculated based on the number of steps that can be survived and the position of the last command. Once it reaches the orbit, it performs a split command. The split command is done with parameter life=1 and others are 0. After splitting, it accelerates in either direction. This is because the orbit is changed from the one that just split. If there is a parameter where it can go in the other orbit, the parameter takes precedence. If there is no such a parameter, it moves outward from the center. In this way, the orbit will be enlarged and the chance of being attacked by the attacker is reduced. If it is not in orbit after it moves, it explore the command sequence to get back into orbit. After that, the splitting will be done again, and so on. This would result in many ships with different orbits, and they will be difficult to defeat unless defenders are very efficient.

In order to reduce the amount of things to consider when accelerating, the parameter distribution for the defenders should first set aside 8 for the cool down per step, and then focus the rest on the life for splitting and the energy for accelerating. The ratio of the life to the energy was preferable at 1:1, but in practice, it took about 2.5 accelerates on average for each split, so we use it to be 2:5.

The attacker will also get into orbit first. Once in orbit, it attempts to attack enemies. For all enemies, calculate the maximum amount of damage that would be dealt if the enemy did not make an accelerate command. The damage is calculated using a formula that was known through our research and is used to calculate the exact value of the damage. The attacker wins if it reduces all four of the enemies' parameters to zero, so it gives priority to the enemy that will take as much damage as possible. For the same amount of damage, target the enemy with the least amount of power needed to deal that damage. If the total power of the attack is increased if it uses an accelerate command together, it will also uses an accelerate command at the same time. If it accelerates, its trajectory will shift, so it will return to orbit as quickly as possible.

The attacker's parameter allocation is 1 to life, and after setting aside a minimum amount of energy to use for accelerate, it is allocated to maximum power of laser and cool down per step. Attackers can determine their own parameters by looking at the defender's parameters, so they can change their parameters depending on whether or not the opponent is likely to split. If the defender is likely to split (life > 1), then the continuous firing speed is probably more important than power, so assign about 14 for cool down and the rest for maximum power. If the defender is not likely to split, the power of a single shot is more important than the continuous firing speed, so we assign about 8 to cool down and the rest to maximum power.

[`./tanakh/super_bot`]: ./tanakh/super_bot/

## Infra

### Galaxy Player

Code: [`./infra/play`]

Demo: https://nya3jp.github.io/icfpc2020/

Galaxy Player is our implementation of Galaxy Pad in TypeScript.

[`./infra/play`]: ./infra/play/

#### Features

- Efficient execution of Galaxy assembly
- Automatic number annotation
- Clickable area detection
- Send log analysis
- State forward/backward navigation like web browsers
- State save/restore
- Jump to replay
- Jump to tutorial stages

#### Gallery

<table>
<tr>
<td style="text-align: center">
<img src="https://github.com/nya3jp/icfpc2020/blob/master/images/galaxy-player.png?raw=true">
Galaxy Player UI
</td>
<td style="text-align: center">
<img src="https://github.com/nya3jp/icfpc2020/blob/master/images/galaxy-player-annotate.png?raw=true">
Automatic number annotation
</td>
</tr>
<tr>
<td style="text-align: center">
<img src="https://github.com/nya3jp/icfpc2020/blob/master/images/galaxy-player-detect.png?raw=true">
Clickable area detection
</td>
<td style="text-align: center">
<img src="https://github.com/nya3jp/icfpc2020/blob/master/images/galaxy-player-logs.png?raw=true">
Send log analysis
</td>
</tr>
</table>

### Dashboard

Code: [`./infra/play/src/dash.ts`]

During the contest, the participants are allowed to request non-rating matches
with the bots of their choice. We used this feature to evaluate the strength of
our bots. This dashboard shows the results of those non-rating matches. In
addition to win-and-lose, it shows links to replay the match by using the
official visualizer. (The official visualizer had an undocumented feature that
you can specify a player key of the match.)

![Screenshot](/images/dashboard.png?raw=true)

[`./infra/play/src/dash.ts`]: ./infra/play/src/dash.ts

### Submission System

Code: [`./infra/make_submissions.sh`]

The script is run on every commit by [a GitHub action] to update submission
branches.

[`./infra/make_submissions.sh`]: ./infra/make_submissions.sh
[a GitHub action]: ./.github/workflows/submit.yml

## Support tools

TODO(everyone): Write

### Tester

Code: [`./infra/tester/tester.py`]

The endpoint `/aliens/send` had a hidden feature that you can generate player
keys to test. If you send a modulated value of `[1, 0]`, you'll receive two
player keys to start a match by yourself. If you send a modulated value of `[1,
N]`, you'll receive a player key to start tutorial stages of your choice.

This tester script utilizes this endpoint to run a match with two bots without
submitting the code. There are two modes: One mode runs two bots locally and let
them fight against each other. The other mode runs one bot locally and let it do
the tutorial stages.

[`./infra/tester/tester.py`]: ./infra/tester/tester.py

### interact.py

Code: [`./infra/interact`]

The bot programs are executed on the organizer's environment and need to
interact with an API server with HTTP. The program is build with no network
access, and the third party libraries must be vendored. However, a minimal setup
like [starterkit-rust] needs 150MiB for vendoring.

Instead, we wrote a small Python program that converts stdin and stdout into
HTTP request and response. When invoked with a server URL, a player key, and an
actual bot program, it execs the bot program and reads and writes its stdin and
stdout. The actual bot program writes a modulated request into stdout and then
the Python program makes a POST request with it. The response body is written
back to the bot program's stdin.

Feedback to the organizer: Making an HTTP request can be hard in some languages.
As shown with this program, the bot programs could have used stdio instead of
HTTP without any substantial rule changes. It would be nice to lift this type of
requirements to be fair to all languages.

[`./infra/interact`]: ./infra/interact
[starterkit-rust]: https://github.com/icfpcontest2020/starterkit-rust

### Galaxy Pad Scheme Transpiler

Code: [`./draftcode/interpreter/cmd`]

In order to implement Galaxy Pad, you need to write an evaluator of a functional
language. Since it's a functional language, it's easy to write a transpiler.
This transpiler translates the input into a Scheme program. See the README there
for details.

[`./draftcode/interpreter/cmd`]: ./draftcode/interpreter/cmd

### Bot to reverse-engineer the game rule.

Code: [`./bot/psh/testbot`]

In order to understand the game rule in precise,
we experimented several situations with variation of parameters.
This is the bot to make some expected situation to measure the results.

[`./bot/psh/testbot`]: ./bot/psh/testbot

### Galaxy Relambder

Code: [`./chunjp/decomp`]

To reverse engineer the Galaxy we wrote a program to convert combinator-ful alien codes into human-readable lambda functions. We added annotations to some arithmetic and list functions.

[`./chunjp/decomp`]: ./chunjp/decomp

### Another decompiler of galaxy.txt

Code: [`./psh/decompiler`]

Created after the end of the contest. Decompiled result is in the
[`./psh/decompiled`] directory, including bitmap images extracted from
the galaxy.txt.

[`./psh/decompiler`]: ./psh/decompiler
[`./psh/decompiled`]: ./psh/decompiled

## Reverse Engineered Game Rule.

Some detailed specs of the contest game is hidden, so it needed to be
reverse engineered.

### Game starting.

First, player needs to join the game. According to the given spec, the message
format is:

```
(2, playerKey, (...unknown list...))
```

Here, `unknown list` is the secret keys, which increase the limit of
`heat` parameter (from 64 to 128) and the limit of `thruster` power (from 1 to 2), which are described below.

Note: looks like the range described in the image on solving tic-tac-toe
looks like a typo (not 64, but 128). It is hard coded :1108 in galaxy.txt.

Those keys are found on solving tic-tac-toe game and pelmanism game.

Then, the player receives the game response, with `gameStage == 0`.
Specifically, it includes `staticGameInfo`, including:
- total_turns of the game.
- the role of the player.
- the limits of the player.
  - the limit of the parameter cost.
  - the limit of thruster.
  - the limit of heat.
- the info of map, including the size of the central star and the size of the map, and
- if the player is the attacker, defender's initial parameters.

Given the info, the player needs to set the initial parameter of the player machine. Importantly, the total cost of parameter is limited. The cost is calculated in the following expression:
```
cost = energy + 4 * laserpower + 12 * cooldown + 2 * life
```
Also, the life must be positive.

### For each turn.

The action is made in the following order:
- Calculation of the velocity.
- Calculation of the position.
- Calculation of the damage/heat.

#### Actions.

Each player can take following actions.

##### Accelerate command.

Each machine can be accelerated by the thruster.
It changes the machine's velocity with the given vec2 value.
It consumes `max(abs(ax), abs(ay))` energy. Also, it costs `max(abs(ax), abs(ay)) * 8` to heats.

##### Denotate command.

Each machine can denonate itself, with damaging near area.
The damage is diminished based on the distance between machines:
```
total_energy = energy + laser_power + cooldown + life
distance = max(abs(x - center_x), abs(y - center_y))
damage = max(128*sqrt(log2(total_energy+1)) - 32 * distance, 0)
```
After this command, the machine is gone.
It won't damage the machines with the same role.

##### Shoot command.

Each machine can shoot a laser. It hits the area around the target point,
and the damage is calculated based on the distance and angle between
the shooter and the target point, in adition,
the distance between the target point and the hit machine.

```
distance1 = max(abs(shooter.x - target.x), abs(shooter.y - target.y))
direct_intensity =
  if distance1 == 0 {
    3 * shooting_power
  } else {
    3 * shooting_power - (distance1 - 1)
  }

offset =
  min(min(abs(shooter.x - target.x), abs(shooter.y - target.y)),
      distance1 - min(abs(shooter.x - target.x), abs(shooter.y - target.y)))
intensity =
  max(if distance1 == 0 {
        direct_intensity
      } else {
        direct_intensity - 6 * shooting_power * offset / distance1
      },
      0)

distance2 = max(abs(machine.x - target.x), abs(machine.y - target.y))
damage = max(intensity - 4 ** distance2, 0)
```

Shooter can decide `shooting_power` in the range of `[1, laser_power]`.
And, it costs `shooting_power` to `heat`.

#### Split commad.

The machine can be split into two, by divising its parameters as the player likes. It is not necessary to be even, but the life parameter for each bot must be positive.
The velocity and position are inherited.
Since the next turn, the player can control two (or more) machines
independently at the same time.

#### Heat calculation.

For each turn, for each machine, `heat` parameter is calculated.
It can take the range of `[0, 64]` (or `[0, 128]` if the secret key is given) inclusive.
The heat is initially 0. If the machine takes some actions, they cost to
`heat` parameter as described above. In addition, if the machine receives damages by the opposite player's action, they are also accumulated.
Then, it cools down by the `cooldown` (the third) parameter.

If the final value is in the range, it will be just carried over to the next turn.
If the value overflows the range, the remaining amount is subtracted from
each parameter from the first parameter (thruster energy) to the last parameter (life) in order

#### Machine broken conditions.

The machine is broken when:
- All parameters become `0` (by the above Heat calculation).
- If the current position is too close to the center.
- If the current position is too far from the center.

#### Winning condition.

- An attacker wins if all defender machines are lost within the given turns.
- An defender wins if all attacker machines are lost within the given turns,
  or if at least one machine of the defender is still alive at the last turn.

## Acknowledgments

Fluffy helped the team by meowing, demanding food, and jumping on to the desk.

![Fluffy](/images/fluffy.jpg?raw=true)
