import { getApiKey } from "./auth";
import { queryServer, startNonRating } from "./utils";

const resultsElem = document.getElementById('results') as HTMLElement;
const refreshElem = document.getElementById('refresh') as HTMLButtonElement;
const missingRunElem = document.getElementById('run_missing') as HTMLButtonElement;

function startMissingResults(): void {
    refreshElem.disabled = true;
    missingRunElem.disabled = true;
    try {
        let [subIdToTeamName, resultsAtk, resultsDef] = getResults();
        let [currentBots, subIdToBranch, subIdToCommit] = getOurLatestBots();
        let topPlayers = getOpponents().slice(0, 10);
        let ourBots = ['bot_kimiyuki', 'bot_psh_testbot', 'tanakh_super_bot'];

        for (var [oppName, oppSubId] of topPlayers) {
            for (var ourBotName of ourBots) {
                const ourSubId = currentBots[ourBotName];
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
        let [currentBots, subIdToBranch, subidToCommit] = getOurLatestBots();
        let topPlayers = getOpponents().slice(0, 10);
        let ourBots = ['bot_kimiyuki', 'bot_psh_testbot', 'tanakh_super_bot'];

        let head = [];
        for (var name of ourBots) {
            head.push('<th>' + name + ' atk <br />(' + currentBots[name] + ', ' + subidToCommit[currentBots[name]].substring(0, 6) + ')</th>');
            head.push('<th>' + name + ' def <br />(' + currentBots[name] + ', ' + subidToCommit[currentBots[name]].substring(0, 6) + ')</th>');
        }

        let rows: Array<string> = [];
        rows.push('<tr><th></th>' + head.join('') + '</tr>');
        for (var [oppName, oppSubId] of topPlayers) {
            let result = "<tr><td>" + oppName + " (" + oppSubId + ")</td>";
            for (var ourBotName of ourBots) {
                const ourSubId = currentBots[ourBotName];
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
    for (var team of scores.teams) {
        if (team.team.teamId == '3dfa39ba-93b8-4173-92ad-51da07002f1b') {
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
        submissions.push([score, name, subid]);
    }

    submissions.sort().reverse();
    let ret: Array<[string, number]> = [];
    for (var [score, name, subid] of submissions) {
        ret.push([name, subid]);
    }
    return ret;
}

function getOurLatestBots(): [Record<string, number>, Record<number, string>, Record<number, string>] {
    const submissions = <Array<Submission>>JSON.parse(queryServer('/submissions'));
    let currentBots: Record<string, number> = {};
    let subidToBranch: Record<number, string> = {};
    let subidToCommit: Record<number, string> = {};
    submissions.reverse();
    for (var sub of submissions) {
        subidToCommit[sub.submissionId] = sub.commitHash;
        if (!sub.branchName) {
            continue;
        }
        if (sub.status != 'Succeeded') {
            continue;
        }
        subidToBranch[sub.submissionId] = sub.branchName;
        if (sub.branchName == 'bot_kimiyuki' ||
        sub.branchName == 'bot_psh_testbot' ||
        sub.branchName == 'tanakh_super_bot') {
            currentBots[sub.branchName] = sub.submissionId;
        }
    }
    return [currentBots, subidToBranch, subidToCommit];
}

function getResults(): [Record<number, string>, Record<number, Record<number, [string, number]>>, Record<number, Record<number, [string, number]>>] {
    const games = <GamesList>JSON.parse(queryServer('/games/non-rating'));
    let subidToTeamName: Record<number, string> = {};
    let resultsAtk: Record<number, Record<number, [string, number]>> = {};
    let resultsDef: Record<number, Record<number, [string, number]>> = {};
    for (var game of games.games) {
        const atkTeamName = game.attacker.team.teamName;
        const atkSubId = game.attacker.submissionId;
        const defTeamName = game.defender.team.teamName;
        const defSubId = game.defender.submissionId;

        if (game.attacker.team.teamId == '3dfa39ba-93b8-4173-92ad-51da07002f1b' && game.defender.team.teamId == '3dfa39ba-93b8-4173-92ad-51da07002f1b') {
            continue;
        }

        let myTeamSubId: number = 0;
        let oppTeamName: string = '';
        let oppSubId: number = 0;
        let mySide: string = '';
        let results: Record<number, Record<number, [string, number]>> = {};
        let playerKey: number = 0;
        if (game.attacker.team.teamId == '3dfa39ba-93b8-4173-92ad-51da07002f1b') {
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