/**
 * Copyright 2020 Google LLC
 * Copyright 2020 Team Spacecat
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
