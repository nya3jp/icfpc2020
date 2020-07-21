# ICFPC 2020, Team Spacecat

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

Galaxy Player is our implementation of Galaxy Pad written in TypeScript.
[A live demo is available](https://nya3jp.github.io/icfpc2020/).

![Screenshot](/images/galaxy-player.png?raw=true)

[`./infra/play`]: ./infra/play/

### Dashboard

Code: [`./infra/play/src/dash.ts`]

TODO(draftcode): Write

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
