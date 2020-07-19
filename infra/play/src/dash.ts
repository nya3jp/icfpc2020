import { getApiKey } from "./auth";
import { queryServer } from "./utils";

const resultsElem = document.getElementById('results') as HTMLElement;
const refreshElem = document.getElementById('refresh') as HTMLElement;

function loadResults(): void {
    let [subIdToTeamName, resultsAtk, resultsDef] = getResults();
    let [currentBots, subIdToBranch] = getOurLatestBots();
    let ourBots = ['bot_kimiyuki', 'bot_psh_testbot', 'tanakh_super_bot'];

    let head = [];
    for (var name of ourBots) {
        head.push('<th>' + name + ' atk (' + currentBots[name] + ')</th>');
        head.push('<th>' + name + ' def (' + currentBots[name] + ')</th>');
    }

    let rows: Array<string> = [];
    rows.push('<tr><th></th>' + head.join('') + '</tr>');
    for (var oppSubId in subIdToTeamName) {
        const oppName = subIdToTeamName[oppSubId];
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
}

// function getOpponents(): Array([string, number]) {

// }

function getOurLatestBots(): [Record<string, number>, Record<number, string>] {
    const submissions = <Array<Submission>>JSON.parse(queryServer('/submissions'));
    let currentBots: Record<string, number> = {};
    let subidToBranch: Record<number, string> = {};
    submissions.reverse();
    for (var sub of submissions) {
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
    return [currentBots, subidToBranch];
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
    refreshElem.addEventListener('refresh', loadResults);

    if (getApiKey() != '') {
        loadResults();
    }
}

init();