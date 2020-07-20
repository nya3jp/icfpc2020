import {getApiKey} from './auth';

export function sendToServer(body: string): string {
    // Synchronous XHR - don't do this at home.
    const xhr = new XMLHttpRequest();
    xhr.open('POST', 'https://icfpc2020-api.testkontur.ru/aliens/send?apiKey=' + getApiKey(), false);
    xhr.setRequestHeader('Accept', '*/*');
    xhr.setRequestHeader('Content-Type', 'text/plain');
    xhr.send(body);
    if (xhr.status !== 200) {
        throw new Error(`HTTP ${xhr.status}`);
    }
    return xhr.responseText;
}

export function queryServer(path: string): string {
    // Synchronous XHR - don't do this at home.
    const xhr = new XMLHttpRequest();
    xhr.open('GET', 'https://icfpc2020-api.testkontur.ru' + path + '?apiKey=' + getApiKey(), false);
    xhr.setRequestHeader('Accept', '*/*');
    xhr.send();
    if (xhr.status !== 200) {
        throw new Error(`HTTP ${xhr.status}`);
    }
    return xhr.responseText;
}

export function queryNonRatingRuns(date: string): string {
    // Synchronous XHR - don't do this at home.
    let url: string = 'https://icfpc2020-api.testkontur.ru/games/non-rating?take=400&apiKey=' + getApiKey();
    if (date != '') {
        url += '&before=' + encodeURIComponent(date);
    }
    const xhr = new XMLHttpRequest();
    xhr.open('GET', url, false);
    xhr.setRequestHeader('Accept', '*/*');
    xhr.send();
    if (xhr.status !== 200) {
        throw new Error(`HTTP ${xhr.status}`);
    }
    return xhr.responseText;
}

export function startNonRating(subId1: number, subId2: number): string {
    // Synchronous XHR - don't do this at home.
    const xhr = new XMLHttpRequest();
    xhr.open('POST', 'https://icfpc2020-api.testkontur.ru/games/non-rating/run?apiKey=' + getApiKey() + 
        '&attackerSubmissionId=' + subId1 + '&defenderSubmissionId=' + subId2, false);
    xhr.setRequestHeader('Accept', '*/*');
    xhr.send('');
    if (xhr.status !== 200) {
        throw new Error(`HTTP ${xhr.status}`);
    }
    return xhr.responseText;
}