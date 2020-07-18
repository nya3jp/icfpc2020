const logs: Array<string> = [];

export function appendLog(log: string): void {
    logs.push(log);
}

export function getLogs(): Array<string> {
    return logs.slice();
}
