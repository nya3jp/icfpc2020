import { getApiKey } from "./auth";
import { queryServer, startNonRating, queryNonRatingRuns } from "./utils";

const resultsElem = document.getElementById('results') as HTMLElement;
const refreshElem = document.getElementById('refresh') as HTMLButtonElement;
const missingRunElem = document.getElementById('run_missing') as HTMLButtonElement;

const MY_TEAM_ID = '3dfa39ba-93b8-4173-92ad-51da07002f1b';
const OUR_BOTS: Array<string> = [
    'tanakh_super_bot',
];
const TEAM_SIZE = 30;

function startMissingResults(): void {
    refreshElem.disabled = true;
    missingRunElem.disabled = true;
    try {
        let [subIdToTeamName, resultsAtk, resultsDef] = getResults();
        let [currentBots, subIdToBranch, subidToCommit, activeSub] = getOurLatestBots();
        let topPlayers = getOpponents();
        let botIDs = Object.values(currentBots);
        if (!botIDs.includes(activeSub)) {
            botIDs.push(activeSub);
        }

        for (var [oppName, oppSubId] of topPlayers) {
            for (var ourSubId of botIDs) {
                if (ourSubId in resultsAtk && oppSubId in resultsAtk[ourSubId]) {
                    // Already exist
                } else {
                    startNonRating(ourSubId, oppSubId);
                    // Query
                }
                if (ourSubId in resultsDef && oppSubId in resultsDef[ourSubId]) {
                    // Already exist
                } else {
                    startNonRating(oppSubId, ourSubId);
                }
            }
        }
    } finally {
        refreshElem.disabled = true;
        missingRunElem.disabled = false;
        loadResults();
    }
}

function loadResults(): void {
    try {
        let [subIdToTeamName, resultsAtk, resultsDef] = getResults();
        let [currentBots, subIdToBranch, subidToCommit, activeSub] = getOurLatestBots();
        let topPlayers = getOpponents();

        let botIDs = Object.values(currentBots);
        if (!botIDs.includes(activeSub)) {
            botIDs.push(activeSub);
        }

        let head = [];
        for (var sub of botIDs) {
            let name = subIdToBranch[sub];
            if (sub == activeSub) {
                name = "[ACTIVE] " + name;
            }
            const commit = subidToCommit[sub];
            head.push('<th>' + name + ' atk <br />(' + sub + ', ' + commit.substring(0, 6) + ')</th>');
            head.push('<th>' + name + ' def <br />(' + sub + ', ' + commit.substring(0, 6) + ')</th>');
        }

        let rows: Array<string> = [];
        rows.push('<tr><th></th>' + head.join('') + '</tr>');
        for (var [oppName, oppSubId] of topPlayers) {
            let result = "<tr><td>" + oppName + " (" + oppSubId + ")</td>";
            for (var ourSubId of botIDs) {
                if (ourSubId in resultsAtk && oppSubId in resultsAtk[ourSubId]) {
                    let [status, playerKey] = resultsAtk[ourSubId][oppSubId];
                    let url = 'https://icfpcontest2020.github.io/#/visualize?playerkey=' + playerKey;
                    result += '<td class=' + status.toLowerCase() + '><a href="' + url + '">' + status + '</a></td>';
                } else {
                    result += "<td></td>";
                }
                if (ourSubId in resultsDef && oppSubId in resultsDef[ourSubId]) {
                    let [status, playerKey] = resultsDef[ourSubId][oppSubId];
                    let url = 'https://icfpcontest2020.github.io/#/visualize?playerkey=' + playerKey;
                    result += '<td class=' + status.toLowerCase() + '><a href="' + url + '">' + status + '</a></td>';
                } else {
                    result += "<td></td>";
                }
            }
            result += "</tr>";
            rows.push(result);
        }
        resultsElem.innerHTML = rows.join('');
    } finally {
        refreshElem.disabled = false;
        missingRunElem.disabled = false;
    }
}

function getOpponents(): Array<[string, number]> {
    const scores = <Scoreboard>JSON.parse(queryServer('/scoreboard'));
    let submissions: Array<[number, string, number]> = [];
    let oldones: Array<[string, number]> = [];
    for (var team of scores.teams) {
        if (team.team.teamId == MY_TEAM_ID) {
            continue;
        }
        const name = team.team.teamName;
        const score = team.score;
        let latestKey: number = 0;
        for (var k in team.tournaments) {
            if (parseInt(k) > latestKey) {
                latestKey = parseInt(k);
            }
        }
        const subid = team.tournaments[latestKey.toString()].submission.submissionId;
        for (var k in team.tournaments) {
            if (subid == team.tournaments[k].submission.submissionId) {
                continue;
            }
            if (team.tournaments[k].score == 50) {
                oldones.push([name + " (Top in round " + k + ")", team.tournaments[k].submission.submissionId])
            }
            if (team.tournaments[k].score == 46) {
                oldones.push([name + " (Second in round " + k + ")", team.tournaments[k].submission.submissionId])
            }
            if (team.tournaments[k].score == 42) {
                oldones.push([name + " (Third in round " + k + ")", team.tournaments[k].submission.submissionId])
            }
        }
        submissions.push([score, name, subid]);
    }

    let ret: Array<[string, number]> = [];
    submissions.sort((a, b) => b[0] - a[0]);
    for (var [score, name, subid] of submissions.slice(0, TEAM_SIZE)) {
        ret.push([name, subid]);
    }
    return ret.concat(oldones);
}

