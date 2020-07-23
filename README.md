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

TODO(tanakh): Write

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

### Galaxy Relambder

Code: [`./chunjp/decomp`]

To reverse engineer the Galaxy we wrote a program to convert combinator-ful alien codes into human-readable lambda functions. We added annotations to some arithmetic and list functions using them.

## Acknowledgments

Fluffy helped the team by meowing, demanding food, and jumping on to the desk.

![Fluffy](/images/fluffy.jpg?raw=true)
