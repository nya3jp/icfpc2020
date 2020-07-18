const logs: Array<string> = [];
const sendLogs: Array<[Expr, Expr]> = [];

import {
    Expr
} from './data';

export function appendLog(log: string): void {
    logs.push(log);
}

export function getLogs(): Array<string> {
    return logs.slice();
}

export function appendSendLog(req: Expr, res: Expr): void {
    sendLogs.push([req,res]);
}

export function getSendLogs(): Array<[Expr, Expr]> {
    return sendLogs.slice();
}
