import {
    Expr, PrettyData
} from './data';

export interface SendLog {
    req: PrettyData
    res: PrettyData
}

const sendLogs: Array<SendLog> = [];

export function appendSendLog(log: SendLog): void {
    sendLogs.push(log);
}

export function getSendLogs(): Array<SendLog> {
    return sendLogs.slice();
}