function getOurLatestBots(): [Record<string, number>, Record<number, string>, Record<number, string>, number] {
    const submissions = <Array<Submission>>JSON.parse(queryServer('/submissions'));
    let currentBots: Record<string, number> = {};
    let subidToBranch: Record<number, string> = {};
    let subidToCommit: Record<number, string> = {};
    let activeSub: number = 0;
    submissions.reverse();
    for (var sub of submissions) {
        subidToCommit[sub.submissionId] = sub.commitHash;
        if (sub.active) {
            activeSub = sub.submissionId;
        }
        if (!sub.branchName) {
            continue;
        }
        if (sub.status != 'Succeeded') {
            continue;
        }
        subidToBranch[sub.submissionId] = sub.branchName;
        if (OUR_BOTS.includes(sub.branchName)) {
            currentBots[sub.branchName] = sub.submissionId;
        }
    }
    return [currentBots, subidToBranch, subidToCommit, activeSub];
}

function getResults(): [Record<number, string>, Record<number, Record<number, [string, number]>>, Record<number, Record<number, [string, number]>>] {
    let games: Array<Game> = [];
    let prevDate = '';
    while (true) {
        const ret = <GamesList>JSON.parse(queryNonRatingRuns(prevDate));
        games = games.concat(ret.games);
        if (ret.hasMore && ret.next) {
            prevDate = ret.next;
            continue;
        }
        break;
    }
    let subidToTeamName: Record<number, string> = {};
    let resultsAtk: Record<number, Record<number, [string, number]>> = {};
    let resultsDef: Record<number, Record<number, [string, number]>> = {};
    for (var game of games) {
        const atkTeamName = game.attacker.team.teamName;
        const atkSubId = game.attacker.submissionId;
        const defTeamName = game.defender.team.teamName;
        const defSubId = game.defender.submissionId;

        if (game.attacker.team.teamId == MY_TEAM_ID && game.defender.team.teamId == MY_TEAM_ID) {
            continue;
        }

        let myTeamSubId: number = 0;
        let oppTeamName: string = '';
        let oppSubId: number = 0;
        let mySide: string = '';
        let results: Record<number, Record<number, [string, number]>> = {};
        let playerKey: number = 0;
        if (game.attacker.team.teamId == MY_TEAM_ID) {
            myTeamSubId = atkSubId;
            oppTeamName = defTeamName;
            oppSubId = defSubId;
            mySide = 'Attacker';
            results = resultsAtk;
            playerKey = game.attacker.playerKey;
        } else {
            myTeamSubId = defSubId;
            oppTeamName = atkTeamName;
            oppSubId = atkSubId;
            mySide = 'Defender';
            results = resultsDef;
            playerKey = game.defender.playerKey;
        }

        subidToTeamName[oppSubId] = oppTeamName;
        let result: string;
        if (!game.winner) {
            result = 'Pending';
        } else if (game.winner == mySide) {
            result = 'Win';
        } else if (game.winner == 'Nobody') {
            result = 'Draw';
        } else {
            result = 'Lose';
        }

        if (!(myTeamSubId in results)) {
            results[myTeamSubId] = {};
        }
        results[myTeamSubId][oppSubId] = [result, playerKey];
    }
    return [subidToTeamName, resultsAtk, resultsDef];
}

interface Submission {
    submissionId: number,
    branchName?: string,
    status: string,
    commitHash: string
    active: boolean,
}

interface Scoreboard {
    teams: Array<TeamScore>
}

interface TeamScore {
    team: Team,
    score: number,
    tournaments: Record<string, Tournament>,
}

interface Tournament {
    submission: TournamentSubmission,
    score: number,
}

interface TournamentSubmission {
    submissionId: number,
}

interface Team {
    teamId: string
    teamName: string
}

interface Player {
    submissionId: number
    team: Team,
    playerKey: number
}

interface Game {
    gameId: string,
    attacker: Player,
    defender: Player,
    winner?: string,
}

interface GamesList {
    hasMore: boolean,
    next?: string,
    games: Array<Game>
}

function init(): void {
    refreshElem.addEventListener('click', loadResults);
    missingRunElem.addEventListener('click', startMissingResults);

    if (getApiKey() != '') {
        loadResults();
    }
}

init();