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