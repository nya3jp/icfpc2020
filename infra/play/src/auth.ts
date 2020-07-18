const STORAGE_NAME = 'API_KEY';

let apiKey = localStorage.getItem(STORAGE_NAME) || '';

const keyElem = document.getElementById('api_key') as HTMLInputElement;

function onApiKeyChanged(ev: Event): void {
    apiKey = keyElem.value;
    localStorage.setItem(STORAGE_NAME, apiKey);
}

export function getApiKey(): string {
    return apiKey;
}

keyElem.value = apiKey;
keyElem.addEventListener('change', onApiKeyChanged);
