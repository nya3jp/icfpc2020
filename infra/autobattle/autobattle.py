import argparse
import logging
import os
import subprocess
import sys
import threading
import urllib.request
import pprint
import json
import collections

MY_TEAM_ID = '3dfa39ba-93b8-4173-92ad-51da07002f1b'
VALID_BOTS = [
    # 'bot_do_nothing',
    # 'bot_simple_stay',
    'bot_kimiyuki',
    'bot_psh_testbot',
    'tanakh_super_bot',
]


def main():
    current_bots, subid_to_branch = get_our_latest_bots()
    subid_to_team_name, results_atk, results_def = get_results()
    opponents = get_opponents()[:10]
    for branch, my_subid in current_bots.items():
        for team_name, opponent_subid in opponents:
            if my_subid not in results_atk or opponent_subid not in results_atk[
                    my_subid]:
                print("Schedule: %s (%d) vs %s (%d)" %
                      (branch, my_subid, team_name, opponent_subid))
                schedule(my_subid, opponent_subid)
            else:
                print("Result: %s (%d) vs %s (%d) => %s" %
                      (branch, my_subid, team_name, opponent_subid,
                       results_atk[my_subid][opponent_subid]))

            if my_subid not in results_def or opponent_subid not in results_def[
                    my_subid]:
                print("Schedule: %s (%d) vs %s (%d)" %
                      (team_name, opponent_subid, branch, my_subid))
                schedule(opponent_subid, my_subid)
            else:
                print("Result: %s (%d) vs %s (%d) => %s" %
                      (team_name, opponent_subid, branch, my_subid,
                       results_def[my_subid][opponent_subid]))


def schedule(subid1, subid2):
    url = 'https://icfpc2020-api.testkontur.ru/games/non-rating/run?apiKey=%s&attackerSubmissionId=%d&defenderSubmissionId=%d' % (
        os.environ['ICFPC_API_KEY'], subid1, subid2)
    req = urllib.request.Request(url=url,
                                 data=''.encode('utf-8'),
                                 method='POST')
    req.add_header('Content-Type', 'text/plain')
    try:
        with urllib.request.urlopen(req) as resp:
            if resp.status != 200:
                logging.error('autobattle: non-200 response %s %s', resp.status, resp.msg)
                pprint.pprint(resp.getheaders())
                print(resp.read().decode('utf-8').strip())
                sys.exit(1)
    except urllib.error.HTTPError as ex:
        logging.error('autobattle: non-200 response %s %s', ex.code, ex.msg)
        pprint.pprint(ex.getheaders())
        print(ex.read().decode('utf-8').strip())
        raise ex


def get_opponents():
    submissions = []
    for team in query_server('/scoreboard')['teams']:
        if team['team']['teamId'] == MY_TEAM_ID:
            continue
        team_name = team['team']['teamName']
        score = team['score']
        latest = max([int(k) for k in team['tournaments'].keys()])
        subid = team['tournaments'][str(latest)]['submission']['submissionId']
        submissions.append((score, team_name, subid))

    ret = []
    for _, team_name, subid in sorted(submissions, reverse=True):
        ret.append((team_name, subid))
    return ret


def get_our_latest_bots():
    resp = query_server('/submissions')
    current_bots = dict()
    subid_to_branch = dict()
    for submission in reversed(resp):
        if "branchName" not in submission:
            continue
        if submission['status'] != 'Succeeded':
            continue
        subid_to_branch[submission['submissionId']] = submission['branchName']
        if submission["branchName"] not in VALID_BOTS:
            continue
        current_bots[submission['branchName']] = submission['submissionId']
    return (current_bots, subid_to_branch)


def get_results():
    resp = query_server('/games/non-rating')
    subid_to_team_name = dict()
    results_atk = collections.defaultdict(dict)
    results_def = collections.defaultdict(dict)
    for game in resp['games']:
        atk_team_name = game['attacker']['team']['teamName']
        atk_bot_submission_id = game['attacker']['submissionId']

        def_team_name = game['defender']['team']['teamName']
        def_bot_submission_id = game['defender']['submissionId']

        if game['attacker']['team']['teamId'] == MY_TEAM_ID:
            my_team_submission_id = atk_bot_submission_id
            opponent_team_name = def_team_name
            opponent_submission_id = def_bot_submission_id
            my_side = 'Attacker'
            results = results_atk
        else:
            my_team_submission_id = def_bot_submission_id
            opponent_team_name = atk_team_name
            opponent_submission_id = atk_bot_submission_id
            my_side = 'Defender'
            results = results_def

        subid_to_team_name[opponent_submission_id] = opponent_team_name

        if 'winner' not in game:
            result = 'Pending'
        elif game['winner'] == my_side:
            result = 'Win'
        elif game['winner'] == 'Nobody':
            result = 'Draw'
        else:
            result = 'Lose'

        results[my_team_submission_id][opponent_submission_id] = result

    return (subid_to_team_name, results_atk, results_def)


def query_server(path):
    req = urllib.request.Request(url='https://icfpc2020-api.testkontur.ru' +
                                 path + '?apiKey=' +
                                 os.environ['ICFPC_API_KEY'],
                                 method='GET')
    with urllib.request.urlopen(req) as resp:
        if resp.status != 200:
            logging.error('autobattle: non-200 response %s', resp)
            sys.exit(1)
        return json.loads(resp.read().decode('utf-8').strip())


if __name__ == "__main__":
    main()
